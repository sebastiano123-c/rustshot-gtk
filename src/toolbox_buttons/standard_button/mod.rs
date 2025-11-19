mod imp;

use crate::geometry::GeometryState;
use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::CSS_CLASS_PRESSED;

glib::wrapper! {
    pub struct StandardButton(ObjectSubclass<imp::StandardButton>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for StandardButton {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl StandardButton {
    fn toggle_drawing<F>(&self, geom: &GeometryState, start_draw: F)
    where
        F: FnOnce(),
    {
        // println!("Button pressed {}", toolbox.button_pressed.get());
        // ---------------------------------------------------------
        // 1️⃣  If we are already drawing, stop it.
        // ---------------------------------------------------------
        if geom.toolbox.is_button_pressed() {
            geom.screenshot_box.set_screenshot_box_sensitivity(true);
            geom.drawing.set_drawing(false);
            geom.toolbox.set_button_pressed(false);
            geom.settings_window.set_visible(false);

            // If the button itself still carries the “pressed” CSS class,
            // just toggle it off and exit early.
            if self.css_classes().iter().any(|c| c == CSS_CLASS_PRESSED) {
                self.remove_css_class(CSS_CLASS_PRESSED);
                return;
            }

            // Otherwise another button was clicked – clear the old toolbox
            // highlight.
            geom.toolbox.remove_css_class(CSS_CLASS_PRESSED);
        }

        // ---------------------------------------------------------
        // 2️⃣  Begin a fresh drawing operation.
        // ---------------------------------------------------------
        geom.screenshot_box.set_screenshot_box_sensitivity(false);
        start_draw(); // <-- caller creates the specific shape
        geom.toolbox.set_button_pressed(true);
        self.add_css_class(CSS_CLASS_PRESSED);
    }
}
