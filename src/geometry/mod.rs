use crate::drawing_area_manager::DrawingAreaManager;
use crate::edge::GrayEdge;
use rustshot_gtk::constants::{CSS_CLASS_PRESSED, CSS_FILE_PATH};
use std::io::Write;
// use crate::handles::Handles;
use crate::drawing_area_settings::SettingsRc;
use crate::screenshot_box::ScreenshotBox;
use crate::toolbox::Toolbox;

use gtk::prelude::*;
use gtk::{gdk, gio, glib};
use rustshot_gtk::constants::CSS_CLASS_TRANSPARENT;

/// Stores the mutable geometry values used by the drag callbacks.
#[derive(Clone)]
pub struct GeometryState {
    pub window: gtk::ApplicationWindow,

    // Edges objects (gray boxes defining the edge of the screenshot)
    pub top_box: GrayEdge,
    pub left_box: GrayEdge,
    pub bottom_box: GrayEdge,
    pub right_box: GrayEdge,

    // Central objects
    pub central_overlay: gtk::Overlay,
    pub screenshot_box: ScreenshotBox,

    pub settings: SettingsRc,
    pub toolbox: Toolbox,

    // Layout
    layout: gtk::Box,
    pub drawing: DrawingAreaManager,

    // Full window size – constant for the life of the widget
    pub full_w: i32,
    pub full_h: i32,
}

impl GeometryState {
    /// New geometry
    pub fn new(app: &gtk::Application) -> Self {
        // let st = std::time::Instant::now();

        // set css style
        Self::set_css();

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // create window object
        let window = gtk::ApplicationWindow::new(app);
        window.set_decorated(false);
        window.add_css_class(CSS_CLASS_TRANSPARENT);

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // Set window size
        let (w, h): (i32, i32) = Self::get_monitor_size();
        window.set_default_size(w, h);

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // create main layout
        let overlay = gtk::Overlay::new();
        overlay.add_css_class(CSS_CLASS_TRANSPARENT);
        window.set_child(Some(&overlay));

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // create drawing area
        let draw = DrawingAreaManager::default();
        overlay.add_overlay(&draw);

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // screenshot boxes layout
        let layout: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        layout.set_width_request(w);
        layout.set_height_request(h);
        layout.add_css_class(CSS_CLASS_TRANSPARENT);
        overlay.add_overlay(&layout);

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // top box
        let top_b: GrayEdge = GrayEdge::default();
        top_b.set_v(h / 2, w, gtk::Align::Start, gtk::Align::End);
        layout.append(&top_b);

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // Central box contains left and right box
        let central_b: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        central_b.set_vexpand(true);
        central_b.add_css_class(CSS_CLASS_TRANSPARENT);
        layout.append(&central_b);

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // Left box
        let left_b: GrayEdge = GrayEdge::default();
        left_b.set_h(w / 2, gtk::Align::End, gtk::Align::Fill);
        central_b.append(&left_b);

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // Screenshot box must be inside an Overlay
        let overlay = gtk::Overlay::new();
        overlay.set_hexpand(true);
        let screenshot_b: ScreenshotBox = ScreenshotBox::default();
        overlay.add_overlay(&screenshot_b);
        central_b.append(&overlay);

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // right box
        let right_b: GrayEdge = GrayEdge::default();
        right_b.set_h(w / 2, gtk::Align::Start, gtk::Align::Fill);
        central_b.append(&right_b);

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // bottom box
        let bottom_b: GrayEdge = GrayEdge::default();
        bottom_b.set_v(h / 2, w, gtk::Align::Start, gtk::Align::Start);
        layout.append(&bottom_b);

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // Create drawing area settings
        let settings_rc: SettingsRc = SettingsRc::new();

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // Create Toolbox object
        let toolbox: Toolbox = Toolbox::default();

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        // Set main window size and present
        window.present();

        // let stop = st.elapsed().as_millis();
        //println!("Elapsed: {}", stop);

        Self {
            window: window.clone(),
            top_box: top_b,
            left_box: left_b,
            bottom_box: bottom_b,
            right_box: right_b,
            central_overlay: overlay,
            screenshot_box: screenshot_b,
            settings: settings_rc,
            toolbox: toolbox,
            layout,
            drawing: draw,
            full_w: w,
            full_h: h,
        }
    }

    pub fn attach_gestures(&self) -> std::io::Result<()> {
        // geometry
        let geom = self;

        // Create toolbox buttons
        self.toolbox.create_toolbox_buttons(geom)?;

        // Make screenshot box resizable
        self.screenshot_box.attach_handles_gesture(geom);

        // layout
        let layout = self.layout.clone();

        // drawing area
        let drawing = self.drawing.clone();

        ////////////////////////////////////////////////
        // Gesture exit window
        ////////////////////////////////////////////////
        let keyboard_ctrl = gtk::EventControllerKey::new();
        geom.window.add_controller(keyboard_ctrl.clone());
        keyboard_ctrl.connect_key_pressed(glib::clone!(
            #[strong]
            geom,
            move |_, _keyval, keycode, _state| {
                // if 'esc' is pressed
                if keycode == 9 {
                    if geom.toolbox.is_button_pressed() {
                        geom.screenshot_box.set_screenshot_box_sensitivity(true);
                        geom.drawing.set_drawing(false);
                        geom.toolbox.set_button_pressed(false);
                        geom.toolbox.remove_css_class(CSS_CLASS_PRESSED);
                    } else {
                        // finally we need to destroy the windows objects
                        geom.window.destroy();
                        return glib::signal::Propagation::Stop;
                    }
                }
                glib::signal::Propagation::Proceed
            }
        ));

        // ---------------------------------------
        // Install drag gesture for the screenshot creation
        // ---------------------------------------
        let gest = gtk::GestureDrag::new();
        layout.add_controller(gest.clone());

        gest.connect_drag_begin(glib::clone!(
            #[strong]
            geom,
            move |_, x, y| {
                geom.drag_begin(x, y);
            }
        ));

        gest.connect_drag_update(glib::clone!(
            #[strong]
            geom,
            move |_, x, y| {
                geom.drag_update(x, y);
            }
        ));

        gest.connect_drag_end(glib::clone!(
            #[strong]
            geom,
            move |gest, x, y| {
                geom.drag_end(x, y);

                // detach gesture after initial setup
                layout.remove_controller(gest);

                // draw toolbox
                geom.toolbox
                    .draw_toolbox(&geom)
                    .expect("GeometryState connect_drag_end error");
            }
        ));

        // draw boxes on screenshot_box box
        let draw_gesture = gtk::GestureDrag::new();
        self.screenshot_box.add_controller(draw_gesture.clone());

        // Clone the initial position
        let left = self.left_box.clone();
        let top = self.top_box.clone();

        // screenshot_box.add_controller(draw_box.clone());
        draw_gesture.connect_drag_begin(glib::clone!(
            #[weak]
            drawing,
            move |_, x, y| {
                if drawing.is_drawing() {
                    drawing
                        // 10 is handle size
                        .drag_begin(
                            left.get_edge_f64() /*+ 10.0*/ + x,
                            top.get_edge_f64() /*+ 10.0*/ + y,
                        );
                    drawing.queue_draw(); // Request a redraw
                }
            }
        ));
        draw_gesture.connect_drag_update(glib::clone!(
            #[weak]
            drawing,
            move |_, x, y| {
                if drawing.is_drawing() {
                    drawing.drag_update(x, y);
                    drawing.queue_draw(); // Request a redraw
                }
            }
        ));
        draw_gesture.connect_drag_end(glib::clone!(move |_, _, _| {
            if drawing.is_drawing() {
                drawing.drag_end();
            }
        }));

        Ok(())
    }

    pub fn set_new_geometry_f64(&self, top: f64, left: f64, bottom: f64, right: f64) {
        self.top_box.set_edge_f64(top);
        self.left_box.set_edge_f64(left);
        self.bottom_box.set_edge_f64(bottom);
        self.right_box.set_edge_f64(right);
    }

    pub fn drag_begin(&self, x: f64, y: f64) {
        // calculate
        let t: f64 = y;
        let l: f64 = x;
        let b: f64 = self.full_h as f64 - t;
        let r: f64 = self.full_w as f64 - l;

        // set new geometry
        self.set_new_geometry_f64(t, l, b, r);
    }

    /// Updates the rectangle defined by the four edge boxes when the user drags.
    pub fn drag_update(&self, x: f64, y: f64) {
        let mut top = self.top_box.get_edge_f64();
        let mut left = self.left_box.get_edge_f64();
        let mut bottom = self.bottom_box.get_edge_f64();
        let mut right = self.right_box.get_edge_f64();

        if x.signum() < 0.0 {
            left += x;
        } else {
            right -= x;
        }

        if y.signum() < 0.0 {
            top += y;
        } else {
            bottom -= y;
        }

        if top < 0.0 || right < 0.0 || bottom < 0.0 || left < 0.0 {
            return;
        }

        // It is important not to update the top, left,... values BUT ONLY THE WIDGET!
        // otherwise the screenshot area moves inconsistently
        self.top_box.set_edge_pending(top as i32);
        self.left_box.set_edge_pending(left as i32);
        self.bottom_box.set_edge_pending(bottom as i32);
        self.right_box.set_edge_pending(right as i32);
    }

    pub fn drag_end(&self, _x: f64, _y: f64) {
        // save
        self.top_box.resolve_edge_pending();
        self.left_box.resolve_edge_pending();
        self.bottom_box.resolve_edge_pending();
        self.right_box.resolve_edge_pending();

        // set screenshot box visible to true
        // it is useful because with Handles it will be continuously set visible/invisible
        self.screenshot_box.set_visible(true);
    }

    /// Get the screenshot box (x, y, w, h)
    /// x: screenshot box x position
    /// y: screenshot box y position
    /// w: screenshot box width
    /// h: screenshot box height
    pub fn get_screenshot_size(&self) -> [i32; 4] {
        let x = self.left_box.get_edge();
        let y = self.top_box.get_edge();
        let w = self.full_w - x - self.right_box.get_edge();
        let h = self.full_h - y - self.bottom_box.get_edge();
        [x, y, w, h]
    }

    pub fn take_screenshot(&self) {
        let dim = self.get_screenshot_size();
        let x = dim[0];
        let y = dim[1];
        let w = dim[2];
        let h = dim[3];

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

    pub fn save_screenshot(&self, file: gio::File) {
        let dim = self.get_screenshot_size();
        let x = dim[0];
        let y = dim[1];
        let w = dim[2];
        let h = dim[3];

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

    pub fn destroy(&self) {
        self.window.destroy();
    }

    fn get_monitor_size() -> (i32, i32) {
        let mut max_w: i32 = 0_i32;
        let mut max_h: i32 = 0_i32;

        // manage multiple monitors
        let screen = gdk::Display::default().expect("No gtk::Display detected!");
        let monitors = screen.monitors();
        let n_monitors = monitors.n_items();

        // get the screens sizes
        for monitor_n in 0..n_monitors {
            let obj = monitors.item(monitor_n).unwrap();
            let primary_monitor = obj.downcast_ref::<gdk::Monitor>().unwrap();
            let geometry = primary_monitor.geometry();

            // fill vectors
            let w: i32 = geometry.x() + geometry.width();
            let h: i32 = geometry.y() + geometry.height();

            // Find maximum
            if w > max_w {
                max_w = w;
            }
            if h > max_h {
                max_h = h;
            }
        }

        (max_w, max_h)
    }

    fn set_css() {
        // set provider style.css
        let provider = gtk::CssProvider::new();
        provider.load_from_string(CSS_FILE_PATH);
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
