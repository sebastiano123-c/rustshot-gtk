use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn create_folder_if_not_exists(path: &str) -> std::io::Result<()> {
    // Create the folder if it does not exist
    if std::fs::metadata(path).is_err() {
        std::fs::create_dir(path)?;
        println!("Folder created: {}", path);
    }
    // else {
    //     println!("Folder already exists: {}", path);
    // }
    Ok(())
}

#[derive(Debug)]
pub struct ScreenRecorder {
    pub is_recording: Rc<Cell<bool>>,
    // captures_folder: String,
    pub output_file: String,
    pub child: RefCell<Option<std::process::Child>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ScreenRecorder {
    const NAME: &'static str = "ScreenRecorder";
    type Type = super::ScreenRecorder;
    type ParentType = gtk::Button;

    fn new() -> Self {
        // Get the home directory
        let home_dir = std::env::var("HOME").expect("$HOME environmental variable is not set!");

        // Create the folder path as a String
        let folder_path = format!("{}/Videos/rustshot-gtk", home_dir);

        // Create folder if not exists
        if let Err(e) = create_folder_if_not_exists(&folder_path) {
            eprintln!("Error creating folder: {}", e);
        }

        // define the output file
        let out_file = format!("{}/out.mkv", folder_path);

        // Return object
        Self {
            is_recording: Rc::new(Cell::new(false)),
            // captures_folder: folder_path,
            output_file: out_file,
            child: None.into(),
        }
    }
}

impl ObjectImpl for ScreenRecorder {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_label("\u{f03d}");
        obj.set_hexpand(false);
        obj.set_vexpand(false);
        obj.set_halign(gtk::Align::End);
        obj.set_valign(gtk::Align::End);
        obj.set_tooltip_text(Some(r#"Record screen"#));
        obj.add_css_class(CSS_CLASS_TOOLBOX_BTN);
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);
    }
}

impl WidgetImpl for ScreenRecorder {}
impl ButtonImpl for ScreenRecorder {}
