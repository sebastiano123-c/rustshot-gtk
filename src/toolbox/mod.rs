mod imp;
use super::geometry::GeometryState;
use crate::toolbox_buttons::*;
use rustshot_gtk::constants::TOOLBOX_BTN_SIZE;
// mod buttons;
use crate::toolbox_bar::ToolboxBar;
use gtk::{glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct Toolbox(ObjectSubclass<imp::Toolbox>)
        @extends gtk::Box,
        @implements gtk::Accessible, gtk::Actionable, gtk::Widget,  gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for Toolbox {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl Toolbox {
    pub fn is_button_pressed(&self) -> bool {
        let imp = self.imp();
        imp.button_pressed.get()
    }

    pub fn set_button_pressed(&self, pressed: bool) {
        let imp = self.imp();
        imp.button_pressed.set(pressed);
    }

    pub fn add_widget_to_btn_list(&self, widget: ToolboxButton) -> std::io::Result<()> {
        // pub fn add_widget_to_btn_list(&self, widget: gtk::Widget) -> std::io::Result<()> {
        let imp = self.imp();
        imp.buttons_list.borrow_mut().push(widget);
        imp.n_buttons.set(imp.n_buttons.get() + 1);
        Ok(())
    }

    pub fn remove_widget_to_btn_list(&self) -> std::io::Result<()> {
        let imp = self.imp();
        imp.buttons_list.borrow_mut().pop();
        imp.n_buttons.set(imp.n_buttons.get() - 1);
        Ok(())
    }

    pub fn set_is_settings_box(&self, is_present: bool) {
        let imp = self.imp();
        imp.is_settings_box.set(is_present);
    }

    pub fn set_settings_box(&self, widget: Option<gtk::Widget>) -> std::io::Result<()> {
        let imp = self.imp();
        *imp.settings_box.borrow_mut() = widget;
        Ok(())
    }

    pub fn create_toolbox_buttons(&self, geom: &GeometryState) -> std::io::Result<()> {
        // Full Circle
        let btn = arc_button::ArcButton::default();
        btn.attach_gesture(geom);
        self.add_widget_to_btn_list(ToolboxButton::FullCircle(btn))?;
        // self.add_widget_to_btn_list(btn.upcast_ref::<gtk::Widget>().clone())?;

        // Full Box
        let btn = box_button::BoxButton::default();
        btn.attach_gesture(geom);
        self.add_widget_to_btn_list(ToolboxButton::FullBox(btn))?;

        // Arrow
        let btn = arrow_button::ArrowButton::default();
        btn.attach_gesture(geom);
        self.add_widget_to_btn_list(ToolboxButton::Arrow(btn))?;

        // Line
        let btn = line_button::LineButton::default();
        btn.attach_gesture(geom);
        self.add_widget_to_btn_list(ToolboxButton::Line(btn))?;

        // Freehand
        let btn = freehand_button::FreehandButton::default();
        btn.attach_gesture(geom);
        self.add_widget_to_btn_list(ToolboxButton::Freehand(btn))?;

        // Numbered circles
        let btn = numbered_circle_button::NumberedCircleButton::default();
        btn.attach_gesture(geom);
        self.add_widget_to_btn_list(ToolboxButton::NumberedCircles(btn))?;

        // Numbered circles
        let btn = input_text_button::InputTextButton::default();
        btn.attach_gesture(geom);
        self.add_widget_to_btn_list(ToolboxButton::InputTexts(btn))?;

        // Fullscreen
        let btn = fullscreen_button::FullscreenButton::default();
        btn.attach_gesture(geom);
        self.add_widget_to_btn_list(ToolboxButton::Fullscreen(btn))?;

        // Take screenshot
        let btn = copy_screenshot_button::CopyScreenshotButton::default();
        btn.attach_gesture(geom);
        self.add_widget_to_btn_list(ToolboxButton::TakeScreenshot(btn))?;

        // Save screenshot
        let btn = save_screenshot_button::SaveScreenshotButton::default();
        btn.attach_gesture(geom);
        self.add_widget_to_btn_list(ToolboxButton::SaveScreenshot(btn))?;

        // Record screen
        let btn = screen_recorder::ScreenRecorder::default();
        btn.attach_gesture(geom);
        self.add_widget_to_btn_list(ToolboxButton::RecordScreen(btn))?;

        Ok(())
    }

    /// Update the settings for the buttons that needs to be updated.
    fn update_settings(&self, geom: &GeometryState) -> std::io::Result<()> {
        let imp = self.imp();

        // Update the numbered_circle number
        imp.buttons_list.borrow_mut()[5].update_number(
            geom.settings
                .numbered_circle
                .get_value("number")
                .get_i32()?,
        )?;

        Ok(())
    }

    pub fn draw_toolbox(&self, geom: &GeometryState) -> std::io::Result<()> {
        let imp = self.imp();

        // Update settings if needed
        self.update_settings(geom)?;

        // Get how many toolbox bar I can fit into the four gray boxes
        // Indices: 0: bottom, 1: right, 2: top, 3: left
        let mut free_space: Vec<i32> = vec![
            geom.bottom_box.get_edge() / TOOLBOX_BTN_SIZE,
            geom.right_box.get_edge() / TOOLBOX_BTN_SIZE,
            geom.top_box.get_edge() / TOOLBOX_BTN_SIZE,
            geom.left_box.get_edge() / TOOLBOX_BTN_SIZE,
        ];

        // Get the number of buttons one can fit into each length
        // Indices: 0: horizontal, 1: vertical
        let btns_fit: Vec<usize> = vec![
            // 6,
            // 0,
            (geom.central_overlay.width() / TOOLBOX_BTN_SIZE) as usize,
            (geom.central_overlay.height() / TOOLBOX_BTN_SIZE) as usize,
        ];

        // Number of buttons left to draw
        let mut n_btns_to_draw = imp.n_buttons.get();

        // Add spacer to left gray box
        let hspacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        hspacer.set_halign(gtk::Align::Fill);
        hspacer.set_hexpand(true);
        geom.left_box.append(&hspacer);

        // Add spacer to left gray box
        let vspacer = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vspacer.set_valign(gtk::Align::Fill);
        vspacer.set_vexpand(true);
        geom.top_box.append(&vspacer);

        // Add the fullscreen case when free_space == [0,0,0,0].
        // If it was impossible to draw a toolbox in each gray box it means that the
        // margins are too small and the central box too big.
        // So we add the toolbox to the central box overlay.
        // the problem is that it creates more space on the bottom!
        if free_space == vec![0, 0, 0, 0] {
            // create toolbox
            let tb = ToolboxBar::default();
            tb.new_fullscreen(10);
            geom.central_overlay.add_overlay(&tb);
            geom.central_overlay.queue_draw();

            // Append buttons
            for ii in imp.buttons_list.borrow().iter() {
                tb.append(ii.as_ref());
            }

            let settings_box = imp.settings_box.borrow().clone();
            if let Some(settings) = settings_box {
                // Create horizontal toolbar
                let tb = ToolboxBar::default();
                tb.new_fullscreen(10 + TOOLBOX_BTN_SIZE);
                geom.central_overlay.add_overlay(&tb);
                geom.central_overlay.queue_draw();

                // estimate the number of buttons to draw
                tb.append(&settings);
            }

            return Ok(());
        }

        // Draw toolboxes
        let mut order_idx = 0;
        while n_btns_to_draw != 0 {
            order_idx += 1;
            if order_idx > 10 {
                break;
            }

            // Fill the bottom box
            if free_space[0] != 0 && n_btns_to_draw != 0 {
                // Create horizontal toolbar
                let tb = ToolboxBar::default();
                tb.new_horizontal(gtk::Align::Start, geom);
                geom.bottom_box.append(&tb);

                // estimate the number of buttons to draw
                let start_idx = imp.n_buttons.get() - n_btns_to_draw;
                let end_idx = start_idx + btns_fit[0].min(n_btns_to_draw);
                for ii in start_idx..end_idx {
                    tb.append(imp.buttons_list.borrow()[ii].as_ref());
                    n_btns_to_draw -= 1;
                }

                // Decrease the number of possible toolbars in this graybox
                free_space[0] -= 1;
            }

            // Fill the right box
            if free_space[1] != 0 && n_btns_to_draw != 0 {
                let tb = ToolboxBar::default();
                tb.new_vertical(TOOLBOX_BTN_SIZE, gtk::Align::Start);
                geom.right_box.append(&tb);

                // estimate the number of buttons to draw
                let start_idx = imp.n_buttons.get() - n_btns_to_draw;
                let end_idx = start_idx + btns_fit[1].min(n_btns_to_draw);
                for ii in start_idx..end_idx {
                    tb.append(imp.buttons_list.borrow()[ii].as_ref());
                    n_btns_to_draw -= 1;
                }

                // Decrease the number of possible toolbars in this graybox
                free_space[1] -= 1;
            }

            // Fill the top box
            if free_space[2] != 0 && n_btns_to_draw != 0 {
                // Create horizontal toolbar
                let tb = ToolboxBar::default();
                tb.new_horizontal(gtk::Align::End, geom);
                geom.top_box.append(&tb);

                // estimate the number of buttons to draw
                let start_idx = imp.n_buttons.get() - n_btns_to_draw;
                let end_idx = start_idx + btns_fit[0].min(n_btns_to_draw);
                for ii in start_idx..end_idx {
                    tb.append(imp.buttons_list.borrow()[ii].as_ref());
                    n_btns_to_draw -= 1;
                }

                // Decrease the number of possible toolbars in this graybox
                free_space[2] -= 1;
            }

            // Fill the left box
            if free_space[3] != 0 && n_btns_to_draw != 0 {
                let tb = ToolboxBar::default();
                tb.new_vertical(TOOLBOX_BTN_SIZE, gtk::Align::End);
                geom.left_box.append(&tb);

                // estimate the number of buttons to draw
                let start_idx = imp.n_buttons.get() - n_btns_to_draw;
                let end_idx = start_idx + btns_fit[1].min(n_btns_to_draw);
                for ii in start_idx..end_idx {
                    tb.append(imp.buttons_list.borrow()[ii].as_ref());
                    n_btns_to_draw -= 1;
                }

                // Decrease the number of possible toolbars in this graybox
                free_space[3] -= 1;
            }
        }

        let settings_box = imp.settings_box.borrow().clone();
        if let Some(settings) = settings_box {
            // Fill the bottom box
            if free_space[0] != 0 {
                // Create horizontal toolbar
                let tb = ToolboxBar::default();
                tb.new_settings(gtk::Align::Start, geom);
                geom.bottom_box.append(&tb);

                // estimate the number of buttons to draw
                tb.append(&settings);

                // Decrease the number of possible toolbars in this graybox
                free_space[0] -= 1;
                return Ok(());
            }

            // Fill the top box
            if free_space[2] != 0 {
                // Create horizontal toolbar
                let tb = ToolboxBar::default();
                tb.new_settings(gtk::Align::End, geom);
                geom.top_box.append(&tb);

                // estimate the number of buttons to draw
                tb.append(&settings);

                // Decrease the number of possible toolbars in this graybox
                free_space[2] -= 1;
                return Ok(());
            }
        }

        Ok(())
    }

    pub fn remove_css_class(&self, class: &str) {
        let imp = self.imp();
        //  every button from their parents before attaching them again
        for btn in &*imp.buttons_list.borrow() {
            btn.as_ref().remove_css_class(class);
        }
    }

    pub fn stop_toolbox(&self, geom: &GeometryState) {
        let imp = self.imp();
        // -----------------------------------------------------------------
        // 1️⃣  Remove every button from the box that currently owns it.
        // -----------------------------------------------------------------
        for btn in imp.buttons_list.borrow().iter() {
            let widget: &gtk::Widget = btn.as_ref();

            // `btn.parent()` returns an `Option<Widget>`.  If it exists and is a
            // `gtk::Box`, we can safely call `remove`.
            if let Some(parent) = widget.parent() {
                // -----------------------------------------------------------------
                // 4️⃣  Try to down‑cast the parent to a `gtk::Box`.
                // -----------------------------------------------------------------
                if let Ok(parent_box) = parent.clone().downcast::<gtk::Box>() {
                    // -----------------------------------------------------------------
                    // 5️⃣  Remove the *inner* widget from the box.
                    // -----------------------------------------------------------------
                    // `remove` wants a reference to something that implements
                    // `IsA<gtk::Widget>`.  `widget` (the inner widget) satisfies
                    // that requirement.
                    parent_box.remove(widget);
                } else {
                    // Parent exists but is not a Box – useful for debugging.
                    eprintln!("toolbox::stop_toolbox – parent of a button is not a gtk::Box");
                }
            } else {
                // Button has no parent at all (already removed, or never added).
                eprintln!("toolbox::stop_toolbox – button had no parent widget");
            }
        }

        // if there is settings box remove the toolbox
        if let Some(settings) = imp.settings_box.borrow().clone() {
            let widget: &gtk::Widget = &settings;

            // `btn.parent()` returns an `Option<Widget>`.  If it exists and is a
            // `gtk::Box`, we can safely call `remove`.
            if let Some(parent) = widget.parent() {
                // -----------------------------------------------------------------
                // 4️⃣  Try to down‑cast the parent to a `gtk::Box`.
                // -----------------------------------------------------------------
                if let Ok(parent_box) = parent.clone().downcast::<gtk::Box>() {
                    // -----------------------------------------------------------------
                    // 5️⃣  Remove the *inner* widget from the box.
                    // -----------------------------------------------------------------
                    // `remove` wants a reference to something that implements
                    // `IsA<gtk::Widget>`.  `widget` (the inner widget) satisfies
                    // that requirement.
                    parent_box.remove(widget);
                } else {
                    // Parent exists but is not a Box – useful for debugging.
                    eprintln!(
                        "toolbox::stop_toolbox settings_box parent of a button is not a gtk::Box"
                    );
                }
            } else {
                // Button has no parent at all (already removed, or never added).
                eprintln!("Warning. toolbox::stop_toolbox – settings_box had no parent widget");
            }
        }

        // -----------------------------------------------------------------
        // 2️⃣  Remove the central overlay *only* when its deepest child is a
        //     `gtk::Button`.
        // -----------------------------------------------------------------
        if let Some(overlay) = geom.central_overlay.last_child()
            && let Some(inner) = overlay.last_child()
            && inner.type_() == gtk::Button::static_type()
        {
            geom.central_overlay.remove_overlay(&overlay);
        }

        // -----------------------------------------------------------------
        // 3️⃣  Drain the four gray‑box containers with a tiny helper.
        // -----------------------------------------------------------------
        geom.top_box.remove_childs();
        geom.left_box.remove_childs();
        geom.bottom_box.remove_childs();
        geom.right_box.remove_childs();
    }
}

// pub fn draw_toolbox_1(&self, geom: &GeometryState) {
//     let imp = self.imp();
//
//     // self.estimate_free_space(&geom);
//
//     // number of buttons left to draw
//     let mut n_btn_left_to_draw: usize = imp.n_buttons.get();
//
//     // counter that accounts the gray box in which the toolbox is placed
//     let mut gray_box_idx = 0;
//
//     // counter that accounts the order of the toolbox.
//     // If the gray box has only one toolbox the order is 1;
//     // If the gray box has two toolboxes the order is 2, and so on...
//     let mut order_idx = 1;
//
//     // if it equals 4 then no space left. Thus draw on the central box
//     let mut no_space_left: u8 = 0;
//
//     // go ahead until there are no buttons to place
//     let mut attempts = 0;
//     while n_btn_left_to_draw != 0 {
//         // just to be sure that the while does not go on forever
//         attempts += 1;
//         if attempts > 1000 {
//             break;
//         }
//
//         // if it was impossible to draw a toolbox in each gray box it means that the
//         // margins are too small and the central box too big.
//         // So we add the toolbox to the central box overlay.
//         // the problem is that it creates more space on the bottom!
//         if no_space_left >= 4 && order_idx == 2 {
//             // create toolbox
//             let tb = ToolboxBar::default();
//             tb.new_fullscreen(10);
//             geom.central_overlay.add_overlay(&tb);
//             geom.central_overlay.queue_draw();
//             tb.fill(
//                 geom.central_overlay.width(),
//                 imp.n_buttons.get(),
//                 &imp.buttons_list.borrow(),
//                 &mut n_btn_left_to_draw,
//                 imp.is_settings_box.get(),
//                 true,
//             );
//         }
//
//         // select gray box
//         match gray_box_idx {
//             // Attach toolbox to bottom gray box
//             0 => {
//                 // else increment counter, so that next iteration goes in the next match case
//                 gray_box_idx += 1;
//
//                 // check if there is room to place the toolbox bar
//                 if order_idx * TOOLBOX_BTN_SIZE > geom.bottom_box.get_edge() {
//                     // add 1 to no space left value
//                     no_space_left += 1;
//
//                     // if not we need to go to the next iteration
//                     continue;
//                 }
//
//                 // Create horizontal toolbar
//                 let tb = ToolboxBar::default();
//                 tb.new_horizontal(gtk::Align::Start, geom);
//                 geom.bottom_box.append(&tb);
//
//                 // insert as many buttons as possible
//                 let l = std::cmp::max(geom.central_overlay.width(), TOOLBOX_BTN_SIZE + 10);
//
//                 // Fill toolbar with buttons
//                 tb.fill(
//                     l,
//                     imp.n_buttons.get(),
//                     &imp.buttons_list.borrow(),
//                     &mut n_btn_left_to_draw,
//                     imp.is_settings_box.get(),
//                     true,
//                 );
//
//                 // set space left to zero
//                 no_space_left = 0;
//             }
//             // Attach toolbox to left gray box
//             1 => {
//                 // check if there is room to place the toolbox
//                 if order_idx * TOOLBOX_BTN_SIZE > geom.left_box.get_edge() {
//                     // else increment counter
//                     gray_box_idx += 1;
//
//                     // add 1 to no space left value
//                     no_space_left += 1;
//
//                     // if not we need to go to the next iteration
//                     continue;
//                 }
//
//                 // the toolbox here aligns with the left otherwise, KEEP THIS LINE
//                 if order_idx == 1 {
//                     let space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
//                     space.set_width_request(geom.left_box.get_edge() - TOOLBOX_BTN_SIZE);
//                     geom.left_box.append(&space);
//                 } else if let Some(space) = geom.left_box.first_child() {
//                     space.set_width_request(
//                         geom.left_box.get_edge() - TOOLBOX_BTN_SIZE * order_idx,
//                     );
//                 }
//
//                 // create toolbox
//                 let tb = ToolboxBar::default();
//                 tb.new_vertical(TOOLBOX_BTN_SIZE, gtk::Align::End);
//                 geom.left_box.append(&tb);
//
//                 // insert as many buttons as possible
//                 tb.fill(
//                     geom.central_overlay.height(),
//                     imp.n_buttons.get(),
//                     &imp.buttons_list.borrow(),
//                     &mut n_btn_left_to_draw,
//                     imp.is_settings_box.get(),
//                     false,
//                 );
//
//                 // set space left to zero
//                 no_space_left = 0;
//
//                 // increment counter
//                 gray_box_idx += 1;
//             }
//             // Attach toolbox to top gray box
//             2 => {
//                 // else increment counter
//                 gray_box_idx += 1;
//
//                 // check if there is room to place the toolbox
//                 if order_idx * TOOLBOX_BTN_SIZE > geom.top_box.get_edge() {
//                     // add 1 to no space left value
//                     no_space_left += 1;
//
//                     // if not we need to go to the next iteration
//                     continue;
//                 }
//
//                 // the toolbox here aligns with the left otherwise, KEEP THIS LINE
//                 if order_idx == 1 {
//                     let space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
//                     space.set_height_request(geom.top_box.get_edge() - TOOLBOX_BTN_SIZE);
//                     geom.top_box.append(&space);
//                 } else if let Some(space) = geom.top_box.first_child() {
//                     space.set_height_request(
//                         geom.top_box.get_edge() - TOOLBOX_BTN_SIZE * order_idx,
//                     );
//                 }
//
//                 // Create horizontal toolbar
//                 let tb = ToolboxBar::default();
//                 tb.new_horizontal(gtk::Align::End, geom);
//                 geom.top_box.append(&tb);
//
//                 // insert as many buttons as possible
//                 let l = std::cmp::max(geom.central_overlay.width(), TOOLBOX_BTN_SIZE + 10);
//
//                 // Fill toolbar with buttons
//                 tb.fill(
//                     l,
//                     imp.n_buttons.get(),
//                     &imp.buttons_list.borrow(),
//                     &mut n_btn_left_to_draw,
//                     imp.is_settings_box.get(),
//                     true,
//                 );
//
//                 // set space left to zero
//                 no_space_left = 0;
//             }
//             // Attach toolbox to right gray box
//             3 => {
//                 // check if there is room to place the toolbox
//                 if order_idx * TOOLBOX_BTN_SIZE > geom.right_box.get_edge() {
//                     // else increment counter
//                     gray_box_idx = 0;
//
//                     // increment order
//                     order_idx += 1;
//
//                     // add 1 to no space right value
//                     no_space_left += 1;
//
//                     // if not we need to go to the next iteration
//                     continue;
//                 }
//
//                 // create toolbox
//                 let tb = ToolboxBar::default();
//                 tb.new_vertical(TOOLBOX_BTN_SIZE, gtk::Align::Start);
//                 geom.right_box.append(&tb);
//
//                 // insert as many buttons as possible
//                 // Fill toolbar with buttons
//                 tb.fill(
//                     geom.central_overlay.height(),
//                     imp.n_buttons.get(),
//                     &imp.buttons_list.borrow(),
//                     &mut n_btn_left_to_draw,
//                     imp.is_settings_box.get(),
//                     false,
//                 );
//
//                 // if the counter is 3 it means we need to reset it for the next iteration
//                 gray_box_idx = 0;
//
//                 // set space left to zero
//                 no_space_left = 0;
//
//                 // we need also to increment the order
//                 order_idx += 1;
//             }
//             _ => {
//                 println!("The loop should not pass here");
//             }
//         }
//     }
//     // println!("order_idx: {}, gray_box_idx: {}", order_idx, gray_box_idx);
//     // self.draw_settings_box(&geom, order_idx);
// }

// fn draw_settings_box(&self, geom: &GeometryState, btns_order_idx: i32) {
//     let imp = self.imp();
//
//     // Is there any settings box?
//     let settings = imp.settings_box.borrow().clone();
//     if let Some(settings_box) = settings {
//         // check if there is room to place the toolbox
//         if (btns_order_idx + 1) * TOOLBOX_BTN_SIZE <= geom.bottom_box.get_edge() {
//             // Is there any settings box?
//             // Create horizontal toolbar
//             let tb = ToolboxBar::default();
//             tb.new_horizontal(gtk::Align::Start, geom);
//             geom.bottom_box.append(&tb);
//             tb.fill_with_settings(settings_box);
//             return;
//         }
//
//         // check if there is room to place the toolbox
//         if (btns_order_idx + 1) * TOOLBOX_BTN_SIZE <= geom.top_box.get_edge() {
//             // // the toolbox here aligns with the left otherwise, KEEP THIS LINE
//             // if btns_order_idx == 1 {
//             //     let space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
//             //     space.set_height_request(geom.top_box.get_edge() - TOOLBOX_BTN_SIZE);
//             //     geom.top_box.append(&space);
//             // } else
//             if let Some(space) = geom.top_box.first_child() {
//                 space.set_height_request(geom.top_box.get_edge() - TOOLBOX_BTN_SIZE);
//             }
//
//             // Create horizontal toolbar
//             let tb = ToolboxBar::default();
//             tb.new_horizontal(gtk::Align::End, geom);
//             geom.top_box.append(&tb);
//             tb.fill_with_settings(settings_box);
//             return;
//         }
//
//         // if it was impossible to draw a toolbox in each gray box it means that the
//         // margins are too small and the central box too big.
//         // So we add the toolbox to the central box overlay.
//         // the problem is that it creates more space on the bottom!
//         // Create horizontal toolbar
//         let tb = ToolboxBar::default();
//         tb.new_fullscreen(geom);
//         geom.central_overlay.add_overlay(&tb);
//         geom.central_overlay.queue_draw();
//         tb.fill_with_settings(settings_box);
//     } else {
//         println!("No settings to show");
//     }
// }
