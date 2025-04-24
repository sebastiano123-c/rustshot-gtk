use gtk::cairo;

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
pub struct DrawingAreaBoxes {
    current_box: DrawingAreaBox,
    boxes: Vec<DrawingAreaBox>,
    pub is_drawing_boxes: bool,
}

impl DrawingAreaBoxes {
    pub fn new(red: f64, green: f64, blue: f64) -> DrawingAreaBoxes {
        DrawingAreaBoxes {
            current_box: DrawingAreaBox {
                start_x: 0.0,
                start_y: 0.0,
                end_x: 0.0,
                end_y: 0.0,
                red,
                green,
                blue,
                drawing: false,
                fill: false,
            },
            boxes: Vec::new(),
            is_drawing_boxes: false,
        }
    }

    pub fn create_new_box(&mut self, fill_rect: bool, red: f64, green: f64, blue: f64) {
        self.current_box = DrawingAreaBox {
            start_x: 0.0,
            start_y: 0.0,
            end_x: 0.0,
            end_y: 0.0,
            red,
            green,
            blue,
            drawing: false,
            fill: fill_rect,
        };

        // emit signal for drawing boxes
        self.is_drawing_boxes = true;
    }

    pub fn is_drawing(&self) -> bool {
        self.is_drawing_boxes
    }

    pub fn set_draw(&mut self, cr: &cairo::Context) {
        // Clear the drawing area
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.0); // White background
        cr.paint().unwrap();

        // draw old boxes
        for bx in &self.boxes {
            cr.set_source_rgb(bx.red, bx.green, bx.blue); // Black color
            cr.rectangle(bx.start_x, bx.start_y, bx.end_x, bx.end_y);
            if bx.fill {
                let _ = cr.fill();
            }
            cr.stroke().unwrap();
        }

        // Draw the rectangle if we are in drawing mode
        let state = self.current_box.clone();
        if state.drawing {
            cr.set_source_rgb(state.red, state.green, state.blue); // Black color
            cr.rectangle(state.start_x, state.start_y, state.end_x, state.end_y);
            if state.fill {
                let _ = cr.fill();
            }
            cr.stroke().unwrap();
        }
    }

    pub fn drag_begin_box(&mut self, x: f64, y: f64) {
        self.current_box.start_x = x;
        self.current_box.start_y = y;
        self.current_box.end_x = 0.0;
        self.current_box.end_y = 0.0;
        self.current_box.drawing = true;
    }

    pub fn drag_update_box(&mut self, x: f64, y: f64) {
        self.current_box.end_x = x;
        self.current_box.end_y = y;
    }

    pub fn drag_end_box(&mut self) {
        self.boxes.push(self.current_box.clone());
    }
}
