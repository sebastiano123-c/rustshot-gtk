pub mod arc_button;
pub mod arc_no_fill_button;
pub mod arrow_button;
pub mod box_button;
pub mod box_no_fill_button;
pub mod color_button; // fare unico tasto?
pub mod copy_screenshot_button;
pub mod freehand_button;
pub mod fullscreen_button;
pub mod line_button;
pub mod numbered_circle_button;
pub mod save_screenshot_button;
pub mod screen_recorder;
pub mod settings_button;

use crate::geometry::GeometryState;
use gtk::prelude::*;
use rustshot_gtk::constants::CSS_CLASS_PRESSED;

fn toggle_drawing<F>(button: &gtk::Widget, geom: &GeometryState, start_draw: F)
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
        if button.css_classes().iter().any(|c| c == CSS_CLASS_PRESSED) {
            button.remove_css_class(CSS_CLASS_PRESSED);
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
    button.add_css_class(CSS_CLASS_PRESSED);
}
