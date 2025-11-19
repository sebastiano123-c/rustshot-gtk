use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};

#[derive(Debug, Default)]
pub struct FreehandButton {}

#[glib::object_subclass]
impl ObjectSubclass for FreehandButton {
    const NAME: &'static str = "FreehandButton";
    type Type = super::FreehandButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for FreehandButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_label("\u{f1fc}");
        obj.set_tooltip_text(Some("Freehand draw"));
        obj.add_css_class(CSS_CLASS_TOOLBOX_BTN);
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);
    }
}

impl WidgetImpl for FreehandButton {}
impl ButtonImpl for FreehandButton {}
