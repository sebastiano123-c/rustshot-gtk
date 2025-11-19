mod imp;

use crate::drawing_area_manager::drawables::{AreaBox, DrawableCollection};
use crate::toolbox_buttons::*;

use crate::geometry::GeometryState;
use gtk::{glib, prelude::*};

glib::wrapper! {
    pub struct BoxNoFillButton(ObjectSubclass<imp::BoxNoFillButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl Default for BoxNoFillButton {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl BoxNoFillButton {
    pub fn attach_gesture(&self, geometry: &GeometryState) {
        self.connect_clicked(glib::clone!(
            #[strong]
            geometry,
            move |b| {
                toggle_drawing(b.upcast_ref::<gtk::Widget>(), &geometry, || {
                    // Create drawable
                    let drawable =
                        DrawableCollection::AreaBoxes(AreaBox::new(&geometry.settings, None));
                    geometry.drawing.create_new_drawable(&drawable);
                });
            }
        ));
    }
}
