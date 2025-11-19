mod imp;

use rustshot_gtk::constants::CSS_CLASS_PRESSED;

use crate::geometry::GeometryState;
use gtk::{glib, prelude::*};

glib::wrapper! {
    pub struct SettingsButton(ObjectSubclass<imp::SettingsButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl Default for SettingsButton {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl SettingsButton {
    pub fn attach_gesture(&self, geometry: &GeometryState) {
        self.connect_clicked(glib::clone!(
            #[strong]
            geometry,
            move |b| {
                let widget = b.upcast_ref::<gtk::Widget>();

                // if drawing, stops
                if geometry.toolbox.is_button_pressed() {
                    // handles.borrow().set_central_box_sensitivity(true);
                    geometry.toolbox.set_button_pressed(false);
                    geometry.settings_window.set_visible(false);
                    // subwin.add_css_class("transparent");
                    // if its class is CSS_CLASS_PRESSED, then we do not want to continue to draw
                    if let Some(_index) = widget
                        .css_classes()
                        .iter()
                        .position(|s| s == CSS_CLASS_PRESSED)
                    {
                        widget.remove_css_class(CSS_CLASS_PRESSED);
                        return;
                    } else {
                        // otherwise, another button was clicked,
                        // set every button's toolbox normal theme
                        geometry.toolbox.remove_css_class(CSS_CLASS_PRESSED);
                    }
                }
                // handles.borrow().set_central_box_sensitivity(false);
                widget.add_css_class(CSS_CLASS_PRESSED);
                // subwin.add_css_class("subwin");
                geometry.toolbox.set_button_pressed(true);
                geometry.settings_window.set_visible(true);
            }
        ));
    }
}
