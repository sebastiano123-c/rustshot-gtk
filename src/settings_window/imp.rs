use gtk::{glib, prelude::*, subclass::prelude::*};

#[derive(Debug, Default)]
pub struct SettingsWindow {}

#[glib::object_subclass]
impl ObjectSubclass for SettingsWindow {
    const NAME: &'static str = "SettingsWindow";
    type Type = super::SettingsWindow;
    type ParentType = gtk::Window;

    // fn class_init(klass: &mut Self::Class) {
    //     // klass.set_css_name(CSS_CLASS);
    // }

    fn new() -> Self {
        Self {}
    }
}

impl ObjectImpl for SettingsWindow {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        obj.set_default_width(400);
        obj.set_visible(false);
        obj.set_title(Some("Settings"));

        let key_controller = gtk::EventControllerKey::new();
        obj.add_controller(key_controller.clone()); // pass a reference, no clone needed

        // Connect to the *key‑pressed* signal.
        // The closure must return a `glib::signal::Propagation`.
        key_controller.connect_key_pressed({
            let subwin = obj.clone();
            move |_, _keyval, keycode, _state| {
                if keycode == 9 {
                    subwin.set_visible(false);
                }
                glib::signal::Propagation::Proceed
            }
        });
    }
}

impl WidgetImpl for SettingsWindow {}
impl WindowImpl for SettingsWindow {}
