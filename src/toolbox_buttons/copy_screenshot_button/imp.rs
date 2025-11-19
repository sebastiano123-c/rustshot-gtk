use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};

#[derive(Debug, Default)]
pub struct CopyScreenshotButton {}

#[glib::object_subclass]
impl ObjectSubclass for CopyScreenshotButton {
    const NAME: &'static str = "CopyScreenshotButton";
    type Type = super::CopyScreenshotButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for CopyScreenshotButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_label("\u{f328}");
        obj.set_tooltip_text(Some(r#"Copy to clipboard"#));
        obj.add_css_class(CSS_CLASS_TOOLBOX_BTN);
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);
    }
}

impl WidgetImpl for CopyScreenshotButton {}
impl ButtonImpl for CopyScreenshotButton {}
