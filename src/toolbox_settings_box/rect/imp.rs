use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Debug, Default)]
pub struct RectSettingsBox {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for RectSettingsBox {
    const NAME: &'static str = "RectSettingsBox";
    type Type = super::RectSettingsBox;
    type ParentType = gtk::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for RectSettingsBox {}

// Trait shared by all widgets
impl BoxImpl for RectSettingsBox {}

// Trait shared by all widgets
impl WidgetImpl for RectSettingsBox {}
