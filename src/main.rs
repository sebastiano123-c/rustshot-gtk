use gtk::prelude::*;
mod drawing_area_manager;
mod handles;
mod rustshot_gui;
mod toolbox;
use rustshot_gui::RustshotGui;

const APP_NAME: &str = "com.seb.rustshot-gtk";

fn main() {
    let app = gtk::Application::new(Some(APP_NAME), Default::default());

    app.connect_activate(|app| {
        let gui = RustshotGui::new(app);
        gui.build_ui();
    });

    app.run();
}
