use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};

#[derive(Debug, Default)]
pub struct ColorButton {}

#[glib::object_subclass]
impl ObjectSubclass for ColorButton {
    const NAME: &'static str = "ColorButton";
    type Type = super::ColorButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for ColorButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        // obj.set_label("\u{f53f}");
        obj.set_tooltip_text(Some("Pick color"));
        obj.add_css_class(CSS_CLASS_TOOLBOX_BTN);
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);
    }
}

impl WidgetImpl for ColorButton {}
impl ButtonImpl for ColorButton {}
