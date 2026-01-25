use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};

#[derive(Debug, Default)]
pub struct ArrowButton {}

#[glib::object_subclass]
impl ObjectSubclass for ArrowButton {
    const NAME: &'static str = "ArrowButton";
    type Type = super::ArrowButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for ArrowButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_label("\u{f061}");
        obj.set_hexpand(false);
        obj.set_vexpand(false);
        obj.set_halign(gtk::Align::End);
        obj.set_valign(gtk::Align::End);
        obj.set_tooltip_text(Some("Draw arrow"));
        obj.add_css_class(CSS_CLASS_TOOLBOX_BTN);
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);
    }
}

impl WidgetImpl for ArrowButton {}
impl ButtonImpl for ArrowButton {}
