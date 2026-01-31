mod imp;

use gtk::prelude::*;
use gtk::{gio, glib, pango};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BAR, TOOLBOX_BTN_SIZE, TOOLBOX_SETTINGS_BTN_SIZE};

use crate::drawing_area_settings::SettingValue;
use crate::geometry::GeometryState;
use crate::spin_button::IntegerInput;

glib::wrapper! {
    pub struct InputTextSettingsBox(ObjectSubclass<imp::InputTextSettingsBox>)
        @extends gtk::Box,
        @implements gtk::Accessible,  gtk::Actionable, gtk::Widget, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for InputTextSettingsBox {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl InputTextSettingsBox {
    pub fn new_vertical(&self, size: i32, align: gtk::Align) {
        self.set_orientation(gtk::Orientation::Vertical);
        self.add_css_class(CSS_CLASS_TOOLBOX_BAR);
        self.set_width_request(size);
        self.set_halign(align);
        self.set_valign(gtk::Align::Center);
    }

    pub fn new_horizontal(&self, align: gtk::Align) {
        self.set_orientation(gtk::Orientation::Horizontal);
        self.add_css_class(CSS_CLASS_TOOLBOX_BAR);
        self.set_height_request(TOOLBOX_SETTINGS_BTN_SIZE);
        self.set_valign(align);
    }

    pub fn new_fullscreen(&self, geom: &GeometryState) {
        self.set_orientation(gtk::Orientation::Horizontal);
        self.add_css_class(CSS_CLASS_TOOLBOX_BAR);
        self.set_halign(gtk::Align::Center);
        self.set_height_request(TOOLBOX_BTN_SIZE);
        self.set_margin_start(geom.full_w / 2);
        self.set_margin_top(10);
        self.set_margin_bottom(geom.full_h - TOOLBOX_BTN_SIZE);
    }

    pub fn populate_with_settings(&self, geom: &GeometryState) -> std::io::Result<()> {
        // get numbered_circle settings
        let settings = geom.settings.input_text.clone();

        // border check box
        let border_checkbox = gtk::CheckButton::with_label("");
        border_checkbox.set_active(settings.get_value("border").get_bool()?);
        border_checkbox.set_height_request(TOOLBOX_SETTINGS_BTN_SIZE);
        border_checkbox.set_hexpand(false);
        border_checkbox.set_tooltip_text(Some("disable border"));
        border_checkbox.connect_toggled(glib::clone!(
            #[strong]
            settings,
            move |cb| {
                let is_active = cb.is_active();
                if is_active {
                    cb.set_tooltip_text(Some("disable border"));
                } else {
                    cb.set_tooltip_text(Some("enable border"));
                }
                settings
                    .set_value("border", SettingValue::Bool(is_active))
                    .expect("border_checkbox error");
            }
        ));

        let spin = IntegerInput::new(settings.get_value("border_size").get_f64()? as i32, 0, 100);
        spin.connect_value_changed(glib::clone!(
            #[strong]
            settings,
            move |s| {
                let sz = s.value();
                if sz.abs() > 0 && sz.signum() > 0 {
                    settings
                        .set_value("border_size", SettingValue::F64(sz as f64))
                        .expect("Error in InputTextSettingButton");
                }
            }
        ));
        spin.set_tooltip_text(Some("change border size"));

        // border color
        let border_color_dialog = gtk::ColorDialog::new();
        let border_color_btn = gtk::ColorDialogButton::new(Some(border_color_dialog.clone()));
        border_color_btn.set_tooltip_text(Some("change border color"));

        // get actual color
        let color = gtk::gdk::RGBA::new(
            settings.get_value("border_r").get_f64()? as f32,
            settings.get_value("border_g").get_f64()? as f32,
            settings.get_value("border_b").get_f64()? as f32,
            settings.get_value("border_a").get_f64()? as f32,
        );
        border_color_btn.set_rgba(&color);
        border_color_btn.set_height_request(TOOLBOX_SETTINGS_BTN_SIZE);
        border_color_btn.set_hexpand(false);
        border_color_btn.set_width_request(TOOLBOX_SETTINGS_BTN_SIZE);
        border_color_btn.set_vexpand(false);

        let gest = gtk::GestureClick::new();
        gest.connect_pressed(glib::clone!(
            #[strong]
            geom,
            #[weak]
            border_color_dialog,
            #[weak]
            border_color_btn,
            move |_, _, _, _| {
                // create color dialog
                let cancellable = gio::Cancellable::new();

                // clone geometry
                let geom = geom.clone();

                // Dialog
                let btn = border_color_btn.clone();
                border_color_dialog.set_title("Pick color");
                border_color_dialog.choose_rgba(
                    Some(&geom.window),
                    Some(&color),
                    Some(&cancellable),
                    move |res| {
                        if let Ok(color) = res {
                            btn.set_rgba(&color);
                            let r: f64 = color.red() as f64;
                            let g: f64 = color.green() as f64;
                            let b: f64 = color.blue() as f64;
                            let a: f64 = color.alpha() as f64;
                            geom.settings
                                .input_text
                                .set_value("border_r", SettingValue::F64(r))
                                .expect("ColorChoser Error");
                            geom.settings
                                .input_text
                                .set_value("border_g", SettingValue::F64(g))
                                .expect("ColorChoser Error");
                            geom.settings
                                .input_text
                                .set_value("border_b", SettingValue::F64(b))
                                .expect("ColorChoser Error");
                            geom.settings
                                .input_text
                                .set_value("border_a", SettingValue::F64(a))
                                .expect("ColorChoser Error");
                            // geom.drawing.set_rgba(r, g, b, a);
                        } else {
                            println!("No color found");
                        }
                    },
                );
            }
        ));
        border_color_btn.add_controller(gest);

        // fill check box
        let fill_checkbox = gtk::CheckButton::with_label("");
        fill_checkbox.set_tooltip_text(Some("disable fill"));
        fill_checkbox.set_active(settings.get_value("fill").get_bool()?);
        fill_checkbox.connect_toggled(glib::clone!(
            #[strong]
            settings,
            move |cb| {
                let is_active = cb.is_active();
                if is_active {
                    cb.set_tooltip_text(Some("disable fill"));
                } else {
                    cb.set_tooltip_text(Some("enable fill"));
                }
                settings
                    .set_value("fill", SettingValue::Bool(is_active))
                    .expect("fill_checkbox error");
            }
        ));

        // fill color
        let fill_color_dialog = gtk::ColorDialog::new();
        let fill_color_btn = gtk::ColorDialogButton::new(Some(fill_color_dialog.clone()));
        fill_color_btn.set_tooltip_text(Some("change fill color"));

        // get actual color
        let color = gtk::gdk::RGBA::new(
            settings.get_value("fill_r").get_f64()? as f32,
            settings.get_value("fill_g").get_f64()? as f32,
            settings.get_value("fill_b").get_f64()? as f32,
            settings.get_value("fill_a").get_f64()? as f32,
        );
        fill_color_btn.set_rgba(&color);

        let gest = gtk::GestureClick::new();
        gest.connect_pressed(glib::clone!(
            #[strong]
            geom,
            #[weak]
            fill_color_dialog,
            #[weak]
            fill_color_btn,
            move |_, _, _, _| {
                // create color dialog
                let cancellable = gio::Cancellable::new();

                // clone geometry
                let geom = geom.clone();

                // Dialog
                let btn = fill_color_btn.clone();
                fill_color_dialog.set_title("Pick color");
                fill_color_dialog.choose_rgba(
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
                                .input_text
                                .set_value("fill_r", SettingValue::F64(r))
                                .expect("ColorChoser Error");
                            geom.settings
                                .input_text
                                .set_value("fill_g", SettingValue::F64(g))
                                .expect("ColorChoser Error");
                            geom.settings
                                .input_text
                                .set_value("fill_b", SettingValue::F64(b))
                                .expect("ColorChoser Error");
                            geom.settings
                                .input_text
                                .set_value("fill_a", SettingValue::F64(a))
                                .expect("ColorChoser Error");
                            // geom.drawing.set_rgba(r, g, b, a);
                        } else {
                            println!("No color found");
                        }
                    },
                );
            }
        ));
        fill_color_btn.add_controller(gest);

        // font color
        let font_color_dialog = gtk::ColorDialog::new();
        let font_color_btn = gtk::ColorDialogButton::new(Some(font_color_dialog.clone()));
        font_color_btn.set_tooltip_text(Some("change font color"));

        // get actual color
        let color = gtk::gdk::RGBA::new(
            settings.get_value("font_r").get_f64()? as f32,
            settings.get_value("font_g").get_f64()? as f32,
            settings.get_value("font_b").get_f64()? as f32,
            settings.get_value("font_a").get_f64()? as f32,
        );
        font_color_btn.set_rgba(&color);

        let gest = gtk::GestureClick::new();
        gest.connect_pressed(glib::clone!(
            #[strong]
            geom,
            #[weak]
            font_color_dialog,
            #[weak]
            font_color_btn,
            move |_, _, _, _| {
                // create color dialog
                let cancellable = gio::Cancellable::new();

                // clone geometry
                let geom = geom.clone();

                // Dialog
                let btn = font_color_btn.clone();
                font_color_dialog.set_title("Pick color");
                font_color_dialog.choose_rgba(
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
                                .input_text
                                .set_value("font_r", SettingValue::F64(r))
                                .expect("ColorChoser Error");
                            geom.settings
                                .input_text
                                .set_value("font_g", SettingValue::F64(g))
                                .expect("ColorChoser Error");
                            geom.settings
                                .input_text
                                .set_value("font_b", SettingValue::F64(b))
                                .expect("ColorChoser Error");
                            geom.settings
                                .input_text
                                .set_value("font_a", SettingValue::F64(a))
                                .expect("ColorChoser Error");
                            // geom.drawing.set_rgba(r, g, b, a);
                        } else {
                            println!("No color found");
                        }
                    },
                );
            }
        ));
        font_color_btn.add_controller(gest);

        // font face
        let font_face = settings
            .get_value("font_face")
            .get_string()
            .expect("InputText font_face error");
        let fd: pango::FontDescription = pango::FontDescription::from_string(font_face.as_str());

        // 1️⃣  Create the FontDialog (the chooser itself)
        let font_dialog = gtk::FontDialog::builder()
            // Optional: give the dialog a title
            .title("Select font")
            .modal(true) // block interaction with the parent window
            .build();

        // 2️⃣  Create the FontDialogButton and attach the dialog
        let font_button = gtk::FontDialogButton::new(Some(font_dialog));

        // Set an initial preview text (what the button shows)
        font_button.set_tooltip_text(Some("Choose font face"));
        font_button.set_font_desc(&fd);

        // 3️⃣  React to the user picking a font
        // `font_desc` is an Option<gdk::pango::FontDescription>
        font_button.connect_font_desc_notify(glib::clone!(
            #[strong]
            settings,
            move |btn| {
                if let Some(desc) = btn.font_desc() {
                    // Convert the Pango FontDescription to a readable string
                    // println!("User selected font: {}", desc.to_string());
                    settings
                        .set_value("font_face", SettingValue::String(desc.to_string()))
                        .expect("Error font_face");
                } else {
                    println!("No font selected");
                }
            }
        ));

        // set children
        let border_box: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        border_box.append(&border_checkbox);
        border_box.append(&spin);
        border_box.append(&border_color_btn);
        crate::toolbox_settings_box::add_expandable_row(
            self, "\u{f853}", "border", "fal", border_box, false,
        );

        let fill_box: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        fill_box.append(&fill_checkbox);
        fill_box.append(&fill_color_btn);
        crate::toolbox_settings_box::add_expandable_row(
            self, "\u{f575}", "fill", "fas", fill_box, false,
        );

        let font_box: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        font_box.append(&font_button);
        font_box.append(&font_color_btn);
        crate::toolbox_settings_box::add_expandable_row(
            self, "\u{f034}", "font", "fas", font_box, false,
        );

        Ok(())
    }
}
