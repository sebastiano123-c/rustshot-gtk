use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Debug, Default)]
pub struct LineSettingsBox {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for LineSettingsBox {
    const NAME: &'static str = "LineSettingsBox";
    type Type = super::LineSettingsBox;
    type ParentType = gtk::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for LineSettingsBox {}

// Trait shared by all widgets
impl BoxImpl for LineSettingsBox {}

// Trait shared by all widgets
impl WidgetImpl for LineSettingsBox {}
