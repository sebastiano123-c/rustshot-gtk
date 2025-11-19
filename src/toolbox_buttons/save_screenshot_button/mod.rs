mod imp;

use crate::geometry::GeometryState;
use gtk::{gio, glib, prelude::*};

glib::wrapper! {
    pub struct SaveScreenshotButton(ObjectSubclass<imp::SaveScreenshotButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl Default for SaveScreenshotButton {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl SaveScreenshotButton {
    pub fn attach_gesture(&self, geometry: &GeometryState) {
        let gesture = gtk::GestureClick::new();
        self.add_controller(gesture.clone());

        gesture.connect_pressed(glib::clone!(
            #[strong]
            geometry,
            move |_, _, _, _| {
                geometry.toolbox.stop_toolbox(&geometry);
            }
        ));

        gesture.connect_stopped(glib::clone!(
            #[strong]
            geometry,
            move |_| {
                // TODO: add year-month-day-hour-minute-seconds.png format
                // file chooser dialog
                let dialog = gtk::FileDialog::builder()
                    .title("Save File")
                    .accept_label("Save")
                    .initial_name("capture.png")
                    .build();

                // Create a cancellable instance
                let cancellable = gio::Cancellable::new();

                // Open the dialog
                let geom = geometry.clone();

                // clone
                dialog.save(Some(&geometry.window), Some(&cancellable), move |file| {
                    match file {
                        Ok(file) => {
                            std::thread::sleep(std::time::Duration::from_millis(500));
                            // save screenshot
                            geom.save_screenshot(file);

                            // since everything went fine, close the application window
                            geom.destroy();
                        }
                        Err(err) => {
                            eprintln!("Error selecting file: {}", err);

                            // probably you exit the file dialog, so you want to continue
                            // editing...
                            geom.toolbox.draw_toolbox(&geom);
                        }
                    }
                });
            }
        ));
    }
}
