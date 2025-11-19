use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};

#[derive(Debug, Default)]
pub struct SaveScreenshotButton {}

#[glib::object_subclass]
impl ObjectSubclass for SaveScreenshotButton {
    const NAME: &'static str = "SaveScreenshotButton";
    type Type = super::SaveScreenshotButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for SaveScreenshotButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_label("\u{f0c7}");
        obj.set_tooltip_text(Some(r#"Save image"#));
        obj.add_css_class(CSS_CLASS_TOOLBOX_BTN);
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);
    }
}

impl WidgetImpl for SaveScreenshotButton {}
impl ButtonImpl for SaveScreenshotButton {}
