mod imp;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct IntegerInput(ObjectSubclass<imp::IntegerInput>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl IntegerInput {
    pub fn new(initial_value: i32, min: i32, max: i32) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.set_min(min);
        obj.set_max(max);
        obj.set_value(initial_value);
        obj
    }

    pub fn value(&self) -> i32 {
        self.imp().value.get()
    }

    pub fn set_value(&self, value: i32) {
        let imp = self.imp();
        let clamped = value.clamp(imp.min.get(), imp.max.get());
        imp.value.set(clamped);
        imp.entry.set_text(&clamped.to_string());
        self.notify("value");
    }

    pub fn min(&self) -> i32 {
        self.imp().min.get()
    }

    pub fn set_min(&self, min: i32) {
        self.imp().min.set(min);
        self.notify("min");
    }

    pub fn max(&self) -> i32 {
        self.imp().max.get()
    }

    pub fn set_max(&self, max: i32) {
        self.imp().max.set(max);
        self.notify("max");
    }

    pub fn connect_value_changed<F>(&self, f: F) -> glib::SignalHandlerId
    where
        F: Fn(&Self) + 'static,
    {
        self.connect_notify_local(Some("value"), move |obj, _| {
            f(obj);
        })
    }
}

impl Default for IntegerInput {
    fn default() -> Self {
        Self::new(0, i32::MIN, i32::MAX)
    }
}
