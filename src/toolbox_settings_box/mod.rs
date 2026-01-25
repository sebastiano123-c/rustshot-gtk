pub mod arc;
pub mod arrow;
pub mod freehand;
pub mod line;
pub mod numbered_circle;
pub mod rect;

use gtk::glib;
use gtk::prelude::*;
use gtk::{Box as GtkBox, Expander, Revealer};

// --------------------------------------------------------------
// Helper that creates a label that expands into a *horizontal*
// row of widgets.
// --------------------------------------------------------------
pub fn add_expandable_row<T: IsA<gtk::Box>>(
    parent: &T,
    label: &str,
    tooltip_text: &str,
    css_class: &str,
    child: GtkBox,
    expand_child_start: bool,
) {
    // 1️⃣  Create a Revealer (you could also use Expander here)
    let revealer = Revealer::new();
    revealer.set_transition_type(gtk::RevealerTransitionType::SlideLeft);
    revealer.set_child(Some(&child));
    revealer.set_reveal_child(expand_child_start);
    revealer.add_css_class("toolbox-settings-revealer");

    // 2️⃣  Create an Expander that will act as the clickable label
    let expander = Expander::new(Some(label));
    expander.add_css_class("toolbox-settings-expander");
    expander.set_tooltip_text(Some(tooltip_text));
    expander.add_css_class(css_class);
    expander.set_vexpand(true);
    expander.set_valign(gtk::Align::Fill);
    expander.set_expanded(!expand_child_start);

    // 3️⃣  When the user clicks the label, toggle the Revealer
    expander.connect_activate(glib::clone!(
        #[strong]
        revealer,
        move |_| {
            let visible = revealer.is_child_revealed();
            revealer.set_reveal_child(!visible);
        }
    ));

    // 4️⃣  Pack both widgets into the parent box
    parent.append(&expander);
    parent.append(&revealer);
}

// pub fn get_factory(line_tools: Vec<&str>) -> std::io::Result<gtk::SignalListItemFactory> {
//     let store = gtk::gio::ListStore::new::<gtk::StringObject>();
//     for (icon, txt) in line_tools.iter() {
//         let combined = format!("{}|{}", icon, txt);
//         store.append(&gtk::StringObject::new(&combined));
//     }
//
//     // -------------------------------------------------------------
//     // 3️⃣ Factory that renders icon + text side‑by‑side
//     // -------------------------------------------------------------
//     let factory = gtk::SignalListItemFactory::new();
//
//     factory.connect_setup(|_, list_item| {
//         let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
//         let icon_lbl = gtk::Label::new(None);
//         icon_lbl.add_css_class("fa-icon");
//         let text_lbl = gtk::Label::new(None);
//         hbox.append(&icon_lbl);
//         hbox.append(&text_lbl);
//         list_item.set_child(Some(&hbox));
//     });
//
//     factory.connect_bind(|_, list_item| {
//         let obj = list_item.item().unwrap();
//         let str_obj = obj
//             .downcast_ref::<gtk::StringObject>()
//             .expect("Item should be a StringObject");
//         let (icon, txt) = {
//             let parts: Vec<&str> = str_obj.string().splitn(2, '|').collect();
//             (parts[0], parts[1])
//         };
//         let hbox = list_item
//             .child()
//             .unwrap()
//             .downcast::<gtk::Box>()
//             .expect("Child should be a Box");
//         let icon_lbl = hbox
//             .first_child()
//             .unwrap()
//             .downcast::<gtk::Label>()
//             .unwrap();
//         let text_lbl = hbox.last_child().unwrap().downcast::<gtk::Label>().unwrap();
//         icon_lbl.set_label(icon);
//         text_lbl.set_label(txt);
//     });
//
//     Ok(factory)
// }
