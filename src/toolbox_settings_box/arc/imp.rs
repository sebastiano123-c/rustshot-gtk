use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Debug, Default)]
pub struct ArcSettingsBox {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ArcSettingsBox {
    const NAME: &'static str = "ArcSettingsBox";
    type Type = super::ArcSettingsBox;
    type ParentType = gtk::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for ArcSettingsBox {}

// Trait shared by all widgets
impl BoxImpl for ArcSettingsBox {}

// Trait shared by all widgets
impl WidgetImpl for ArcSettingsBox {}
