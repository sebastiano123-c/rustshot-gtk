use crate::spin_button::IntegerInput;
use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Debug, Default)]
pub struct NumberedCircleSettingsBox {
    pub number_spin: IntegerInput,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for NumberedCircleSettingsBox {
    const NAME: &'static str = "NumberedCircleSettingsBox";
    type Type = super::NumberedCircleSettingsBox;
    type ParentType = gtk::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for NumberedCircleSettingsBox {
    fn constructed(&self) {
        self.parent_constructed();

        self.number_spin.set_value(1);
        self.number_spin.set_min(0);
        self.number_spin.set_max(200);
    }
}

// Trait shared by all widgets
impl BoxImpl for NumberedCircleSettingsBox {}

// Trait shared by all widgets
impl WidgetImpl for NumberedCircleSettingsBox {}
