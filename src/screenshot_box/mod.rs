mod imp;
use crate::geometry::GeometryState;
use crate::handle::Handle;
use rustshot_gtk::constants::HANDLE_SIZE_PX;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct ScreenshotBox(ObjectSubclass<imp::ScreenshotBox>)
        @extends gtk::Box,
        @implements gtk::Accessible, gtk::Actionable, gtk::Widget, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ScreenshotBox {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ScreenshotBox {
    pub fn set_screenshot_box_sensitivity(&self, sensitive: bool) {
        let imp = self.imp();

        for handle in &*imp.handles.borrow_mut() {
            handle.set_sensitive(sensitive);
        }
    }

    pub fn get_screenshot_box_sensitivity(&self) -> bool {
        let imp = self.imp();
        imp.central_handle_sensitive.get()
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
            sp.set_height_request(HANDLE_SIZE_PX);
        }

        // Create handles
        let l_handle: Handle = Handle::default();
        l_handle.set_position(0, position);
        l_handle.set_width_request(HANDLE_SIZE_PX);
        let c_handle: Handle = Handle::default();
        c_handle.set_position(1, position);
        c_handle.set_hexpand(true);
        let r_handle: Handle = Handle::default();
        r_handle.set_position(2, position);
        r_handle.set_width_request(HANDLE_SIZE_PX);
        r_handle.set_halign(gtk::Align::End);

        // Save handles
        let imp = self.imp();
        imp.handles.borrow_mut().push(l_handle.clone());
        imp.handles.borrow_mut().push(c_handle.clone());
        imp.handles.borrow_mut().push(r_handle.clone());

        // Attach handles
        sp.append(&l_handle);
        sp.append(&c_handle);
        sp.append(&r_handle);

        sp
    }

    pub fn attach_handles_gesture(&self, geom: &GeometryState) {
        let imp = self.imp();
        for handle in &*imp.handles.borrow_mut() {
            handle.attach_gesture(geom);
        }
    }
}
