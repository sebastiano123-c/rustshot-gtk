mod imp;

use crate::drawing_area_settings::HandleSettings;
use crate::geometry::GeometryState;
use gtk::{gio, glib, prelude::*};

glib::wrapper! {
    pub struct ColorButton(ObjectSubclass<imp::ColorButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl Default for ColorButton {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ColorButton {
    pub fn attach_gesture(&self, geometry: &GeometryState) {
        self.connect_clicked(glib::clone!(
            #[strong]
            geometry,
            move |_b| {
                // get actual color
                let color = gtk::gdk::RGBA::new(
                    geometry.settings.color.get_value("red") as f32,
                    geometry.settings.color.get_value("green") as f32,
                    geometry.settings.color.get_value("blue") as f32,
                    geometry.settings.color.get_value("alpha") as f32,
                );
                // create color dialog
                let cancellable = gio::Cancellable::new();
                let color_dialog = gtk::ColorDialog::new();

                // clone geometry
                let geom = geometry.clone();

                // Dialog
                color_dialog.set_title("Pick color");
                color_dialog.choose_rgba(
                    Some(&geometry.settings_window),
                    Some(&color),
                    Some(&cancellable),
                    // gtk::gio::Cancellable::NONE,
                    move |res| {
                        if let Ok(color) = res {
                            let r: f64 = color.red() as f64;
                            let g: f64 = color.green() as f64;
                            let b: f64 = color.blue() as f64;
                            let a: f64 = color.alpha() as f64;
                            geom.settings.color.set_value("red", r);
                            geom.settings.color.set_value("green", g);
                            geom.settings.color.set_value("blue", b);
                            geom.settings.color.set_value("alpha", a);
                            geom.drawing.set_rgba(r, g, b, a);
                        } else {
                            println!("No color found");
                        }
                    },
                );
            }
        ));
    }
}
