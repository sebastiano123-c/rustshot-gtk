use gtk::prelude::*;
mod drawing_area_manager;
mod drawing_area_settings;
mod edge;
mod geometry;
mod handle;
mod screenshot_box;
mod spin_button;
mod toolbox;
mod toolbox_bar;
mod toolbox_buttons;
mod toolbox_settings_box;
use geometry::GeometryState;
use rustshot_gtk::constants::APP_NAME;

fn main() -> std::io::Result<()> {
    let app = gtk::Application::new(Some(APP_NAME), Default::default());

    app.connect_activate(|app| {
        // Create GgeometryState with widgets
        let geom: GeometryState = GeometryState::new(app);
        geom.attach_gestures().expect("Error in attaching gesture");
    });

    app.run();

    Ok(())
}
