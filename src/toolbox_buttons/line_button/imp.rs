use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};

#[derive(Debug, Default)]
pub struct LineButton {}

#[glib::object_subclass]
impl ObjectSubclass for LineButton {
    const NAME: &'static str = "LineButton";
    type Type = super::LineButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for LineButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_label("\u{f068}");
        obj.set_tooltip_text(Some("Draw line"));
        obj.add_css_class(CSS_CLASS_TOOLBOX_BTN);
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);
    }
}

impl WidgetImpl for LineButton {}
impl ButtonImpl for LineButton {}
