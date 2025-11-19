use rustshot_gtk::constants::DRAWING_AREA_SETTINGS_TOML;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::fs;
use std::rc::Rc;

// Top level struct to hold the TOML data.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Settings {
    pub size: Size,
    pub color: Color,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
}

impl Color {
    pub fn get_value(&self, color: &str) -> f64 {
        match color {
            "red" => self.red,
            "green" => self.green,
            "blue" => self.blue,
            "alpha" => self.alpha,
            _ => {
                println!("Color::set_color '{}' not found", color);
                0.0
            }
        }
    }
}

// Settings struct holds to data from the `[config]` section.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Size {
    init_arrow_size: f64,
    init_arrow_width: f64,
    init_freehand_size: f64,
    init_line_size: f64,
    init_box_border_size: f64,
    init_arc_border_size: f64,
    init_numbered_circles_radius: f64,
    init_numbered_circles_font_size: f64,
    init_numbered_circles_font_color_r: f64,
    init_numbered_circles_font_color_g: f64,
    init_numbered_circles_font_color_b: f64,
}

impl Size {
    pub fn get_value(&self, setting: &str) -> f64 {
        match setting {
            "init_arrow_size" => self.init_arrow_size,
            "init_arrow_width" => self.init_arrow_width,
            "init_freehand_size" => self.init_freehand_size,
            "init_line_size" => self.init_line_size,
            "init_box_border_size" => self.init_box_border_size,
            "init_arc_border_size" => self.init_arc_border_size,
            "init_numbered_circles_radius" => self.init_numbered_circles_radius,
            "init_numbered_circles_font_size" => self.init_numbered_circles_font_size,
            "init_numbered_circles_font_color_r" => self.init_numbered_circles_font_color_r,
            "init_numbered_circles_font_color_g" => self.init_numbered_circles_font_color_g,
            "init_numbered_circles_font_color_b" => self.init_numbered_circles_font_color_b,
            _ => {
                println!("Color::set_color '{}' not found", setting);
                0.0
            }
        }
    }
}

// Top level struct to hold the TOML data.
#[derive(Debug, Clone)]
pub struct SettingsRc {
    pub size: SizeRc,
    pub color: ColorRc,
}

#[derive(Debug, Clone)]
pub struct ColorRc {
    red: Rc<Cell<f64>>,
    green: Rc<Cell<f64>>,
    blue: Rc<Cell<f64>>,
    alpha: Rc<Cell<f64>>,
}

// Settings struct holds to data from the `[config]` section.
#[derive(Debug, Clone)]
pub struct SizeRc {
    init_arrow_size: Rc<Cell<f64>>,
    init_arrow_width: Rc<Cell<f64>>,
    init_freehand_size: Rc<Cell<f64>>,
    init_line_size: Rc<Cell<f64>>,
    init_box_border_size: Rc<Cell<f64>>,
    init_arc_border_size: Rc<Cell<f64>>,
    init_numbered_circles_radius: Rc<Cell<f64>>,
    init_numbered_circles_font_size: Rc<Cell<f64>>,
    init_numbered_circles_font_color_r: Rc<Cell<f64>>,
    init_numbered_circles_font_color_g: Rc<Cell<f64>>,
    init_numbered_circles_font_color_b: Rc<Cell<f64>>,
}

pub trait HandleSettings {
    fn get_value(&self, value: &str) -> f64;
    fn set_value(&self, setting: &str, value: f64);
}
impl HandleSettings for ColorRc {
    fn get_value(&self, color: &str) -> f64 {
        match color {
            "red" => self.red.get(),
            "green" => self.green.get(),
            "blue" => self.blue.get(),
            "alpha" => self.alpha.get(),
            _ => {
                println!("Color::set_color '{}' not found", color);
                0.0
            }
        }
    }
    fn set_value(&self, setting: &str, value: f64) {
        match setting {
            "red" => self.red.set(value),
            "green" => self.green.set(value),
            "blue" => self.blue.set(value),
            "alpha" => self.alpha.set(value),
            _ => {
                println!("Color::set_color '{}' not found", setting)
            }
        }
    }
}
impl HandleSettings for SizeRc {
    fn get_value(&self, setting: &str) -> f64 {
        match setting {
            "init_arrow_size" => self.init_arrow_size.get(),
            "init_arrow_width" => self.init_arrow_width.get(),
            "init_freehand_size" => self.init_freehand_size.get(),
            "init_line_size" => self.init_line_size.get(),
            "init_box_border_size" => self.init_box_border_size.get(),
            "init_arc_border_size" => self.init_arc_border_size.get(),
            "init_numbered_circles_radius" => self.init_numbered_circles_radius.get(),
            "init_numbered_circles_font_size" => self.init_numbered_circles_font_size.get(),
            "init_numbered_circles_font_color_r" => self.init_numbered_circles_font_color_r.get(),
            "init_numbered_circles_font_color_g" => self.init_numbered_circles_font_color_g.get(),
            "init_numbered_circles_font_color_b" => self.init_numbered_circles_font_color_b.get(),
            _ => {
                println!("Color::set_color '{}' not found", setting);
                0.0
            }
        }
    }
    fn set_value(&self, setting: &str, value: f64) {
        match setting {
            "init_arrow_size" => self.init_arrow_size.set(value),
            "init_arrow_width" => self.init_arrow_width.set(value),
            "init_freehand_size" => self.init_freehand_size.set(value),
            "init_line_size" => self.init_line_size.set(value),
            "init_box_border_size" => self.init_box_border_size.set(value),
            "init_arc_border_size" => self.init_arc_border_size.set(value),
            "init_numbered_circles_radius" => self.init_numbered_circles_radius.set(value),
            "init_numbered_circles_font_size" => self.init_numbered_circles_font_size.set(value),
            "init_numbered_circles_font_color_r" => {
                self.init_numbered_circles_font_color_r.set(value)
            }
            "init_numbered_circles_font_color_g" => {
                self.init_numbered_circles_font_color_g.set(value)
            }
            "init_numbered_circles_font_color_b" => {
                self.init_numbered_circles_font_color_b.set(value)
            }
            _ => {
                println!("Color::set_color '{}' not found", setting);
            }
        }
    }
}

impl SettingsRc {
    pub fn new() -> Self {
        let raw = Settings::new();

        let color_rc = ColorRc {
            red: Rc::new(Cell::new(raw.color.red)),
            green: Rc::new(Cell::new(raw.color.green)),
            blue: Rc::new(Cell::new(raw.color.blue)),
            alpha: Rc::new(Cell::new(raw.color.alpha)),
        };

        let size_rc = SizeRc {
            init_arrow_size: Rc::new(Cell::new(raw.size.init_arrow_size)),
            init_arrow_width: Rc::new(Cell::new(raw.size.init_arrow_width)),
            init_freehand_size: Rc::new(Cell::new(raw.size.init_freehand_size)),
            init_line_size: Rc::new(Cell::new(raw.size.init_line_size)),
            init_box_border_size: Rc::new(Cell::new(raw.size.init_box_border_size)),
            init_arc_border_size: Rc::new(Cell::new(raw.size.init_arc_border_size)),
            init_numbered_circles_radius: Rc::new(Cell::new(raw.size.init_numbered_circles_radius)),
            init_numbered_circles_font_size: Rc::new(Cell::new(
                raw.size.init_numbered_circles_font_size,
            )),
            init_numbered_circles_font_color_r: Rc::new(Cell::new(
                raw.size.init_numbered_circles_font_color_r,
            )),
            init_numbered_circles_font_color_g: Rc::new(Cell::new(
                raw.size.init_numbered_circles_font_color_g,
            )),
            init_numbered_circles_font_color_b: Rc::new(Cell::new(
                raw.size.init_numbered_circles_font_color_b,
            )),
        };

        SettingsRc {
            size: size_rc,
            color: color_rc,
        }
    }

    pub fn hard_copy(&self) -> Settings {
        // Create hard copy of the size
        let sz: Size = Size {
            init_arrow_size: self.size.init_arrow_size.get(),
            init_arrow_width: self.size.init_arrow_width.get(),
            init_freehand_size: self.size.init_freehand_size.get(),
            init_line_size: self.size.init_line_size.get(),
            init_box_border_size: self.size.init_box_border_size.get(),
            init_arc_border_size: self.size.init_arc_border_size.get(),
            init_numbered_circles_radius: self.size.init_numbered_circles_radius.get(),
            init_numbered_circles_font_size: self.size.init_numbered_circles_font_size.get(),
            init_numbered_circles_font_color_r: self.size.init_numbered_circles_font_color_r.get(),
            init_numbered_circles_font_color_g: self.size.init_numbered_circles_font_color_g.get(),
            init_numbered_circles_font_color_b: self.size.init_numbered_circles_font_color_b.get(),
        };

        let cl: Color = Color {
            red: self.color.red.get(),
            green: self.color.green.get(),
            blue: self.color.blue.get(),
            alpha: self.color.alpha.get(),
        };

        Settings {
            size: sz,
            color: cl,
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        let filename = DRAWING_AREA_SETTINGS_TOML;

        let toml_str = fs::read_to_string(filename).expect("Failed to read Cargo.toml file");

        let data: Settings = toml::from_str(&toml_str).expect("Failed to deserialize Cargo.toml");

        Self {
            size: data.size,
            color: data.color,
        }
    }

    pub fn save_settings(&self) -> std::io::Result<()> {
        // 4️⃣ Serialize back to a TOML string
        let updated_toml = toml::to_string_pretty(self).map_err(|e| {
            std::io::Error::other(format!("Failed to serialize back to TOML: {}", e))
        })?;

        // 5️⃣ Write the updated content back to disk
        fs::write(DRAWING_AREA_SETTINGS_TOML, updated_toml).map_err(|e| {
            std::io::Error::new(
                e.kind(),
                format!("Failed to write {}: {}", DRAWING_AREA_SETTINGS_TOML, e),
            )
        })?;

        Ok(())
    }
}
