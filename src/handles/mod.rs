use super::toolbox::Toolbox;
use gtk::glib;
use gtk::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

/// Small helper that groups the four side boxes as weak references.
/// Using weak refs prevents reference cycles when the gesture callbacks
/// outlive the widgets that created them.
#[derive(Clone)]
struct SideBoxes {
    top: glib::WeakRef<gtk::Box>,
    left: glib::WeakRef<gtk::Box>,
    bottom: glib::WeakRef<gtk::Box>,
    right: glib::WeakRef<gtk::Box>,
}

impl SideBoxes {
    fn upgrade(&self) -> Option<(gtk::Box, gtk::Box, gtk::Box, gtk::Box)> {
        Some((
            self.top.upgrade()?,
            self.left.upgrade()?,
            self.bottom.upgrade()?,
            self.right.upgrade()?,
        ))
    }
}

/// Stores the mutable geometry values used by the drag callbacks.
#[derive(Clone)]
struct GeometryState {
    // Current positions (0.0 … full size)
    top: Rc<Cell<f64>>,
    left: Rc<Cell<f64>>,
    bottom: Rc<Cell<f64>>,
    right: Rc<Cell<f64>>,

    // Screenshot‑area helpers (the same type you used before)
    sx: Rc<Cell<f64>>,
    sy: Rc<Cell<f64>>,
    sw: Rc<Cell<f64>>,
    sh: Rc<Cell<f64>>,

    // Full window size – constant for the life of the widget
    full_w: f64,
    full_h: f64,
}

impl GeometryState {
    /// Clamp the values so none become negative and keep the layout sane.
    fn clamp(&self, l: f64, r: f64, t: f64, b: f64) -> (f64, f64, f64, f64) {
        let l = l.max(0.0);
        let r = r.max(0.0);
        let t = t.max(0.0);
        let b = b.max(0.0);
        (l, r, t, b)
    }

    /// Apply the new geometry to the UI and to the stored properties.
    fn apply(
        &self,
        left: f64,
        right: f64,
        top: f64,
        bottom: f64,
        side_boxes: &(gtk::Box, gtk::Box, gtk::Box, gtk::Box),
    ) {
        let (top_box, left_box, bottom_box, right_box) = side_boxes;

        // --------------------------------------------------------------------
        // 1️⃣  Update the actual widget sizes (GTK expects i32)
        // --------------------------------------------------------------------
        top_box.set_height_request(top as i32);
        bottom_box.set_height_request(bottom as i32);
        right_box.set_width_request(right as i32);
        left_box.set_width_request(left as i32);

        // --------------------------------------------------------------------
        // 2️⃣  Sync the internal property values
        // --------------------------------------------------------------------
        self.sx.set(left);
        self.sy.set(top);
        self.sw.set(self.full_w - right - left);
        self.sh.set(self.full_h - top - bottom);

        self.top.set(top);
        self.bottom.set(bottom);
        self.left.set(left);
        self.right.set(right);
    }
}

#[derive(Debug, Clone)]
pub struct Handles {
    // margins
    top: Rc<Cell<f64>>,
    bottom: Rc<Cell<f64>>,
    left: Rc<Cell<f64>>,
    right: Rc<Cell<f64>>,
    // boxes
    top_box: gtk::Box,
    bottom_box: gtk::Box,
    left_box: gtk::Box,
    right_box: gtk::Box,
    // central_box: gtk::Box,
    screenshot_box: gtk::Box,
    // screenshot box start and size
    x: Rc<Cell<f64>>,
    y: Rc<Cell<f64>>,
    w: Rc<Cell<f64>>,
    h: Rc<Cell<f64>>,
    // margin for the screeshot border handles
    handles_size: i32,
    full_w: f64,
    full_h: f64,
    central_handle: gtk::Box,
    central_handle_sensitive: Rc<Cell<bool>>,
    toolbox: Toolbox,
}

impl Handles {
    pub fn new(
        top_margin: Rc<Cell<f64>>,
        bottom_margin: Rc<Cell<f64>>,
        left_margin: Rc<Cell<f64>>,
        right_margin: Rc<Cell<f64>>,
        top_box: gtk::Box,
        bottom_box: gtk::Box,
        left_box: gtk::Box,
        right_box: gtk::Box,
        screenshot_box: gtk::Box,
        screenshot_x: Rc<Cell<f64>>,
        screenshot_y: Rc<Cell<f64>>,
        screenshot_w: Rc<Cell<f64>>,
        screenshot_h: Rc<Cell<f64>>,
        handles_sz: i32,
        full_w: f64,
        full_h: f64,
        toolbox: Toolbox,
    ) -> Self {
        Self {
            top: top_margin,
            bottom: bottom_margin,
            left: left_margin,
            right: right_margin,
            top_box,
            bottom_box,
            left_box,
            right_box,
            screenshot_box,
            x: screenshot_x,
            y: screenshot_y,
            w: screenshot_w,
            h: screenshot_h,
            handles_size: handles_sz,
            central_handle: gtk::Box::new(gtk::Orientation::Horizontal, 0),
            central_handle_sensitive: Rc::new(Cell::new(true)),
            full_w,
            full_h,
            toolbox,
        }
    }

    pub fn attach_handles(&self) {
        //clone
        let screenshot_box = self.screenshot_box.clone();

        // define class
        let css_class = "transparent";

        // create handles top
        let top_spacer = self.make_spacer(0, css_class, gtk::Align::Start);

        // create handles center
        let center_spacer = self.make_spacer(1, css_class, gtk::Align::Fill);

        // create handles bottom
        let bottom_spacer = self.make_spacer(2, css_class, gtk::Align::End);

        // append to screenshot box
        screenshot_box.append(&top_spacer);
        screenshot_box.append(&center_spacer);
        screenshot_box.append(&bottom_spacer);
    }

    pub fn add_controller_to_central_handle(&self, gest: gtk::GestureDrag) {
        self.central_handle.add_controller(gest);
    }

    pub fn set_central_box_sensitivity(&mut self, sensitive: bool) {
        // let c = self.central_handle.clone();
        // c.set_sensitive(sensitive);
        self.central_handle_sensitive.set(sensitive);
        // println!("sens set to {}", sensitive);
    }

    /// Helper that creates a “spacer” (the container that holds three handles).
    fn make_spacer(&self, position: u8, css_class: &str, align: gtk::Align) -> gtk::Box {
        let sp = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        sp.add_css_class(css_class);
        sp.set_valign(align);

        // The size is only relevant for the *cross* axis:
        // – Horizontal spacers (top / bottom) need a height.
        // – Vertical spacers (center) need a width.
        if position == 1 {
            sp.set_vexpand(true);
        } else {
            sp.set_height_request(self.handles_size);
        }

        let l_handle = self.create_handle(0, position);
        l_handle.set_width_request(self.handles_size);
        let c_handle = self.create_handle(1, position);
        c_handle.set_hexpand(true);
        let r_handle = self.create_handle(2, position);
        r_handle.set_width_request(self.handles_size);
        r_handle.set_halign(gtk::Align::End);
        sp.append(&l_handle);
        sp.append(&c_handle);
        sp.append(&r_handle);

        sp
    }

    /// Creates a draggable resize handle.
    ///
    /// * `col` / `row` – identify which side of the window the handle belongs to.
    ///   `col == 1 && row == 1` denotes the central handle.
    /// * Returns a `gtk::Box` that already has a `GestureDrag` controller attached.
    ///
    /// The function assumes the parent widget lives longer than any drag operation;
    /// weak references are upgraded at the start of each callback and silently ignored
    /// if the widget has already been destroyed.
    pub fn create_handle(&self, col: u8, row: u8) -> gtk::Box {
        // --------------------------------------------------------------
        // 1️⃣  Pick the correct base widget (central vs corner)
        // --------------------------------------------------------------
        let handle = if col == 1 && row == 1 {
            self.central_handle.clone()
        } else {
            let b = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            b.add_css_class("corner-handle");
            b
        };

        // --------------------------------------------------------------
        // 2️⃣  Install a drag gesture controller
        // --------------------------------------------------------------
        let gesture = gtk::GestureDrag::new();
        handle.add_controller(gesture.clone());

        // --------------------------------------------------------------
        // 3️⃣  Bundle the side boxes as weak refs for the callbacks
        // --------------------------------------------------------------
        let side_boxes = SideBoxes {
            top: self.top_box.downgrade(),
            left: self.left_box.downgrade(),
            bottom: self.bottom_box.downgrade(),
            right: self.right_box.downgrade(),
        };

        // --------------------------------------------------------------
        // 4️⃣  Clone everything we need inside the closures
        // --------------------------------------------------------------
        let toolbox = self.toolbox.clone();

        // GeometryState holds all the mutable numbers.
        let geom_state = GeometryState {
            top: self.top.clone(),
            left: self.left.clone(),
            bottom: self.bottom.clone(),
            right: self.right.clone(),

            sx: self.x.clone(),
            sy: self.y.clone(),
            sw: self.w.clone(),
            sh: self.h.clone(),

            full_w: self.full_w,
            full_h: self.full_h,
        };

        let central_sensitive = self.central_handle_sensitive.clone();

        // --------------------------------------------------------------
        // 5️⃣  Drag‑begin: hide the toolbox while the user resizes
        // --------------------------------------------------------------
        gesture.connect_drag_begin(glib::clone!(
            #[strong]
            toolbox,
            move |_, _, _| {
                toolbox.stop_toolbox();
            }
        ));

        // --------------------------------------------------------------
        // 6️⃣  Drag‑update: compute new geometry and apply it
        // --------------------------------------------------------------
        gesture.connect_drag_update(glib::clone!(
            #[strong]
            side_boxes,
            #[strong]
            geom_state,
            #[strong]
            central_sensitive,
            move |_, dx, dy| {
                // Upgrade weak refs; abort if any side box disappeared.
                let (top_box, left_box, bottom_box, right_box) = match side_boxes.upgrade() {
                    Some(v) => v,
                    None => return,
                };

                // -----------------------------------------------------------------
                // 6a. Read the current values from the Cells
                // -----------------------------------------------------------------
                let mut l = geom_state.left.get();
                let mut r = geom_state.right.get();
                let mut t = geom_state.top.get();
                let mut b = geom_state.bottom.get();

                // -----------------------------------------------------------------
                // 6b. Adjust according to which handle is being dragged
                // -----------------------------------------------------------------
                match col {
                    0 => l += dx, // left side
                    1 => {
                        // centre column – only move horizontally when the central
                        // handle is marked as “sensitive”.
                        if row == 1 && central_sensitive.get() {
                            l += dx;
                            r -= dx;
                            t += dy;
                            b -= dy;
                        }
                    }
                    2 => r -= dx, // right side
                    _ => {}
                }

                match row {
                    0 => t += dy, // top side
                    2 => b -= dy, // bottom side
                    _ => {}
                }

                // -----------------------------------------------------------------
                // 6c. Clamp to avoid negative dimensions
                // -----------------------------------------------------------------
                let (l, r, t, b) = geom_state.clamp(l, r, t, b);

                // -----------------------------------------------------------------
                // 6d. Apply the new geometry to the UI and to the stored Cells
                // -----------------------------------------------------------------
                geom_state.apply(l, r, t, b, &(top_box, left_box, bottom_box, right_box));
            }
        ));

        // --------------------------------------------------------------
        // 7️⃣  Drag‑end: redraw the toolbox now that resizing is done
        // --------------------------------------------------------------
        gesture.connect_drag_end(glib::clone!(
            #[strong]
            toolbox,
            move |_, _, _| {
                toolbox.draw_toolbox();
            }
        ));

        handle
    }
}
