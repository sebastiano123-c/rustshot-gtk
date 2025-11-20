pub const APP_NAME: &str = "com.rust.rustshot-gtk";

// CSS
pub const CSS_FILE_PATH: &str = include_str!("../styles/mocha.css");
pub const CSS_CLASS_TRANSPARENT: &str = "transparent";
pub const CSS_CLASS_SOLID: &str = "fas";
pub const CSS_CLASS_GRAY_BOX: &str = "gray-box";
pub const CSS_CLASS_PRESSED: &str = "pressed";
pub const CSS_CLASS_PRESSED_PERSISTENT: &str = "pressed-persistent";
pub const CSS_CLASS_HANDLES: &str = "corner-handle";
pub const CSS_CLASS_TOOLBOX_BTN: &str = "toolbox-btn";
// pub const CSS_CLASS_TOOLBOX_BAR: &str = "test";
pub const CSS_CLASS_TOOLBOX_BAR: &str = "transparent";

pub const HANDLE_SIZE_PX: i32 = 10;
pub const TOOLBOX_BTN_SIZE: i32 = 50;

// Drawing area settings toml
pub const DRAWING_AREA_SETTINGS_TOML: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/drawing_area_settings/drawing_area_settings.toml"
);
