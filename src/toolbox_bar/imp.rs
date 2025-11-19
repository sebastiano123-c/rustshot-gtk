use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Debug, Default)]
pub struct ToolboxBar {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ToolboxBar {
    const NAME: &'static str = "ToolboxBar";
    type Type = super::ToolboxBar;
    type ParentType = gtk::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for ToolboxBar {}

// Trait shared by all widgets
impl BoxImpl for ToolboxBar {}

// Trait shared by all widgets
impl WidgetImpl for ToolboxBar {}
