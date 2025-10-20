use super::toolbox::Toolbox;
use gtk::glib;
use gtk::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

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
        let top_spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        top_spacer.add_css_class(css_class);
        top_spacer.set_height_request(self.handles_size);
        top_spacer.set_valign(gtk::Align::Start);
        let tl_handle = self.create_handle(0, 0);
        tl_handle.set_width_request(self.handles_size);
        let tc_handle = self.create_handle(1, 0);
        tc_handle.set_hexpand(true);
        let tr_handle = self.create_handle(2, 0);
        tr_handle.set_width_request(self.handles_size);
        tr_handle.set_halign(gtk::Align::End);
        top_spacer.append(&tl_handle);
        top_spacer.append(&tc_handle);
        top_spacer.append(&tr_handle);

        // create handles center
        let center_spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        center_spacer.add_css_class(css_class);
        center_spacer.set_vexpand(true);
        let cl_handle = self.create_handle(0, 1);
        cl_handle.set_width_request(self.handles_size);
        let cc_handle = self.create_handle(1, 1);
        cc_handle.set_hexpand(true);
        // cc_handle.add_css_class(css_class);
        let cr_handle = self.create_handle(2, 1);
        cr_handle.set_width_request(self.handles_size);
        cr_handle.set_halign(gtk::Align::End);
        center_spacer.append(&cl_handle);
        center_spacer.append(&cc_handle);
        center_spacer.append(&cr_handle);

        // create handles bottom
        let bottom_spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        bottom_spacer.add_css_class(css_class);
        bottom_spacer.set_height_request(self.handles_size);
        bottom_spacer.set_valign(gtk::Align::End);
        let bl_handle = self.create_handle(0, 2);
        bl_handle.set_width_request(self.handles_size);
        let bc_handle = self.create_handle(1, 2);
        bc_handle.set_hexpand(true);
        let br_handle = self.create_handle(2, 2);
        br_handle.set_width_request(self.handles_size);
        br_handle.set_halign(gtk::Align::End);
        bottom_spacer.append(&bl_handle);
        bottom_spacer.append(&bc_handle);
        bottom_spacer.append(&br_handle);

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

    fn create_handle(&self, col: u8, row: u8) -> gtk::Box {
        // create handle
        let mut hdl: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        // discriminate if is the central handle or not
        if col == 1_u8 && row == 1_u8 {
            hdl = self.central_handle.clone();
        } else {
            hdl.add_css_class("corner-handle");
        }

        // clone
        let (top_box, left_box, bottom_box, right_box) = (
            self.top_box.clone(),
            self.left_box.clone(),
            self.bottom_box.clone(),
            self.right_box.clone(),
        );
        let sensitive = self.central_handle_sensitive.clone();

        // margins
        // let (full_w, full_h) = (self.full_w, self.full_h);
        let (top, left, bottom, right) = (
            self.top.clone(),
            self.left.clone(),
            self.bottom.clone(),
            self.right.clone(),
        );

        // screenshot dimensions
        let (sx, sy, sw, sh) = (
            self.x.clone(),
            self.y.clone(),
            self.w.clone(),
            self.h.clone(),
        );

        // attach gesture to handle
        let gest = gtk::GestureDrag::new();
        hdl.add_controller(gest.clone());

        // get full width and height
        let (full_w, full_h) = (self.full_w, self.full_h);

        // toolbox
        let toolbox = self.toolbox.clone();

        gest.connect_drag_begin({
            let toolbox = toolbox.clone();
            move |_, _, _| {
                toolbox.stop_toolbox();
            }
        });

        gest.connect_drag_update(glib::clone!(
            #[weak]
            top_box,
            #[weak]
            left_box,
            #[weak]
            bottom_box,
            #[weak]
            right_box,
            move |_, x, y| {
                // calculate
                let mut l = left.get();
                let mut r = right.get();
                let mut t = top.get();
                let mut b = bottom.get();

                // case column
                match col {
                    0_u8 => {
                        l += x;
                    }
                    1_u8 => {
                        if row == 1_u8 && sensitive.get() {
                            l += x;
                            r -= x;
                            t += y;
                            b -= y;
                        }
                    }
                    2_u8 => {
                        r -= x;
                    }
                    _ => {}
                }

                // case row
                match row {
                    0_u8 => {
                        t += y;
                    }
                    2_u8 => {
                        b -= y;
                    }
                    _ => {}
                }

                // prevent negative values
                if l < 0.0 {
                    l = 0.0;
                    r = right_box.width() as f64;
                } else if r < 0.0 {
                    l = left_box.width() as f64;
                    r = 0.0;
                }
                if t < 0.0 {
                    t = 0.0;
                    b = bottom_box.height() as f64;
                } else if b < 0.0 {
                    t = top_box.height() as f64;
                    b = 0.0;
                }

                // set
                top_box.set_height_request(t as i32);
                bottom_box.set_height_request(b as i32);
                right_box.set_width_request(r as i32);
                left_box.set_width_request(l as i32);

                // update
                sx.set(l);
                sy.set(t);
                sw.set(full_w - r - l);
                sh.set(full_h - t - b);

                // save
                top.set(t);
                bottom.set(b);
                left.set(l);
                right.set(r);
            }
        ));

        gest.connect_drag_end(move |_, _, _| {
            // #[weak]
            // top_box,
            // #[weak]
            // left_box,
            // #[weak]
            // bottom_box,
            // #[weak]
            // right_box,
            // redraw toolbox
            toolbox.draw_toolbox();
        });

        hdl
    }
}
