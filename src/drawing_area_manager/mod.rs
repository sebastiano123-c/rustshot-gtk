pub mod drawables;
mod imp;

use crate::drawing_area_manager::drawables::{
    DragBegin, DragEnd, DragUpdate, Draw, DrawableCollection,
};

use gtk::{cairo, glib, pango, prelude::WidgetExt, subclass::prelude::*};

glib::wrapper! {
    pub struct DrawingAreaManager(ObjectSubclass<imp::DrawingAreaManager>)
        @extends gtk::DrawingArea,
        @implements
            gtk::Accessible,
            gtk::Buildable,
            gtk::ConstraintTarget,
            gtk::Widget;
}

impl Default for DrawingAreaManager {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl DrawingAreaManager {
    pub fn create_new_drawable(&self, drawable: &DrawableCollection) {
        let imp = self.imp();
        *imp.current_item.borrow_mut() = Some(drawable.clone());
        imp.is_drawing.set(true);
    }

    pub fn is_drawing(&self) -> bool {
        let imp = self.imp();
        imp.is_drawing.get()
    }

    pub fn set_drawing(&self, flag: bool) {
        let imp = self.imp();
        imp.is_drawing.set(flag)
    }

    // pub fn set_rgba(&self, r: f64, g: f64, b: f64, a: f64) {
    //     let imp = self.imp();
    //     if let Some(ref item) = *imp.current_item.borrow() {
    //         item.set_rgba(r, g, b, a);
    //     }
    // }

    fn draw_elements(&self, cr: &cairo::Context) {
        let imp = self.imp();
        let pg: pango::Layout = self.create_pango_layout(None);
        for element in &*imp.drawn_items.borrow() {
            element.draw_with_saved_settings(cr, &pg);
        }
    }

    fn draw_current_element(&self, cr: &cairo::Context) {
        let imp = self.imp();
        let pg: pango::Layout = self.create_pango_layout(None);
        if let Some(ref item) = *imp.current_item.borrow() {
            item.draw(cr, &pg);
        }
    }

    pub fn set_draw(&self, cr: &cairo::Context) {
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.0); // transparent background
        cr.paint().unwrap();

        self.draw_elements(cr);
        self.draw_current_element(cr);
    }

    pub fn drag_begin(&self, x: f64, y: f64) {
        let imp = self.imp();
        if let Some(drawable) = imp.current_item.borrow_mut().as_mut() {
            drawable.drag_begin(x, y);
        } else {
            eprintln!("⚠️ drag_begin called but no current_item is set");
        }
    }

    pub fn drag_update(&self, x: f64, y: f64) {
        let imp = self.imp();
        if let Some(drawable) = imp.current_item.borrow_mut().as_mut() {
            drawable.drag_update(x, y);
        } else {
            eprintln!("⚠️ drag_update called but no current_item is set");
        }
    }

    pub fn drag_end(&self) {
        // Borrow the current item
        let imp = self.imp();
        let mut maybe_drawable = imp.current_item.borrow_mut();

        // Create the next drawable
        let mut new_drawable: Option<DrawableCollection> = None;

        match &*maybe_drawable {
            Some(drawable) => {
                // Store it in the drawn items list
                imp.drawn_items.borrow_mut().push(drawable.clone());

                // Create new drawable
                new_drawable = Some(drawable.drag_end());
            }
            _ => eprintln!("⚠️ drag_end called but no current_item is set"),
        }

        // Create the next drawable
        *maybe_drawable = Some(new_drawable.clone().unwrap());
    }
}
