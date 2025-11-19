use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};

#[derive(Debug, Default)]
pub struct FullscreenButton {}

#[glib::object_subclass]
impl ObjectSubclass for FullscreenButton {
    const NAME: &'static str = "FullscreenButton";
    type Type = super::FullscreenButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for FullscreenButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_label("\u{f424}");
        obj.set_tooltip_text(Some("Set fullscreen"));
        obj.add_css_class(CSS_CLASS_TOOLBOX_BTN);
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);
    }
}

impl WidgetImpl for FullscreenButton {}
impl ButtonImpl for FullscreenButton {}
