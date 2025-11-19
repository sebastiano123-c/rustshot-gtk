use crate::edge::GrayEdge;
use gtk::prelude::*;
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BAR, TOOLBOX_BTN_SIZE};

#[derive(Debug)]
pub struct HorizontalToolboxBar {
    toolbox: gtk::Box,
}

impl HorizontalToolboxBar {
    pub fn new(
        parent_widget: &GrayEdge,
        align: gtk::Align,
        left_space: i32,
        right_space: i32,
    ) -> Self {
        // create full toolbox
        let full_tb = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        full_tb.set_height_request(TOOLBOX_BTN_SIZE);
        full_tb.set_valign(align);

        // create left spacing
        let l_space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        l_space.set_width_request(std::cmp::max(left_space - TOOLBOX_BTN_SIZE / 2 - 5, 0));
        l_space.set_height_request(TOOLBOX_BTN_SIZE);
        l_space.set_halign(gtk::Align::Start);

        // create toolbox
        let tb_cont = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        tb_cont.set_hexpand(true);
        let tb = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        tb.add_css_class(CSS_CLASS_TOOLBOX_BAR);
        tb.set_halign(gtk::Align::Center);
        tb.set_hexpand(true);
        tb_cont.append(&tb);

        // create right spacing
        let r_space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        r_space.set_width_request(std::cmp::max(right_space - TOOLBOX_BTN_SIZE / 2 - 5, 0));
        r_space.set_height_request(TOOLBOX_BTN_SIZE);
        r_space.set_halign(gtk::Align::End);

        // append it to bottom gray box
        full_tb.append(&l_space);
        full_tb.append(&tb_cont);
        full_tb.append(&r_space);
        parent_widget.append(&full_tb);

        Self { toolbox: tb }
    }

    pub fn new_fullscreen(
        overlay: &gtk::Overlay,
        align: gtk::Align,
        left_space: i32,
        right_space: i32,
    ) -> Self {
        // create full toolbox
        let full_tb = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        full_tb.set_height_request(TOOLBOX_BTN_SIZE);
        full_tb.set_valign(align);

        // create left spacing
        let l_space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        l_space.set_width_request(std::cmp::max(left_space - TOOLBOX_BTN_SIZE / 2 - 5, 0));
        l_space.set_height_request(TOOLBOX_BTN_SIZE);
        l_space.set_halign(gtk::Align::Start);

        // create toolbox
        let tb_cont = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        tb_cont.set_hexpand(true);
        let tb = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        tb.add_css_class(CSS_CLASS_TOOLBOX_BAR);
        tb.set_halign(gtk::Align::Center);
        tb.set_hexpand(true);
        tb_cont.append(&tb);

        // create right spacing
        let r_space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        r_space.set_width_request(std::cmp::max(right_space - TOOLBOX_BTN_SIZE / 2 - 5, 0));
        r_space.set_height_request(TOOLBOX_BTN_SIZE);
        r_space.set_halign(gtk::Align::End);

        // append it to bottom gray box
        full_tb.append(&l_space);
        full_tb.append(&tb_cont);
        full_tb.append(&r_space);
        overlay.add_overlay(&full_tb);
        overlay.queue_draw();

        Self { toolbox: tb }
    }

    pub fn fill_with_buttons(
        &self,
        length: i32,
        total_buttons: usize,
        buttons_list: &[gtk::Button],
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
            self.toolbox.append(&buttons_list[last_idx + btn_idx]);
        }

        *n_btns_left_to_draw -= max_n_for_bottom_toolbox;
    }
}

#[derive(Debug)]
pub struct VerticalToolboxBar {
    toolbox: gtk::Box,
}

impl VerticalToolboxBar {
    pub fn new(parent_widget: &GrayEdge, size: i32, align: gtk::Align) -> Self {
        let tb = gtk::Box::new(gtk::Orientation::Vertical, 0);

        tb.set_width_request(size);
        tb.set_halign(align);
        tb.set_valign(gtk::Align::Center);

        parent_widget.append(&tb);

        Self { toolbox: tb }
    }

    pub fn fill_with_buttons(
        &self,
        length: i32,
        total_buttons: usize,
        buttons_list: &[gtk::Button],
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
            self.toolbox.append(&buttons_list[last_idx + btn_idx]);
        }

        *n_btns_left_to_draw -= max_n_for_bottom_toolbox;
    }
}
