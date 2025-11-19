use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};

#[derive(Debug, Default)]
pub struct SettingsButton {}

#[glib::object_subclass]
impl ObjectSubclass for SettingsButton {
    const NAME: &'static str = "SettingsButton";
    type Type = super::SettingsButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for SettingsButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_label("\u{f013}");
        obj.set_tooltip_text(Some("Settings"));
        obj.add_css_class(CSS_CLASS_TOOLBOX_BTN);
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);
    }
}

impl WidgetImpl for SettingsButton {}
impl ButtonImpl for SettingsButton {}
