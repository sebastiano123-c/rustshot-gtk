use gtk::{glib, subclass::prelude::*};

use rustshot_gtk::constants::CSS_CLASS_TRANSPARENT;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Toolbox {
    pub n_buttons: Rc<Cell<usize>>,
    pub buttons_list: RefCell<Vec<gtk::Widget>>,
    pub button_pressed: Rc<Cell<bool>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Toolbox {
    const NAME: &'static str = "Toolbox";
    type Type = super::Toolbox;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.set_css_name(CSS_CLASS_TRANSPARENT);
    }

    fn new() -> Self {
        Self {
            n_buttons: Rc::new(Cell::new(0)),
            buttons_list: RefCell::new(Vec::new()),
            button_pressed: Rc::new(Cell::new(false)),
        }
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Toolbox {}

// Trait shared by all widgets
impl BoxImpl for Toolbox {}

// Trait shared by all widgets
impl WidgetImpl for Toolbox {}
