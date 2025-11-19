mod imp;

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::geometry::GeometryState;

glib::wrapper! {
    pub struct ScreenRecorder(ObjectSubclass<imp::ScreenRecorder>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
}

impl Default for ScreenRecorder {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ScreenRecorder {
    pub fn attach_gesture(&self, geometry: &GeometryState) {
        let gesture = gtk::GestureClick::new();
        self.add_controller(gesture.clone());
        // TODO: how to interrupt video recording?

        gesture.connect_pressed(glib::clone!(
            #[strong]
            geometry,
            move |_, _, _, _| {
                geometry.toolbox.stop_toolbox(&geometry);
            }
        ));

        let se = self;
        gesture.connect_stopped(glib::clone!(
            #[strong]
            geometry,
            #[weak]
            se,
            move |_| {
                let dim = geometry.get_screenshot_size();
                // start recording screen
                se.start_record_screen(dim[0], dim[1], dim[2], dim[3]);
            }
        ));
    }

    pub fn start_record_screen(&self, x: i32, y: i32, w: i32, h: i32) {
        // WARNING: I would like to use FFMPEG (a tool to convert video formats, but also it is a
        // way to record the screen.) However, I do not know how to properly use it in wayland. I
        // get errors and the docs were not so useful. I will use instead 'wf-recorder' (a
        // wlroots-based screen recorder).
        //
        // The way it works is
        //  wf-recorder -a -g "x,y wxh" --file="..." -y
        //
        //  where:
        //  -a, --audio [=DEVICE] = Starts recording the screen with audio. DEVICE argument is optional.
        //  -g, --geometry = Selects a specific part of the screen. The format is "x,y WxH".
        //  --file = the output file
        //  -y, --overwrite = Force overwriting the output file without prompting.
        let imp = self.imp();

        // Build the --geometry string
        let geometry = format!("{},{} {}x{}", x, y, w, h);

        // Execute the `grim` command
        let child = std::process::Command::new("wf-recorder")
            .arg("-a")
            .arg("-g")
            .arg(&geometry)
            .arg("-f")
            .arg(&imp.output_file)
            .arg("-y")
            .stdout(std::process::Stdio::piped())
            // .stderr(std::process::Stdio::piped()) // Capture stderr for error messages
            .spawn() // Execute the command
            .expect("Failed to execute wf-recorder");

        *imp.child.borrow_mut() = Some(child);
        imp.is_recording.set(true);
    }

    pub fn stop_recording(&self) {
        let imp = self.imp();

        // Exit if it is not recording
        if !imp.is_recording.get() {
            println!("Reconding is not yet started!");
            return;
        }

        // Otherwise, get child
        if let Some(child) = imp.child.borrow_mut().as_mut() {
            // send SIGINT to the child
            nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(child.id() as i32),
                nix::sys::signal::Signal::SIGINT,
            )
            .expect("cannot send ctrl-c");

            // // Optionally, wait for the process to exit
            // let _ = child.wait().expect("Process wasn't running");
            // println!("Process stopped.");

            // stop recording
            imp.is_recording.set(false);
        } else {
            println!("Reconding is not yet started!");
        }
    }

    pub fn get_file_path(&self) -> String {
        let imp = self.imp();
        imp.output_file.clone()
    }
}
