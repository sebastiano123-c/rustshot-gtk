mod imp;

use gtk::glib;
use gtk::prelude::*;
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BAR, TOOLBOX_BTN_SIZE};

use crate::geometry::GeometryState;

glib::wrapper! {
    pub struct ToolboxBar(ObjectSubclass<imp::ToolboxBar>)
        @extends gtk::Box,
        @implements gtk::Accessible, gtk::Actionable, gtk::Widget, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ToolboxBar {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ToolboxBar {
    pub fn new_vertical(&self, size: i32, align: gtk::Align) {
        self.set_orientation(gtk::Orientation::Vertical);
        self.add_css_class(CSS_CLASS_TOOLBOX_BAR);
        self.set_width_request(size);
        self.set_halign(align);
        self.set_valign(gtk::Align::Center);
    }

    pub fn new_horizontal(&self, align: gtk::Align, geom: &GeometryState) {
        // compute screenshot box size
        self.set_orientation(gtk::Orientation::Horizontal);
        self.add_css_class(CSS_CLASS_TOOLBOX_BAR);
        self.set_height_request(TOOLBOX_BTN_SIZE);
        self.set_margin_end(geom.right_box.get_edge() + 5);
        self.set_margin_start(geom.left_box.get_edge() + 5);
        self.set_valign(align);
    }

    pub fn new_fullscreen(&self, geom: &GeometryState) {
        // create toolbox
        self.set_orientation(gtk::Orientation::Horizontal);
        self.add_css_class(CSS_CLASS_TOOLBOX_BAR);
        self.set_halign(gtk::Align::Center);
        self.set_height_request(TOOLBOX_BTN_SIZE);
        self.set_margin_start(geom.full_w / 2);
        self.set_margin_top(10);
        self.set_margin_bottom(geom.full_h - TOOLBOX_BTN_SIZE);
    }

    pub fn fill_with_buttons(
        &self,
        length: i32,
        total_buttons: usize,
        buttons_list: &[gtk::Widget],
        n_btns_left_to_draw: &mut usize,
    ) {
        let l = length;

        // get how many buttons can be placed inside bottom toolbox
        let n: usize = (l / (TOOLBOX_BTN_SIZE)) as usize;

        // find the minimum
        let max_n_for_bottom_toolbox: usize = std::cmp::min(*n_btns_left_to_draw, n);

        // last idx
        let last_idx = total_buttons - *n_btns_left_to_draw;

        // fill the bottom toolbox until the maximum is reached
        for btn_idx in 0..max_n_for_bottom_toolbox {
            self.append(&buttons_list[last_idx + btn_idx]);
        }

        *n_btns_left_to_draw -= max_n_for_bottom_toolbox;
    }
}
