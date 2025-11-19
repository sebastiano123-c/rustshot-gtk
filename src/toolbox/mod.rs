mod imp;
use super::geometry::GeometryState;
// use crate::toolbox::buttons::{
//     ArcButton, ArcButtonNoFill, ArrowButton, BoxButton, BoxButtonNoFill, ColorButton,
//     CopyScreenshotButton, FreeHandButton, FullScreenButton, LineButton, NumberedCircleButton,
//     SaveScreenshotButton, SettingsButton,
// };
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

    pub fn create_toolbox_buttons(&self, geom: &GeometryState) {
        let imp = self.imp();

        // Full Circle
        let btn = arc_button::ArcButton::default();
        btn.attach_gesture(geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);

        // Circle
        let btn = arc_no_fill_button::ArcNoFillButton::default();
        btn.attach_gesture(geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);

        // Full Box
        let btn = box_button::BoxButton::default();
        btn.attach_gesture(geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);

        // Box
        let btn = box_no_fill_button::BoxNoFillButton::default();
        btn.attach_gesture(geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);

        // Arrow
        let btn = arrow_button::ArrowButton::default();
        btn.attach_gesture(&geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);

        // Line
        let btn = line_button::LineButton::default();
        btn.attach_gesture(&geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);

        // Freehand
        let btn = freehand_button::FreehandButton::default();
        btn.attach_gesture(&geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);

        // Numbered circles
        let btn = numbered_circle_button::NumberedCircleButton::default();
        btn.attach_gesture(&geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);

        // Colors
        let btn = color_button::ColorButton::default();
        btn.attach_gesture(&geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);

        // Fullscreen
        let btn = fullscreen_button::FullscreenButton::default();
        btn.attach_gesture(&geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);

        // Take screenshot
        let btn = copy_screenshot_button::CopyScreenshotButton::default();
        btn.attach_gesture(&geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);

        // Save screenshot
        let btn = save_screenshot_button::SaveScreenshotButton::default();
        btn.attach_gesture(&geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);

        // Record screen
        let btn = screen_recorder::ScreenRecorder::default();
        btn.attach_gesture(&geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);

        // Settings
        let btn = settings_button::SettingsButton::default();
        btn.attach_gesture(&geom);
        imp.buttons_list
            .borrow_mut()
            .push(btn.upcast_ref::<gtk::Widget>().clone());
        imp.n_buttons.set(imp.n_buttons.get() + 1);
    }

    pub fn set_button_pressed(&self, pressed: bool) {
        let imp = self.imp();
        imp.button_pressed.set(pressed);
    }

    pub fn draw_toolbox(&self, geom: &GeometryState) {
        let imp = self.imp();

        // number of buttons left to draw
        let mut n_btn_left_to_draw: usize = imp.n_buttons.get();

        // counter that accounts the gray box in which the toolbox is placed
        let mut gray_box_idx = 0;

        // counter that accounts the order of the toolbox.
        // If the gray box has only one toolbox the order is 1;
        // If the gray box has two toolboxes the order is 2, and so on...
        let mut order_idx = 1;

        // if it equals 4 then no space left. Thus draw on the central box
        let mut no_space_left: u8 = 0;

        // go ahead until there are no buttons to place
        let mut attempts = 0;
        while n_btn_left_to_draw != 0 {
            // just to be sure that the while does not go on forever
            attempts += 1;
            if attempts > 1000 {
                break;
            }

            // if it was impossible to draw a toolbox in each gray box it means that the
            // margins are too small and the central box too big.
            // So we add the toolbox to the central box overlay.
            // the problem is that it creates more space on the bottom!
            if no_space_left >= 4 && order_idx == 2 {
                // create toolbox
                let tb = ToolboxBar::default();
                tb.new_fullscreen(&geom);
                geom.central_overlay.add_overlay(&tb);
                geom.central_overlay.queue_draw();
                tb.fill_with_buttons(
                    geom.central_overlay.width(),
                    imp.n_buttons.get(),
                    &imp.buttons_list.borrow(),
                    &mut n_btn_left_to_draw,
                );
            }

            // select gray box
            match gray_box_idx {
                // Attach toolbox to bottom gray box
                0 => {
                    // check if there is room to place the toolbox
                    if order_idx * TOOLBOX_BTN_SIZE > geom.bottom_box.get_edge() {
                        // else increment counter
                        gray_box_idx += 1;

                        // add 1 to no space left value
                        no_space_left += 1;

                        // if not we need to go to the next iteration
                        continue;
                    }

                    // Create horizontal toolbar
                    let tb = ToolboxBar::default();
                    tb.new_horizontal(gtk::Align::Start, &geom);
                    geom.bottom_box.append(&tb);

                    // insert as many buttons as possible
                    let l = std::cmp::max(geom.central_overlay.width(), TOOLBOX_BTN_SIZE + 10);

                    // Fill toolbar with buttons
                    tb.fill_with_buttons(
                        l,
                        imp.n_buttons.get(),
                        &imp.buttons_list.borrow(),
                        &mut n_btn_left_to_draw,
                    );

                    // set space left to zero
                    no_space_left = 0;

                    // increment counter
                    gray_box_idx += 1;
                }
                // Attach toolbox to left gray box
                1 => {
                    // check if there is room to place the toolbox
                    if order_idx * TOOLBOX_BTN_SIZE > geom.left_box.get_edge() {
                        // else increment counter
                        gray_box_idx += 1;

                        // add 1 to no space left value
                        no_space_left += 1;

                        // if not we need to go to the next iteration
                        continue;
                    }

                    // the toolbox here aligns with the left otherwise, KEEP THIS LINE
                    if order_idx == 1 {
                        let space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                        space.set_width_request(geom.left_box.get_edge() - TOOLBOX_BTN_SIZE);
                        geom.left_box.append(&space);
                    } else if let Some(space) = geom.left_box.first_child() {
                        space.set_width_request(
                            geom.left_box.get_edge() - TOOLBOX_BTN_SIZE * order_idx,
                        );
                    }

                    // create toolbox
                    let tb = ToolboxBar::default();
                    tb.new_vertical(TOOLBOX_BTN_SIZE, gtk::Align::End);
                    geom.left_box.append(&tb);

                    // insert as many buttons as possible
                    tb.fill_with_buttons(
                        geom.central_overlay.height(),
                        imp.n_buttons.get(),
                        &imp.buttons_list.borrow(),
                        &mut n_btn_left_to_draw,
                    );

                    // set space left to zero
                    no_space_left = 0;

                    // increment counter
                    gray_box_idx += 1;
                }
                // Attach toolbox to top gray box
                2 => {
                    // check if there is room to place the toolbox
                    if order_idx * TOOLBOX_BTN_SIZE > geom.top_box.get_edge() {
                        // else increment counter
                        gray_box_idx += 1;

                        // add 1 to no space left value
                        no_space_left += 1;

                        // if not we need to go to the next iteration
                        continue;
                    }

                    // the toolbox here aligns with the left otherwise, KEEP THIS LINE
                    if order_idx == 1 {
                        let space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                        space.set_height_request(geom.top_box.get_edge() - TOOLBOX_BTN_SIZE);
                        geom.top_box.append(&space);
                    } else if let Some(space) = geom.top_box.first_child() {
                        space.set_height_request(
                            geom.top_box.get_edge() - TOOLBOX_BTN_SIZE * order_idx,
                        );
                    }

                    // Create horizontal toolbar
                    let tb = ToolboxBar::default();
                    tb.new_horizontal(gtk::Align::End, &geom);
                    geom.top_box.append(&tb);

                    // insert as many buttons as possible
                    let l = std::cmp::max(geom.central_overlay.width(), TOOLBOX_BTN_SIZE + 10);

                    // Fill toolbar with buttons
                    tb.fill_with_buttons(
                        l,
                        imp.n_buttons.get(),
                        &imp.buttons_list.borrow(),
                        &mut n_btn_left_to_draw,
                    );

                    // set space left to zero
                    no_space_left = 0;

                    // increment counter
                    gray_box_idx += 1;
                }
                // Attach toolbox to right gray box
                3 => {
                    // check if there is room to place the toolbox
                    if order_idx * TOOLBOX_BTN_SIZE > geom.right_box.get_edge() {
                        // else increment counter
                        gray_box_idx = 0;

                        // increment order
                        order_idx += 1;

                        // add 1 to no space right value
                        no_space_left += 1;

                        // if not we need to go to the next iteration
                        continue;
                    }

                    // create toolbox
                    let tb = ToolboxBar::default();
                    tb.new_vertical(TOOLBOX_BTN_SIZE, gtk::Align::Start);
                    geom.right_box.append(&tb);

                    // insert as many buttons as possible
                    // Fill toolbar with buttons
                    tb.fill_with_buttons(
                        geom.central_overlay.height(),
                        imp.n_buttons.get(),
                        &imp.buttons_list.borrow(),
                        &mut n_btn_left_to_draw,
                    );

                    // if the counter is 3 it means we need to reset it for the next iteration
                    gray_box_idx = 0;

                    // set space left to zero
                    no_space_left = 0;

                    // we need also to increment the order
                    order_idx += 1;
                }
                _ => {
                    println!("The loop should not pass here");
                }
            }
        }
    }

    pub fn remove_css_class(&self, class: &str) {
        let imp = self.imp();
        //  every button from their parents before attaching them again
        for btn in &*imp.buttons_list.borrow() {
            btn.remove_css_class(class);
        }
    }

    pub fn stop_toolbox(&self, geom: &GeometryState) {
        let imp = self.imp();
        // -----------------------------------------------------------------
        // 1️⃣  Remove every button from the box that currently owns it.
        // -----------------------------------------------------------------
        for btn in imp.buttons_list.borrow().iter() {
            let widget: &gtk::Widget = btn;

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
