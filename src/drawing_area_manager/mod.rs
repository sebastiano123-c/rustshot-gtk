mod drawables;
use core::f64;
use drawables::{
    Arc, AreaBox, Arrow, DragBegin, DragUpdate, Draw, DrawableCollection, FreeHandDraw, Line,
    NumberedCircle, SetRGBA,
};
use gtk::cairo;

#[derive(Debug, Clone)]
pub struct DrawingAreaManager {
    drawn_items: Vec<DrawableCollection>,
    current_item: Option<DrawableCollection>,
    numbered_circle_idx: usize,
    pub is_drawing: bool,
}

impl DrawingAreaManager {
    pub fn new() -> Self {
        Self {
            current_item: None,
            drawn_items: Vec::new(),
            numbered_circle_idx: 1,
            is_drawing: false,
        }
    }

    pub fn create_new_freehand_draw(&mut self, size: f64, r: f64, g: f64, b: f64, a: f64) {
        self.reset();
        self.current_item = Some(DrawableCollection::FreeHands(FreeHandDraw {
            x0: 0.0,
            y0: 0.0,
            x: Vec::new(),
            y: Vec::new(),
            size,
            red: r,
            green: g,
            blue: b,
            alpha: a,
            drawing: false,
        }));

        // emit signal for drawing boxes
        self.is_drawing = true;
    }

    pub fn create_new_line(&mut self, arrow_size: f64, r: f64, g: f64, b: f64, a: f64) {
        self.reset();
        self.current_item = Some(DrawableCollection::Lines(Line {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
            size: arrow_size,
            red: r,
            green: g,
            blue: b,
            alpha: a,
            drawing: false,
        }));

        // emit signal for drawing boxes
        self.is_drawing = true;
    }

    pub fn create_new_arrow(
        &mut self,
        arrow_size: f64,
        arrow_width: f64,
        r: f64,
        g: f64,
        b: f64,
        a: f64,
    ) {
        self.reset();
        self.current_item = Some(DrawableCollection::Arrows(Arrow {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
            size: arrow_size,
            width: arrow_width,
            red: r,
            green: g,
            blue: b,
            alpha: a,
            drawing: false,
        }));

        // emit signal for drawing boxes
        self.is_drawing = true;
    }

    pub fn create_new_numbered_circle(
        &mut self,
        rad: f64,
        font_size: f64,
        // font_name: &str,
        font_color_r: f64,
        font_color_g: f64,
        font_color_b: f64,
        red: f64,
        green: f64,
        blue: f64,
        alpha: f64,
    ) {
        self.reset();

        // compute number
        let mut buffer = itoa::Buffer::new();
        let num_str = buffer.format(self.numbered_circle_idx);
        self.numbered_circle_idx += 1;

        self.current_item = Some(DrawableCollection::NumberedCircles(NumberedCircle {
            radius: rad,
            x0: 0.0,
            y0: 0.0,
            font_size,
            // font_name: font_name,
            number_text: num_str.to_string(),
            font_r: font_color_r,
            font_g: font_color_g,
            font_b: font_color_b,
            red,
            green,
            blue,
            alpha,
        }));

        // emit signal for drawing boxes
        self.is_drawing = true;
    }

    pub fn create_new_arc(
        &mut self,
        red: f64,
        green: f64,
        blue: f64,
        alpha: f64,
        border_size: Option<f64>,
    ) {
        self.reset();

        self.current_item = Some(DrawableCollection::Arcs(Arc {
            radius: 0.0,
            center_x: 0.0,
            center_y: 0.0,
            red,
            green,
            blue,
            alpha,
            drawing: false,
            border: border_size,
        }));

        // emit signal for drawing boxes
        self.is_drawing = true;
    }

    pub fn create_new_box(
        &mut self,
        red: f64,
        green: f64,
        blue: f64,
        alpha: f64,
        border_size: Option<f64>,
    ) {
        self.reset();
        self.current_item = Some(DrawableCollection::AreaBoxes(AreaBox {
            start_x: 0.0,
            start_y: 0.0,
            end_x: 0.0,
            end_y: 0.0,
            red,
            green,
            blue,
            alpha,
            drawing: false,
            border: border_size,
        }));

        // emit signal for drawing boxes
        self.is_drawing = true;
    }

    pub fn is_drawing(&self) -> bool {
        self.is_drawing
    }

    pub fn set_rgba(&mut self, r: f64, g: f64, b: f64, a: f64) {
        if let Some(item) = self.current_item.as_mut() {
            item.set_rgba(r, g, b, a);
        }
    }

    fn draw_elements(&self, cr: &cairo::Context) {
        for element in &self.drawn_items {
            element.draw(cr);
        }
    }

    fn draw_current_element(&mut self, cr: &cairo::Context) {
        if let Some(item) = self.current_item.as_mut() {
            item.draw(cr);
        }
    }

    pub fn set_draw(&mut self, cr: &cairo::Context) {
        // Clear the drawing area
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.0); // transparent background
        cr.paint().unwrap();

        // Draw all the elements
        self.draw_elements(cr);

        // Draw the current element if it's being drawn
        self.draw_current_element(cr);
    }

    pub fn drag_begin(&mut self, x: f64, y: f64) {
        self.current_item
            .as_mut()
            .expect("current_item is None")
            .drag_begin(x, y);
    }

    pub fn drag_update(&mut self, x: f64, y: f64) {
        self.current_item
            .as_mut()
            .expect("current_item is None")
            .drag_update(x, y);
    }

    pub fn drag_end(&mut self) {
        // Borrow the current item
        let current_item = self
            .current_item
            .take()
            .expect("drag_end current_item is None");

        // Store it in the drawn items list
        self.drawn_items.push(current_item.clone());

        // Once the current item is stored there is nothing left to draw.
        // We need to trigger the generation of a new current item.
        // Otherwise, we will need to press again the button
        self.generate_new_item(&current_item);
    }

    fn generate_new_item(&mut self, current_item: &DrawableCollection) {
        match current_item {
            DrawableCollection::AreaBoxes(state) => {
                let (f, r, g, b, a) = (
                    state.border,
                    state.red,
                    state.green,
                    state.blue,
                    state.alpha,
                );
                self.create_new_box(r, g, b, a, f);
            }
            DrawableCollection::Arcs(state) => {
                let (f, r, g, b, a) = (
                    state.border,
                    state.red,
                    state.green,
                    state.blue,
                    state.alpha,
                );
                self.create_new_arc(r, g, b, a, f);
            }
            DrawableCollection::Lines(state) => {
                let (sz, r, g, b, a) =
                    (state.size, state.red, state.green, state.blue, state.alpha);
                self.create_new_line(sz, r, g, b, a);
            }
            DrawableCollection::Arrows(state) => {
                let (sz, w, r, g, b, a) = (
                    state.size,
                    state.width,
                    state.red,
                    state.green,
                    state.blue,
                    state.alpha,
                );
                self.create_new_arrow(sz, w, r, g, b, a);
            }
            DrawableCollection::FreeHands(state) => {
                let (sz, r, g, b, a) =
                    (state.size, state.red, state.green, state.blue, state.alpha);
                self.create_new_freehand_draw(sz, r, g, b, a);
            }
            DrawableCollection::NumberedCircles(state) => {
                let (rad, fs, fr, fg, fb, r, g, b, a) = (
                    state.radius,
                    state.font_size,
                    state.font_r,
                    state.font_g,
                    state.font_b,
                    state.red,
                    state.green,
                    state.blue,
                    state.alpha,
                );
                self.create_new_numbered_circle(rad, fs, fr, fg, fb, r, g, b, a);
            }
        }
    }

    fn reset(&mut self) {
        self.current_item = None;
    }
}
