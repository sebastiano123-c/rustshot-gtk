mod imp;

use crate::drawing_area_manager::drawables::{AreaBox, DrawableCollection};
use crate::toolbox_buttons::*;

use crate::geometry::GeometryState;
use crate::toolbox_settings_box::rect::RectSettingsBox;
use gtk::{glib, prelude::*};

glib::wrapper! {
    pub struct BoxButton(ObjectSubclass<imp::BoxButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl Default for BoxButton {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl BoxButton {
    pub fn attach_gesture(&self, geom: &GeometryState) {
        // Create settings box
        let settings_box = RectSettingsBox::default();
        settings_box.new_horizontal(gtk::Align::Center);
        settings_box
            .populate_with_settings(geom)
            .expect("ArcButton attach_gesture error");

        self.connect_clicked(glib::clone!(
            #[strong]
            geom,
            #[strong]
            settings_box,
            move |b| {
                toggle_drawing(b.upcast_ref::<gtk::Widget>(), &geom, || {
                    // Create drawable
                    let drawable = DrawableCollection::AreaBoxes(AreaBox::new(&geom.settings));
                    geom.drawing.create_new_drawable(&drawable);

                    // Set settings box
                    geom.toolbox.stop_toolbox(&geom);
                    geom.toolbox
                        .set_settings_box(Some(settings_box.upcast_ref::<gtk::Widget>().clone()))
                        .expect("BoxButton error in gesture connect_clicked set_settings_box");
                    geom.toolbox
                        .draw_toolbox(&geom)
                        .expect("BoxButton error in gesture connect_click draw_toolbox");
                });
            }
        ));
    }
}
