use super::drawing_area_manager::DrawingAreaManager;
use super::handles::Handles;
use super::toolbox::Toolbox;
use gtk::prelude::*;
use gtk::{gdk, gio, glib};
use std::cell::Cell;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

#[derive(Debug)]
pub struct RustshotGui {
    // full screen sizes
    full_w: f64,
    full_h: f64,
    // main window
    window: gtk::ApplicationWindow,
    drawing: gtk::DrawingArea,
    screenshot_layout: gtk::Box,
    // margins
    top: Rc<Cell<f64>>,
    bottom: Rc<Cell<f64>>,
    left: Rc<Cell<f64>>,
    right: Rc<Cell<f64>>,
    // boxes
    top_box: gtk::Box,
    bottom_box: gtk::Box,
    left_box: gtk::Box,
    right_box: gtk::Box,
    screenshot_box: gtk::Box,
    // screenshot box start and size
    x: Rc<Cell<f64>>,
    y: Rc<Cell<f64>>,
    w: Rc<Cell<f64>>,
    h: Rc<Cell<f64>>,
    // toolbox buttons
    toolbox: Toolbox,
    copy_to_clipboard: gtk::Button,
    fullscreen: gtk::Button,
    save_to_file: gtk::Button,
    add_arc_fill: gtk::Button,
    add_arc_no_fill: gtk::Button,
    add_rect_fill: gtk::Button,
    add_rect_no_fill: gtk::Button,
    add_arrow: gtk::Button,
    add_line: gtk::Button,
    add_freehand: gtk::Button,
    add_numbered_circles: gtk::Button,
    change_color: gtk::Button,
    draw_manager: Rc<RefCell<DrawingAreaManager>>,
    // stretch handles
    handles: Rc<RefCell<Handles>>,
    // current color
    red: Rc<Cell<f64>>,
    green: Rc<Cell<f64>>,
    blue: Rc<Cell<f64>>,
    alpha: Rc<Cell<f64>>,
}

impl RustshotGui {
    pub fn new(app: &gtk::Application) -> Self {
        // set css style
        Self::set_css();

        // get screen size
        // let (w, h): (i32, i32) = (1000, 1000);
        let (w, h): (i32, i32) = Self::get_monitor_size();
        let (full_w_f64, full_h_f64): (f64, f64) = (w as f64, h as f64);

        // create margins
        let t = Rc::new(Cell::new(full_h_f64 / 2.0));
        let b = Rc::new(Cell::new(full_h_f64 / 2.0));
        let r = Rc::new(Cell::new(full_w_f64 / 2.0));
        let l = Rc::new(Cell::new(full_w_f64 / 2.0));

        // create window object
        let win = gtk::ApplicationWindow::new(app);
        win.set_default_size(w, h);

        // create main layout
        let overlay = gtk::Overlay::new();
        win.set_child(Some(&overlay));

        // create drawing area
        let draw = gtk::DrawingArea::new();
        overlay.add_overlay(&draw);

        // screenshot boxes layout
        let layout: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        overlay.add_overlay(&layout);

        // top box
        let top_b: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        top_b.add_css_class("gray-box");
        top_b.set_height_request(h / 2);
        layout.append(&top_b);

        // Central box contains left and right box
        let central_b: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        central_b.set_vexpand(true);
        central_b.set_hexpand(true);
        // central_b.set_halign(gtk::Align::End);
        layout.append(&central_b);

        // Left box
        let left_b: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        left_b.add_css_class("gray-box");
        // leave gtk::Align::Start otherwise will get rendering errors
        // in particular, the left_box will float horizontally.
        left_b.set_halign(gtk::Align::Start);
        central_b.append(&left_b);

        // spacer (or screenshot box)
        let handle_size_px = 10;
        let ov = gtk::Overlay::new();
        ov.set_hexpand(true);
        let spacer = gtk::Box::new(gtk::Orientation::Vertical, 0);
        spacer.set_hexpand(true);
        ov.add_overlay(&spacer);
        central_b.append(&ov);

        // right box
        let right_b: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        right_b.add_css_class("gray-box");
        right_b.set_halign(gtk::Align::End);
        // right_b.set_width_request((full_w_f64 / 3.0) as i32);
        central_b.append(&right_b);

        // bottom box
        let bottom_b: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        bottom_b.add_css_class("gray-box");
        bottom_b.set_height_request(h / 2);
        layout.append(&bottom_b);

        // define boxes
        let (red, green, blue, alpha) = (1.0, 0.0, 0.0, 1.0);
        let boxes = DrawingAreaManager::new();

        // add toolbox buttons
        let tb_btn_s: i32 = 50;
        let mut tb = Toolbox::new(
            tb_btn_s,
            ov.clone(),
            top_b.clone(),
            left_b.clone(),
            bottom_b.clone(),
            right_b.clone(),
        ); //, b.clone(), r.clone());
        let fullscree_btn = tb.create_toolbox_button("\u{f424}", Some("Set fullscreen"));
        let add_box_no_fill_btn =
            tb.create_toolbox_button("\u{f096}", Some("Draw rectangle without fill"));
        let add_rect_btn = tb.create_toolbox_button("\u{f096}", Some("Draw filled rectangle")); //f04d
        add_rect_btn.add_css_class("fas");
        let add_arc_no_fill_btn =
            tb.create_toolbox_button("\u{f111}", Some("Draw circle without fill"));
        let add_arc_btn = tb.create_toolbox_button("\u{f111}", Some("Draw filled circle")); //f04d
        add_arc_btn.add_css_class("fas");
        let add_arrow_btn = tb.create_toolbox_button("\u{f061}", Some("Draw arrow"));
        let add_line_btn = tb.create_toolbox_button("\u{f068}", Some("Draw line"));
        let add_freehand_btn = tb.create_toolbox_button("\u{f1fc}", Some("Freehand draw"));
        let add_num_circ_btn = tb.create_toolbox_button("\u{f06a}", Some("Add numbered circles"));
        let pick_color_btn = tb.create_toolbox_button("\u{f53f}", Some("Pick color"));
        let copy_clipboard_btn = tb.create_toolbox_button("\u{f328}", Some(r#"Copy to clipboard"#)); // f24d, f030
        let save_to_file_btn = tb.create_toolbox_button("\u{f0c7}", Some(r#"Save image"#));

        // screenshot box start and size
        let sx = Rc::new(Cell::new(0.0));
        let sy = Rc::new(Cell::new(0.0));
        let sw = Rc::new(Cell::new(0.0));
        let sh = Rc::new(Cell::new(0.0));

        // create handles
        let handles = Handles::new(
            t.clone(),
            b.clone(),
            l.clone(),
            r.clone(),
            top_b.clone(),
            bottom_b.clone(),
            left_b.clone(),
            right_b.clone(),
            spacer.clone(),
            sx.clone(),
            sy.clone(),
            sw.clone(),
            sh.clone(),
            handle_size_px,
            full_w_f64,
            full_h_f64,
            tb.clone(),
        );

        win.present();

        Self {
            // full screen sizes
            full_w: full_w_f64,
            full_h: full_h_f64,
            // main window
            window: win,
            drawing: draw,
            screenshot_layout: layout,
            // margins
            top: t,
            bottom: b,
            left: l,
            right: r,
            // boxes
            top_box: top_b,
            bottom_box: bottom_b,
            left_box: left_b,
            right_box: right_b,
            screenshot_box: spacer,
            // screenshot box start and size
            x: sx,
            y: sy,
            w: sw,
            h: sh,
            // toolbox buttons
            toolbox: tb,
            copy_to_clipboard: copy_clipboard_btn,
            fullscreen: fullscree_btn,
            save_to_file: save_to_file_btn,
            add_rect_fill: add_rect_btn,
            add_rect_no_fill: add_box_no_fill_btn,
            add_arc_fill: add_arc_btn,
            add_arc_no_fill: add_arc_no_fill_btn,
            add_arrow: add_arrow_btn,
            add_line: add_line_btn,
            add_freehand: add_freehand_btn,
            add_numbered_circles: add_num_circ_btn,
            change_color: pick_color_btn,
            draw_manager: Rc::new(RefCell::new(boxes)),
            handles: Rc::new(RefCell::new(handles)),
            // color
            red: Rc::new(Cell::new(red)),
            green: Rc::new(Cell::new(green)),
            blue: Rc::new(Cell::new(blue)),
            alpha: Rc::new(Cell::new(alpha)),
        }
    }

    pub fn build_ui(&self) {
        ////////////////////////////////////////////////
        // Gesture to create the screenshot
        ////////////////////////////////////////////////
        self.gesture_screenshot_box();

        ////////////////////////////////////////////////
        // Gesture to create the toolbox
        ////////////////////////////////////////////////
        self.gesture_toolbox_buttons();
    }

    fn gesture_toolbox_buttons(&self) {
        // - create a toolbox to append icons (now they are too big)
        // - create a dynamic way to handle icons (change position around the screenshot box)
        // - create gesture for each button

        // clone gray boxes
        let window = self.window.clone();
        let top_box = self.top_box.clone();
        let left_box = self.left_box.clone();
        let right_box = self.right_box.clone();
        let bottom_box = self.bottom_box.clone();
        let screenshot_box = self.screenshot_box.clone();
        let drawing = self.drawing.clone();
        let full_w = self.full_w;
        let full_h = self.full_h;
        let top = self.top.clone();
        let right = self.right.clone();
        let left = self.left.clone();
        let bottom = self.bottom.clone();

        // clone toolbox
        let toolbox = Rc::new(RefCell::new(self.toolbox.clone()));
        let boxes = self.draw_manager.clone();
        let handles = self.handles.clone();

        // clone color
        let (red, green, blue, alpha) = (
            self.red.clone(),
            self.green.clone(),
            self.blue.clone(),
            self.alpha.clone(),
        );

        // condition where button is clicked
        let pressed = Rc::new(Cell::new(false));

        ////////////////////////////////////////////////
        // Gesture exit window
        ////////////////////////////////////////////////
        let keyboard_ctrl = gtk::EventControllerKey::new();
        window.add_controller(keyboard_ctrl.clone());
        keyboard_ctrl.connect_key_pressed({
            let window = self.window.clone();
            let pressed = Rc::clone(&pressed);
            let handles = Rc::clone(&handles);
            let boxes = Rc::clone(&boxes);
            let toolbox = self.toolbox.clone();
            move |_, _keyval, keycode, _state| {
                // if 'esc' is pressed
                if keycode == 9 {
                    if pressed.get() == true {
                        handles.borrow().set_central_box_sensitivity(true);
                        boxes.borrow_mut().is_drawing = false;
                        pressed.set(false);
                        toolbox.remove_css_class("pressed");
                    } else {
                        window.destroy();
                        return glib::signal::Propagation::Stop;
                    }
                }
                glib::signal::Propagation::Proceed
            }
        });

        ////////////////////////////////////////////////
        // Change color
        ////////////////////////////////////////////////
        let cc = self.change_color.clone();
        cc.connect_clicked(glib::clone!(
            #[weak]
            window,
            #[weak]
            boxes,
            #[weak]
            red,
            #[weak]
            green,
            #[weak]
            blue,
            #[weak]
            alpha,
            move |_btn| {
                // get actual color
                let color = gtk::gdk::RGBA::new(
                    red.get() as f32,
                    green.get() as f32,
                    blue.get() as f32,
                    alpha.get() as f32,
                );
                // create color dialog
                let cancellable = gio::Cancellable::new();
                let color_dialog = gtk::ColorDialog::new();
                color_dialog.set_title("Pick color");
                color_dialog.choose_rgba(
                    Some(&window),
                    Some(&color),
                    Some(&cancellable),
                    // gtk::gio::Cancellable::NONE,
                    move |a| {
                        if let Ok(color) = a {
                            red.set(color.red() as f64);
                            green.set(color.green() as f64);
                            blue.set(color.blue() as f64);
                            alpha.set(color.alpha() as f64);
                            boxes.borrow_mut().set_rgba(
                                red.get(),
                                green.get(),
                                blue.get(),
                                alpha.get(),
                            );
                        } else {
                            println!("No color found");
                        }
                    },
                );
            }
        ));

        ////////////////////////////////////////////////
        // Draw rectangles on screenshot area
        ////////////////////////////////////////////////
        drawing.set_draw_func(glib::clone!(
            #[weak]
            boxes,
            move |_, cr, _width, _height| {
                boxes.borrow_mut().set_draw(&cr);
            }
        ));

        // Create numbered circles
        let add_numbered_circles = self.add_numbered_circles.clone();
        add_numbered_circles.connect_clicked(glib::clone!(
            #[weak]
            boxes,
            #[weak]
            handles,
            #[weak]
            red,
            #[weak]
            green,
            #[weak]
            blue,
            #[weak]
            alpha,
            #[weak]
            toolbox,
            #[weak]
            pressed,
            move |b| {
                // if drawing, stops
                if pressed.get() {
                    handles.borrow().set_central_box_sensitivity(true);
                    boxes.borrow_mut().is_drawing = false;
                    pressed.set(false);
                    // if its class is "pressed", then we do not want to continue to draw
                    if let Some(_index) = b.css_classes().iter().position(|s| s == "pressed") {
                        b.remove_css_class("pressed");
                        return;
                    } else {
                        // otherwise, another button was clicked,
                        // set every button's toolbox normal theme
                        toolbox.borrow().remove_css_class("pressed");
                    }
                }
                handles.borrow().set_central_box_sensitivity(false);
                boxes.borrow_mut().create_new_numbered_circle(
                    20.0,
                    17.0,
                    1.0,
                    1.0,
                    1.0,
                    red.get(),
                    green.get(),
                    blue.get(),
                    alpha.get(),
                );
                pressed.set(true);
                b.add_css_class("pressed");
            }
        ));

        // Create line
        let btn_line = self.add_line.clone();
        btn_line.connect_clicked(glib::clone!(
            #[weak]
            boxes,
            #[weak]
            handles,
            #[weak]
            red,
            #[weak]
            green,
            #[weak]
            blue,
            #[weak]
            alpha,
            #[weak]
            toolbox,
            #[weak]
            pressed,
            move |b| {
                // if drawing, stops
                if pressed.get() {
                    handles.borrow().set_central_box_sensitivity(true);
                    boxes.borrow_mut().is_drawing = false;
                    pressed.set(false);
                    // if its class is "pressed", then we do not want to continue to draw
                    if let Some(_index) = b.css_classes().iter().position(|s| s == "pressed") {
                        b.remove_css_class("pressed");
                        return;
                    } else {
                        // otherwise, another button was clicked,
                        // set every button's toolbox normal theme
                        toolbox.borrow().remove_css_class("pressed");
                    }
                }
                handles.borrow().set_central_box_sensitivity(false);
                boxes.borrow_mut().create_new_line(
                    2.0,
                    red.get(),
                    green.get(),
                    blue.get(),
                    alpha.get(),
                );
                pressed.set(true);
                b.add_css_class("pressed");
            }
        ));

        // Filled circles to image
        let btn_arc_fill = self.add_arc_fill.clone();
        btn_arc_fill.connect_clicked(glib::clone!(
            #[weak]
            boxes,
            #[weak]
            handles,
            #[weak]
            red,
            #[weak]
            green,
            #[weak]
            blue,
            #[weak]
            alpha,
            #[weak]
            toolbox,
            #[weak]
            pressed,
            move |b| {
                // if drawing, stops
                if pressed.get() {
                    handles.borrow().set_central_box_sensitivity(true);
                    boxes.borrow_mut().is_drawing = false;
                    pressed.set(false);
                    // if its class is "pressed", then we do not want to continue to draw
                    if let Some(_index) = b.css_classes().iter().position(|s| s == "pressed") {
                        b.remove_css_class("pressed");
                        return;
                    } else {
                        // otherwise, another button was clicked,
                        // set every button's toolbox normal theme
                        toolbox.borrow().remove_css_class("pressed");
                    }
                }

                handles.borrow().set_central_box_sensitivity(false);
                boxes.borrow_mut().create_new_arc(
                    red.get(),
                    green.get(),
                    blue.get(),
                    alpha.get(),
                    true,
                );

                b.add_css_class("pressed");
                pressed.set(true);
            }
        ));

        // Unfilled circles
        let btn_arc_no_fill = self.add_arc_no_fill.clone();
        btn_arc_no_fill.connect_clicked(glib::clone!(
            #[weak]
            boxes,
            #[weak]
            handles,
            #[weak]
            red,
            #[weak]
            green,
            #[weak]
            blue,
            #[weak]
            alpha,
            #[weak]
            toolbox,
            #[weak]
            pressed,
            move |b| {
                // if drawing, stops
                if pressed.get() {
                    handles.borrow().set_central_box_sensitivity(true);
                    boxes.borrow_mut().is_drawing = false;
                    pressed.set(false);
                    // if its class is "pressed", then we do not want to continue to draw
                    if let Some(_index) = b.css_classes().iter().position(|s| s == "pressed") {
                        b.remove_css_class("pressed");
                        return;
                    } else {
                        // otherwise, another button was clicked,
                        // set every button's toolbox normal theme
                        toolbox.borrow().remove_css_class("pressed");
                    }
                }

                handles.borrow().set_central_box_sensitivity(false);
                boxes.borrow_mut().create_new_arc(
                    red.get(),
                    green.get(),
                    blue.get(),
                    alpha.get(),
                    false,
                );

                b.add_css_class("pressed");
                pressed.set(true);
            }
        ));

        // Filled boxes to image
        let btn_fill = self.add_rect_fill.clone();
        btn_fill.connect_clicked(glib::clone!(
            #[weak]
            boxes,
            #[weak]
            handles,
            #[weak]
            red,
            #[weak]
            green,
            #[weak]
            blue,
            #[weak]
            alpha,
            #[weak]
            toolbox,
            #[weak]
            pressed,
            move |b| {
                // if drawing, stops
                if pressed.get() {
                    handles.borrow().set_central_box_sensitivity(true);
                    boxes.borrow_mut().is_drawing = false;
                    pressed.set(false);
                    // if its class is "pressed", then we do not want to continue to draw
                    if let Some(_index) = b.css_classes().iter().position(|s| s == "pressed") {
                        b.remove_css_class("pressed");
                        return;
                    } else {
                        // otherwise, another button was clicked,
                        // set every button's toolbox normal theme
                        toolbox.borrow().remove_css_class("pressed");
                    }
                }

                handles.borrow().set_central_box_sensitivity(false);
                boxes.borrow_mut().create_new_box(
                    red.get(),
                    green.get(),
                    blue.get(),
                    alpha.get(),
                    true,
                );
                pressed.set(true);
                b.add_css_class("pressed");
            }
        ));

        // Unfilled boxes
        let btn_no_fill = self.add_rect_no_fill.clone();
        btn_no_fill.connect_clicked(glib::clone!(
            #[weak]
            boxes,
            #[weak]
            handles,
            #[weak]
            red,
            #[weak]
            green,
            #[weak]
            blue,
            #[weak]
            alpha,
            #[weak]
            toolbox,
            #[weak]
            pressed,
            move |b| {
                // if drawing, stops
                if pressed.get() {
                    handles.borrow().set_central_box_sensitivity(true);
                    boxes.borrow_mut().is_drawing = false;
                    pressed.set(false);
                    // if its class is "pressed", then we do not want to continue to draw
                    if let Some(_index) = b.css_classes().iter().position(|s| s == "pressed") {
                        b.remove_css_class("pressed");
                        return;
                    } else {
                        // otherwise, another button was clicked,
                        // set every button's toolbox normal theme
                        toolbox.borrow().remove_css_class("pressed");
                    }
                }

                handles.borrow().set_central_box_sensitivity(false);
                boxes.borrow_mut().create_new_box(
                    red.get(),
                    green.get(),
                    blue.get(),
                    alpha.get(),
                    false,
                );
                pressed.set(true);
                b.add_css_class("pressed");
            }
        ));

        // Free hand drawing
        let add_freehand = self.add_freehand.clone();
        add_freehand.connect_clicked(glib::clone!(
            #[weak]
            boxes,
            #[weak]
            handles,
            #[weak]
            red,
            #[weak]
            green,
            #[weak]
            blue,
            #[weak]
            alpha,
            #[weak]
            toolbox,
            #[weak]
            pressed,
            move |b| {
                // if drawing, stops
                if pressed.get() {
                    handles.borrow().set_central_box_sensitivity(true);
                    boxes.borrow_mut().is_drawing = false;
                    pressed.set(false);
                    // if its class is "pressed", then we do not want to continue to draw
                    if let Some(_index) = b.css_classes().iter().position(|s| s == "pressed") {
                        b.remove_css_class("pressed");
                        return;
                    } else {
                        // otherwise, another button was clicked,
                        // set every button's toolbox normal theme
                        toolbox.borrow().remove_css_class("pressed");
                    }
                }
                // TODO: something strange when I redraw. The init is the same as the previous end
                handles.borrow().set_central_box_sensitivity(false);
                boxes.borrow_mut().create_new_freehand_draw(
                    2.0,
                    red.get(),
                    green.get(),
                    blue.get(),
                    alpha.get(),
                );
                b.add_css_class("pressed");
                pressed.set(true);
            }
        ));

        // Draw arrow
        let btn_arrow = self.add_arrow.clone();
        btn_arrow.connect_clicked(glib::clone!(
            #[weak]
            boxes,
            move |b| {
                // if drawing, stops
                if pressed.get() {
                    handles.borrow().set_central_box_sensitivity(true);
                    boxes.borrow_mut().is_drawing = false;
                    pressed.set(false);
                    // if its class is "pressed", then we do not want to continue to draw
                    if let Some(_index) = b.css_classes().iter().position(|s| s == "pressed") {
                        b.remove_css_class("pressed");
                        return;
                    } else {
                        // otherwise, another button was clicked,
                        // set every button's toolbox normal theme
                        toolbox.borrow().remove_css_class("pressed");
                    }
                }

                handles.borrow().set_central_box_sensitivity(false);
                boxes.borrow_mut().create_new_arrow(
                    10.0,
                    2.0,
                    red.get(),
                    green.get(),
                    blue.get(),
                    alpha.get(),
                );
                b.add_css_class("pressed");
                pressed.set(true);
            }
        ));

        // draw boxes on central box
        let draw_box = gtk::GestureDrag::new();
        screenshot_box.add_controller(draw_box.clone());
        draw_box.connect_drag_begin(glib::clone!(
            #[weak]
            drawing,
            #[weak]
            boxes,
            #[weak]
            top,
            #[weak]
            left,
            move |_, x, y| {
                if boxes.borrow().is_drawing() == true {
                    boxes.borrow_mut().drag_begin(left.get() + x, top.get() + y);
                    drawing.queue_draw(); // Request a redraw
                }
            }
        ));
        draw_box.connect_drag_update(glib::clone!(
            #[weak]
            boxes,
            move |_, x, y| {
                if boxes.borrow_mut().is_drawing() == true {
                    boxes.borrow_mut().drag_update(x, y);
                    drawing.queue_draw(); // Request a redraw
                }
            }
        ));
        draw_box.connect_drag_end(move |_, _, _| {
            if boxes.borrow_mut().is_drawing() == true {
                boxes.borrow_mut().drag_end();
                // boxes.borrow_mut().is_drawing = false;
                // handles.borrow().set_central_box_sensitivity(true);
            }
        });

        ////////////////////////////////////////////////
        // Copy screenshot to clipboard
        ////////////////////////////////////////////////
        let csc = self.copy_to_clipboard.clone();
        let gesture = gtk::GestureClick::new();
        csc.add_controller(gesture.clone());

        gesture.connect_pressed({
            let toolbox = self.toolbox.clone();
            move |_, _, _, _| {
                toolbox.stop_toolbox();
            }
        });

        gesture.connect_stopped({
            let screen_x = self.x.clone();
            let screen_y = self.y.clone();
            let screen_w = self.w.clone();
            let screen_h = self.h.clone();
            let window = self.window.clone();
            move |_| {
                let x = screen_x.get() as i32;
                let y = screen_y.get() as i32;
                let w = screen_w.get() as i32;
                let h = screen_h.get() as i32;

                // take screenshot
                Self::take_screenshot(x, y, w, h);

                // destroy window
                window.destroy();
                //toolbox.set_visible(true);
            }
        });

        ////////////////////////////////////////////////
        // Save screenshot to file
        ////////////////////////////////////////////////
        // command to be executed: grim -g "$(slurp)" "$folder"/"$(date +%Y%m%d-%H%M%S)".png
        let stf = self.save_to_file.clone();
        let gesture = gtk::GestureClick::new();
        stf.add_controller(gesture.clone());

        gesture.connect_pressed({
            let toolbox = self.toolbox.clone();
            move |_, _, _, _| {
                toolbox.stop_toolbox();
            }
        });

        gesture.connect_stopped({
            let window = self.window.clone();
            let (screen_x, screen_y) = (self.x.clone(), self.y.clone());
            let (screen_w, screen_h) = (self.w.clone(), self.h.clone());
            let toolbox = self.toolbox.clone();
            move |_| {
                // file chooser dialog
                let dialog = gtk::FileDialog::builder()
                    .title("Save File")
                    .accept_label("Save")
                    .initial_name("capture.png")
                    .build();

                // Create a cancellable instance
                let cancellable = gio::Cancellable::new();

                // Open the dialog
                let win_clone = window.clone();
                let toolbox = toolbox.clone();

                let (screen_x, screen_y) = (screen_x.clone(), screen_y.clone());
                let (screen_w, screen_h) = (screen_w.clone(), screen_h.clone());

                // clone
                dialog.save(Some(&window), Some(&cancellable), move |file| {
                    match file {
                        Ok(file) => {
                            std::thread::sleep(std::time::Duration::from_millis(500));
                            // save screenshot
                            Self::save_screenshot(
                                file,
                                screen_x.get() as i32,
                                screen_y.get() as i32,
                                screen_w.get() as i32,
                                screen_h.get() as i32,
                            );

                            // since everything went fine, close the application window
                            win_clone.destroy();
                        }
                        Err(err) => {
                            eprintln!("Error selecting file: {}", err);

                            // probably you exit the file dialog, so you want to continue
                            // editing...
                            toolbox.draw_toolbox();
                        }
                    }
                });
            }
        });

        ////////////////////////////////////////////////
        // Toggle fullscreen
        ////////////////////////////////////////////////
        let tfs = self.fullscreen.clone();
        let gesture = gtk::GestureClick::new();
        tfs.add_controller(gesture.clone());

        gesture.connect_pressed({
            let toolbox = self.toolbox.clone();
            let x = self.x.clone();
            let y = self.y.clone();
            let w = self.w.clone();
            let h = self.h.clone();
            move |_, _, _, _| {
                // stop toolbox before redraw
                toolbox.stop_toolbox();

                // set fullscreen
                top_box.set_height_request(0);
                bottom_box.set_height_request(0);
                right_box.set_width_request(0);
                left_box.set_width_request(0);

                // update values
                x.set(0.0);
                y.set(0.0);
                w.set(full_w as f64);
                h.set(full_h as f64);
                top.set(0.0);
                right.set(0.0);
                left.set(0.0);
                bottom.set(0.0);
            }
        });

        gesture.connect_stopped({
            let toolbox = self.toolbox.clone();
            move |_| {
                toolbox.draw_toolbox();
            }
        });
    }

    fn gesture_screenshot_box(&self) {
        // clone overlay
        let layout = self.screenshot_layout.clone();
        let (top_box, left_box, bottom_box, right_box) = (
            self.top_box.clone(),
            self.left_box.clone(),
            self.bottom_box.clone(),
            self.right_box.clone(),
        );
        let screenshot_box = self.screenshot_box.clone();

        // margins
        let (full_w, full_h) = (self.full_w, self.full_h);
        let (top, left, bottom, right) = (
            self.top.clone(),
            self.left.clone(),
            self.bottom.clone(),
            self.right.clone(),
        );

        // screenshot dims
        let (sx, sy, sw, sh) = (
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
            self.h.clone(),
        );

        // clone toolbox object
        let toolbox = self.toolbox.clone();

        // create handles
        let handles = self.handles.clone();

        // gesture
        let gest = gtk::GestureDrag::new();
        layout.add_controller(gest.clone());

        gest.connect_drag_begin(glib::clone!(
            #[weak]
            top,
            #[weak]
            left,
            #[weak]
            bottom,
            #[weak]
            right,
            #[weak]
            top_box,
            #[weak]
            left_box,
            #[weak]
            bottom_box,
            #[weak]
            right_box,
            // #[weak]
            // screenshot_box,
            move |_, x, y| {
                // TODO: uncomment this for multiple screenshot redrawing (still work to do before)
                // toolbox_1.stop_toolbox();

                // hide children of screenshot box
                // screenshot_box.set_visible(false);

                // calculate
                let t = y;
                let l = x;
                let r = full_w as f64 - x;
                let b = full_h as f64 - y;

                // change size
                top_box.set_height_request(t as i32);
                left_box.set_width_request(l as i32);
                bottom_box.set_height_request(b as i32);
                right_box.set_width_request(r as i32);

                // save
                top.set(t);
                bottom.set(b);
                left.set(l);
                right.set(r);
            }
        ));

        gest.connect_drag_update(glib::clone!(
            #[weak]
            top,
            #[weak]
            left,
            #[weak]
            bottom,
            #[weak]
            right,
            #[weak]
            bottom_box,
            #[weak]
            right_box,
            #[weak]
            top_box,
            #[weak]
            left_box,
            move |_, x, y| {
                // get values
                let mut t = top.get();
                let mut b = bottom.get();
                let mut r = right.get();
                let mut l = left.get();

                // allow to create screenshot in both directions
                if x < 0.0 {
                    l += x;
                } else {
                    r -= x;
                }
                if y < 0.0 {
                    t += y;
                } else {
                    b -= y;
                }

                // prevent negative values
                if t < 0.0 || r < 0.0 || b < 0.0 || l < 0.0 {
                    return;
                }

                // change size
                left_box.set_width_request(l as i32);
                top_box.set_height_request(t as i32);
                bottom_box.set_height_request(b as i32);
                right_box.set_width_request(r as i32);

                // save screenshot width and height
                sx.set(l);
                sy.set(t);
                sw.set(&full_w - l - r);
                sh.set(&full_h - t - b);
            }
        ));

        gest.connect_drag_end(glib::clone!(
            #[weak]
            bottom_box,
            #[weak]
            right_box,
            #[weak]
            top_box,
            #[weak]
            left_box,
            #[weak]
            layout,
            move |_gest, _, _| {
                // save
                top.set(top_box.height() as f64);
                left.set(left_box.width() as f64);
                bottom.set(bottom_box.height() as f64);
                right.set(right_box.width() as f64);
                screenshot_box.set_visible(true);

                // create handles to resize the screenshot box
                handles.borrow().attach_handles();

                // TODO: allow multiple redrawing (remove this line once achieved)
                layout.remove_controller(_gest);

                // draw toolbox
                toolbox.draw_toolbox();
            }
        ));
    }

    // fn gesture_exit(&self) {
    //     let window = self.window.clone();
    //     let keyboard_ctrl = gtk::EventControllerKey::new();
    //     window.add_controller(keyboard_ctrl.clone());
    //     keyboard_ctrl.connect_key_pressed({
    //         let window = self.window.clone();
    //         move |_, _keyval, keycode, _state| {
    //             // if 'esc' is pressed
    //             if keycode == 9 {
    //                 window.destroy();
    //                 glib::signal::Propagation::Stop
    //             } else {
    //                 glib::signal::Propagation::Proceed
    //             }
    //         }
    //     });
    // }
    //
    fn take_screenshot(x: i32, y: i32, w: i32, h: i32) {
        // build the grim string like "10,20 400x900"
        // we need to subtract the border of the screenbox (which is 2px, see style.css)
        let grim_string = &format!("{},{} {}x{}", x + 1, y + 1, w, h);

        // Execute the `grim` command
        let grim_output = std::process::Command::new("grim")
            .arg("-g")
            .arg(grim_string)
            .arg("-")
            .stdout(std::process::Stdio::piped()) // Capture the output
            .output() // Execute the command
            .expect("Failed to execute grim");

        // Check if the grim command was successful
        if grim_output.status.success() {
            // Write the output of grim to wl-copy's stdin
            let mut wl_copy_process = std::process::Command::new("wl-copy")
                .stdin(std::process::Stdio::piped()) // Use a pipe for stdin
                .spawn() // Start the wl-copy process
                .expect("Failed to start wl-copy");

            // Write the grim output to wl-copy's stdin
            if let Some(stdin) = wl_copy_process.stdin.as_mut() {
                stdin
                    .write_all(&grim_output.stdout)
                    .expect("Failed to write to wl-copy");
            }

            // Wait for wl-copy to finish
            let _ = wl_copy_process
                .wait()
                .expect("wl-copy did not run successfully");

            println!("Screenshot (region: {}) copied to clipboard.", grim_string);
        } else {
            // Handle the error case for grim
            let stderr = String::from_utf8_lossy(&grim_output.stderr);
            eprintln!("Error executing grim: {}", stderr);
        }
    }

    fn save_screenshot(file: gio::File, x: i32, y: i32, w: i32, h: i32) {
        // get filename
        let filename = file.path().expect("Couldn't get file path");

        // build the grim string like "10,20 400x900"
        // we need to subtract the border of the screenbox (which is 2px, see style.css)
        let grim_string = &format!("{},{} {}x{}", x + 1, y + 1, w, h);

        // Execute the `grim` command
        let _grim_output = std::process::Command::new("grim")
            .arg("-g")
            .arg(grim_string)
            .arg(filename)
            .output() // Execute the command
            .expect("Failed to execute grim");
    }

    fn set_css() {
        // --- style.css
        let provider = gtk::CssProvider::new();
        // provider.load_from_data(include_str!("style.css"));
        provider.load_from_data(include_str!("../style.css"));
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn get_monitor_size() -> (i32, i32) {
        // vectors storing monitors informations
        let mut vec_w: Vec<i32> = Vec::new();
        let mut vec_h: Vec<i32> = Vec::new();

        // manage multiple monitors
        let screen = gdk::Display::default().unwrap();
        let monitors = screen.monitors();
        let n_monitors = monitors.n_items();

        // get the screens sizes
        for monitor_n in 0..n_monitors {
            let obj = monitors.item(monitor_n).unwrap();
            let primary_monitor = obj.downcast_ref::<gdk::Monitor>().unwrap();
            let geometry = primary_monitor.geometry();
            // fill vectors
            vec_w.push(geometry.x() + geometry.width());
            vec_h.push(geometry.y() + geometry.height());
            //(monitor_width, monitor_height) = (geometry.width(), geometry.height())
        }

        // compute the full desktop size
        let monitor_width = vec_w.iter().max().expect("no maximum width");
        let monitor_height = vec_h.iter().max().expect("no maximum height");
        // println!("{},{}", &monitor_width, &monitor_width);

        (*monitor_width, *monitor_height)
    }
}
