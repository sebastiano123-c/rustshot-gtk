mod imp;

use gtk::{glib, prelude::WidgetExt, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::CSS_CLASS_HANDLES;

use crate::geometry::GeometryState;

glib::wrapper! {
    pub struct Handle(ObjectSubclass<imp::Handle>)
        @extends gtk::Box,
        @implements gtk::Accessible, gtk::Actionable, gtk::Widget, gtk::Orientable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for Handle {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl Handle {
    pub fn set_position(&self, col: u8, row: u8) {
        let imp = self.imp();
        imp.col.set(col);
        imp.row.set(row);

        // 1️⃣  Pick the correct base widget (central vs corner)
        if col != 1 || row != 1 {
            self.add_css_class(CSS_CLASS_HANDLES);
        }
    }

    pub fn attach_gesture(&self, geometry: &GeometryState) {
        let obj = self.imp();

        // Install a drag gesture controller
        let gesture = gtk::GestureDrag::new();
        self.add_controller(gesture.clone());

        // 5️⃣  Drag‑begin: hide the toolbox while the user resizes
        gesture.connect_drag_begin(glib::clone!(
            #[strong]
            geometry,
            move |_, _, _| {
                geometry.toolbox.stop_toolbox(&geometry);
            }
        ));

        // 6️⃣  Drag‑update: compute new geometry and apply it
        gesture.connect_drag_update(glib::clone!(
            #[strong]
            geometry,
            #[weak]
            obj,
            move |_, dx, dy| {
                // Move handles on if the screenshot box is sensitive
                if geometry.screenshot_box.get_screenshot_box_sensitivity() {
                    // Read the current values from the Cells
                    let mut l = geometry.left_box.get_edge_f64();
                    let mut r = geometry.right_box.get_edge_f64();
                    let mut t = geometry.top_box.get_edge_f64();
                    let mut b = geometry.bottom_box.get_edge_f64();

                    // Adjust according to which handle is being dragged
                    match obj.col.get() {
                        0 => l += dx, // left side
                        1 => {
                            // centre column – only move horizontally when the central
                            // handle is marked as “sensitive”.
                            if obj.row.get() == 1 {
                                l += dx;
                                r -= dx;
                                t += dy;
                                b -= dy;

                                if l < 0.0 {
                                    l = 0.0;
                                    r += dx + geometry.left_box.get_edge_f64();
                                }
                                if r < 0.0 {
                                    r = 0.0;
                                    l -= dx - geometry.right_box.get_edge_f64();
                                }

                                if t < 0.0 {
                                    t = 0.0;
                                    b += dy + geometry.top_box.get_edge_f64();
                                }
                                if b < 0.0 {
                                    b = 0.0;
                                    t -= dy - geometry.bottom_box.get_edge_f64();
                                }
                            }
                        }
                        2 => r -= dx, // right side
                        _ => {}
                    }

                    match obj.row.get() {
                        0 => {
                            t += dy; // top side
                        }
                        2 => {
                            b -= dy; // bottom side
                        }
                        _ => {}
                    }

                    if l < 0.0 {
                        l = 0.0;
                    }
                    if r < 0.0 {
                        r = 0.0;
                    }

                    if t < 0.0 {
                        t = 0.0;
                    }
                    if b < 0.0 {
                        b = 0.0;
                    }

                    // Apply the new geometry to the UI
                    geometry.set_new_geometry_f64(t, l, b, r);
                }
            }
        ));

        // 7️⃣  Drag‑end: redraw the toolbox now that resizing is one
        gesture.connect_drag_end(glib::clone!(
            #[strong]
            geometry,
            move |_, _, _| {
                println!("Handle::drag_end");
                geometry
                    .toolbox
                    .draw_toolbox(&geometry)
                    .expect("handle connect_drag_end error");
            }
        ));
    }
}
