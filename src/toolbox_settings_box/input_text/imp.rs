use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Debug, Default)]
pub struct InputTextSettingsBox {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for InputTextSettingsBox {
    const NAME: &'static str = "InputTextSettingsBox";
    type Type = super::InputTextSettingsBox;
    type ParentType = gtk::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for InputTextSettingsBox {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl BoxImpl for InputTextSettingsBox {}

// Trait shared by all widgets
impl WidgetImpl for InputTextSettingsBox {}
