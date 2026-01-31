pub mod drawables;
mod imp;

use crate::drawing_area_manager::drawables::{
    ControllerKey, DragBegin, DragEnd, DragUpdate, Draw, DrawableCollection,
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

        match imp.current_item.borrow().as_ref() {
            Some(drawable) => match drawable {
                DrawableCollection::InputTexts(item) => {
                    // TODO: when changing the color, the same applies to all text...
                    // See the "event_controller_key" function for the first part of the logic explanation.
                    //
                    // The user has pressed "esc" because he has finished typing. He is actually typing on the
                    // drawn_items.last() element. So, to continue with the second InputText (which is actually the
                    // current_item), we need to:
                    //     1. create a new InputText instance via the drawable.stop_controller_key() function;
                    //     2. push the current_item into the drawn_items so that event_controller_key can access
                    //        and change its text;
                    //     3. make the new InputText the new current_item.
                    //
                    // Finally, note that if another drawable (like Arc, Box, Freehand, ...) is created, the
                    // current_item is normally filled, and the transition is smooth.
                    let mut items = imp.drawn_items.borrow_mut();
                    if let Some(last_drawable) = items.last() {
                        let _ = last_drawable.stop_controller_key();
                    }

                    items.push(drawable.clone());
                    item.drag_begin(x, y);
                }
                _ => {
                    drawable.drag_begin(x, y);
                }
            },
            None => {
                eprintln!("⚠️ drag_begin called but no current_item is set");
            }
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

        // Create the next drawable
        let mut new_drawable: Option<DrawableCollection> = None;

        if let Some(drawable) = imp.current_item.borrow_mut().as_mut() {
            // Store it in the drawn items list
            imp.drawn_items.borrow_mut().push(drawable.clone());

            // Create new drawable
            new_drawable = drawable.drag_end();
        } else {
            eprintln!("⚠️ drag_end called but no current_item is set");
        }

        // Create the next drawable
        *imp.current_item.borrow_mut() = new_drawable.clone();
    }

    /// This function is useful only for the InputText drawables.
    /// The logic for InputText items is the following.
    ///     1. Press the button and create the item;
    ///     2. Drag_begin instatiate the position of the box;
    ///     3. Drag_end stores the current InputText and returns a new InputText that becomes the
    ///        new current_item.
    ///
    /// Up to now, there is: the first InputText as drawn_items.last(), and the next InputText is
    /// the current_item.
    ///
    /// A listener is create in geometry::mod::attach_gestures(), so whenever the user writes on
    /// the keyboard, the settings.input_text["text"] is filled.
    /// If the key pressed is not "esc", the text increases and the "event_controller_key()"
    /// function is called. So, this function needs to get drawn_items.last() and modify its
    /// self.text and then queue a new redraw.
    ///
    /// See the "stop_controller_key" function for a continuation.
    pub fn event_controller_key(&self) {
        let imp = self.imp();

        // The last element that needs to be written is InputText
        if let Some(drawable) = imp.drawn_items.borrow_mut().last() {
            drawable.event_controller_key();
            self.queue_draw();
        } else {
            eprintln!("⚠️ event_controller_key called but no current_item is set");
        }
    }
}
