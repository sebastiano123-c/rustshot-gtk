use crate::drawing_area_settings::{HandleSettings, Settings, SettingsRc};
use core::f64;
use gtk::cairo;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

static mut NUMBERED_CIRCLE_IDX: i32 = 0;

#[derive(Debug, Clone)]
pub struct NumberedCircle {
    pub x0: Rc<Cell<f64>>,
    pub y0: Rc<Cell<f64>>,
    pub number_text: String,
    pub settings: SettingsRc,
    pub saved_settings: Rc<Cell<Option<Settings>>>,
    pub drawing: Rc<Cell<bool>>,
}

#[derive(Debug, Clone)]
pub struct FreeHandDraw {
    pub x0: Rc<Cell<f64>>,
    pub y0: Rc<Cell<f64>>,
    pub x: RefCell<Vec<f64>>,
    pub y: RefCell<Vec<f64>>,
    pub settings: SettingsRc,
    pub saved_settings: Rc<Cell<Option<Settings>>>,
    pub drawing: Rc<Cell<bool>>,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub x1: Rc<Cell<f64>>,
    pub y1: Rc<Cell<f64>>,
    pub x2: Rc<Cell<f64>>,
    pub y2: Rc<Cell<f64>>,
    pub settings: SettingsRc,
    pub saved_settings: Rc<Cell<Option<Settings>>>,
    pub drawing: Rc<Cell<bool>>,
}

#[derive(Debug, Clone)]
pub struct Arrow {
    pub x1: Rc<Cell<f64>>,
    pub y1: Rc<Cell<f64>>,
    pub x2: Rc<Cell<f64>>,
    pub y2: Rc<Cell<f64>>,
    pub settings: SettingsRc,
    pub saved_settings: Rc<Cell<Option<Settings>>>,
    pub drawing: Rc<Cell<bool>>,
    // fill: bool,
}

#[derive(Debug, Clone)]
pub struct Arc {
    pub radius: Rc<Cell<f64>>,
    pub center_x: Rc<Cell<f64>>,
    pub center_y: Rc<Cell<f64>>,
    pub settings: SettingsRc,
    pub border: Option<f64>,
    pub saved_settings: Rc<Cell<Option<Settings>>>,
    pub drawing: Rc<Cell<bool>>,
}

#[derive(Debug, Clone)]
pub struct AreaBox {
    pub start_x: Rc<Cell<f64>>,
    pub start_y: Rc<Cell<f64>>,
    pub end_x: Rc<Cell<f64>>,
    pub end_y: Rc<Cell<f64>>,
    pub settings: SettingsRc,
    pub border: Option<f64>,
    pub saved_settings: Rc<Cell<Option<Settings>>>,
    pub drawing: Rc<Cell<bool>>,
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

impl AreaBox {
    pub fn new(settings_rc: &SettingsRc, border_size: Option<f64>) -> Self {
        Self {
            start_x: Rc::new(Cell::new(0.0)),
            start_y: Rc::new(Cell::new(0.0)),
            end_x: Rc::new(Cell::new(0.0)),
            end_y: Rc::new(Cell::new(0.0)),
            settings: settings_rc.clone(),
            saved_settings: Rc::new(Cell::new(None)),
            drawing: Rc::new(Cell::new(true)),
            border: border_size,
        }
    }
    pub fn is_drawing(&self) -> bool {
        self.drawing.get()
    }
}
impl FreeHandDraw {
    pub fn new(settings_rc: &SettingsRc, _border_size: Option<f64>) -> Self {
        let vex_x: Vec<f64> = Vec::new();
        let vex_y: Vec<f64> = Vec::new();

        Self {
            x0: Rc::new(Cell::new(0.0)),
            y0: Rc::new(Cell::new(0.0)),
            x: RefCell::new(vex_x),
            y: RefCell::new(vex_y),
            settings: settings_rc.clone(),
            saved_settings: Rc::new(Cell::new(None)),
            drawing: Rc::new(Cell::new(false)),
        }
    }
    pub fn is_drawing(&self) -> bool {
        self.drawing.get()
    }
}
impl Line {
    pub fn new(settings_rc: &SettingsRc, _border_size: Option<f64>) -> Self {
        Self {
            x1: Rc::new(Cell::new(0.0)),
            y1: Rc::new(Cell::new(0.0)),
            x2: Rc::new(Cell::new(0.0)),
            y2: Rc::new(Cell::new(0.0)),
            settings: settings_rc.clone(),
            saved_settings: Rc::new(Cell::new(None)),
            drawing: Rc::new(Cell::new(false)),
        }
    }
    pub fn is_drawing(&self) -> bool {
        self.drawing.get()
    }
}
impl Arrow {
    pub fn new(settings_rc: &SettingsRc, _border_size: Option<f64>) -> Self {
        Arrow {
            x1: Rc::new(Cell::new(0.0)),
            y1: Rc::new(Cell::new(0.0)),
            x2: Rc::new(Cell::new(0.0)),
            y2: Rc::new(Cell::new(0.0)),
            settings: settings_rc.clone(),
            saved_settings: Rc::new(Cell::new(None)),
            drawing: Rc::new(Cell::new(false)),
        }
    }
    pub fn is_drawing(&self) -> bool {
        self.drawing.get()
    }
}
impl NumberedCircle {
    pub fn new(settings_rc: &SettingsRc, _border_size: Option<f64>) -> Self {
        let mut buffer = itoa::Buffer::new();
        let num_str: &str;

        unsafe {
            num_str = buffer.format(NUMBERED_CIRCLE_IDX);
            NUMBERED_CIRCLE_IDX += 1;
        }

        Self {
            x0: Rc::new(Cell::new(0.0)),
            y0: Rc::new(Cell::new(0.0)),
            number_text: num_str.to_string(),
            settings: settings_rc.clone(),
            saved_settings: Rc::new(Cell::new(None)),
            drawing: Rc::new(Cell::new(false)),
        }
    }
    pub fn is_drawing(&self) -> bool {
        self.drawing.get()
    }
}
impl Arc {
    pub fn new(settings_rc: &SettingsRc, border_size: Option<f64>) -> Self {
        Self {
            radius: Rc::new(Cell::new(0.0)),
            center_x: Rc::new(Cell::new(0.0)),
            center_y: Rc::new(Cell::new(0.0)),
            settings: settings_rc.clone(),
            border: border_size,
            saved_settings: Rc::new(Cell::new(None)),
            drawing: Rc::new(Cell::new(false)),
        }
    }
    pub fn is_drawing(&self) -> bool {
        self.drawing.get()
    }
}

// // Trait is drawing
// pub trait IsDrawing {
//     fn is_drawing(&self) -> bool;
// }
// impl IsDrawing for DrawableCollection {
//     fn is_drawing(&self) -> bool {
//         match self {
//             DrawableCollection::FreeHands(state) => state.is_drawing(),
//             DrawableCollection::Lines(state) => state.is_drawing(),
//             DrawableCollection::AreaBoxes(state) => state.is_drawing(),
//             DrawableCollection::Arcs(state) => state.is_drawing(),
//             DrawableCollection::Arrows(state) => state.is_drawing(),
//             DrawableCollection::NumberedCircles(state) => state.is_drawing(),
//         }
//     }
// }

// Drag Begin trait
pub trait DragBegin {
    fn drag_begin(&self, x: f64, y: f64);
}
impl DragBegin for AreaBox {
    fn drag_begin(&self, x: f64, y: f64) {
        self.start_x.set(x);
        self.start_y.set(y);
        self.end_x.set(0.0);
        self.end_y.set(0.0);
        self.drawing.set(true);
    }
}
impl DragBegin for Arc {
    fn drag_begin(&self, x: f64, y: f64) {
        self.center_x.set(x);
        self.center_y.set(y);
        self.radius.set(0.0);
        self.drawing.set(true);
    }
}
impl DragBegin for Arrow {
    fn drag_begin(&self, x: f64, y: f64) {
        self.x1.set(x);
        self.y1.set(y);
        self.x2.set(x);
        self.y2.set(y);
        self.drawing.set(true);
    }
}
impl DragBegin for Line {
    fn drag_begin(&self, x: f64, y: f64) {
        self.x1.set(x);
        self.y1.set(y);
        self.x2.set(x);
        self.y2.set(y);
        self.drawing.set(true);
    }
}
impl DragBegin for FreeHandDraw {
    fn drag_begin(&self, x: f64, y: f64) {
        self.x0.set(x);
        self.y0.set(y);
        self.drawing.set(true);
    }
}
impl DragBegin for NumberedCircle {
    fn drag_begin(&self, x: f64, y: f64) {
        self.x0.set(x);
        self.y0.set(y);
    }
}
impl DragBegin for DrawableCollection {
    fn drag_begin(&self, x: f64, y: f64) {
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
    fn drag_update(&self, x: f64, y: f64);
}

impl DragUpdate for AreaBox {
    fn drag_update(&self, x: f64, y: f64) {
        self.end_x.set(x);
        self.end_y.set(y);
    }
}
impl DragUpdate for Arc {
    fn drag_update(&self, x: f64, y: f64) {
        self.radius.set(f64::sqrt(x * x + y * y));
    }
}
impl DragUpdate for Line {
    fn drag_update(&self, x: f64, y: f64) {
        self.x1.set(self.x2.get() + x);
        self.y1.set(self.y2.get() + y);
    }
}
impl DragUpdate for Arrow {
    fn drag_update(&self, x: f64, y: f64) {
        self.x1.set(self.x2.get() + x);
        self.y1.set(self.y2.get() + y);
    }
}
impl DragUpdate for FreeHandDraw {
    fn drag_update(&self, x: f64, y: f64) {
        self.x.borrow_mut().push(self.x0.get() + x);
        self.y.borrow_mut().push(self.y0.get() + y);
    }
}
impl DragUpdate for DrawableCollection {
    fn drag_update(&self, x: f64, y: f64) {
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

pub trait DragEnd {
    fn drag_end(&self) -> DrawableCollection;
}

impl DragEnd for AreaBox {
    fn drag_end(&self) -> DrawableCollection {
        // Create an hard copy of the settings for future draws
        self.saved_settings.set(Some(self.settings.hard_copy()));

        // Shallow clone last settings and return the new element
        let (b, s) = (self.border, self.settings.clone());
        DrawableCollection::AreaBoxes(AreaBox::new(&s, b))
    }
}
impl DragEnd for Arc {
    fn drag_end(&self) -> DrawableCollection {
        // Create an hard copy of the settings for future draws
        self.saved_settings.set(Some(self.settings.hard_copy()));

        // Shallow clone last settings and return the new element
        let (b, s) = (self.border, self.settings.clone());
        DrawableCollection::Arcs(Arc::new(&s, b))
    }
}
impl DragEnd for Line {
    fn drag_end(&self) -> DrawableCollection {
        // Create an hard copy of the settings for future draws
        self.saved_settings.set(Some(self.settings.hard_copy()));

        // Shallow clone last settings and return the new element
        let (b, s) = (Option::None, self.settings.clone());
        DrawableCollection::Lines(Line::new(&s, b))
    }
}
impl DragEnd for Arrow {
    fn drag_end(&self) -> DrawableCollection {
        // Create an hard copy of the settings for future draws
        self.saved_settings.set(Some(self.settings.hard_copy()));

        // Shallow clone last settings and return the new element
        let (b, s) = (Option::None, self.settings.clone());
        DrawableCollection::Arrows(Arrow::new(&s, b))
    }
}
impl DragEnd for FreeHandDraw {
    fn drag_end(&self) -> DrawableCollection {
        // Create an hard copy of the settings for future draws
        self.saved_settings.set(Some(self.settings.hard_copy()));

        // Shallow clone last settings and return the new element
        let (b, s) = (Option::None, self.settings.clone());
        DrawableCollection::FreeHands(FreeHandDraw::new(&s, b))
    }
}
impl DragEnd for NumberedCircle {
    fn drag_end(&self) -> DrawableCollection {
        // Create an hard copy of the settings for future draws
        self.saved_settings.set(Some(self.settings.hard_copy()));

        // Shallow clone last settings and return the new element
        let (b, s) = (Option::None, self.settings.clone());
        DrawableCollection::NumberedCircles(NumberedCircle::new(&s, b))
    }
}
impl DragEnd for DrawableCollection {
    fn drag_end(&self) -> DrawableCollection {
        match self {
            DrawableCollection::AreaBoxes(state) => state.drag_end(),
            DrawableCollection::Arcs(state) => state.drag_end(),
            DrawableCollection::Lines(state) => state.drag_end(),
            DrawableCollection::Arrows(state) => state.drag_end(),
            DrawableCollection::FreeHands(state) => state.drag_end(),
            DrawableCollection::NumberedCircles(state) => state.drag_end(),
        }
    }
}

// Draw trait
pub trait Draw {
    fn draw(&self, cr: &cairo::Context);
    fn draw_with_saved_settings(&self, cr: &cairo::Context);
}

impl Draw for FreeHandDraw {
    fn draw(&self, cr: &cairo::Context) {
        // get freehands size
        let sz = self.x.borrow().iter().len();

        if sz == 0 {
            return;
        }

        // define previous point
        let mut x_prev = self.x.borrow()[0];
        let mut y_prev = self.y.borrow()[0];

        // for a continuous draw we need to fill the blank space between points
        let mut cc: usize = 1;
        while cc < sz {
            // get current x and y
            let (x, y) = (self.x.borrow()[cc], self.y.borrow()[cc]);

            cr.set_source_rgba(
                self.settings.color.get_value("red"),
                self.settings.color.get_value("green"),
                self.settings.color.get_value("blue"),
                self.settings.color.get_value("alpha"),
            ); // Set color RGBA

            // Draw the line
            cr.move_to(x_prev, y_prev);
            cr.line_to(x, y);
            // Set line properties
            cr.set_line_width(self.settings.size.get_value("init_freehand_size"));
            cr.set_line_cap(cairo::LineCap::Round);
            cr.stroke().unwrap();

            x_prev = x;
            y_prev = y;

            // update counter
            cc += 1;
        }
    }

    fn draw_with_saved_settings(&self, cr: &cairo::Context) {
        // get freehands size
        let sz = self.x.borrow().iter().len();

        if sz == 0 {
            return;
        }

        // define previous point
        let mut x_prev = self.x.borrow()[0];
        let mut y_prev = self.y.borrow()[0];

        // for a continuous draw we need to fill the blank space between points
        let mut cc: usize = 1;
        while cc < sz {
            // get current x and y
            let (x, y) = (self.x.borrow()[cc], self.y.borrow()[cc]);

            // Get the saved settings
            if let Some(settings) = self.saved_settings.get() {
                cr.set_source_rgba(
                    settings.color.get_value("red"),
                    settings.color.get_value("green"),
                    settings.color.get_value("blue"),
                    settings.color.get_value("alpha"),
                );
            } else {
                println!("Warning! saved_settings is None!");
            }

            // Draw the line
            cr.move_to(x_prev, y_prev);
            cr.line_to(x, y);
            // Set line properties
            cr.set_line_width(self.settings.size.get_value("init_freehand_size"));
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
        cr.set_source_rgba(
            self.settings.color.get_value("red"),
            self.settings.color.get_value("green"),
            self.settings.color.get_value("blue"),
            self.settings.color.get_value("alpha"),
        ); // Set color RGBA

        // Draw the line
        cr.move_to(self.x1.get(), self.y1.get());
        cr.line_to(self.x2.get(), self.y2.get());

        // Set line properties
        cr.set_line_width(self.settings.size.get_value("init_line_size"));
        cr.set_line_cap(cairo::LineCap::Round);
        cr.stroke().unwrap();
    }
    fn draw_with_saved_settings(&self, cr: &cairo::Context) {
        // Get the saved settings
        if let Some(settings) = self.saved_settings.get() {
            cr.set_source_rgba(
                settings.color.get_value("red"),
                settings.color.get_value("green"),
                settings.color.get_value("blue"),
                settings.color.get_value("alpha"),
            );
        } else {
            println!("Warning! saved_settings is None!");
        }

        // Draw the line
        cr.move_to(self.x1.get(), self.y1.get());
        cr.line_to(self.x2.get(), self.y2.get());

        // Set line properties
        cr.set_line_width(self.settings.size.get_value("init_line_size"));
        cr.set_line_cap(cairo::LineCap::Round);
        cr.stroke().unwrap();
    }
}
impl Draw for Arrow {
    fn draw(&self, cr: &cairo::Context) {
        cr.set_source_rgba(
            self.settings.color.get_value("red"),
            self.settings.color.get_value("green"),
            self.settings.color.get_value("blue"),
            self.settings.color.get_value("alpha"),
        ); // Set color RGBA

        // Draw the line
        cr.move_to(self.x1.get(), self.y1.get());
        cr.line_to(self.x2.get(), self.y2.get());

        // Draw the arrowhead
        let arrow_size = self.settings.size.get_value("init_arrow_size");
        let angle = (self.y2.get() - self.y1.get()).atan2(self.x2.get() - self.x1.get());
        cr.line_to(
            self.x2.get() - arrow_size * angle.cos() + arrow_size * angle.sin(),
            self.y2.get() - arrow_size * angle.sin() - arrow_size * angle.cos(),
        );
        cr.move_to(self.x2.get(), self.y2.get());
        cr.line_to(
            self.x2.get() - arrow_size * angle.cos() - arrow_size * angle.sin(),
            self.y2.get() - arrow_size * angle.sin() + arrow_size * angle.cos(),
        );

        // Set line properties
        cr.set_line_width(self.settings.size.get_value("init_arrow_width"));
        cr.set_line_cap(cairo::LineCap::Round);
        cr.stroke().unwrap();
    }
    fn draw_with_saved_settings(&self, cr: &cairo::Context) {
        // Get the saved settings
        if let Some(settings) = self.saved_settings.get() {
            cr.set_source_rgba(
                settings.color.get_value("red"),
                settings.color.get_value("green"),
                settings.color.get_value("blue"),
                settings.color.get_value("alpha"),
            );
        } else {
            println!("Warning! saved_settings is None!");
        }

        // Draw the line
        cr.move_to(self.x1.get(), self.y1.get());
        cr.line_to(self.x2.get(), self.y2.get());

        // Draw the arrowhead
        let arrow_size = self.settings.size.get_value("init_arrow_size");
        let angle = (self.y2.get() - self.y1.get()).atan2(self.x2.get() - self.x1.get());
        cr.line_to(
            self.x2.get() - arrow_size * angle.cos() + arrow_size * angle.sin(),
            self.y2.get() - arrow_size * angle.sin() - arrow_size * angle.cos(),
        );
        cr.move_to(self.x2.get(), self.y2.get());
        cr.line_to(
            self.x2.get() - arrow_size * angle.cos() - arrow_size * angle.sin(),
            self.y2.get() - arrow_size * angle.sin() + arrow_size * angle.cos(),
        );

        // Set line properties
        cr.set_line_width(self.settings.size.get_value("init_arrow_width"));
        cr.set_line_cap(cairo::LineCap::Round);
        cr.stroke().unwrap();
    }
}
impl Draw for Arc {
    fn draw(&self, cr: &cairo::Context) {
        cr.set_source_rgba(
            self.settings.color.get_value("red"),
            self.settings.color.get_value("green"),
            self.settings.color.get_value("blue"),
            self.settings.color.get_value("alpha"),
        ); // Set color RGBA
        cr.arc(
            self.center_x.get(),
            self.center_y.get(),
            self.radius.get(),
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
    fn draw_with_saved_settings(&self, cr: &cairo::Context) {
        // Get the saved settings
        if let Some(settings) = self.saved_settings.get() {
            cr.set_source_rgba(
                settings.color.get_value("red"),
                settings.color.get_value("green"),
                settings.color.get_value("blue"),
                settings.color.get_value("alpha"),
            );
        } else {
            println!("Warning! saved_settings is None!");
        }

        cr.arc(
            self.center_x.get(),
            self.center_y.get(),
            self.radius.get(),
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
        cr.set_source_rgba(
            self.settings.color.get_value("red"),
            self.settings.color.get_value("green"),
            self.settings.color.get_value("blue"),
            self.settings.color.get_value("alpha"),
        ); // Set color RGBA
        cr.rectangle(
            self.start_x.get(),
            self.start_y.get(),
            self.end_x.get(),
            self.end_y.get(),
        );
        if let Some(border_size) = self.border {
            cr.set_line_width(border_size);
        } else {
            cr.fill().expect("No arc fill to unwrap");
        }
        cr.stroke().unwrap();
    }
    fn draw_with_saved_settings(&self, cr: &cairo::Context) {
        // Get the saved settings
        if let Some(settings) = self.saved_settings.get() {
            cr.set_source_rgba(
                settings.color.get_value("red"),
                settings.color.get_value("green"),
                settings.color.get_value("blue"),
                settings.color.get_value("alpha"),
            );
        } else {
            println!("Warning! saved_settings is None!");
        }

        cr.rectangle(
            self.start_x.get(),
            self.start_y.get(),
            self.end_x.get(),
            self.end_y.get(),
        );
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
        cr.set_source_rgba(
            self.settings.color.get_value("red"),
            self.settings.color.get_value("green"),
            self.settings.color.get_value("blue"),
            self.settings.color.get_value("alpha"),
        );
        cr.arc(
            self.x0.get(),
            self.y0.get(),
            self.settings.size.get_value("init_numbered_circles_radius"),
            0.0,
            2.0 * f64::consts::PI,
        );
        cr.fill().expect("No numbered circle fill to unwrap");
        cr.stroke().unwrap();

        // Set text properties
        let font_size = self
            .settings
            .size
            .get_value("init_numbered_circles_font_size");
        cr.set_source_rgba(
            self.settings
                .size
                .get_value("init_numbered_circles_font_color_r"),
            self.settings
                .size
                .get_value("init_numbered_circles_font_color_g"),
            self.settings
                .size
                .get_value("init_numbered_circles_font_color_b"),
            self.settings.color.get_value("alpha"),
        );
        cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
        cr.set_font_size(font_size);

        // Draw text in the center of the rectangle
        let text = self.number_text.clone(); // format!("{}", self.numbered_circles.iter().le());
        let (text_x, text_y) = (
            self.x0.get() - cr.text_extents(&text).unwrap().width() / 2.0 - 1.0,
            self.y0.get() + font_size / 2.0,
        );
        cr.move_to(text_x, text_y);
        cr.show_text(&text).expect("Failed to show text");
        cr.stroke().unwrap();
    }
    fn draw_with_saved_settings(&self, cr: &cairo::Context) {
        // Get the saved settings
        if let Some(settings) = self.saved_settings.get() {
            cr.set_source_rgba(
                settings.color.get_value("red"),
                settings.color.get_value("green"),
                settings.color.get_value("blue"),
                settings.color.get_value("alpha"),
            );
        } else {
            println!("Warning! saved_settings is None!");
        }

        cr.arc(
            self.x0.get(),
            self.y0.get(),
            self.settings.size.get_value("init_numbered_circles_radius"),
            0.0,
            2.0 * f64::consts::PI,
        );
        cr.fill().expect("No numbered circle fill to unwrap");
        cr.stroke().unwrap();

        // Set text properties
        let font_size = self
            .settings
            .size
            .get_value("init_numbered_circles_font_size");
        cr.set_source_rgba(
            self.settings
                .size
                .get_value("init_numbered_circles_font_color_r"),
            self.settings
                .size
                .get_value("init_numbered_circles_font_color_g"),
            self.settings
                .size
                .get_value("init_numbered_circles_font_color_b"),
            self.settings.color.get_value("alpha"),
        );
        cr.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
        cr.set_font_size(font_size);

        // Draw text in the center of the rectangle
        let text = self.number_text.clone(); // format!("{}", self.numbered_circles.iter().le());
        let (text_x, text_y) = (
            self.x0.get() - cr.text_extents(&text).unwrap().width() / 2.0 - 1.0,
            self.y0.get() + font_size / 2.0,
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
    fn draw_with_saved_settings(&self, cr: &cairo::Context) {
        match self {
            DrawableCollection::FreeHands(state) => {
                state.draw_with_saved_settings(cr);
            }
            DrawableCollection::AreaBoxes(state) => {
                state.draw_with_saved_settings(cr);
            }
            DrawableCollection::Lines(state) => {
                state.draw_with_saved_settings(cr);
            }
            DrawableCollection::Arcs(state) => {
                state.draw_with_saved_settings(cr);
            }
            DrawableCollection::Arrows(state) => {
                state.draw_with_saved_settings(cr);
            }
            DrawableCollection::NumberedCircles(state) => {
                state.draw_with_saved_settings(cr);
            }
        }
    }
}

// Set RGBA trait
pub trait SetRGBA {
    fn set_rgba(&self, r: f64, g: f64, b: f64, a: f64);
}

impl SetRGBA for AreaBox {
    fn set_rgba(&self, r: f64, g: f64, b: f64, a: f64) {
        self.settings.color.set_value("red", r);
        self.settings.color.set_value("green", g);
        self.settings.color.set_value("blue", b);
        self.settings.color.set_value("alpha", a);
    }
}
impl SetRGBA for Arc {
    fn set_rgba(&self, r: f64, g: f64, b: f64, a: f64) {
        self.settings.color.set_value("red", r);
        self.settings.color.set_value("green", g);
        self.settings.color.set_value("blue", b);
        self.settings.color.set_value("alpha", a);
    }
}
impl SetRGBA for NumberedCircle {
    fn set_rgba(&self, r: f64, g: f64, b: f64, a: f64) {
        self.settings.color.set_value("red", r);
        self.settings.color.set_value("green", g);
        self.settings.color.set_value("blue", b);
        self.settings.color.set_value("alpha", a);
    }
}
impl SetRGBA for Line {
    fn set_rgba(&self, r: f64, g: f64, b: f64, a: f64) {
        self.settings.color.set_value("red", r);
        self.settings.color.set_value("green", g);
        self.settings.color.set_value("blue", b);
        self.settings.color.set_value("alpha", a);
    }
}
impl SetRGBA for Arrow {
    fn set_rgba(&self, r: f64, g: f64, b: f64, a: f64) {
        self.settings.color.set_value("red", r);
        self.settings.color.set_value("green", g);
        self.settings.color.set_value("blue", b);
        self.settings.color.set_value("alpha", a);
    }
}
impl SetRGBA for FreeHandDraw {
    fn set_rgba(&self, r: f64, g: f64, b: f64, a: f64) {
        self.settings.color.set_value("red", r);
        self.settings.color.set_value("green", g);
        self.settings.color.set_value("blue", b);
        self.settings.color.set_value("alpha", a);
    }
}
impl SetRGBA for DrawableCollection {
    fn set_rgba(&self, r: f64, g: f64, b: f64, a: f64) {
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
