use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};

use crate::toolbox_settings_box::numbered_circle::NumberedCircleSettingsBox;

#[derive(Debug, Default)]
pub struct NumberedCircleButton {
    pub settings_box: NumberedCircleSettingsBox,
}

#[glib::object_subclass]
impl ObjectSubclass for NumberedCircleButton {
    const NAME: &'static str = "NumberedCircleButton";
    type Type = super::NumberedCircleButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for NumberedCircleButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_label("\u{31}");
        obj.set_hexpand(false);
        obj.set_vexpand(false);
        obj.set_halign(gtk::Align::End);
        obj.set_valign(gtk::Align::End);
        obj.set_tooltip_text(Some("Add numbered circles"));
        obj.add_css_class(CSS_CLASS_TOOLBOX_BTN);
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);

        self.settings_box.new_horizontal(gtk::Align::Center);
    }
}

impl WidgetImpl for NumberedCircleButton {}
impl ButtonImpl for NumberedCircleButton {}
