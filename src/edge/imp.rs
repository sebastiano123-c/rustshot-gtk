use gtk::glib;
use gtk::subclass::prelude::*;

use gtk::prelude::*;
use rustshot_gtk::constants::CSS_CLASS_GRAY_BOX;
use std::cell::Cell;
use std::rc::Rc;

// Object holding the state
#[derive(Debug, Default)]
pub struct GrayEdge {
    pub edge: Rc<Cell<i32>>,
    pub edge_f64: Rc<Cell<f64>>,
    pub orientation: Rc<Cell<bool>>, // 0: H, 1: V
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for GrayEdge {
    const NAME: &'static str = "GrayEdge";
    type Type = super::GrayEdge;
    type ParentType = gtk::Box;

    fn new() -> Self {
        Self {
            edge: Rc::new(Cell::new(0)),
            edge_f64: Rc::new(Cell::new(0.0)),
            orientation: Rc::new(Cell::new(false)),
        }
    }
}

// Trait shared by all GObjects
impl ObjectImpl for GrayEdge {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.add_css_class(CSS_CLASS_GRAY_BOX);
    }
}

// Trait shared by all widgets
impl BoxImpl for GrayEdge {}

// Trait shared by all widgets
impl WidgetImpl for GrayEdge {}
