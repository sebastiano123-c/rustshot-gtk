mod imp;

use crate::geometry::GeometryState;
use gtk::{glib, prelude::*};

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
                geometry.take_screenshot();
                geometry.save_screenshot();
            }
        ));
    }
}
