// use gtk::glib;
use gtk::prelude::*;
// use gtk::{gdk, glib};
// use std::cell::Cell;
// use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Toolbox {
    buttons_width: i32,
    n_buttons: usize,
    central_box: gtk::Overlay,
    bottom_box: gtk::Box,
    right_box: gtk::Box,
    top_box: gtk::Box,
    left_box: gtk::Box,
    buttons_list: Vec<gtk::Button>,
    // is_toolbox_drawn: bool,
}

impl Toolbox {
    pub fn new(
        btn_width: i32,
        central: gtk::Overlay,
        top: gtk::Box,
        left: gtk::Box,
        bottom: gtk::Box,
        right: gtk::Box,
    ) -> Self {
        Self {
            buttons_width: btn_width,
            n_buttons: 0,
            central_box: central,
            bottom_box: bottom,
            right_box: right,
            top_box: top,
            left_box: left,
            buttons_list: Vec::new(),
            // is_toolbox_drawn: false,
        }
    }

    fn create_toolbox(size: i32, orientation: gtk::Orientation, align: gtk::Align) -> gtk::Box {
        let tb = gtk::Box::new(orientation, 0);
        // tb.add_css_class("toolbox");

        if orientation == gtk::Orientation::Horizontal {
            tb.set_height_request(size);
            tb.set_valign(align);
            tb.set_halign(gtk::Align::Center);
            tb.set_hexpand(true);
        } else {
            tb.set_width_request(size);
            tb.set_halign(align);
            tb.set_valign(gtk::Align::Center);
        }
        tb
    }

    pub fn create_toolbox_button(
        &mut self,
        label: &str,
        tooltip_text: Option<&str>,
    ) -> gtk::Button {
        // Load Font Awesome Solid font
        let btn = gtk::Button::with_label(label);
        btn.set_tooltip_text(tooltip_text);
        btn.add_css_class("toolbox-btn");
        btn.set_width_request(self.buttons_width);
        btn.set_height_request(self.buttons_width);

        // add to list
        self.buttons_list.push(btn.clone());
        self.n_buttons += 1;

        // return
        btn
    }

    pub fn draw_toolbox(&self) {
        // number of buttons left to draw
        let mut n_btn_left_to_draw: usize = self.n_buttons;

        // counter that accounts the gray box in which the toolbox is placed
        let mut gray_box_idx = 0;

        // counter that accounts the order of the toolbox.
        // If the gray box has only one toolbox the order is 1;
        // If the gray box has two toolboxes the order is 2, and so on...
        let mut order_idx = 1;

        // if it equals 4 then no space left. Thus draw on the central box
        let mut no_space_left: u8 = 0;

        // go ahead until there are no buttons to place
        let mut cc = 0;
        while n_btn_left_to_draw != 0 {
            // just to be sure that the while does not go on forever
            cc += 1;
            if cc > 1000 {
                break;
            }

            // if it was impossible to draw a toolbox in each gray box it means that the
            // margins are too small and the central box too big.
            // So we add the toolbox to the central box overlay.
            // the problem is that it creates more space on the bottom!
            if no_space_left >= 4 && order_idx == 2 {
                // create toolbox
                let tb = Self::create_toolbox(
                    self.buttons_width,
                    gtk::Orientation::Horizontal,
                    gtk::Align::Start,
                );
                tb.set_halign(gtk::Align::Center);

                // append it to bottom gray box
                self.central_box.add_overlay(&tb);
                self.central_box.queue_draw();

                // insert as many buttons as possible
                self.attach_to_toolbox(self.central_box.width(), tb, &mut n_btn_left_to_draw);
            }

            // println!("{},{}", no_space_left, order_idx);
            // select gray box
            match gray_box_idx {
                // Attach toolbox to bottom gray box
                0 => {
                    // check if there is room to place the toolbox
                    if order_idx * self.buttons_width > self.bottom_box.height() {
                        // else increment counter
                        gray_box_idx += 1;

                        // add 1 to no space left value
                        no_space_left += 1;

                        // if not we need to go to the next iteration
                        continue;
                    }

                    // create full toolbox
                    let full_tb = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    full_tb.set_height_request(self.buttons_width);

                    // create left spacing
                    let l_space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    l_space.set_width_request(std::cmp::max(
                        self.left_box.width() - self.buttons_width / 2 - 5,
                        0,
                    ));
                    l_space.set_height_request(self.buttons_width);
                    l_space.set_halign(gtk::Align::Start);

                    // create toolbox
                    let tb_cont = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    tb_cont.set_hexpand(true);
                    let tb = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    tb.set_halign(gtk::Align::Center);
                    tb.set_hexpand(true);
                    tb_cont.append(&tb);

                    // create right spacing
                    let r_space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    r_space.set_width_request(std::cmp::max(
                        self.right_box.width() - self.buttons_width / 2 - 5,
                        0,
                    ));
                    r_space.set_height_request(self.buttons_width);
                    r_space.set_halign(gtk::Align::End);

                    // append it to bottom gray box
                    full_tb.append(&l_space);
                    full_tb.append(&tb_cont);
                    full_tb.append(&r_space);
                    self.bottom_box.append(&full_tb);
                    self.bottom_box.queue_draw();

                    // insert as many buttons as possible
                    let l = std::cmp::max(self.central_box.width(), self.buttons_width + 10);
                    self.attach_to_toolbox(l, tb, &mut n_btn_left_to_draw);

                    // set space left to zero
                    no_space_left = 0;

                    // increment counter
                    gray_box_idx += 1;
                }
                // Attach toolbox to right gray box
                1 => {
                    // check if there is room to place the toolbox
                    if order_idx * self.buttons_width > self.right_box.width() {
                        // else increment counter
                        gray_box_idx += 1;

                        // add 1 to no space left value
                        no_space_left += 1;

                        // if not we need to go to the next iteration
                        continue;
                    }

                    // create toolbox
                    let tb = Self::create_toolbox(
                        self.buttons_width,
                        gtk::Orientation::Vertical,
                        gtk::Align::Start,
                    );

                    // set space left to zero
                    no_space_left = 0;

                    // append it to bottom gray box
                    self.right_box.append(&tb);
                    self.right_box.queue_draw();

                    // insert as many buttons as possible
                    self.attach_to_toolbox(self.right_box.height(), tb, &mut n_btn_left_to_draw);

                    // increment counter
                    gray_box_idx += 1;
                }
                // Attach toolbox to top gray box
                2 => {
                    // check if there is room to place the toolbox
                    if order_idx * self.buttons_width > self.top_box.height() {
                        // else increment counter
                        gray_box_idx += 1;

                        // add 1 to no space left value
                        no_space_left += 1;

                        // if not we need to go to the next iteration
                        continue;
                    }

                    // the toolbox here aligns with the left otherwise, KEEP THIS LINE
                    if order_idx == 1 {
                        let space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                        space.set_height_request(self.top_box.height() - self.buttons_width);
                        self.top_box.append(&space);
                    } else {
                        if let Some(space) = self.top_box.first_child() {
                            space.set_height_request(
                                self.top_box.height() - self.buttons_width * order_idx,
                            );
                        }
                    }

                    // create full toolbox
                    let full_tb = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    full_tb.set_height_request(self.buttons_width);
                    full_tb.set_valign(gtk::Align::End);

                    // create left spacing
                    let l_space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    l_space.set_width_request(std::cmp::max(
                        self.left_box.width() - self.buttons_width / 2 - 5,
                        0,
                    ));
                    l_space.set_height_request(self.buttons_width);
                    l_space.set_halign(gtk::Align::Start);

                    // create toolbox
                    let tb_cont = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    tb_cont.set_hexpand(true);
                    let tb = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    tb.set_halign(gtk::Align::Center);
                    tb.set_hexpand(true);
                    tb_cont.append(&tb);

                    // create right spacing
                    let r_space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                    r_space.set_width_request(std::cmp::max(
                        self.right_box.width() - self.buttons_width / 2 - 5,
                        0,
                    ));
                    r_space.set_height_request(self.buttons_width);
                    r_space.set_halign(gtk::Align::End);

                    // append it to bottom gray box
                    full_tb.append(&l_space);
                    full_tb.append(&tb_cont);
                    full_tb.append(&r_space);
                    self.top_box.append(&full_tb);
                    self.top_box.queue_draw();

                    // insert as many buttons as possible
                    let l = std::cmp::max(self.central_box.width(), self.buttons_width + 10);
                    self.attach_to_toolbox(l, tb, &mut n_btn_left_to_draw);

                    // set space left to zero
                    no_space_left = 0;

                    // increment counter
                    gray_box_idx += 1;
                }
                // Attach toolbox to left gray box
                3 => {
                    // check if there is room to place the toolbox
                    if order_idx * self.buttons_width > self.left_box.width() {
                        // else increment counter
                        gray_box_idx = 0;

                        // increment order
                        order_idx += 1;

                        // add 1 to no space left value
                        no_space_left += 1;

                        // if not we need to go to the next iteration
                        continue;
                    }

                    // create toolbox
                    let tb = Self::create_toolbox(
                        self.buttons_width,
                        gtk::Orientation::Vertical,
                        gtk::Align::End,
                    );

                    // the toolbox here aligns with the left otherwise, KEEP THIS LINE
                    if order_idx == 1 {
                        let space = gtk::Box::new(gtk::Orientation::Horizontal, 0);
                        space.set_width_request(self.left_box.width() - self.buttons_width);
                        self.left_box.append(&space);
                    } else {
                        if let Some(space) = self.left_box.first_child() {
                            space.set_width_request(
                                self.left_box.width() - self.buttons_width * order_idx,
                            );
                        }
                    }

                    // append it to bottom gray box
                    self.left_box.append(&tb);
                    self.left_box.queue_draw();

                    // insert as many buttons as possible
                    self.attach_to_toolbox(self.left_box.height(), tb, &mut n_btn_left_to_draw);

                    // if the counter is 3 it means we need to reset it for the next iteration
                    gray_box_idx = 0;

                    // set space left to zero
                    no_space_left = 0;

                    // we need also to increment the order
                    order_idx += 1;
                }
                _ => {
                    println!("The loop should not pass here");
                }
            }
        }

        // // set bool to true
        // let mut is_drawn = self.is_toolbox_drawn.clone();
        // is_drawn = true;
    }

    fn attach_to_toolbox(&self, length: i32, toolbox: gtk::Box, n_btns_left_to_draw: &mut usize) {
        let l = length;

        // get how many buttons can be placed inside bottom toolbox
        let n: usize = (&l / (&self.buttons_width)) as usize;

        // find the minimum
        let max_n_for_bottom_toolbox: usize = std::cmp::min(*n_btns_left_to_draw, n);

        // last idx
        let last_idx = self.n_buttons - *n_btns_left_to_draw;

        // fill the bottom toolbox until the maximum is reached
        for btn_idx in 0..max_n_for_bottom_toolbox {
            toolbox.append(&self.buttons_list[last_idx + btn_idx]);
        }

        *n_btns_left_to_draw -= max_n_for_bottom_toolbox;
    }

    pub fn stop_toolbox(&self) {
        // // if toolbox is not drawn yet, return
        // if self.is_toolbox_drawn == false {
        //     return;
        // }

        //  every button from their parents before attaching them again
        for btn in &self.buttons_list {
            // Get the parent of the button
            if let Some(parent) = btn.parent() {
                // Attempt to downcast the parent to gtk::Box
                if let Some(parent_box) = parent.downcast_ref::<gtk::Box>() {
                    parent_box.remove(btn);
                } else {
                    println!("Parent is not a gtk::Box");
                }
            } else {
                println!("No parent found for button");
            }
        }

        // remove toolbox from central box if any
        if let Some(child) = self.central_box.last_child() {
            if let Some(child_child) = child.last_child() {
                if child_child.type_() == gtk::Button::static_type() {
                    self.central_box.remove_overlay(&child);
                }
            }
        }

        // remove every toolbox from gray boxes
        while let Some(child) = self.bottom_box.first_child() {
            self.bottom_box.remove(&child);
        }
        while let Some(child) = self.right_box.first_child() {
            self.right_box.remove(&child);
        }
        while let Some(child) = self.top_box.first_child() {
            self.top_box.remove(&child);
        }
        while let Some(child) = self.left_box.first_child() {
            self.left_box.remove(&child);
        }
    }
}
