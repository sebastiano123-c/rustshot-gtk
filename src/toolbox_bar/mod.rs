mod imp;

use gtk::glib;
use gtk::prelude::*;
// use rustshot_gtk::constants::CSS_CLASS_GRAY_BOX;
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

        self.set_vexpand(false);
        self.set_hexpand(false);

        self.set_halign(align);
        self.set_valign(gtk::Align::Center);
    }

    pub fn new_horizontal(&self, align: gtk::Align, geom: &GeometryState) {
        // compute screenshot box size
        self.set_orientation(gtk::Orientation::Horizontal);
        self.add_css_class(CSS_CLASS_TOOLBOX_BAR);
        self.set_height_request(TOOLBOX_BTN_SIZE);

        self.set_vexpand(false);
        self.set_hexpand(false);

        // BUG
        // description: when a settings_box gtk::Widget is appended to a ToolboxBar, if the
        //              settings_box exceeds the central_overlay width AND the screenshot box
        //              is near the right border, GTK somehow increases the size of the toolbox
        //              bar.
        // interpretation: GTK resizes on his own the gray boxes even if you don't change their
        //                 width of height. This is because grayboxes resize when a longer object
        //                 is placed inside.
        // solution: in the following, I set the margin right if the left gray box is larger; If
        //           smaller otherwise. Since this is not the behavior I want, I signal this a
        //           bug.
        let r_width = geom.right_box.get_edge();
        self.set_margin_end(r_width);

        let l_width = geom.left_box.get_edge();
        self.set_margin_start(l_width);

        self.set_valign(align);
        self.set_halign(gtk::Align::Center);
    }

    pub fn new_settings(&self, align: gtk::Align, geom: &GeometryState) {
        // compute screenshot box size
        self.set_orientation(gtk::Orientation::Horizontal);
        // self.add_css_class(CSS_CLASS_GRAY_BOX);
        self.set_height_request(TOOLBOX_BTN_SIZE);

        self.set_vexpand(false);
        self.set_hexpand(false);

        self.set_valign(align);

        let r_width = geom.right_box.get_edge();
        let l_width = geom.left_box.get_edge();
        let diff = l_width - r_width;

        // If right overflow (when right edge and screenshotbox are less than the settings size)
        let space = r_width + geom.screenshot_box.width();
        let safe_bound = geom.full_w / 3;
        if space < safe_bound {
            self.set_halign(gtk::Align::End);
            self.set_margin_end(r_width);
            return;
        }

        // Else if left overflow (when left edge and screenshotbox are less than the settings size)
        let space = l_width + geom.screenshot_box.width();
        let safe_bound = geom.full_w / 3;
        if space < safe_bound {
            self.set_halign(gtk::Align::Start);
            self.set_margin_start(l_width);
            return;
        }

        // Otherwise, center the alignment and apply a small diff to center the setting with the
        // screenshotbox
        self.set_halign(gtk::Align::Center);
        if diff > 0 {
            self.set_margin_start(diff);
            // println!("diff > 0, l_width: {}, r_width: {}", l_width, r_width);
        } else if diff < 0 {
            self.set_margin_end(-diff);
            // println!("diff < 0, l_width: {}, r_width: {}", l_width, r_width);
        }
    }

    pub fn new_fullscreen(&self, margin_top: i32) {
        self.set_orientation(gtk::Orientation::Horizontal);
        self.add_css_class(CSS_CLASS_TOOLBOX_BAR);
        self.set_halign(gtk::Align::Center);
        self.set_valign(gtk::Align::Start);
        self.set_height_request(TOOLBOX_BTN_SIZE);
        self.set_margin_top(margin_top);
    }

    pub fn fill_with_settings(&self, widget: gtk::Widget) {
        self.append(&widget);
    }

    pub fn fill(
        &self,
        length: i32,
        total_buttons: usize,
        buttons_list: &[gtk::Widget],
        n_btns_left_to_draw: &mut usize,
        if_settings_box: bool,
        show_settings_box: bool,
    ) {
        if if_settings_box {
            if *n_btns_left_to_draw == 1 && show_settings_box {
                // Draw the setting box if it is the only thing to draw
                self.fill_with_buttons(length, total_buttons, buttons_list, n_btns_left_to_draw);
            } else {
                // create a temp n_btns value.
                // since the last element of buttons list is the settings box, we need to
                // remove it. However we need to add it in the end
                let mut temp_n_btns: usize = *n_btns_left_to_draw - 1;

                // draw buttons
                self.fill_with_buttons(length, total_buttons, buttons_list, &mut temp_n_btns);

                // add 1 as reminder that the settings box is still to be drawn
                *n_btns_left_to_draw = temp_n_btns + 1;
            }
        } else {
            self.fill_with_buttons(length, total_buttons, buttons_list, n_btns_left_to_draw);
        }
    }

    pub fn fill_with_buttons(
        &self,
        length: i32,
        total_buttons: usize,
        buttons_list: &[gtk::Widget],
        n_btns_left_to_draw: &mut usize,
    ) {
        // toolbox available length to be filled with
        let l = length;

        // get how many buttons can be placed inside bottom toolbox
        let n: usize = (l / (TOOLBOX_BTN_SIZE)) as usize;

        // find the minimum
        let max_n_for_toolbox: usize = std::cmp::min(*n_btns_left_to_draw, n);

        // last idx
        let last_idx = total_buttons - *n_btns_left_to_draw;

        // fill the bottom toolbox until the maximum is reached
        for btn_idx in 0..max_n_for_toolbox {
            self.append(&buttons_list[last_idx + btn_idx]);
        }

        *n_btns_left_to_draw -= max_n_for_toolbox;
    }
}
