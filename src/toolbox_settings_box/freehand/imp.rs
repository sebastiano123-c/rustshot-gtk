use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Debug, Default)]
pub struct FreehandSettingsBox {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for FreehandSettingsBox {
    const NAME: &'static str = "FreehandSettingsBox";
    type Type = super::FreehandSettingsBox;
    type ParentType = gtk::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for FreehandSettingsBox {}

// Trait shared by all widgets
impl BoxImpl for FreehandSettingsBox {}

// Trait shared by all widgets
impl WidgetImpl for FreehandSettingsBox {}
