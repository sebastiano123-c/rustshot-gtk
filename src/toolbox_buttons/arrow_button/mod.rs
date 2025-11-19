mod imp;

use crate::drawing_area_manager::drawables::{Arrow, DrawableCollection};
use crate::drawing_area_settings::HandleSettings;
use crate::toolbox_buttons::*;

use crate::geometry::GeometryState;
use gtk::{glib, prelude::*};

glib::wrapper! {
    pub struct ArrowButton(ObjectSubclass<imp::ArrowButton>)
        @extends gtk::Button,  gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl Default for ArrowButton {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ArrowButton {
    pub fn attach_gesture(&self, geometry: &GeometryState) {
        self.connect_clicked(glib::clone!(
            #[strong]
            geometry,
            move |b| {
                toggle_drawing(b.upcast_ref::<gtk::Widget>(), &geometry, || {
                    // Create drawable
                    let drawable = DrawableCollection::Arrows(Arrow::new(
                        &geometry.settings,
                        Some(geometry.settings.size.get_value("init_arrow_border_size")),
                    ));
                    geometry.drawing.create_new_drawable(&drawable);
                });
            }
        ));
    }
}
