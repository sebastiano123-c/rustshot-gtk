use core::f64;
use gtk::cairo;

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
    fill: bool,
}

#[derive(Debug, Clone)]
pub struct DrawingAreaManager {
    current_box: Option<DrawingAreaBox>,
    current_arc: Option<DrawingAreaArc>,
    current_arrow: Option<DrawingAreaArrow>,
    boxes: Vec<DrawingAreaBox>,
    arcs: Vec<DrawingAreaArc>,
    arrows: Vec<DrawingAreaArrow>,
    pub is_drawing: bool,
}

impl DrawingAreaManager {
    pub fn new() -> Self {
        Self {
            current_box: None,
            current_arc: None,
            current_arrow: None,
            boxes: Vec::new(),
            arcs: Vec::new(),
            arrows: Vec::new(),
            is_drawing: false,
        }
    }

    pub fn create_new_arrow(&mut self, arrow_size: f64, arrow_width: f64, r: f64, g: f64, b: f64) {
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
            drawing: false,
        });

        // emit signal for drawing boxes
        self.is_drawing = true;
    }

    pub fn create_new_arc(&mut self, red: f64, green: f64, blue: f64, fill_rect: bool) {
        self.reset();

        self.current_arc = Some(DrawingAreaArc {
            radius: 0.0,
            center_x: 0.0,
            center_y: 0.0,
            red,
            green,
            blue,
            drawing: false,
            fill: fill_rect,
        });

        // emit signal for drawing boxes
        self.is_drawing = true;
    }

    pub fn create_new_box(&mut self, red: f64, green: f64, blue: f64, fill_rect: bool) {
        self.reset();
        self.current_box = Some(DrawingAreaBox {
            start_x: 0.0,
            start_y: 0.0,
            end_x: 0.0,
            end_y: 0.0,
            red,
            green,
            blue,
            drawing: false,
            fill: fill_rect,
        });

        // emit signal for drawing boxes
        self.is_drawing = true;
    }

    pub fn is_drawing(&self) -> bool {
        self.is_drawing
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
        for bx in &self.arcs {
            Self::draw_arc(&bx, &cr);
        }

        // draw old boxes
        for bx in &self.boxes {
            Self::draw_box(&bx, &cr);
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
        }
    }

    fn draw_arrow(arrow: &DrawingAreaArrow, cr: &cairo::Context) {
        cr.set_source_rgb(arrow.red, arrow.green, arrow.blue); // Black color

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
        cr.set_source_rgb(arc.red, arc.green, arc.blue); // Black color
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
        cr.set_source_rgb(rect.red, rect.green, rect.blue); // Black color
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
    }

    pub fn drag_update(&mut self, x: f64, y: f64) {
        if let Some(current_box) = &mut self.current_box {
            current_box.end_x = x;
            current_box.end_y = y;
        }
        if let Some(current_arc) = &mut self.current_arc {
            current_arc.radius = f64::sqrt(x * x + y * y);
        }
        if let Some(current_arrow) = &mut self.current_arrow {
            current_arrow.x1 = current_arrow.x2 + x;
            current_arrow.y1 = current_arrow.y2 + y;
        }
    }

    pub fn drag_end(&mut self) {
        if let Some(current_box) = &mut self.current_box {
            self.boxes.push(current_box.clone());
        }
        if let Some(current_arc) = &mut self.current_arc {
            self.arcs.push(current_arc.clone());
        }
        if let Some(current_arrow) = &mut self.current_arrow {
            self.arrows.push(current_arrow.clone());
        }
    }

    fn reset(&mut self) {
        self.current_arrow = None;
        self.current_arc = None;
        self.current_box = None;
    }
}

// #[derive(Debug, Clone)]
// pub struct DrawingAreaBoxes {
//     current_box: DrawingAreaBox,
//     boxes: Vec<DrawingAreaBox>,
//     pub is_drawing_boxes: bool,
// }
//
// impl DrawingAreaBoxes {
//     pub fn new(red: f64, green: f64, blue: f64) -> DrawingAreaBoxes {
//         DrawingAreaBoxes {
//             current_box: DrawingAreaBox {
//                 start_x: 0.0,
//                 start_y: 0.0,
//                 end_x: 0.0,
//                 end_y: 0.0,
//                 red,
//                 green,
//                 blue,
//                 drawing: false,
//                 fill: false,
//             },
//             boxes: Vec::new(),
//             is_drawing_boxes: false,
//         }
//     }
//
//     pub fn create_new_box(&mut self, fill_rect: bool, red: f64, green: f64, blue: f64) {
//         self.current_box = DrawingAreaBox {
//             start_x: 0.0,
//             start_y: 0.0,
//             end_x: 0.0,
//             end_y: 0.0,
//             red,
//             green,
//             blue,
//             drawing: false,
//             fill: fill_rect,
//         };
//
//         // emit signal for drawing boxes
//         self.is_drawing_boxes = true;
//     }
//
//     pub fn is_drawing(&self) -> bool {
//         self.is_drawing_boxes
//     }
//
//     pub fn set_draw(&mut self, cr: &cairo::Context) {
//         // Clear the drawing area
//         cr.set_source_rgba(0.0, 0.0, 0.0, 0.0); // White background
//         cr.paint().unwrap();
//
//         // draw old boxes
//         for bx in &self.boxes {
//             cr.set_source_rgb(bx.red, bx.green, bx.blue); // Black color
//             cr.rectangle(bx.start_x, bx.start_y, bx.end_x, bx.end_y);
//             if bx.fill {
//                 let _ = cr.fill();
//             }
//             cr.stroke().unwrap();
//         }
//
//         // Draw the rectangle if we are in drawing mode
//         let state = self.current_box.clone();
//         if state.drawing {
//             cr.set_source_rgb(state.red, state.green, state.blue); // Black color
//             cr.rectangle(state.start_x, state.start_y, state.end_x, state.end_y);
//             if state.fill {
//                 let _ = cr.fill();
//             }
//             cr.stroke().unwrap();
//         }
//     }
//
//     pub fn drag_begin_box(&mut self, x: f64, y: f64) {
//         self.current_box.start_x = x;
//         self.current_box.start_y = y;
//         self.current_box.end_x = 0.0;
//         self.current_box.end_y = 0.0;
//         self.current_box.drawing = true;
//     }
//
//     pub fn drag_update_box(&mut self, x: f64, y: f64) {
//         self.current_box.end_x = x;
//         self.current_box.end_y = y;
//     }
//
//     pub fn drag_end_box(&mut self) {
//         self.boxes.push(self.current_box.clone());
//     }
// }
