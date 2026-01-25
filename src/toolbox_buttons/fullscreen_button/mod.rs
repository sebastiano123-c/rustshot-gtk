mod imp;

use crate::geometry::GeometryState;
use gtk::{glib, prelude::*};

glib::wrapper! {
    pub struct FullscreenButton(ObjectSubclass<imp::FullscreenButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl Default for FullscreenButton {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl FullscreenButton {
    pub fn attach_gesture(&self, geometry: &GeometryState) {
        let gesture = gtk::GestureClick::new();
        self.add_controller(gesture.clone());

        gesture.connect_pressed(glib::clone!(
            #[strong]
            geometry,
            move |_, _, _, _| {
                // stop toolbox before redraw
                geometry.toolbox.stop_toolbox(&geometry);

                // set fullscreen
                geometry.top_box.set_edge(0);
                geometry.bottom_box.set_edge(0);
                geometry.right_box.set_edge(0);
                geometry.left_box.set_edge(0);
            }
        ));

        gesture.connect_stopped(glib::clone!(
            #[strong]
            geometry,
            move |_| {
                geometry
                    .toolbox
                    .draw_toolbox(&geometry)
                    .expect("FullscreenButton error");
            }
        ));
    }
}
