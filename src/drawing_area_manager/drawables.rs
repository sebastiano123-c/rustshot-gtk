use core::f64;
use gtk::cairo;

#[derive(Debug, Clone)]
pub struct NumberedCircle {
    pub x0: f64,
    pub y0: f64,
    pub radius: f64,
    pub font_size: f64,
    pub number_text: String,
    // pub number_text: &'static str,
    // font_name: &' str,
    pub font_r: f64,
    pub font_g: f64,
    pub font_b: f64,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

#[derive(Debug, Clone)]
pub struct FreeHandDraw {
    pub x0: f64,
    pub y0: f64,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub size: f64,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
    pub drawing: bool,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub size: f64,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
    pub drawing: bool,
}

#[derive(Debug, Clone)]
pub struct Arrow {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub size: f64,
    pub width: f64,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
    pub drawing: bool,
    // fill: bool,
}

#[derive(Debug, Clone)]
pub struct Arc {
    pub radius: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub drawing: bool,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
    pub border: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct AreaBox {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub drawing: bool,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
    pub border: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum Drawable {
    NumberedCircle,
    FreeHandDraw,
    Line,
    Arrow,
    Arc,
    AreaBox,
}

#[derive(Debug, Clone)]
pub enum DrawableCollection {
    NumberedCircles(NumberedCircle),
    FreeHands(FreeHandDraw),
    Lines(Line),
    Arrows(Arrow),
    Arcs(Arc),
    AreaBoxes(AreaBox),
}

// Drag Begin trait
pub trait DragBegin {
    fn drag_begin(&mut self, x: f64, y: f64);
}

impl DragBegin for AreaBox {
    fn drag_begin(&mut self, x: f64, y: f64) {
        self.start_x = x;
        self.start_y = y;
        self.end_x = 0.0;
        self.end_y = 0.0;
        self.drawing = true;
    }
}
impl DragBegin for Arc {
    fn drag_begin(&mut self, x: f64, y: f64) {
        self.center_x = x;
        self.center_y = y;
        self.radius = 0.0;
        self.drawing = true;
    }
}
impl DragBegin for Arrow {
    fn drag_begin(&mut self, x: f64, y: f64) {
        self.x1 = x;
        self.y1 = y;
        self.x2 = x;
        self.y2 = y;
        self.drawing = true;
    }
}
impl DragBegin for Line {
    fn drag_begin(&mut self, x: f64, y: f64) {
        self.x1 = x;
        self.y1 = y;
        self.x2 = x;
        self.y2 = y;
        self.drawing = true;
    }
}
impl DragBegin for FreeHandDraw {
    fn drag_begin(&mut self, x: f64, y: f64) {
        self.x0 = x;
        self.y0 = y;
        self.drawing = true;
    }
}
impl DragBegin for NumberedCircle {
    fn drag_begin(&mut self, x: f64, y: f64) {
        self.x0 = x;
        self.y0 = y;
    }
}
impl DragBegin for DrawableCollection {
    fn drag_begin(&mut self, x: f64, y: f64) {
        match self {
            DrawableCollection::FreeHands(state) => {
                state.drag_begin(x, y);
            }
            DrawableCollection::Lines(state) => {
                state.drag_begin(x, y);
            }
            DrawableCollection::AreaBoxes(state) => {
                state.drag_begin(x, y);
            }
            DrawableCollection::Arcs(state) => {
                state.drag_begin(x, y);
            }
            DrawableCollection::Arrows(state) => {
                state.drag_begin(x, y);
            }
            DrawableCollection::NumberedCircles(state) => {
                state.drag_begin(x, y);
            }
        }
    }
}

// Drag Update trait
pub trait DragUpdate {
    fn drag_update(&mut self, x: f64, y: f64);
}

impl DragUpdate for AreaBox {
    fn drag_update(&mut self, x: f64, y: f64) {
        self.end_x = x;
        self.end_y = y;
    }
}
impl DragUpdate for Arc {
    fn drag_update(&mut self, x: f64, y: f64) {
        self.radius = f64::sqrt(x * x + y * y);
    }
}
impl DragUpdate for Line {
    fn drag_update(&mut self, x: f64, y: f64) {
        self.x1 = self.x2 + x;
        self.y1 = self.y2 + y;
    }
}
impl DragUpdate for Arrow {
    fn drag_update(&mut self, x: f64, y: f64) {
        self.x1 = self.x2 + x;
        self.y1 = self.y2 + y;
    }
}
impl DragUpdate for FreeHandDraw {
    fn drag_update(&mut self, x: f64, y: f64) {
        self.x.push(self.x0 + x);
        self.y.push(self.y0 + y);
    }
}
impl DragUpdate for DrawableCollection {
    fn drag_update(&mut self, x: f64, y: f64) {
        match self {
            DrawableCollection::FreeHands(state) => {
                state.drag_update(x, y);
            }
            DrawableCollection::Lines(state) => {
                state.drag_update(x, y);
            }
            DrawableCollection::AreaBoxes(state) => {
                state.drag_update(x, y);
            }
            DrawableCollection::Arcs(state) => {
                state.drag_update(x, y);
            }
            DrawableCollection::Arrows(state) => {
                state.drag_update(x, y);
            }
            _ => {}
        }
    }
}

// // Drag End trait
// pub trait DragEnd {
//     fn drag_end(&mut self);
// }

// Draw trait
pub trait Draw {
    fn draw(&self, cr: &cairo::Context);
}

impl Draw for FreeHandDraw {
    fn draw(&self, cr: &cairo::Context) {
        // get freehands size
        let sz = self.x.iter().len();

        if sz == 0 {
            return;
        }

        // define previous point
        let mut x_prev = self.x[0];
        let mut y_prev = self.y[0];

        // for a continuous draw we need to fill the blank space between points
        let mut cc: usize = 1;
        while cc < sz {
            // get current x and y
            let (x, y) = (self.x[cc], self.y[cc]);

            cr.set_source_rgba(self.red, self.green, self.blue, self.alpha); // Set color
            // RGBA
            // Draw the line
            cr.move_to(x_prev, y_prev);
            cr.line_to(x, y);
            // Set line properties
            cr.set_line_width(self.size);
            cr.set_line_cap(cairo::LineCap::Round);
            cr.stroke().unwrap();

            x_prev = x;
            y_prev = y;

            // update counter
            cc += 1;
        }
    }
}
impl Draw for Line {
    fn draw(&self, cr: &cairo::Context) {
        cr.set_source_rgba(self.red, self.green, self.blue, self.alpha); // Set color RGBA

        // Draw the line
        cr.move_to(self.x1, self.y1);
        cr.line_to(self.x2, self.y2);

        // Set line properties
        cr.set_line_width(self.size);
        cr.set_line_cap(cairo::LineCap::Round);
        cr.stroke().unwrap();
    }
}
impl Draw for Arrow {
    fn draw(&self, cr: &cairo::Context) {
        cr.set_source_rgba(self.red, self.green, self.blue, self.alpha); // Set color RGBA

        // Draw the line
        cr.move_to(self.x1, self.y1);
        cr.line_to(self.x2, self.y2);

        // Draw the arrowhead
        let arrow_size = self.size;
        let angle = (self.y2 - self.y1).atan2(self.x2 - self.x1);
        cr.line_to(
            self.x2 - arrow_size * angle.cos() + arrow_size * angle.sin(),
            self.y2 - arrow_size * angle.sin() - arrow_size * angle.cos(),
        );
        cr.move_to(self.x2, self.y2);
        cr.line_to(
            self.x2 - arrow_size * angle.cos() - arrow_size * angle.sin(),
            self.y2 - arrow_size * angle.sin() + arrow_size * angle.cos(),
        );

        // Set line properties
        cr.set_line_width(self.width);
        cr.set_line_cap(cairo::LineCap::Round);
        cr.stroke().unwrap();
    }
}
impl Draw for Arc {
    fn draw(&self, cr: &cairo::Context) {
        cr.set_source_rgba(self.red, self.green, self.blue, self.alpha); // Set color RGBA
        cr.arc(
            self.center_x,
            self.center_y,
            self.radius,
            0.0,
            2.0 * f64::consts::PI,
        );
        if let Some(border_size) = self.border {
            cr.set_line_width(border_size);
        } else {
            cr.fill().expect("No arc fill to unwrap");
        }
        cr.stroke().unwrap();
    }
}
impl Draw for AreaBox {
    fn draw(&self, cr: &cairo::Context) {
        cr.set_source_rgba(self.red, self.green, self.blue, self.alpha); // Set color RGBA
        cr.rectangle(self.start_x, self.start_y, self.end_x, self.end_y);
        if let Some(border_size) = self.border {
            cr.set_line_width(border_size);
        } else {
            cr.fill().expect("No arc fill to unwrap");
        }
        cr.stroke().unwrap();
    }
}
impl Draw for NumberedCircle {
    fn draw(&self, cr: &cairo::Context) {
        cr.set_source_rgba(self.red, self.green, self.blue, self.alpha); // Set color RGBA
        cr.arc(self.x0, self.y0, self.radius, 0.0, 2.0 * f64::consts::PI);
        cr.fill().expect("No numbered circle fill to unwrap");
        cr.stroke().unwrap();

        // Set text properties
        cr.set_source_rgb(self.font_r, self.font_g, self.font_b); // Set text color
        cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
        cr.set_font_size(self.font_size);

        // Draw text in the center of the rectangle
        let text = self.number_text.clone(); // format!("{}", self.numbered_circles.iter().le());
        let (text_x, text_y) = (
            self.x0 - cr.text_extents(&text).unwrap().width() / 2.0 - 1.0,
            self.y0 + self.font_size / 2.0,
        );
        cr.move_to(text_x, text_y);
        cr.show_text(&text).expect("Failed to show text");
        cr.stroke().unwrap();
    }
}
impl Draw for DrawableCollection {
    fn draw(&self, cr: &cairo::Context) {
        match self {
            DrawableCollection::FreeHands(state) => {
                state.draw(cr);
            }
            DrawableCollection::AreaBoxes(state) => {
                state.draw(cr);
            }
            DrawableCollection::Lines(state) => {
                state.draw(cr);
            }
            DrawableCollection::Arcs(state) => {
                state.draw(cr);
            }
            DrawableCollection::Arrows(state) => {
                state.draw(cr);
            }
            DrawableCollection::NumberedCircles(state) => {
                state.draw(cr);
            }
        }
    }
}

// Set RGBA trait
pub trait SetRGBA {
    fn set_rgba(&mut self, r: f64, g: f64, b: f64, a: f64);
}

impl SetRGBA for AreaBox {
    fn set_rgba(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.red = r;
        self.green = g;
        self.blue = b;
        self.alpha = a;
    }
}
impl SetRGBA for Arc {
    fn set_rgba(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.red = r;
        self.green = g;
        self.blue = b;
        self.alpha = a;
    }
}
impl SetRGBA for NumberedCircle {
    fn set_rgba(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.red = r;
        self.green = g;
        self.blue = b;
        self.alpha = a;
    }
}
impl SetRGBA for Line {
    fn set_rgba(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.red = r;
        self.green = g;
        self.blue = b;
        self.alpha = a;
    }
}
impl SetRGBA for Arrow {
    fn set_rgba(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.red = r;
        self.green = g;
        self.blue = b;
        self.alpha = a;
    }
}
impl SetRGBA for FreeHandDraw {
    fn set_rgba(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.red = r;
        self.green = g;
        self.blue = b;
        self.alpha = a;
    }
}
impl SetRGBA for DrawableCollection {
    fn set_rgba(&mut self, r: f64, g: f64, b: f64, a: f64) {
        match self {
            DrawableCollection::AreaBoxes(state) => {
                state.set_rgba(r, g, b, a);
            }
            DrawableCollection::Lines(state) => {
                state.set_rgba(r, g, b, a);
            }
            DrawableCollection::Arcs(state) => {
                state.set_rgba(r, g, b, a);
            }
            DrawableCollection::Arrows(state) => {
                state.set_rgba(r, g, b, a);
            }
            DrawableCollection::FreeHands(state) => {
                state.set_rgba(r, g, b, a);
            }
            DrawableCollection::NumberedCircles(state) => {
                state.set_rgba(r, g, b, a);
            }
        }
    }
}
