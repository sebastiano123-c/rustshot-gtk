use gtk::{glib, prelude::*, subclass::prelude::*};
use rustshot_gtk::constants::{CSS_CLASS_TOOLBOX_BTN, TOOLBOX_BTN_SIZE};

#[derive(Debug, Default)]
pub struct StandardButton {}

#[glib::object_subclass]
impl ObjectSubclass for StandardButton {
    const NAME: &'static str = "StandardButton";
    type Type = super::StandardButton;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        // The layout manager determines how child widgets are laid out.
        klass.set_layout_manager_type::<gtk::BinLayout>();

        // Make it look like a GTK button.
        klass.set_css_name(CSS_CLASS_TOOLBOX_BTN);

        // Make it appear as a button to accessibility tools.
        klass.set_accessible_role(gtk::AccessibleRole::Button);
    }
}

impl ObjectImpl for StandardButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_width_request(TOOLBOX_BTN_SIZE);
        obj.set_height_request(TOOLBOX_BTN_SIZE);
    }
}
impl WidgetImpl for StandardButton {}
