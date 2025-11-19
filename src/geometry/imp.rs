use crate::drawing_area_manager::DrawingAreaManager;
use crate::edge::GrayEdge;
use rustshot_gtk::constants::CSS_FILE_PATH;
use std::io::Write;
// use crate::handles::Handles;
use crate::drawing_area_settings::SettingsRc;
use crate::screenshot_box::ScreenshotBox;
use crate::settings_window::SettingsWindow;
use crate::toolbox::Toolbox;

use gtk::prelude::*;
use gtk::{gdk, gio, glib, subclass::prelude::*};
use rustshot_gtk::constants::CSS_CLASS_TRANSPARENT;

/// Stores the mutable geometry values used by the drag callbacks.
#[derive(Debug, Clone)]
pub struct GeometryState {
    // Edges objects (gray boxes defining the edge of the screenshot)
    pub top_box: GrayEdge,
    pub left_box: GrayEdge,
    pub bottom_box: GrayEdge,
    pub right_box: GrayEdge,

    // Central objects
    pub central_overlay: gtk::Overlay,
    pub screenshot_box: ScreenshotBox,

    pub settings: SettingsRc,
    pub settings_window: SettingsWindow,
    pub toolbox: Toolbox,

    // Layout
    pub layout: gtk::Box,
    pub drawing: DrawingAreaManager,

    // Full window size – constant for the life of the widget
    pub full_w: i32,
    pub full_h: i32,
}

#[glib::object_subclass]
impl ObjectSubclass for GeometryState {
    const NAME: &'static str = "GeometryState";
    type Type = super::GeometryState;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        let provider = gtk::CssProvider::new();
        provider.load_from_string(CSS_FILE_PATH);
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        klass.set_css_name(CSS_CLASS_TRANSPARENT);
    }

    /// New geometry
    fn new() -> Self {
        // create main layout
        let overlay = gtk::Overlay::new();
        overlay.add_css_class(CSS_CLASS_TRANSPARENT);

        // create drawing area
        let draw = DrawingAreaManager::default();
        overlay.add_overlay(&draw);

        // screenshot boxes layout
        let layout: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        layout.add_css_class(CSS_CLASS_TRANSPARENT);
        overlay.add_overlay(&layout);

        // top box
        let top_b: GrayEdge = GrayEdge::default();
        layout.append(&top_b);

        // Central box contains left and right box
        let central_b: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        central_b.add_css_class(CSS_CLASS_TRANSPARENT);
        central_b.set_vexpand(true);
        central_b.set_hexpand(true);
        layout.append(&central_b);

        // Left box
        let left_b: GrayEdge = GrayEdge::default();
        central_b.append(&left_b);

        // Screenshot box must be inside an Overlay
        let overlay = gtk::Overlay::new();
        overlay.set_hexpand(true);
        let screenshot_b: ScreenshotBox = ScreenshotBox::default();
        overlay.add_overlay(&screenshot_b);
        central_b.append(&overlay);

        // right box
        // let right_b: GrayEdge = GrayEdge::new(&central_b, w / 2, Some(gtk::Align::End));
        let right_b: GrayEdge = GrayEdge::default();
        central_b.append(&right_b);

        // bottom box
        // let bottom_b: GrayEdge = GrayEdge::new(&layout, h / 2, None);
        let bottom_b: GrayEdge = GrayEdge::default();
        layout.append(&bottom_b);

        // Create drawing area settings
        let settings_rc: SettingsRc = SettingsRc::new();

        // Create drawing area settings
        let settings_window: SettingsWindow = SettingsWindow::default();
        settings_window.create_window(&settings_rc);

        // Create Toolbox object
        let toolbox: Toolbox = Toolbox::default();

        Self {
            top_box: top_b,
            left_box: left_b,
            bottom_box: bottom_b,
            right_box: right_b,
            central_overlay: overlay,
            screenshot_box: screenshot_b,
            settings: settings_rc,
            settings_window: settings_window,
            toolbox: toolbox,
            layout,
            drawing: draw,
            full_w: w,
            full_h: h,
        }
    }
}

impl ObjectImpl for GeometryState {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        // Set window size
        let (w, h): (i32, i32) = Self::get_monitor_size();
        obj.set_default_size(w, h);

        // create main layout
        let overlay = gtk::Overlay::new();
        overlay.add_css_class(CSS_CLASS_TRANSPARENT);
        obj.set_child(Some(&overlay));

        // create drawing area
        let draw = DrawingAreaManager::default();
        overlay.add_overlay(&draw);

        // screenshot boxes layout
        let layout: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        layout.add_css_class(CSS_CLASS_TRANSPARENT);
        overlay.add_overlay(&layout);

        // top box
        // let top_b: GrayEdge = GrayEdge::new(&layout, h / 2, None);
        let top_b: GrayEdge = GrayEdge::default();
        top_b.set(h / 2, None);
        layout.append(&top_b);

        // Central box contains left and right box
        let central_b: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        central_b.add_css_class(CSS_CLASS_TRANSPARENT);
        central_b.set_vexpand(true);
        central_b.set_hexpand(true);
        layout.append(&central_b);

        // Left box
        // let left_b: GrayEdge = GrayEdge::new(&central_b, w / 2, Some(gtk::Align::Start));
        let left_b: GrayEdge = GrayEdge::default();
        left_b.set(w / 2, Some(gtk::Align::Start));
        central_b.append(&left_b);

        // Screenshot box must be inside an Overlay
        let overlay = gtk::Overlay::new();
        overlay.set_hexpand(true);
        let screenshot_b: ScreenshotBox = ScreenshotBox::default();
        overlay.add_overlay(&screenshot_b);
        central_b.append(&overlay);

        // right box
        // let right_b: GrayEdge = GrayEdge::new(&central_b, w / 2, Some(gtk::Align::End));
        let right_b: GrayEdge = GrayEdge::default();
        right_b.set(w / 2, Some(gtk::Align::End));
        central_b.append(&right_b);

        // bottom box
        // let bottom_b: GrayEdge = GrayEdge::new(&layout, h / 2, None);
        let bottom_b: GrayEdge = GrayEdge::default();
        bottom_b.set(h / 2, None);
        layout.append(&bottom_b);
    }
}

impl WidgetImpl for GeometryState {}
impl WindowImpl for GeometryState {}
