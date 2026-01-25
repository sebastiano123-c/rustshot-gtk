pub mod arc_button;
pub mod arc_no_fill_button;
pub mod arrow_button;
pub mod box_button;
pub mod box_no_fill_button;
pub mod copy_screenshot_button;
pub mod freehand_button;
pub mod fullscreen_button;
pub mod line_button;
pub mod numbered_circle_button;
pub mod save_screenshot_button;
pub mod screen_recorder;

use crate::geometry::GeometryState;
use gtk::prelude::*;
use rustshot_gtk::constants::CSS_CLASS_PRESSED;

fn toggle_drawing<F>(button: &gtk::Widget, geom: &GeometryState, start_draw: F)
where
    F: FnOnce(),
{
    // ---------------------------------------------------------
    // 1️⃣  If we are already drawing, stop it.
    // ---------------------------------------------------------
    if geom.toolbox.is_button_pressed() {
        geom.screenshot_box.set_screenshot_box_sensitivity(true);
        geom.drawing.set_drawing(false);
        geom.toolbox.set_button_pressed(false);
        // geom.settings_window.set_visible(false);

        geom.toolbox.stop_toolbox(geom);
        geom.toolbox
            .set_settings_box(None)
            .expect("toogle_drawing error");
        geom.toolbox
            .draw_toolbox(geom)
            .expect("toggle_drawing error");

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

#[derive(Debug)]
pub enum ToolboxButton {
    // Full Circle
    FullCircle(arc_button::ArcButton),
    // Full Box
    FullBox(box_button::BoxButton),
    // Arrow
    Arrow(arrow_button::ArrowButton),
    // Line
    Line(line_button::LineButton),
    // Freehand
    Freehand(freehand_button::FreehandButton),
    // Numbered circles
    NumberedCircles(numbered_circle_button::NumberedCircleButton),
    // Fullscreen
    Fullscreen(fullscreen_button::FullscreenButton),
    // Take screenshot
    TakeScreenshot(copy_screenshot_button::CopyScreenshotButton),
    // Save screenshot
    SaveScreenshot(save_screenshot_button::SaveScreenshotButton),
    // Record screen
    RecordScreen(screen_recorder::ScreenRecorder),
}

impl ToolboxButton {
    pub fn update_number(&self, value: i32) -> std::io::Result<()> {
        if let Self::NumberedCircles(button) = self {
            button.update_number(value)?;
        }
        Ok(())
    }
}

impl AsRef<gtk::Widget> for ToolboxButton {
    fn as_ref(&self) -> &gtk::Widget {
        match self {
            ToolboxButton::FullCircle(btn) => btn.as_ref(),
            ToolboxButton::FullBox(btn) => btn.as_ref(),
            ToolboxButton::Arrow(btn) => btn.as_ref(),
            ToolboxButton::Line(btn) => btn.as_ref(),
            ToolboxButton::Freehand(btn) => btn.as_ref(),
            ToolboxButton::NumberedCircles(btn) => btn.as_ref(),
            ToolboxButton::Fullscreen(btn) => btn.as_ref(),
            ToolboxButton::TakeScreenshot(btn) => btn.as_ref(),
            ToolboxButton::SaveScreenshot(btn) => btn.as_ref(),
            ToolboxButton::RecordScreen(btn) => btn.as_ref(),
        }
    }
}
