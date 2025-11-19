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
    pub fn set(&self, edge: i32, align: Option<gtk::Align>) {
        let imp = self.imp();

        if let Some(a) = align {
            self.set_halign(a);
            self.set_orientation(gtk::Orientation::Horizontal);
        } else {
            self.set_height_request(edge);
            self.set_orientation(gtk::Orientation::Vertical);
            imp.orientation.set(true);
        }
    }

    pub fn get_edge(&self) -> i32 {
        let imp = self.imp();
        imp.edge.get()
        // self.widget.width()
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
