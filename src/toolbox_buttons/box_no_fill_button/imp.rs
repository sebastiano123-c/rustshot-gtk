use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};

#[derive(Debug, Default)]
pub struct BoxNoFillButton {}

#[glib::object_subclass]
impl ObjectSubclass for BoxNoFillButton {
    const NAME: &'static str = "BoxNoFillButton";
    type Type = super::BoxNoFillButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for BoxNoFillButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_label("\u{f096}");
        obj.set_tooltip_text(Some("Draw rectangle without fill"));
        obj.add_css_class(CSS_CLASS_TOOLBOX_BTN);
        obj.add_css_class("fas");
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);
    }
}

impl WidgetImpl for BoxNoFillButton {}
impl ButtonImpl for BoxNoFillButton {}
