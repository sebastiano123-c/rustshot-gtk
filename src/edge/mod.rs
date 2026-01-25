mod imp;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct GrayEdge(ObjectSubclass<imp::GrayEdge>)
        @extends gtk::Box,
        @implements gtk::Accessible, gtk::Actionable, gtk::Widget, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for GrayEdge {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl GrayEdge {
    pub fn set_h(&self, w: i32, h_align: gtk::Align, v_align: gtk::Align) {
        self.set_orientation(gtk::Orientation::Horizontal);

        self.set_width_request(w);

        self.set_hexpand(false);
        self.set_vexpand(true);

        self.set_halign(h_align);
        self.set_valign(v_align);
    }

    pub fn set_v(&self, h: i32, w: i32, h_align: gtk::Align, v_align: gtk::Align) {
        let imp = self.imp();
        self.set_orientation(gtk::Orientation::Vertical);
        imp.orientation.set(true);

        self.set_height_request(h);
        self.set_width_request(w);

        self.set_halign(h_align);
        self.set_valign(v_align);

        self.set_hexpand(false);
        self.set_vexpand(false);
    }

    pub fn get_edge(&self) -> i32 {
        let imp = self.imp();
        imp.edge.get()
    }

    pub fn get_edge_f64(&self) -> f64 {
        let imp = self.imp();
        imp.edge_f64.get()
    }

    pub fn resolve_edge_pending(&self) {
        let imp = self.imp();
        if imp.orientation.get() {
            imp.edge.set(self.height());
        } else {
            imp.edge.set(self.width());
        }
        imp.edge_f64.set(imp.edge.get() as f64);
    }

    pub fn set_edge_pending(&self, edge: i32) {
        let imp = self.imp();
        if imp.orientation.get() {
            self.set_height_request(edge);
        } else {
            self.set_width_request(edge);
        }
    }

    pub fn set_edge(&self, edge: i32) {
        let imp = self.imp();
        imp.edge.set(edge);
        imp.edge_f64.set(edge as f64);
        self.set_edge_pending(edge);
    }

    pub fn set_edge_f64(&self, edge: f64) {
        let imp = self.imp();
        imp.edge_f64.set(edge);
        self.set_edge(edge.round() as i32);
    }

    pub fn remove_childs(&self) {
        while let Some(child) = self.first_child() {
            self.remove(&child);
        }
    }
}
