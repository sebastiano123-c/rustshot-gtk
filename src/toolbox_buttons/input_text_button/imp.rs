use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};

use crate::toolbox_settings_box::input_text::InputTextSettingsBox;

#[derive(Debug, Default)]
pub struct InputTextButton {
    pub settings_box: InputTextSettingsBox,
}

#[glib::object_subclass]
impl ObjectSubclass for InputTextButton {
    const NAME: &'static str = "InputTextButton";
    type Type = super::InputTextButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for InputTextButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_label("\u{54}");
        obj.set_hexpand(false);
        obj.set_vexpand(false);
        obj.set_halign(gtk::Align::End);
        obj.set_valign(gtk::Align::End);
        obj.set_tooltip_text(Some("Add label"));
        obj.add_css_class(CSS_CLASS_TOOLBOX_BTN);
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);

        self.settings_box.new_horizontal(gtk::Align::Center);
    }
}

impl WidgetImpl for InputTextButton {}
impl ButtonImpl for InputTextButton {}
