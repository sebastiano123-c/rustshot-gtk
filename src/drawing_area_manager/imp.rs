use crate::drawing_area_manager::drawables::DrawableCollection;
use gtk::prelude::DrawingAreaExtManual;
use gtk::{glib, subclass::prelude::*};

use rustshot_gtk::constants::CSS_CLASS_TRANSPARENT;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct DrawingAreaManager {
    pub drawn_items: RefCell<Vec<DrawableCollection>>,
    pub current_item: RefCell<Option<DrawableCollection>>,
    pub is_drawing: Rc<Cell<bool>>,
}

#[glib::object_subclass]
impl ObjectSubclass for DrawingAreaManager {
    const NAME: &'static str = "DrawingAreaManager";
    type Type = super::DrawingAreaManager;
    type ParentType = gtk::DrawingArea;

    fn class_init(klass: &mut Self::Class) {
        klass.set_css_name(CSS_CLASS_TRANSPARENT);
    }

    fn new() -> Self {
        // Here we set the default orientation.
        Self {
            current_item: None.into(),
            drawn_items: Vec::new().into(),
            // numbered_circle_idx: 1.into(),
            is_drawing: Rc::new(Cell::new(false)),
        }
    }
}

impl ObjectImpl for DrawingAreaManager {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.set_draw_func(glib::clone!(
            #[weak]
            obj,
            move |_, cr, _, _| {
                obj.set_draw(cr);
            }
        ));
    }
}

impl WidgetImpl for DrawingAreaManager {}
impl DrawingAreaImpl for DrawingAreaManager {}
