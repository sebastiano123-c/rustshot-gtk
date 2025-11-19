use gtk::prelude::*;
mod drawing_area_manager;
mod drawing_area_settings;
mod edge;
mod geometry;
mod handle;
mod rustshot_gui;
mod screenshot_box;
mod settings_window;
mod toolbox;
mod toolbox_bar;
mod toolbox_buttons;
use geometry::GeometryState;
use rustshot_gtk::constants::APP_NAME;

fn main() {
    let app = gtk::Application::new(Some(APP_NAME), Default::default());

    app.connect_activate(|app| {
        // Create GgeometryState with widgets
        let geom: GeometryState = GeometryState::new(&app);
        geom.attach_gestures();
    });

    app.run();
}
