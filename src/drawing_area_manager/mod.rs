use core::f64;
use gtk::cairo;

#[derive(Debug, Clone)]
struct DrawingAreaFreeHandDraw {
    x0: f64,
    y0: f64,
    x: Vec<f64>,
    y: Vec<f64>,
    size: f64,
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
    drawing: bool,
}

#[derive(Debug, Clone)]
struct DrawingAreaLine {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    size: f64,
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
    drawing: bool,
}

#[derive(Debug, Clone)]
struct DrawingAreaArrow {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    size: f64,
    width: f64,
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
    drawing: bool,
    // fill: bool,
}

#[derive(Debug, Clone)]
struct DrawingAreaArc {
    radius: f64,
    center_x: f64,
    center_y: f64,
    drawing: bool,
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
    fill: bool,
}

#[derive(Debug, Clone)]
struct DrawingAreaBox {
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
    drawing: bool,
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
    fill: bool,
}

#[derive(Debug, Clone)]
pub struct DrawingAreaManager {
    current_box: Option<DrawingAreaBox>,
    current_arc: Option<DrawingAreaArc>,
    current_arrow: Option<DrawingAreaArrow>,
    current_line: Option<DrawingAreaLine>,
    current_freehand: Option<DrawingAreaFreeHandDraw>,
    boxes: Vec<DrawingAreaBox>,
    arcs: Vec<DrawingAreaArc>,
    arrows: Vec<DrawingAreaArrow>,
    lines: Vec<DrawingAreaLine>,
    freehands: Vec<DrawingAreaFreeHandDraw>,
    pub is_drawing: bool,
}

impl DrawingAreaManager {
    pub fn new() -> Self {
        Self {
            current_box: None,
            current_arc: None,
            current_arrow: None,
            current_line: None,
            current_freehand: None,
            boxes: Vec::new(),
            arcs: Vec::new(),
            arrows: Vec::new(),
            lines: Vec::new(),
            freehands: Vec::new(),
            is_drawing: false,
        }
    }

    pub fn create_new_freehand_draw(&mut self, size: f64, r: f64, g: f64, b: f64, a: f64) {
        self.reset();
        self.current_freehand = Some(DrawingAreaFreeHandDraw {
            x0: 0.0,
            y0: 0.0,
            x: Vec::new(),
            y: Vec::new(),
            size: size,
            red: r,
            green: g,
            blue: b,
            alpha: a,
            drawing: false,
        });

        // emit signal for drawing boxes
        self.is_drawing = true;
    }

    pub fn create_new_line(&mut self, arrow_size: f64, r: f64, g: f64, b: f64, a: f64) {
        self.reset();
        self.current_line = Some(DrawingAreaLine {
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
        });

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
        self.current_arrow = Some(DrawingAreaArrow {
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
        });

        // emit signal for drawing boxes
        self.is_drawing = true;
    }

    pub fn create_new_arc(&mut self, red: f64, green: f64, blue: f64, alpha: f64, fill_rect: bool) {
        self.reset();

        self.current_arc = Some(DrawingAreaArc {
            radius: 0.0,
            center_x: 0.0,
            center_y: 0.0,
            red,
            green,
            blue,
            alpha,
            drawing: false,
            fill: fill_rect,
        });

        // emit signal for drawing boxes
        self.is_drawing = true;
    }

    pub fn create_new_box(&mut self, red: f64, green: f64, blue: f64, alpha: f64, fill_rect: bool) {
        self.reset();
        self.current_box = Some(DrawingAreaBox {
            start_x: 0.0,
            start_y: 0.0,
            end_x: 0.0,
            end_y: 0.0,
            red,
            green,
            blue,
            alpha,
            drawing: false,
            fill: fill_rect,
        });

        // emit signal for drawing boxes
        self.is_drawing = true;
    }

    pub fn is_drawing(&self) -> bool {
        self.is_drawing
    }

    pub fn set_rgba(&mut self, r: f64, g: f64, b: f64, a: f64) {
        if let Some(current_box) = &mut self.current_box {
            current_box.red = r;
            current_box.green = g;
            current_box.blue = b;
            current_box.alpha = a;
        }
        if let Some(current_arc) = &mut self.current_arc {
            current_arc.red = r;
            current_arc.green = g;
            current_arc.blue = b;
            current_arc.alpha = a;
        }
        if let Some(current_line) = &mut self.current_line {
            current_line.red = r;
            current_line.green = g;
            current_line.blue = b;
            current_line.alpha = a;
        }
        if let Some(current_arrow) = &mut self.current_arrow {
            current_arrow.red = r;
            current_arrow.green = g;
            current_arrow.blue = b;
            current_arrow.alpha = a;
        }
        if let Some(current_freehand) = &mut self.current_freehand {
            current_freehand.red = r;
            current_freehand.green = g;
            current_freehand.blue = b;
            current_freehand.alpha = a;
        }
    }

    pub fn set_draw(&mut self, cr: &cairo::Context) {
        // Clear the drawing area
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.0); // transparent background
        cr.paint().unwrap();

        // draw old arcs
        for bx in &self.arrows {
            Self::draw_arrow(&bx, &cr);
        }

        // draw old arcs
        for bx in &self.lines {
            Self::draw_line(&bx, &cr);
        }

        // draw old arcs
        for bx in &self.arcs {
            Self::draw_arc(&bx, &cr);
        }

        // draw old boxes
        for bx in &self.boxes {
            Self::draw_box(&bx, &cr);
        }

        // draw old freehand drawings
        for bx in &self.freehands {
            Self::draw_freehand(&bx, &cr);
        }

        // Draw the rectangle if we are in drawing mode
        if let Some(state) = &mut self.current_box {
            if state.drawing {
                Self::draw_box(&state, &cr);
            }
        } else if let Some(state) = &mut self.current_arc {
            if state.drawing {
                Self::draw_arc(&state, &cr);
            }
        } else if let Some(state) = &mut self.current_arrow {
            if state.drawing {
                Self::draw_arrow(&state, &cr);
            }
        } else if let Some(state) = &mut self.current_freehand {
            if state.drawing {
                Self::draw_freehand(&state, &cr);
            }
        } else if let Some(state) = &mut self.current_line {
            if state.drawing {
                Self::draw_line(&state, &cr);
            }
        }
    }

    fn draw_freehand(freehand: &DrawingAreaFreeHandDraw, cr: &cairo::Context) {
        // get freehands size
        let sz = freehand.x.iter().len();

        if sz == 0 {
            return;
        }

        // define previous point
        let mut x_prev = freehand.x[0];
        let mut y_prev = freehand.y[0];

        // for a continuous draw we need to fill the blank space between points
        let mut cc: usize = 1;
        while cc < sz {
            // get current x and y
            let (x, y) = (freehand.x[cc], freehand.y[cc]);

            cr.set_source_rgba(freehand.red, freehand.green, freehand.blue, freehand.alpha); // Set color
            // RGBA
            // Draw the line
            cr.move_to(x_prev, y_prev);
            cr.line_to(x, y);
            // Set line properties
            cr.set_line_width(freehand.size);
            cr.set_line_cap(cairo::LineCap::Round);
            cr.stroke().unwrap();

            x_prev = x;
            y_prev = y;

            // update counter
            cc += 1;
        }
    }

    fn draw_line(line: &DrawingAreaLine, cr: &cairo::Context) {
        cr.set_source_rgba(line.red, line.green, line.blue, line.alpha); // Set color RGBA

        // Draw the line
        cr.move_to(line.x1, line.y1);
        cr.line_to(line.x2, line.y2);

        // Set line properties
        cr.set_line_width(line.size);
        cr.set_line_cap(cairo::LineCap::Round);
        cr.stroke().unwrap();
    }

    fn draw_arrow(arrow: &DrawingAreaArrow, cr: &cairo::Context) {
        cr.set_source_rgba(arrow.red, arrow.green, arrow.blue, arrow.alpha); // Set color RGBA

        // Draw the line
        cr.move_to(arrow.x1, arrow.y1);
        cr.line_to(arrow.x2, arrow.y2);

        // Draw the arrowhead
        let arrow_size = arrow.size;
        let angle = (arrow.y2 - arrow.y1).atan2(arrow.x2 - arrow.x1);
        cr.line_to(
            arrow.x2 - arrow_size * angle.cos() + arrow_size * angle.sin(),
            arrow.y2 - arrow_size * angle.sin() - arrow_size * angle.cos(),
        );
        cr.move_to(arrow.x2, arrow.y2);
        cr.line_to(
            arrow.x2 - arrow_size * angle.cos() - arrow_size * angle.sin(),
            arrow.y2 - arrow_size * angle.sin() + arrow_size * angle.cos(),
        );

        // Set line properties
        cr.set_line_width(arrow.width);
        cr.set_line_cap(cairo::LineCap::Round);
        cr.stroke().unwrap();
    }

    fn draw_arc(arc: &DrawingAreaArc, cr: &cairo::Context) {
        cr.set_source_rgba(arc.red, arc.green, arc.blue, arc.alpha); // Set color RGBA
        cr.arc(
            arc.center_x,
            arc.center_y,
            arc.radius,
            0.0,
            2.0 * f64::consts::PI,
        );
        if arc.fill {
            let _ = cr.fill();
        }
        cr.stroke().unwrap();
    }

    fn draw_box(rect: &DrawingAreaBox, cr: &cairo::Context) {
        cr.set_source_rgba(rect.red, rect.green, rect.blue, rect.alpha); // Set color RGBA
        cr.rectangle(rect.start_x, rect.start_y, rect.end_x, rect.end_y);
        if rect.fill {
            let _ = cr.fill();
        }
        cr.stroke().unwrap();
    }

    pub fn drag_begin(&mut self, x: f64, y: f64) {
        if let Some(current_box) = &mut self.current_box {
            current_box.start_x = x;
            current_box.start_y = y;
            current_box.end_x = 0.0;
            current_box.end_y = 0.0;
            current_box.drawing = true;
        }
        if let Some(current_arc) = &mut self.current_arc {
            current_arc.radius = 0.0;
            current_arc.center_x = x;
            current_arc.center_y = y;
            current_arc.drawing = true;
        }
        if let Some(current_arrow) = &mut self.current_arrow {
            current_arrow.x1 = x;
            current_arrow.y1 = y;
            current_arrow.x2 = x;
            current_arrow.y2 = y;
            current_arrow.drawing = true;
        }
        if let Some(current_line) = &mut self.current_line {
            current_line.x1 = x;
            current_line.y1 = y;
            current_line.x2 = x;
            current_line.y2 = y;
            current_line.drawing = true;
        }
        if let Some(current_freehand) = &mut self.current_freehand {
            current_freehand.x0 = x;
            current_freehand.y0 = y;
            current_freehand.drawing = true;
        }
    }

    pub fn drag_update(&mut self, x: f64, y: f64) {
        if let Some(current_box) = &mut self.current_box {
            current_box.end_x = x;
            current_box.end_y = y;
        }
        if let Some(current_arc) = &mut self.current_arc {
            current_arc.radius = f64::sqrt(x * x + y * y);
        }
        if let Some(current_line) = &mut self.current_line {
            current_line.x1 = current_line.x2 + x;
            current_line.y1 = current_line.y2 + y;
        }
        if let Some(current_arrow) = &mut self.current_arrow {
            current_arrow.x1 = current_arrow.x2 + x;
            current_arrow.y1 = current_arrow.y2 + y;
        }
        if let Some(current_freehand) = &mut self.current_freehand {
            current_freehand.x.push(current_freehand.x0 + x);
            current_freehand.y.push(current_freehand.y0 + y);
        }
    }

    pub fn drag_end(&mut self) {
        if let Some(current_box) = &mut self.current_box {
            self.boxes.push(current_box.clone());
        }
        if let Some(current_arc) = &mut self.current_arc {
            self.arcs.push(current_arc.clone());
        }
        if let Some(current_line) = &mut self.current_line {
            self.lines.push(current_line.clone());
        }
        if let Some(current_arrow) = &mut self.current_arrow {
            self.arrows.push(current_arrow.clone());
        }
        if let Some(current_freehand) = &mut self.current_freehand {
            self.freehands.push(current_freehand.clone());
        }
    }

    fn reset(&mut self) {
        self.current_freehand = None;
        self.current_arrow = None;
        self.current_line = None;
        self.current_arc = None;
        self.current_box = None;
    }
}
