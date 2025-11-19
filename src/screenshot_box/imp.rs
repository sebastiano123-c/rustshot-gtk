use gtk::glib;
use gtk::subclass::prelude::*;

use rustshot_gtk::constants::CSS_CLASS_TRANSPARENT;

use crate::handle::Handle;
use gtk::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

// Object holding the state
#[derive(Debug, Default)]
pub struct ScreenshotBox {
    pub central_handle_sensitive: Rc<Cell<bool>>,
    pub handles: RefCell<Vec<Handle>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ScreenshotBox {
    const NAME: &'static str = "ScreenshotBox";
    type Type = super::ScreenshotBox;
    type ParentType = gtk::Box;

    fn new() -> Self {
        Self {
            handles: RefCell::new(Vec::new()),
            central_handle_sensitive: Rc::new(Cell::new(true)),
        }
    }
}

// Trait shared by all GObjects
impl ObjectImpl for ScreenshotBox {
    fn constructed(&self) {
        let obj = self.obj();
        obj.set_hexpand(true);
        obj.set_orientation(gtk::Orientation::Vertical);

        // create handles top
        let top_spacer = obj.make_spacer(0, CSS_CLASS_TRANSPARENT, gtk::Align::Start);

        // create handles center
        let center_spacer = obj.make_spacer(1, CSS_CLASS_TRANSPARENT, gtk::Align::Fill);

        // create handles bottom
        let bottom_spacer = obj.make_spacer(2, CSS_CLASS_TRANSPARENT, gtk::Align::End);

        // append to screenshot box
        obj.append(&top_spacer);
        obj.append(&center_spacer);
        obj.append(&bottom_spacer);
    }
}

// Trait shared by all widgets
impl BoxImpl for ScreenshotBox {}

// Trait shared by all widgets
impl WidgetImpl for ScreenshotBox {}
