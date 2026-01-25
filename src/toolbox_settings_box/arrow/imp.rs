use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Debug, Default)]
pub struct ArrowSettingsBox {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ArrowSettingsBox {
    const NAME: &'static str = "ArrowSettingsBox";
    type Type = super::ArrowSettingsBox;
    type ParentType = gtk::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for ArrowSettingsBox {}

// Trait shared by all widgets
impl BoxImpl for ArrowSettingsBox {}

// Trait shared by all widgets
impl WidgetImpl for ArrowSettingsBox {}
