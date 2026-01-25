mod imp;

use gtk::prelude::*;
use gtk::{gio, glib};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BAR, TOOLBOX_BTN_SIZE, TOOLBOX_SETTINGS_BTN_SIZE};

use crate::drawing_area_settings::SettingValue;
use crate::geometry::GeometryState;
use crate::spin_button::IntegerInput;

glib::wrapper! {
    pub struct ArrowSettingsBox(ObjectSubclass<imp::ArrowSettingsBox>)
        @extends gtk::Box,
        @implements gtk::Accessible,  gtk::Actionable, gtk::Widget, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ArrowSettingsBox {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ArrowSettingsBox {
    pub fn new_vertical(&self, size: i32, align: gtk::Align) {
        self.set_orientation(gtk::Orientation::Vertical);
        self.add_css_class(CSS_CLASS_TOOLBOX_BAR);
        self.set_width_request(size);
        self.set_halign(align);
        self.set_valign(gtk::Align::Center);
    }

    pub fn new_horizontal(&self, align: gtk::Align) {
        // compute screenshot box size
        self.set_orientation(gtk::Orientation::Horizontal);
        self.add_css_class(CSS_CLASS_TOOLBOX_BAR);
        self.set_height_request(TOOLBOX_SETTINGS_BTN_SIZE);
        // self.set_margin_end(geom.right_box.get_edge() + 5);
        // self.set_margin_start(geom.left_box.get_edge() + 5);
        self.set_valign(align);
    }

    pub fn new_fullscreen(&self, geom: &GeometryState) {
        // create toolbox
        self.set_orientation(gtk::Orientation::Horizontal);
        self.add_css_class(CSS_CLASS_TOOLBOX_BAR);
        self.set_halign(gtk::Align::Center);
        self.set_height_request(TOOLBOX_BTN_SIZE);
        self.set_margin_start(geom.full_w / 2);
        self.set_margin_top(10);
        self.set_margin_bottom(geom.full_h - TOOLBOX_BTN_SIZE);
    }

    pub fn populate_with_settings(&self, geom: &GeometryState) -> std::io::Result<()> {
        // get arrow settings
        let settings = geom.settings.arrow.clone();

        // size
        let spin = IntegerInput::new(settings.get_value("size").get_f64()? as i32, 0, 100);
        spin.connect_value_changed(glib::clone!(
            // #[strong]
            // spin,
            #[strong]
            settings,
            move |s| {
                let sz = s.value();
                if sz.abs() > 0 && sz.signum() > 0 {
                    settings
                        .set_value("size", SettingValue::F64(sz as f64))
                        .expect("Error in ArrowSettingButton");
                }
            }
        ));
        spin.set_tooltip_text(Some("change size"));

        // border color
        let color_color_dialog = gtk::ColorDialog::new();
        let color_color_btn = gtk::ColorDialogButton::new(Some(color_color_dialog.clone()));

        // get actual color
        let color = gtk::gdk::RGBA::new(
            settings.get_value("color_r").get_f64()? as f32,
            settings.get_value("color_g").get_f64()? as f32,
            settings.get_value("color_b").get_f64()? as f32,
            settings.get_value("color_a").get_f64()? as f32,
        );
        color_color_btn.set_rgba(&color);

        let gest = gtk::GestureClick::new();
        gest.connect_pressed(glib::clone!(
            #[strong]
            geom,
            #[weak]
            color_color_dialog,
            #[weak]
            color_color_btn,
            move |_, _, _, _| {
                // create color dialog
                let cancellable = gio::Cancellable::new();

                // clone geometry
                let geom = geom.clone();

                // Dialog
                let btn = color_color_btn.clone();
                color_color_dialog.set_title("Pick color");
                color_color_dialog.choose_rgba(
                    Some(&geom.window),
                    Some(&color),
                    Some(&cancellable),
                    // gtk::gio::Cancellable::NONE,
                    move |res| {
                        if let Ok(color) = res {
                            btn.set_rgba(&color);
                            let r: f64 = color.red() as f64;
                            let g: f64 = color.green() as f64;
                            let b: f64 = color.blue() as f64;
                            let a: f64 = color.alpha() as f64;
                            geom.settings
                                .arrow
                                .set_value("color_r", SettingValue::F64(r))
                                .expect("ColorChoser Error");
                            geom.settings
                                .arrow
                                .set_value("color_g", SettingValue::F64(g))
                                .expect("ColorChoser Error");
                            geom.settings
                                .arrow
                                .set_value("color_b", SettingValue::F64(b))
                                .expect("ColorChoser Error");
                            geom.settings
                                .arrow
                                .set_value("color_a", SettingValue::F64(a))
                                .expect("ColorChoser Error");
                            // geom.drawing.set_rgba(r, g, b, a);
                        } else {
                            println!("No color found");
                        }
                    },
                );
            }
        ));
        color_color_btn.add_controller(gest);

        // set child
        let fill_box: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        fill_box.append(&color_color_btn);
        fill_box.append(&spin);
        crate::toolbox_settings_box::add_expandable_row(
            self, "\u{f575}", "fill", "fas", fill_box, true,
        );

        Ok(())
    }
}
