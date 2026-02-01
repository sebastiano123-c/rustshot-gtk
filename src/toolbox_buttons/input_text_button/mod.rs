mod imp;

use crate::drawing_area_manager::drawables::{DrawableCollection, InputText};
use crate::toolbox_buttons::*;

use crate::geometry::GeometryState;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::{glib, prelude::*};

glib::wrapper! {
    pub struct InputTextButton(ObjectSubclass<imp::InputTextButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl Default for InputTextButton {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl InputTextButton {
    pub fn attach_gesture(&self, geom: &GeometryState) {
        let imp = self.imp();

        // Create settings box
        let settings_box = imp.settings_box.clone();
        settings_box
            .populate_with_settings(geom)
            .expect("InputTextSettingsBox attach_gesture error");

        self.connect_clicked(glib::clone!(
            #[strong]
            geom,
            #[strong]
            settings_box,
            move |b| {
                toggle_drawing(b.upcast_ref::<gtk::Widget>(), &geom, || {
                    // Create drawable
                    let drawable = DrawableCollection::InputTexts(InputText::new(&geom.settings));
                    geom.drawing.create_new_drawable(&drawable);

                    // Set settings box
                    geom.toolbox.stop_toolbox(&geom);
                    geom.toolbox
                        .set_settings_box(Some(settings_box.upcast_ref::<gtk::Widget>().clone()))
                        .expect(
                            "InputTextButton error in gesture connect_clicked set_settings_box",
                        );
                    geom.toolbox
                        .draw_toolbox(&geom)
                        .expect("InputTextButton error in gesture connect_click draw_toolbox");
                });
            }
        ));
    }
}
