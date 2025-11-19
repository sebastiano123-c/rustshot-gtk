mod imp;

use crate::drawing_area_manager::drawables::{Arc, DrawableCollection};
use crate::drawing_area_settings::HandleSettings;
use crate::toolbox_buttons::*;

use crate::geometry::GeometryState;
use gtk::{glib, prelude::*};

glib::wrapper! {
    pub struct ArcButton(ObjectSubclass<imp::ArcButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::Actionable, gtk::ConstraintTarget;
}

impl Default for ArcButton {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ArcButton {
    pub fn attach_gesture(&self, geometry: &GeometryState) {
        let se = self;
        let click = gtk::GestureClick::new();
        self.add_controller(click.clone());
        click.connect_pressed(glib::clone!(
            #[strong]
            geometry,
            #[weak]
            se,
            move |_, _, _, _| {
                toggle_drawing(se.upcast_ref::<gtk::Widget>(), &geometry, || {
                    // Create drawable
                    let drawable = DrawableCollection::Arcs(Arc::new(
                        &geometry.settings,
                        Some(geometry.settings.size.get_value("init_arc_border_size")),
                    ));
                    geometry.drawing.create_new_drawable(&drawable);
                });
            }
        ));
    }
}
