mod imp;

use crate::drawing_area_manager::drawables::{Arc, DrawableCollection};
use crate::toolbox_buttons::*;

use crate::geometry::GeometryState;
use gtk::{glib, prelude::*};

glib::wrapper! {
    pub struct ArcNoFillButton(ObjectSubclass<imp::ArcNoFillButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl Default for ArcNoFillButton {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ArcNoFillButton {
    pub fn attach_gesture(&self, geometry: &GeometryState) {
        self.connect_clicked(glib::clone!(
            #[strong]
            geometry,
            move |b| {
                toggle_drawing(b.upcast_ref::<gtk::Widget>(), &geometry, || {
                    // Create drawable
                    let drawable = DrawableCollection::Arcs(Arc::new(&geometry.settings));
                    geometry.drawing.create_new_drawable(&drawable);
                });
            }
        ));
    }
}
