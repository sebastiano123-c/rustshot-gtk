use gtk::prelude::*;
mod drawing_area_manager;
mod handles;
mod rustshot_gui;
mod toolbox;
use rustshot_gui::RustshotGui;

fn main() {
    let app = gtk::Application::new(Some("com.seb.rustshot-gtk"), Default::default());

    app.connect_activate(|app| {
        let gui = RustshotGui::new(app);
        gui.build_ui();
    });

    app.run();
}
