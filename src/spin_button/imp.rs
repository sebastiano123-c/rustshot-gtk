use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Align, Orientation};
use std::cell::Cell;

#[derive(Default)]
pub struct IntegerInput {
    pub(super) entry: gtk::Entry,
    pub(super) value: Cell<i32>,
    pub(super) min: Cell<i32>,
    pub(super) max: Cell<i32>,
    overlay: gtk::Overlay,
}

#[glib::object_subclass]
impl ObjectSubclass for IntegerInput {
    const NAME: &'static str = "IntegerInput";
    type Type = super::IntegerInput;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
        klass.set_css_name("integer-input");
    }
}

impl ObjectImpl for IntegerInput {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        // Load a tiny CSS snippet
        let css = r#"
    integer-input {
        min-width: 8px;
        padding: 0px;
        margin: 0px;
        border-radius: 0px;
        border-width: 0px;
    }
    .spin-button {
        min-width: 4px;
        min-height: 2px;
        padding: 0px;
        margin: 0px;
        border-radius: 0px;
        background: none;
        background-color: transparent;
        border: none;
        border-width: 0;
        outline-width: 0;
        box-shadow: none;
    }

.spin-button:hover {
    background-color: rgba(0, 0, 0, 0.08);
    transform: scale(1.05);
}
"#;
        let provider = gtk::CssProvider::new();
        provider.load_from_string(css);
        gtk::style_context_add_provider_for_display(
            &obj.display(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // ---------- Entry ----------
        self.entry.set_hexpand(false);
        self.entry.set_max_width_chars(3);
        self.entry.set_vexpand(false);
        self.entry.add_css_class("integer-input");

        // ---------- Buttons ----------
        let button_box = gtk::Box::new(Orientation::Vertical, 0);
        button_box.set_valign(Align::Center);
        button_box.set_halign(Align::End);
        button_box.add_css_class("spin-buttons");
        button_box.set_spacing(0);

        let increment_btn = gtk::Button::with_label("+");
        increment_btn.add_css_class("spin-button");
        increment_btn.add_css_class("spin-up");
        increment_btn.set_width_request(4);
        increment_btn.set_height_request(2);

        let decrement_btn = gtk::Button::with_label("-");
        decrement_btn.add_css_class("spin-button");
        decrement_btn.add_css_class("spin-down");
        decrement_btn.set_width_request(4);
        decrement_btn.set_height_request(2);

        button_box.append(&increment_btn);
        button_box.append(&decrement_btn);

        // ---------- Overlay ----------
        self.overlay.set_child(Some(&self.entry));
        self.overlay.add_overlay(&button_box);
        self.overlay.set_halign(Align::Start);
        self.overlay.set_valign(Align::Center);
        self.overlay.set_margin_start(0);
        self.overlay.set_margin_end(0);
        self.overlay.set_margin_top(0);
        self.overlay.set_margin_bottom(0);
        self.overlay.set_parent(&*self.obj());

        // Connect increment button
        increment_btn.connect_clicked(glib::clone!(
            #[weak]
            obj,
            move |_| {
                let imp = obj.imp();
                let current = imp.value.get();
                let max = imp.max.get();
                if current < max {
                    obj.set_value(current + 1);
                }
            }
        ));

        // Connect decrement button
        decrement_btn.connect_clicked(glib::clone!(
            #[weak]
            obj,
            move |_| {
                let imp = obj.imp();
                let current = imp.value.get();
                let min = imp.min.get();
                if current > min {
                    obj.set_value(current - 1);
                }
            }
        ));

        // Handle manual text entry
        self.entry.connect_changed(glib::clone!(
            #[weak]
            obj,
            move |entry| {
                let imp = obj.imp();
                if let Ok(new_value) = entry.text().parse::<i32>() {
                    let min = imp.min.get();
                    let max = imp.max.get();
                    if new_value >= min && new_value <= max {
                        imp.value.set(new_value);
                        obj.notify("value");
                    }
                }
            }
        ));
    }

    fn dispose(&self) {
        self.overlay.unparent();
    }

    fn properties() -> &'static [glib::ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecInt::builder("value")
                    .default_value(0)
                    .build(),
                glib::ParamSpecInt::builder("min")
                    .default_value(i32::MIN)
                    .build(),
                glib::ParamSpecInt::builder("max")
                    .default_value(i32::MAX)
                    .build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "value" => self.value.get().to_value(),
            "min" => self.min.get().to_value(),
            "max" => self.max.get().to_value(),
            _ => unimplemented!(),
        }
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "value" => {
                let val = value.get().unwrap();
                self.obj().set_value(val);
            }
            "min" => {
                let val = value.get().unwrap();
                self.min.set(val);
            }
            "max" => {
                let val = value.get().unwrap();
                self.max.set(val);
            }
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for IntegerInput {}
