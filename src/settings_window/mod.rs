mod imp;
use crate::drawing_area_settings::{HandleSettings, SettingsRc};
use gtk::{gio, glib, prelude::*};
use std::rc::Rc;

glib::wrapper! {
    pub struct SettingsWindow(ObjectSubclass<imp::SettingsWindow>)
   @extends gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Default for SettingsWindow {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl SettingsWindow {
    pub fn create_window(&self, settings: &SettingsRc) {
        let vlayout = gtk::Box::new(gtk::Orientation::Vertical, 0);
        self.set_child(Some(&vlayout));
        // line size
        let hbox = self.create_settings_entry(&settings, "Line size", "init_line_size");
        vlayout.append(&hbox);
        // Arrow size
        let hbox = self.create_settings_entry(&settings, "Arrow size", "init_arrow_size");
        vlayout.append(&hbox);
        // Arrow width
        let hbox = self.create_settings_entry(&settings, "Arrow width", "init_arrow_width");
        vlayout.append(&hbox);
        // free hand size
        let hbox = self.create_settings_entry(&settings, "Freehand size", "init_freehand_size");
        vlayout.append(&hbox);
        // box border size
        let hbox = self.create_settings_entry(&settings, "Box border", "init_box_border_size");
        vlayout.append(&hbox);
        // arc border size
        let hbox = self.create_settings_entry(&settings, "Circle border", "init_arc_border_size");
        vlayout.append(&hbox);
    }

    fn create_settings_entry(
        &self,
        settings: &SettingsRc,
        entry_label: &str,
        settings_label: &str,
    ) -> gtk::Box {
        // create box containing label, slide, and entry
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        hbox.set_width_request(400);

        // create label
        let lbl = gtk::Label::new(Some(entry_label));
        lbl.set_width_request(100);

        // settings
        let label = String::from(settings_label);
        let label_2 = String::from(settings_label);

        // create slide widget
        let val = gtk::Scale::new(
            gtk::Orientation::Horizontal,
            Some(&gtk::Adjustment::new(
                settings.size.get_value(&label),
                1.0,
                20.0,
                1.0,
                10.0,
                2.0,
            )),
        );
        val.set_hexpand(true);
        val.set_width_request(250);

        // create entry
        let ent = gtk::Entry::new();
        ent.set_width_request(20);
        ent.set_text(&*settings.size.get_value(&label).to_string());

        let size_rc = Rc::new(settings.clone());

        // function when slide change
        val.connect_value_changed(glib::clone!(
            #[weak]
            ent,
            #[weak]
            size_rc,
            move |value| {
                // update entry value
                let print_res = format!("{:.1}", value.value());
                ent.set_text(&print_res);

                // update global value
                size_rc.size.set_value(&label, value.value());
            }
        ));

        // function when entry change
        ent.connect_activate(glib::clone!(
            #[weak]
            val,
            // #[weak]
            // settings,
            move |value| {
                let text = &value.text();
                match text.parse::<f64>() {
                    Ok(num) => {
                        // update global value
                        size_rc.size.set_value(&label_2, num);

                        // update entry value
                        val.set_value(num);
                    }
                    Err(_) => {
                        println!("Invalid input, not a valid f64.");
                    }
                }
            }
        ));

        // append to entry box
        hbox.append(&lbl);
        hbox.append(&val);
        hbox.append(&ent);

        // return
        hbox
    }
}
