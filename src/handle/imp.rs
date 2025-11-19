use std::cell::Cell;
use std::rc::Rc;

use gtk::{glib, subclass::prelude::*};

#[derive(Debug, Default)]
pub struct Handle {
    pub col: Rc<Cell<u8>>,
    pub row: Rc<Cell<u8>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Handle {
    const NAME: &'static str = "Handle";
    type Type = super::Handle;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        // The layout manager determines how child widgets are laid out.
        klass.set_layout_manager_type::<gtk::BinLayout>();

        // // Make it look like a GTK button.
        // klass.set_css_name(CSS_CLASS_HANDLES);
        //
        // // Make it appear as a button to accessibility tools.
        // klass.set_accessible_role(gtk::AccessibleRole::Button);
    }

    fn new() -> Self {
        Self {
            col: Rc::new(Cell::new(0)),
            row: Rc::new(Cell::new(0)),
        }
    }
}

impl ObjectImpl for Handle {}

// impl WidgetImplExt for Handle {}
impl WidgetImpl for Handle {}
impl BoxImpl for Handle {}
