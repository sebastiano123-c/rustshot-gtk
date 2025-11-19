mod imp;

use crate::geometry::GeometryState;
use gtk::{glib, prelude::*};

glib::wrapper! {
    pub struct CopyScreenshotButton(ObjectSubclass<imp::CopyScreenshotButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl Default for CopyScreenshotButton {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl CopyScreenshotButton {
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
                geometry.destroy();
                //toolbox.set_visible(true);
            }
        ));
    }
}
