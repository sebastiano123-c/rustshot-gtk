use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};

#[derive(Debug, Default)]
pub struct BoxButton {}

#[glib::object_subclass]
impl ObjectSubclass for BoxButton {
    const NAME: &'static str = "BoxButton";
    type Type = super::BoxButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for BoxButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_label("\u{f096}");
        obj.set_tooltip_text(Some("Draw rectangle"));
        obj.add_css_class(CSS_CLASS_TOOLBOX_BTN);
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);
    }
}

impl WidgetImpl for BoxButton {}
impl ButtonImpl for BoxButton {}
