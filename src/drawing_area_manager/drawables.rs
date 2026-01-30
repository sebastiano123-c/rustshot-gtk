use crate::drawing_area_settings::{Settings, SettingsRc};
use core::f64;
use gtk::cairo;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct NumberedCircle {
    pub x0: Rc<Cell<f64>>,
    pub y0: Rc<Cell<f64>>,
    pub settings: SettingsRc,
    pub saved_settings: Rc<RefCell<Option<Settings>>>,
    pub drawing: Rc<Cell<bool>>,
}

#[derive(Clone)]
pub struct FreeHandDraw {
    pub x0: Rc<Cell<f64>>,
    pub y0: Rc<Cell<f64>>,
    pub x: RefCell<Vec<f64>>,
    pub y: RefCell<Vec<f64>>,
    pub settings: SettingsRc,
    pub saved_settings: Rc<RefCell<Option<Settings>>>,
    pub drawing: Rc<Cell<bool>>,
}

#[derive(Clone)]
pub struct Line {
    pub x1: Rc<Cell<f64>>,
    pub y1: Rc<Cell<f64>>,
    pub x2: Rc<Cell<f64>>,
    pub y2: Rc<Cell<f64>>,
    pub settings: SettingsRc,
    pub saved_settings: Rc<RefCell<Option<Settings>>>,
    pub drawing: Rc<Cell<bool>>,
}

#[derive(Clone)]
pub struct Arrow {
    pub x1: Rc<Cell<f64>>,
    pub y1: Rc<Cell<f64>>,
    pub x2: Rc<Cell<f64>>,
    pub y2: Rc<Cell<f64>>,
    pub settings: SettingsRc,
    pub saved_settings: Rc<RefCell<Option<Settings>>>,
    pub drawing: Rc<Cell<bool>>,
    // fill: bool,
}

#[derive(Clone)]
pub struct Arc {
    pub radius: Rc<Cell<f64>>,
    pub center_x: Rc<Cell<f64>>,
    pub center_y: Rc<Cell<f64>>,
    pub settings: SettingsRc,
    pub saved_settings: Rc<RefCell<Option<Settings>>>,
    pub drawing: Rc<Cell<bool>>,
}

#[derive(Clone)]
pub struct AreaBox {
    pub start_x: Rc<Cell<f64>>,
    pub start_y: Rc<Cell<f64>>,
    pub end_x: Rc<Cell<f64>>,
    pub end_y: Rc<Cell<f64>>,
    pub settings: SettingsRc,
    pub saved_settings: Rc<RefCell<Option<Settings>>>,
    pub drawing: Rc<Cell<bool>>,
}

#[derive(Clone)]
pub enum DrawableCollection {
    NumberedCircles(NumberedCircle),
    FreeHands(FreeHandDraw),
    Lines(Line),
    Arrows(Arrow),
    Arcs(Arc),
    AreaBoxes(AreaBox),
}

impl AreaBox {
    pub fn new(settings_rc: &SettingsRc) -> Self {
        Self {
            start_x: Rc::new(Cell::new(0.0)),
            start_y: Rc::new(Cell::new(0.0)),
            end_x: Rc::new(Cell::new(0.0)),
            end_y: Rc::new(Cell::new(0.0)),
            settings: settings_rc.clone(),
            saved_settings: Rc::new(RefCell::new(None)),
            drawing: Rc::new(Cell::new(true)),
        }
    }
    pub fn is_drawing(&self) -> bool {
        self.drawing.get()
    }
}
impl FreeHandDraw {
    pub fn new(settings_rc: &SettingsRc) -> Self {
        let vex_x: Vec<f64> = Vec::new();
        let vex_y: Vec<f64> = Vec::new();

        Self {
            x0: Rc::new(Cell::new(0.0)),
            y0: Rc::new(Cell::new(0.0)),
            x: RefCell::new(vex_x),
            y: RefCell::new(vex_y),
            settings: settings_rc.clone(),
            saved_settings: Rc::new(RefCell::new(None)),
            drawing: Rc::new(Cell::new(false)),
        }
    }
    pub fn is_drawing(&self) -> bool {
        self.drawing.get()
    }
}
impl Line {
    pub fn new(settings_rc: &SettingsRc) -> Self {
        Self {
            x1: Rc::new(Cell::new(0.0)),
            y1: Rc::new(Cell::new(0.0)),
            x2: Rc::new(Cell::new(0.0)),
            y2: Rc::new(Cell::new(0.0)),
            settings: settings_rc.clone(),
            saved_settings: Rc::new(RefCell::new(None)),
            drawing: Rc::new(Cell::new(false)),
        }
    }
    pub fn is_drawing(&self) -> bool {
        self.drawing.get()
    }
}
impl Arrow {
    pub fn new(settings_rc: &SettingsRc) -> Self {
        Arrow {
            x1: Rc::new(Cell::new(0.0)),
            y1: Rc::new(Cell::new(0.0)),
            x2: Rc::new(Cell::new(0.0)),
            y2: Rc::new(Cell::new(0.0)),
            settings: settings_rc.clone(),
            saved_settings: Rc::new(RefCell::new(None)),
            drawing: Rc::new(Cell::new(false)),
        }
    }
    pub fn is_drawing(&self) -> bool {
        self.drawing.get()
    }
}
impl NumberedCircle {
    pub fn new(settings_rc: &SettingsRc) -> Self {
        Self {
            x0: Rc::new(Cell::new(0.0)),
            y0: Rc::new(Cell::new(0.0)),
            settings: settings_rc.clone(),
            saved_settings: Rc::new(RefCell::new(None)),
            drawing: Rc::new(Cell::new(false)),
        }
    }
    pub fn is_drawing(&self) -> bool {
        self.drawing.get()
    }
}
impl Arc {
    pub fn new(settings_rc: &SettingsRc) -> Self {
        Self {
            radius: Rc::new(Cell::new(0.0)),
            center_x: Rc::new(Cell::new(0.0)),
            center_y: Rc::new(Cell::new(0.0)),
            settings: settings_rc.clone(),
            saved_settings: Rc::new(RefCell::new(None)),
            drawing: Rc::new(Cell::new(false)),
        }
    }
    pub fn is_drawing(&self) -> bool {
        self.drawing.get()
    }
}

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
        self.x2.set(self.x1.get() + x);
        self.y2.set(self.y1.get() + y);
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

// Drag End
pub trait DragEnd {
    fn drag_end(&self) -> DrawableCollection;
}

impl DragEnd for AreaBox {
    fn drag_end(&self) -> DrawableCollection {
        // Create an hard copy of the settings for future draws
        *self.saved_settings.borrow_mut() = Some(self.settings.hard_copy());

        // Shallow clone last settings and return the new element
        let s = self.settings.clone();
        DrawableCollection::AreaBoxes(AreaBox::new(&s))
    }
}
impl DragEnd for Arc {
    fn drag_end(&self) -> DrawableCollection {
        // Create an hard copy of the settings for future draws
        *self.saved_settings.borrow_mut() = Some(self.settings.hard_copy());

        // Shallow clone last settings and return the new element
        let s = self.settings.clone();
        DrawableCollection::Arcs(Arc::new(&s))
    }
}
impl DragEnd for Line {
    fn drag_end(&self) -> DrawableCollection {
        // Create an hard copy of the settings for future draws
        *self.saved_settings.borrow_mut() = Some(self.settings.hard_copy());

        // Shallow clone last settings and return the new element
        let s = self.settings.clone();
        DrawableCollection::Lines(Line::new(&s))
    }
}
impl DragEnd for Arrow {
    fn drag_end(&self) -> DrawableCollection {
        // Create an hard copy of the settings for future draws
        *self.saved_settings.borrow_mut() = Some(self.settings.hard_copy());

        // Shallow clone last settings and return the new element
        let s = self.settings.clone();
        DrawableCollection::Arrows(Arrow::new(&s))
    }
}
impl DragEnd for FreeHandDraw {
    fn drag_end(&self) -> DrawableCollection {
        // Create an hard copy of the settings for future draws
        *self.saved_settings.borrow_mut() = Some(self.settings.hard_copy());

        // Shallow clone last settings and return the new element
        let s = self.settings.clone();
        DrawableCollection::FreeHands(FreeHandDraw::new(&s))
    }
}
impl DragEnd for NumberedCircle {
    fn drag_end(&self) -> DrawableCollection {
        // Create an hard copy of the settings for future draws
        *self.saved_settings.borrow_mut() = Some(self.settings.hard_copy());

        // Set new value
        let num: i32 = self
            .settings
            .numbered_circle
            .get_value("number")
            .get_i32()
            .expect("drag_end error")
            + 1_i32;
        self.settings
            .numbered_circle
            .set_value(
                "number",
                crate::drawing_area_settings::SettingValue::I32(num),
            )
            .expect("NumberedCircle::new error");

        // Shallow clone last settings and return the new element
        let s = self.settings.clone();
        DrawableCollection::NumberedCircles(NumberedCircle::new(&s))
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
    fn draw(&self, cr: &cairo::Context, pg: &gtk::pango::Layout);
    fn draw_with_saved_settings(&self, cr: &cairo::Context, pg: &gtk::pango::Layout);
}

impl Draw for FreeHandDraw {
    fn draw(&self, cr: &cairo::Context, _pg: &gtk::pango::Layout) {
        let settings = self.settings.freehand.clone();

        // get freehands size
        let sz = self.x.borrow().iter().len();

        if sz == 0 {
            return;
        }

        // define previous point
        let mut x_prev = self.x.borrow()[0];
        let mut y_prev = self.y.borrow()[0];

        // Get the tool
        let tool = settings.get_value("tool").get_string().expect("tool error");

        // cr.set_line_join(cairo::LineJoin::Round);
        // cr.set_line_join(cairo::LineJoin::Miter);
        // cr.set_line_join(cairo::LineJoin::Bevel);

        // for a continuous draw we need to fill the blank space between points
        let mut cc: usize = 1;
        while cc < sz {
            // get current x and y
            let (x, y) = (self.x.borrow()[cc], self.y.borrow()[cc]);

            // Draw the line
            cr.move_to(x_prev, y_prev);
            cr.line_to(x, y);
            // Set line properties
            cr.set_line_width(
                settings
                    .get_value("size")
                    .get_f64()
                    .expect("Freehand error"),
            );

            match tool.as_str() {
                "pen" => {
                    cr.set_line_cap(cairo::LineCap::Round);

                    // Get the saved settings
                    cr.set_source_rgba(
                        settings
                            .get_value("color_r")
                            .get_f64()
                            .expect("freehand error"),
                        settings
                            .get_value("color_g")
                            .get_f64()
                            .expect("freehand error"),
                        settings
                            .get_value("color_b")
                            .get_f64()
                            .expect("freehand error"),
                        settings
                            .get_value("color_a")
                            .get_f64()
                            .expect("freehand error"),
                    );
                }
                "highlighter" => {
                    cr.set_line_cap(cairo::LineCap::Square);

                    // Get the saved settings
                    cr.set_source_rgba(
                        settings
                            .get_value("color_r")
                            .get_f64()
                            .expect("freehand error"),
                        settings
                            .get_value("color_g")
                            .get_f64()
                            .expect("freehand error"),
                        settings
                            .get_value("color_b")
                            .get_f64()
                            .expect("freehand error"),
                        0.1,
                    );
                }
                "fountain-pen" => {
                    cr.set_line_cap(cairo::LineCap::Round);

                    // Get the saved settings
                    cr.set_source_rgba(
                        settings
                            .get_value("color_r")
                            .get_f64()
                            .expect("freehand error"),
                        settings
                            .get_value("color_g")
                            .get_f64()
                            .expect("freehand error"),
                        settings
                            .get_value("color_b")
                            .get_f64()
                            .expect("freehand error"),
                        0.3,
                    );
                }
                _ => println!("Error, no line_cap found"),
            };

            cr.stroke().unwrap();

            x_prev = x;
            y_prev = y;

            // update counter
            cc += 1;
        }
    }

    fn draw_with_saved_settings(&self, cr: &cairo::Context, _pg: &gtk::pango::Layout) {
        // get freehands size
        let sz = self.x.borrow().iter().len();

        if sz == 0 {
            return;
        }

        // define previous point
        let mut x_prev = self.x.borrow()[0];
        let mut y_prev = self.y.borrow()[0];

        let settings_hard_copy = self.saved_settings.borrow().clone();
        if let Some(settings_to_unwrap) = settings_hard_copy {
            let settings = settings_to_unwrap.freehand;

            // Get tool
            let tool = settings.get_value("tool").get_string().expect("tool error");

            // for a continuous draw we need to fill the blank space between points
            let mut cc: usize = 1;
            while cc < sz {
                // get current x and y
                let (x, y) = (self.x.borrow()[cc], self.y.borrow()[cc]);

                // Draw the line
                cr.move_to(x_prev, y_prev);
                cr.line_to(x, y);
                // Set line properties
                cr.set_line_width(
                    settings
                        .get_value("size")
                        .get_f64()
                        .expect("Freehand error"),
                );

                match tool.as_str() {
                    "pen" => {
                        cr.set_line_cap(cairo::LineCap::Round);

                        // Get the saved settings
                        cr.set_source_rgba(
                            settings
                                .get_value("color_r")
                                .get_f64()
                                .expect("freehand error"),
                            settings
                                .get_value("color_g")
                                .get_f64()
                                .expect("freehand error"),
                            settings
                                .get_value("color_b")
                                .get_f64()
                                .expect("freehand error"),
                            settings
                                .get_value("color_a")
                                .get_f64()
                                .expect("freehand error"),
                        );
                    }
                    "highlighter" => {
                        cr.set_line_cap(cairo::LineCap::Square);

                        // Get the saved settings
                        cr.set_source_rgba(
                            settings
                                .get_value("color_r")
                                .get_f64()
                                .expect("freehand error"),
                            settings
                                .get_value("color_g")
                                .get_f64()
                                .expect("freehand error"),
                            settings
                                .get_value("color_b")
                                .get_f64()
                                .expect("freehand error"),
                            0.1,
                        );
                    }
                    "fountain-pen" => {
                        cr.set_line_cap(cairo::LineCap::Round);

                        // Get the saved settings
                        cr.set_source_rgba(
                            settings
                                .get_value("color_r")
                                .get_f64()
                                .expect("freehand error"),
                            settings
                                .get_value("color_g")
                                .get_f64()
                                .expect("freehand error"),
                            settings
                                .get_value("color_b")
                                .get_f64()
                                .expect("freehand error"),
                            0.3,
                        );
                    }
                    _ => println!("Error, no line_cap found"),
                };

                cr.stroke().unwrap();

                x_prev = x;
                y_prev = y;

                // update counter
                cc += 1;
            }
        } else {
            println!("Warning! saved_settings is None!");
        }
    }
}
impl Draw for Line {
    fn draw(&self, cr: &cairo::Context, _pg: &gtk::pango::Layout) {
        let settings = self.settings.line.clone();
        // Set color RGBA
        cr.set_source_rgba(
            settings
                .get_value("color_r")
                .get_f64()
                .expect("draw line error"),
            settings
                .get_value("color_g")
                .get_f64()
                .expect("draw line error"),
            settings
                .get_value("color_b")
                .get_f64()
                .expect("draw line error"),
            settings
                .get_value("color_a")
                .get_f64()
                .expect("draw line error"),
        );

        // Draw the line
        cr.move_to(self.x1.get(), self.y1.get());
        cr.line_to(self.x2.get(), self.y2.get());

        // Set line properties
        cr.set_line_width(
            settings
                .get_value("size")
                .get_f64()
                .expect("draw line error"),
        );

        // Set line cap
        match settings
            .get_value("line_cap")
            .get_string()
            .expect("No line_cap value found")
            .as_str()
        {
            "round" => cr.set_line_cap(cairo::LineCap::Round),
            "butt" => cr.set_line_cap(cairo::LineCap::Butt),
            "square" => cr.set_line_cap(cairo::LineCap::Square),
            _ => println!("Error, no line_cap found"),
        };

        // Set line join
        match settings
            .get_value("line_join")
            .get_string()
            .expect("No line_join value found")
            .as_str()
        {
            "round" => cr.set_line_join(cairo::LineJoin::Round),
            "miter" => cr.set_line_join(cairo::LineJoin::Miter),
            "bevel" => cr.set_line_join(cairo::LineJoin::Bevel),
            _ => println!("Error, no line_join found"),
        };
        cr.stroke().unwrap();
    }
    fn draw_with_saved_settings(&self, cr: &cairo::Context, _pg: &gtk::pango::Layout) {
        let settings_hard_copy = self.saved_settings.borrow().clone();
        if let Some(settings_to_unwrap) = settings_hard_copy {
            let settings = settings_to_unwrap.line;
            // Set color RGBA
            cr.set_source_rgba(
                settings
                    .get_value("color_r")
                    .get_f64()
                    .expect("draw line error"),
                settings
                    .get_value("color_g")
                    .get_f64()
                    .expect("draw line error"),
                settings
                    .get_value("color_b")
                    .get_f64()
                    .expect("draw line error"),
                settings
                    .get_value("color_a")
                    .get_f64()
                    .expect("draw line error"),
            );

            // Draw the line
            cr.move_to(self.x1.get(), self.y1.get());
            cr.line_to(self.x2.get(), self.y2.get());

            // Set line properties
            cr.set_line_width(
                settings
                    .get_value("size")
                    .get_f64()
                    .expect("draw line error"),
            );

            // Set line cap
            match settings
                .get_value("line_cap")
                .get_string()
                .expect("No line_cap value found")
                .as_str()
            {
                "round" => cr.set_line_cap(cairo::LineCap::Round),
                "butt" => cr.set_line_cap(cairo::LineCap::Butt),
                "square" => cr.set_line_cap(cairo::LineCap::Square),
                _ => println!("Error, no line_cap found"),
            };

            // Set line join
            match settings
                .get_value("line_join")
                .get_string()
                .expect("No line_join value found")
                .as_str()
            {
                "round" => cr.set_line_join(cairo::LineJoin::Round),
                "miter" => cr.set_line_join(cairo::LineJoin::Miter),
                "bevel" => cr.set_line_join(cairo::LineJoin::Bevel),
                _ => println!("Error, no line_join found"),
            };
            cr.stroke().unwrap();
        } else {
            println!("Warning! saved_settings is None!");
        }
    }
}
impl Draw for Arrow {
    fn draw(&self, cr: &cairo::Context, _pg: &gtk::pango::Layout) {
        let settings = self.settings.arrow.clone();

        // Set color RGBA
        cr.set_source_rgba(
            settings
                .get_value("color_r")
                .get_f64()
                .expect("draw line error"),
            settings
                .get_value("color_g")
                .get_f64()
                .expect("draw line error"),
            settings
                .get_value("color_b")
                .get_f64()
                .expect("draw line error"),
            settings
                .get_value("color_a")
                .get_f64()
                .expect("draw line error"),
        );

        let x2 = self.x1.get();
        let y2 = self.y1.get();
        let x1 = self.x2.get();
        let y1 = self.y2.get();

        // Draw the line
        cr.move_to(x1, y1);
        cr.line_to(x2, y2);

        // Draw the arrowhead
        let arrow_size = settings
            .get_value("arrow_size")
            .get_f64()
            .expect("Arrow arrow_size");
        let angle = (y2 - y1).atan2(x2 - x1);
        cr.line_to(
            x2 - arrow_size * angle.cos() + arrow_size * angle.sin(),
            y2 - arrow_size * angle.sin() - arrow_size * angle.cos(),
        );
        cr.move_to(x2, y2);
        cr.line_to(
            x2 - arrow_size * angle.cos() - arrow_size * angle.sin(),
            y2 - arrow_size * angle.sin() + arrow_size * angle.cos(),
        );

        // Set line properties
        cr.set_line_width(settings.get_value("size").get_f64().expect("Arrow size"));
        cr.set_line_cap(cairo::LineCap::Round);
        cr.stroke().unwrap();
    }
    fn draw_with_saved_settings(&self, cr: &cairo::Context, _pg: &gtk::pango::Layout) {
        let settings_hard_copy = self.saved_settings.borrow().clone();
        if let Some(settings_to_unwrap) = settings_hard_copy {
            let settings = settings_to_unwrap.arrow;
            cr.set_source_rgba(
                settings
                    .get_value("color_r")
                    .get_f64()
                    .expect("Arrow error"),
                settings
                    .get_value("color_g")
                    .get_f64()
                    .expect("Arrow error"),
                settings
                    .get_value("color_b")
                    .get_f64()
                    .expect("Arrow error"),
                settings
                    .get_value("color_a")
                    .get_f64()
                    .expect("Arrow error"),
            );

            let x2 = self.x1.get();
            let y2 = self.y1.get();
            let x1 = self.x2.get();
            let y1 = self.y2.get();

            // Draw the line
            cr.move_to(x1, y1);
            cr.line_to(x2, y2);

            // Draw the arrowhead
            let arrow_size = settings
                .get_value("arrow_size")
                .get_f64()
                .expect("Arrow arrow_size");
            let angle = (y2 - y1).atan2(x2 - x1);
            cr.line_to(
                x2 - arrow_size * angle.cos() + arrow_size * angle.sin(),
                y2 - arrow_size * angle.sin() - arrow_size * angle.cos(),
            );
            cr.move_to(x2, y2);
            cr.line_to(
                x2 - arrow_size * angle.cos() - arrow_size * angle.sin(),
                y2 - arrow_size * angle.sin() + arrow_size * angle.cos(),
            );

            // Set line properties
            cr.set_line_width(settings.get_value("size").get_f64().expect("Arrow size"));
            cr.set_line_cap(cairo::LineCap::Round);
            cr.stroke().unwrap();
        } else {
            println!("Warning! saved_settings is None!");
        }
    }
}
impl Draw for Arc {
    fn draw(&self, cr: &cairo::Context, _pg: &gtk::pango::Layout) {
        let settings = self.settings.arc.clone();

        if settings.get_value("fill").get_bool().expect("draw error") {
            cr.arc(
                self.center_x.get(),
                self.center_y.get(),
                self.radius.get(),
                0.0,
                2.0 * f64::consts::PI,
            );

            cr.set_source_rgba(
                settings.get_value("fill_r").get_f64().expect("draw error"),
                settings.get_value("fill_g").get_f64().expect("draw error"),
                settings.get_value("fill_b").get_f64().expect("draw error"),
                settings.get_value("fill_a").get_f64().expect("draw error"),
            );
            cr.fill().expect("No arc fill to unwrap");
            cr.stroke().unwrap();
        }

        if settings.get_value("border").get_bool().expect("draw error") {
            cr.arc(
                self.center_x.get(),
                self.center_y.get(),
                self.radius.get(),
                0.0,
                2.0 * f64::consts::PI,
            );

            cr.set_source_rgba(
                settings
                    .get_value("border_r")
                    .get_f64()
                    .expect("draw error"),
                settings
                    .get_value("border_g")
                    .get_f64()
                    .expect("draw error"),
                settings
                    .get_value("border_b")
                    .get_f64()
                    .expect("draw error"),
                settings
                    .get_value("border_a")
                    .get_f64()
                    .expect("draw error"),
            );
            let border_size = settings
                .get_value("border_size")
                .get_f64()
                .expect("draw error");
            cr.set_line_width(border_size);
            cr.stroke().unwrap();
        }
    }
    fn draw_with_saved_settings(&self, cr: &cairo::Context, _pg: &gtk::pango::Layout) {
        // Get the saved settings
        let settings_hard_copy = self.saved_settings.borrow().clone();
        if let Some(settings_to_unwrap) = settings_hard_copy {
            let settings = settings_to_unwrap.arc;

            if settings.get_value("fill").get_bool().expect("draw error") {
                cr.arc(
                    self.center_x.get(),
                    self.center_y.get(),
                    self.radius.get(),
                    0.0,
                    2.0 * f64::consts::PI,
                );

                cr.set_source_rgba(
                    settings.get_value("fill_r").get_f64().expect("draw error"),
                    settings.get_value("fill_g").get_f64().expect("draw error"),
                    settings.get_value("fill_b").get_f64().expect("draw error"),
                    settings.get_value("fill_a").get_f64().expect("draw error"),
                );
                cr.fill().expect("No arc fill to unwrap");
                cr.stroke().unwrap();
            }

            if settings.get_value("border").get_bool().expect("draw error") {
                cr.arc(
                    self.center_x.get(),
                    self.center_y.get(),
                    self.radius.get(),
                    0.0,
                    2.0 * f64::consts::PI,
                );

                cr.set_source_rgba(
                    settings
                        .get_value("border_r")
                        .get_f64()
                        .expect("draw error"),
                    settings
                        .get_value("border_g")
                        .get_f64()
                        .expect("draw error"),
                    settings
                        .get_value("border_b")
                        .get_f64()
                        .expect("draw error"),
                    settings
                        .get_value("border_a")
                        .get_f64()
                        .expect("draw error"),
                );
                let border_size = settings
                    .get_value("border_size")
                    .get_f64()
                    .expect("draw error");
                cr.set_line_width(border_size);
                cr.stroke().unwrap();
            }
        } else {
            println!("Warning! saved_settings is None!");
        }
    }
}
impl Draw for AreaBox {
    fn draw(&self, cr: &cairo::Context, _pg: &gtk::pango::Layout) {
        let settings = self.settings.rect.clone();

        if settings.get_value("fill").get_bool().expect("draw error") {
            cr.rectangle(
                self.start_x.get(),
                self.start_y.get(),
                self.end_x.get(),
                self.end_y.get(),
            );

            cr.set_source_rgba(
                settings.get_value("fill_r").get_f64().expect("draw error"),
                settings.get_value("fill_g").get_f64().expect("draw error"),
                settings.get_value("fill_b").get_f64().expect("draw error"),
                settings.get_value("fill_a").get_f64().expect("draw error"),
            );
            cr.fill().expect("No arc fill to unwrap");
            cr.stroke().unwrap();
        }

        if settings.get_value("border").get_bool().expect("draw error") {
            cr.rectangle(
                self.start_x.get(),
                self.start_y.get(),
                self.end_x.get(),
                self.end_y.get(),
            );

            cr.set_source_rgba(
                settings
                    .get_value("border_r")
                    .get_f64()
                    .expect("draw error"),
                settings
                    .get_value("border_g")
                    .get_f64()
                    .expect("draw error"),
                settings
                    .get_value("border_b")
                    .get_f64()
                    .expect("draw error"),
                settings
                    .get_value("border_a")
                    .get_f64()
                    .expect("draw error"),
            );
            let border_size = settings
                .get_value("border_size")
                .get_f64()
                .expect("draw error");
            cr.set_line_width(border_size);
            cr.stroke().unwrap();
        }
    }
    fn draw_with_saved_settings(&self, cr: &cairo::Context, _pg: &gtk::pango::Layout) {
        let settings_hard_copy = self.saved_settings.borrow().clone();
        if let Some(settings_to_unwrap) = settings_hard_copy {
            let settings = settings_to_unwrap.rect;
            if settings.get_value("fill").get_bool().expect("draw error") {
                cr.rectangle(
                    self.start_x.get(),
                    self.start_y.get(),
                    self.end_x.get(),
                    self.end_y.get(),
                );

                cr.set_source_rgba(
                    settings.get_value("fill_r").get_f64().expect("draw error"),
                    settings.get_value("fill_g").get_f64().expect("draw error"),
                    settings.get_value("fill_b").get_f64().expect("draw error"),
                    settings.get_value("fill_a").get_f64().expect("draw error"),
                );
                cr.fill().expect("No arc fill to unwrap");
                cr.stroke().unwrap();
            }

            if settings.get_value("border").get_bool().expect("draw error") {
                cr.rectangle(
                    self.start_x.get(),
                    self.start_y.get(),
                    self.end_x.get(),
                    self.end_y.get(),
                );

                cr.set_source_rgba(
                    settings
                        .get_value("border_r")
                        .get_f64()
                        .expect("draw error"),
                    settings
                        .get_value("border_g")
                        .get_f64()
                        .expect("draw error"),
                    settings
                        .get_value("border_b")
                        .get_f64()
                        .expect("draw error"),
                    settings
                        .get_value("border_a")
                        .get_f64()
                        .expect("draw error"),
                );
                let border_size = settings
                    .get_value("border_size")
                    .get_f64()
                    .expect("draw error");
                cr.set_line_width(border_size);
                cr.stroke().unwrap();
            }
        } else {
            println!("AreaBox error, no save_settings!");
        }
    }
}
impl Draw for NumberedCircle {
    fn draw(&self, cr: &cairo::Context, pg: &gtk::pango::Layout) {
        let settings = self.settings.numbered_circle.clone();
        let radius = settings
            .get_value("radius")
            .get_f64()
            .expect("Missing setting 'radius'");

        cr.arc(
            self.x0.get(),
            self.y0.get(),
            radius,
            0.0,
            2.0 * f64::consts::PI,
        );

        if settings.get_value("fill").get_bool().expect("draw error") {
            cr.set_source_rgba(
                settings.get_value("fill_r").get_f64().expect("draw error"),
                settings.get_value("fill_g").get_f64().expect("draw error"),
                settings.get_value("fill_b").get_f64().expect("draw error"),
                settings.get_value("fill_a").get_f64().expect("draw error"),
            );
            cr.fill_preserve()
                .expect("Failed to fill the numbered_circle");
        }

        if settings.get_value("border").get_bool().expect("draw error") {
            cr.set_source_rgba(
                settings
                    .get_value("border_r")
                    .get_f64()
                    .expect("draw error"),
                settings
                    .get_value("border_g")
                    .get_f64()
                    .expect("draw error"),
                settings
                    .get_value("border_b")
                    .get_f64()
                    .expect("draw error"),
                settings
                    .get_value("border_a")
                    .get_f64()
                    .expect("draw error"),
            );
            let border_size = settings
                .get_value("border_size")
                .get_f64()
                .expect("draw error");
            cr.set_line_width(border_size);
            cr.stroke().unwrap();
        } else {
            cr.new_path();
        }

        // Set text properties
        cr.set_source_rgba(
            settings.get_value("font_r").get_f64().expect("error"),
            settings.get_value("font_g").get_f64().expect("error"),
            settings.get_value("font_b").get_f64().expect("error"),
            settings.get_value("font_a").get_f64().expect("error"),
        );
        // Font face
        let font_face = settings
            .get_value("font_face")
            .get_string()
            .expect("NumberedCircle font_face error");
        let fd: gtk::pango::FontDescription =
            gtk::pango::FontDescription::from_string(font_face.as_str());
        pg.set_font_description(Some(&fd));

        // Draw text in the center of the rectangle
        let num = settings
            .get_value("number")
            .get_i32()
            .expect("NumberedCircle::new error");
        let text = num.to_string();
        pg.set_text(&text);

        // Get logical extents (in Pango units) and convert to device units.
        let (_ink_rect, logical_rect) = pg.extents();
        let text_width = logical_rect.width() as f64 / gtk::pango::SCALE as f64;
        let text_height = logical_rect.height() as f64 / gtk::pango::SCALE as f64;

        // Position so the text is centred on (x0, y0).
        let text_x = self.x0.get() - text_width / 2.0;
        let text_y = self.y0.get() - text_height / 2.0;

        // Move the Cairo cursor (not strictly required for pangocairo, but keeps
        // the state tidy) and render the layout.
        cr.move_to(text_x, text_y);
        pangocairo::functions::show_layout(cr, pg);
        cr.stroke().expect("Failed to stroke the text");
    }
    fn draw_with_saved_settings(&self, cr: &cairo::Context, pg: &gtk::pango::Layout) {
        let settings_hard_copy = self.saved_settings.borrow().clone();
        if let Some(settings_to_unwrap) = settings_hard_copy {
            let settings = settings_to_unwrap.numbered_circle;
            let radius = settings
                .get_value("radius")
                .get_f64()
                .expect("Missing setting 'radius'");

            cr.arc(
                self.x0.get(),
                self.y0.get(),
                radius,
                0.0,
                2.0 * f64::consts::PI,
            );

            if settings.get_value("fill").get_bool().expect("draw error") {
                cr.set_source_rgba(
                    settings.get_value("fill_r").get_f64().expect("draw error"),
                    settings.get_value("fill_g").get_f64().expect("draw error"),
                    settings.get_value("fill_b").get_f64().expect("draw error"),
                    settings.get_value("fill_a").get_f64().expect("draw error"),
                );
                cr.fill_preserve()
                    .expect("Failed to fill the numbered_circle");
            }

            if settings.get_value("border").get_bool().expect("draw error") {
                cr.set_source_rgba(
                    settings
                        .get_value("border_r")
                        .get_f64()
                        .expect("draw error"),
                    settings
                        .get_value("border_g")
                        .get_f64()
                        .expect("draw error"),
                    settings
                        .get_value("border_b")
                        .get_f64()
                        .expect("draw error"),
                    settings
                        .get_value("border_a")
                        .get_f64()
                        .expect("draw error"),
                );
                let border_size = settings
                    .get_value("border_size")
                    .get_f64()
                    .expect("draw error");
                cr.set_line_width(border_size);
                cr.stroke().unwrap();
            } else {
                cr.new_path();
            }

            // Set text properties
            cr.set_source_rgba(
                settings.get_value("font_r").get_f64().expect("error"),
                settings.get_value("font_g").get_f64().expect("error"),
                settings.get_value("font_b").get_f64().expect("error"),
                settings.get_value("font_a").get_f64().expect("error"),
            );
            // Font face
            let font_face = settings
                .get_value("font_face")
                .get_string()
                .expect("NumberedCircle font_face error");
            let fd: gtk::pango::FontDescription =
                gtk::pango::FontDescription::from_string(font_face.as_str());
            pg.set_font_description(Some(&fd));

            // Draw text in the center of the rectangle
            let num = settings
                .get_value("number")
                .get_i32()
                .expect("NumberedCircle::new error");
            let text = num.to_string();
            pg.set_text(&text);

            // Get logical extents (in Pango units) and convert to device units.
            let (_ink_rect, logical_rect) = pg.extents();
            let text_width = logical_rect.width() as f64 / gtk::pango::SCALE as f64;
            let text_height = logical_rect.height() as f64 / gtk::pango::SCALE as f64;

            // Position so the text is centred on (x0, y0).
            let text_x = self.x0.get() - text_width / 2.0;
            let text_y = self.y0.get() - text_height / 2.0;

            // Move the Cairo cursor (not strictly required for pangocairo, but keeps
            // the state tidy) and render the layout.
            cr.move_to(text_x, text_y);
            pangocairo::functions::show_layout(cr, pg);
            cr.stroke().expect("Failed to stroke the text");
        } else {
            println!("Warning! saved_settings is None!");
        }
    }
}
impl Draw for DrawableCollection {
    fn draw(&self, cr: &cairo::Context, pg: &gtk::pango::Layout) {
        match self {
            DrawableCollection::FreeHands(state) => {
                state.draw(cr, pg);
            }
            DrawableCollection::AreaBoxes(state) => {
                state.draw(cr, pg);
            }
            DrawableCollection::Lines(state) => {
                state.draw(cr, pg);
            }
            DrawableCollection::Arcs(state) => {
                state.draw(cr, pg);
            }
            DrawableCollection::Arrows(state) => {
                state.draw(cr, pg);
            }
            DrawableCollection::NumberedCircles(state) => {
                state.draw(cr, pg);
            }
        }
    }
    fn draw_with_saved_settings(&self, cr: &cairo::Context, pg: &gtk::pango::Layout) {
        match self {
            DrawableCollection::FreeHands(state) => {
                state.draw_with_saved_settings(cr, pg);
            }
            DrawableCollection::AreaBoxes(state) => {
                state.draw_with_saved_settings(cr, pg);
            }
            DrawableCollection::Lines(state) => {
                state.draw_with_saved_settings(cr, pg);
            }
            DrawableCollection::Arcs(state) => {
                state.draw_with_saved_settings(cr, pg);
            }
            DrawableCollection::Arrows(state) => {
                state.draw_with_saved_settings(cr, pg);
            }
            DrawableCollection::NumberedCircles(state) => {
                state.draw_with_saved_settings(cr, pg);
            }
        }
    }
}
