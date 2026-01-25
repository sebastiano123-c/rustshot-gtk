use rustshot_gtk::constants::DRAWING_AREA_SETTINGS_TOML;
use serde::{Deserialize, Serialize};
use std::cell::{Cell, RefCell};
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::rc::Rc;

/// Enum that can hold any of the possible setting values.
#[derive(Clone, PartialEq)]
pub enum SettingValue {
    Bool(bool),
    I32(i32),
    F64(f64),
    String(String),
}

impl SettingValue {
    pub fn get_bool(&self) -> Result<bool> {
        match self {
            SettingValue::Bool(v) => Ok(*v),
            SettingValue::F64(v) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("SettingValue {} is not bool", v),
            )),
            SettingValue::I32(v) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("SettingValue {} is not bool", v),
            )),
            SettingValue::String(v) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("SettingValue {} is not bool", v),
            )),
        }
    }
    pub fn get_i32(&self) -> Result<i32> {
        match self {
            SettingValue::I32(v) => Ok(*v),
            SettingValue::Bool(v) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("SettingValue {} is not i32", v),
            )),
            SettingValue::F64(v) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("SettingValue {} is not i32", v),
            )),
            SettingValue::String(v) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("SettingValue {} is not i32", v),
            )),
        }
    }
    pub fn get_f64(&self) -> Result<f64> {
        match self {
            SettingValue::F64(v) => Ok(*v),
            SettingValue::Bool(v) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("SettingValue {} is not f64", v),
            )),
            SettingValue::I32(v) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("SettingValue {} is not f64", v),
            )),
            SettingValue::String(v) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("SettingValue {} is not f64", v),
            )),
        }
    }
    pub fn get_string(&self) -> Result<String> {
        match self {
            SettingValue::String(v) => Ok((*(v.clone())).to_string()),
            SettingValue::Bool(v) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("SettingValue {} is not string", v),
            )),
            SettingValue::I32(v) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("SettingValue {} is not string", v),
            )),
            SettingValue::F64(v) => Err(Error::new(
                ErrorKind::InvalidData,
                format!("SettingValue {} is not string", v),
            )),
        }
    }
}

// Top level struct to hold the TOML data.
#[derive(Default, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub arc: ArcSettings,
    pub rect: RectSettings,
    pub line: LineSettings,
    pub freehand: FreehandSettings,
    pub numbered_circle: NumberedCircleSettings,
    pub arrow: ArrowSettings,
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct ArcSettings {
    fill: bool,
    fill_r: f64,
    fill_g: f64,
    fill_b: f64,
    fill_a: f64,
    border: bool,
    border_size: f64,
    border_r: f64,
    border_g: f64,
    border_b: f64,
    border_a: f64,
}

impl ArcSettings {
    /// Get a setting by name, returning a `SettingValue`.
    pub fn get_value(&self, setting_name: &str) -> SettingValue {
        match setting_name {
            "fill" => SettingValue::Bool(self.fill),
            "fill_r" => SettingValue::F64(self.fill_r),
            "fill_g" => SettingValue::F64(self.fill_g),
            "fill_b" => SettingValue::F64(self.fill_b),
            "fill_a" => SettingValue::F64(self.fill_a),
            "border" => SettingValue::Bool(self.border),
            "border_size" => SettingValue::F64(self.border_size),
            "border_r" => SettingValue::F64(self.border_r),
            "border_g" => SettingValue::F64(self.border_g),
            "border_b" => SettingValue::F64(self.border_b),
            "border_a" => SettingValue::F64(self.border_a),
            other => {
                eprintln!("ArcSettings::get_value: unknown key '{}'", other);
                // Default – you can change this to whatever makes sense.
                SettingValue::Bool(false)
            }
        }
    }
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct RectSettings {
    fill: bool,
    fill_r: f64,
    fill_g: f64,
    fill_b: f64,
    fill_a: f64,
    border: bool,
    border_size: f64,
    border_r: f64,
    border_g: f64,
    border_b: f64,
    border_a: f64,
}

impl RectSettings {
    /// Get a setting by name, returning a `SettingValue`.
    pub fn get_value(&self, setting_name: &str) -> SettingValue {
        match setting_name {
            "fill" => SettingValue::Bool(self.fill),
            "fill_r" => SettingValue::F64(self.fill_r),
            "fill_g" => SettingValue::F64(self.fill_g),
            "fill_b" => SettingValue::F64(self.fill_b),
            "fill_a" => SettingValue::F64(self.fill_a),
            "border" => SettingValue::Bool(self.border),
            "border_size" => SettingValue::F64(self.border_size),
            "border_r" => SettingValue::F64(self.border_r),
            "border_g" => SettingValue::F64(self.border_g),
            "border_b" => SettingValue::F64(self.border_b),
            "border_a" => SettingValue::F64(self.border_a),
            other => {
                eprintln!("RectSettings::get_value: unknown key '{}'", other);
                // Default – you can change this to whatever makes sense.
                SettingValue::Bool(false)
            }
        }
    }
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct LineSettings {
    color_r: f64,
    color_g: f64,
    color_b: f64,
    color_a: f64,
    size: f64,
    line_cap: String,
    line_join: String,
}

impl LineSettings {
    /// Get a setting by name, returning a `SettingValue`.
    pub fn get_value(&self, setting_name: &str) -> SettingValue {
        match setting_name {
            "color_r" => SettingValue::F64(self.color_r),
            "color_g" => SettingValue::F64(self.color_g),
            "color_b" => SettingValue::F64(self.color_b),
            "color_a" => SettingValue::F64(self.color_a),
            "size" => SettingValue::F64(self.size),
            "line_cap" => SettingValue::String(self.line_cap.clone()),
            "line_join" => SettingValue::String(self.line_join.clone()),
            other => {
                eprintln!("LineSettings::get_value: unknown key '{}'", other);
                // Default – you can change this to whatever makes sense.
                SettingValue::Bool(false)
            }
        }
    }
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct ArrowSettings {
    color_r: f64,
    color_g: f64,
    color_b: f64,
    color_a: f64,
    size: f64,
    line_cap: String,
    line_join: String,
    arrow_size: f64,
}

impl ArrowSettings {
    /// Get a setting by name, returning a `SettingValue`.
    pub fn get_value(&self, setting_name: &str) -> SettingValue {
        match setting_name {
            "color_r" => SettingValue::F64(self.color_r),
            "color_g" => SettingValue::F64(self.color_g),
            "color_b" => SettingValue::F64(self.color_b),
            "color_a" => SettingValue::F64(self.color_a),
            "size" => SettingValue::F64(self.size),
            "line_cap" => SettingValue::String(self.line_cap.clone()),
            "line_join" => SettingValue::String(self.line_join.clone()),
            "arrow_size" => SettingValue::F64(self.arrow_size),
            other => {
                eprintln!("LineSettings::get_value: unknown key '{}'", other);
                // Default – you can change this to whatever makes sense.
                SettingValue::Bool(false)
            }
        }
    }
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct FreehandSettings {
    color_r: f64,
    color_g: f64,
    color_b: f64,
    color_a: f64,
    size: f64,
    tool: String,
}

impl FreehandSettings {
    /// Get a setting by name, returning a `SettingValue`.
    pub fn get_value(&self, setting_name: &str) -> SettingValue {
        match setting_name {
            "color_r" => SettingValue::F64(self.color_r),
            "color_g" => SettingValue::F64(self.color_g),
            "color_b" => SettingValue::F64(self.color_b),
            "color_a" => SettingValue::F64(self.color_a),
            "size" => SettingValue::F64(self.size),
            "tool" => SettingValue::String(self.tool.clone()),
            other => {
                eprintln!("FreehandSettings::get_value: unknown key '{}'", other);
                // Default – you can change this to whatever makes sense.
                SettingValue::Bool(false)
            }
        }
    }
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct NumberedCircleSettings {
    fill: bool,
    fill_r: f64,
    fill_g: f64,
    fill_b: f64,
    fill_a: f64,
    border: bool,
    border_size: f64,
    border_r: f64,
    border_g: f64,
    border_b: f64,
    border_a: f64,
    number: i32,
    font_size: f64,
    // font_face = "Sans"
    font_face: String,
    font_weight: String,
    font_slant: String,
    font_r: f64,
    font_g: f64,
    font_b: f64,
    font_a: f64,
    radius: f64,
}

impl NumberedCircleSettings {
    /// Get a setting by name, returning a `SettingValue`.
    pub fn get_value(&self, setting_name: &str) -> SettingValue {
        match setting_name {
            "fill" => SettingValue::Bool(self.fill),
            "fill_r" => SettingValue::F64(self.fill_r),
            "fill_g" => SettingValue::F64(self.fill_g),
            "fill_b" => SettingValue::F64(self.fill_b),
            "fill_a" => SettingValue::F64(self.fill_a),
            "border" => SettingValue::Bool(self.border),
            "border_size" => SettingValue::F64(self.border_size),
            "border_r" => SettingValue::F64(self.border_r),
            "border_g" => SettingValue::F64(self.border_g),
            "border_b" => SettingValue::F64(self.border_b),
            "border_a" => SettingValue::F64(self.border_a),
            "number" => SettingValue::I32(self.number),
            "font_size" => SettingValue::F64(self.font_size),
            "font_face" => SettingValue::String(self.font_face.clone()),
            "font_weight" => SettingValue::String(self.font_weight.clone()),
            "font_slant" => SettingValue::String(self.font_slant.clone()),
            "font_r" => SettingValue::F64(self.font_r),
            "font_g" => SettingValue::F64(self.font_g),
            "font_b" => SettingValue::F64(self.font_b),
            "font_a" => SettingValue::F64(self.font_a),
            "radius" => SettingValue::F64(self.radius),
            other => {
                eprintln!("NumberedCircleSettings::get_value: unknown key '{}'", other);
                // Default – you can change this to whatever makes sense.
                SettingValue::Bool(false)
            }
        }
    }
}

// Top level struct to hold the TOML data.
#[derive(Default, Clone)]
pub struct SettingsRc {
    pub arc: ArcSettingsRc,
    pub rect: RectSettingsRc,
    pub line: LineSettingsRc,
    pub arrow: ArrowSettingsRc,
    pub freehand: FreehandSettingsRc,
    pub numbered_circle: NumberedCircleSettingsRc,
}

#[derive(Default, Clone)]
pub struct ArcSettingsRc {
    fill: Rc<Cell<bool>>,
    fill_r: Rc<Cell<f64>>,
    fill_g: Rc<Cell<f64>>,
    fill_b: Rc<Cell<f64>>,
    fill_a: Rc<Cell<f64>>,
    border: Rc<Cell<bool>>,
    border_size: Rc<Cell<f64>>,
    border_r: Rc<Cell<f64>>,
    border_g: Rc<Cell<f64>>,
    border_b: Rc<Cell<f64>>,
    border_a: Rc<Cell<f64>>,
}

impl ArcSettingsRc {
    /// Get a setting by name, returning a `SettingValue`.
    pub fn get_value(&self, setting_name: &str) -> SettingValue {
        match setting_name {
            "fill" => SettingValue::Bool(self.fill.get()),
            "fill_r" => SettingValue::F64(self.fill_r.get()),
            "fill_g" => SettingValue::F64(self.fill_g.get()),
            "fill_b" => SettingValue::F64(self.fill_b.get()),
            "fill_a" => SettingValue::F64(self.fill_a.get()),
            "border" => SettingValue::Bool(self.border.get()),
            "border_size" => SettingValue::F64(self.border_size.get()),
            "border_r" => SettingValue::F64(self.border_r.get()),
            "border_g" => SettingValue::F64(self.border_g.get()),
            "border_b" => SettingValue::F64(self.border_b.get()),
            "border_a" => SettingValue::F64(self.border_a.get()),
            other => {
                eprintln!("ArcSettings::get_value: unknown key '{}'", other);
                // Default – you can change this to whatever makes sense.
                SettingValue::Bool(false)
            }
        }
    }
    /// Set a setting by name, returning a `SettingValue`.
    pub fn set_value(&self, setting_name: &str, value: SettingValue) -> std::io::Result<()> {
        match (setting_name, value) {
            // ---- fill -------------------------------------------------------
            ("fill", SettingValue::Bool(v)) => {
                self.fill.set(v);
                Ok(())
            }
            ("fill_r", SettingValue::F64(v)) => {
                self.fill_r.set(v);
                Ok(())
            }
            ("fill_g", SettingValue::F64(v)) => {
                self.fill_g.set(v);
                Ok(())
            }
            ("fill_b", SettingValue::F64(v)) => {
                self.fill_b.set(v);
                Ok(())
            }
            ("fill_a", SettingValue::F64(v)) => {
                self.fill_a.set(v);
                Ok(())
            }

            // ---- border -----------------------------------------------------
            ("border", SettingValue::Bool(v)) => {
                self.border.set(v);
                Ok(())
            }
            ("border_size", SettingValue::F64(v)) => {
                self.border_size.set(v);
                Ok(())
            }
            ("border_r", SettingValue::F64(v)) => {
                self.border_r.set(v);
                Ok(())
            }
            ("border_g", SettingValue::F64(v)) => {
                self.border_g.set(v);
                Ok(())
            }
            ("border_b", SettingValue::F64(v)) => {
                self.border_b.set(v);
                Ok(())
            }
            ("border_a", SettingValue::F64(v)) => {
                self.border_a.set(v);
                Ok(())
            }

            // ---- mismatched type -------------------------------------------------
            (key, _wrong_type) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("ArcSettings::get_value: unknown key '{}'", key),
            )),
        }
    }
}

#[derive(Default, Clone)]
pub struct RectSettingsRc {
    fill: Rc<Cell<bool>>,
    fill_r: Rc<Cell<f64>>,
    fill_g: Rc<Cell<f64>>,
    fill_b: Rc<Cell<f64>>,
    fill_a: Rc<Cell<f64>>,
    border: Rc<Cell<bool>>,
    border_size: Rc<Cell<f64>>,
    border_r: Rc<Cell<f64>>,
    border_g: Rc<Cell<f64>>,
    border_b: Rc<Cell<f64>>,
    border_a: Rc<Cell<f64>>,
}

impl RectSettingsRc {
    /// Get a setting by name, returning a `SettingValue`.
    pub fn get_value(&self, setting_name: &str) -> SettingValue {
        match setting_name {
            "fill" => SettingValue::Bool(self.fill.get()),
            "fill_r" => SettingValue::F64(self.fill_r.get()),
            "fill_g" => SettingValue::F64(self.fill_g.get()),
            "fill_b" => SettingValue::F64(self.fill_b.get()),
            "fill_a" => SettingValue::F64(self.fill_a.get()),
            "border" => SettingValue::Bool(self.border.get()),
            "border_size" => SettingValue::F64(self.border_size.get()),
            "border_r" => SettingValue::F64(self.border_r.get()),
            "border_g" => SettingValue::F64(self.border_g.get()),
            "border_b" => SettingValue::F64(self.border_b.get()),
            "border_a" => SettingValue::F64(self.border_a.get()),
            other => {
                eprintln!("RectSettings::get_value: unknown key '{}'", other);
                // Default – you can change this to whatever makes sense.
                SettingValue::Bool(false)
            }
        }
    }
    /// Set a setting by name, returning a `SettingValue`.
    pub fn set_value(&self, setting_name: &str, value: SettingValue) -> std::io::Result<()> {
        match (setting_name, value) {
            // ---- fill -------------------------------------------------------
            ("fill", SettingValue::Bool(v)) => {
                self.fill.set(v);
                Ok(())
            }
            ("fill_r", SettingValue::F64(v)) => {
                self.fill_r.set(v);
                Ok(())
            }
            ("fill_g", SettingValue::F64(v)) => {
                self.fill_g.set(v);
                Ok(())
            }
            ("fill_b", SettingValue::F64(v)) => {
                self.fill_b.set(v);
                Ok(())
            }
            ("fill_a", SettingValue::F64(v)) => {
                self.fill_a.set(v);
                Ok(())
            }

            // ---- border -----------------------------------------------------
            ("border", SettingValue::Bool(v)) => {
                self.border.set(v);
                Ok(())
            }
            ("border_size", SettingValue::F64(v)) => {
                self.border_size.set(v);
                Ok(())
            }
            ("border_r", SettingValue::F64(v)) => {
                self.border_r.set(v);
                Ok(())
            }
            ("border_g", SettingValue::F64(v)) => {
                self.border_g.set(v);
                Ok(())
            }
            ("border_b", SettingValue::F64(v)) => {
                self.border_b.set(v);
                Ok(())
            }
            ("border_a", SettingValue::F64(v)) => {
                self.border_a.set(v);
                Ok(())
            }

            // ---- mismatched type -------------------------------------------------
            (key, _wrong_type) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("RectSettings::get_value: unknown key '{}'", key),
            )),
        }
    }
}

#[derive(Default, Clone)]
pub struct LineSettingsRc {
    color_r: Rc<Cell<f64>>,
    color_g: Rc<Cell<f64>>,
    color_b: Rc<Cell<f64>>,
    color_a: Rc<Cell<f64>>,
    size: Rc<Cell<f64>>,
    line_cap: Rc<RefCell<String>>,
    line_join: Rc<RefCell<String>>,
}

impl LineSettingsRc {
    /// Get a setting by name, returning a `SettingValue`.
    pub fn get_value(&self, setting_name: &str) -> SettingValue {
        match setting_name {
            "color_r" => SettingValue::F64(self.color_r.get()),
            "color_g" => SettingValue::F64(self.color_g.get()),
            "color_b" => SettingValue::F64(self.color_b.get()),
            "color_a" => SettingValue::F64(self.color_a.get()),
            "size" => SettingValue::F64(self.size.get()),
            "line_cap" => SettingValue::String(self.line_cap.borrow().to_string()),
            "line_join" => SettingValue::String(self.line_join.borrow().to_string()),
            other => {
                eprintln!("LineSettings::get_value: unknown key '{}'", other);
                // Default – you can change this to whatever makes sense.
                SettingValue::Bool(false)
            }
        }
    }
    /// Set a setting by name, returning a `SettingValue`.
    pub fn set_value(&self, setting_name: &str, value: SettingValue) -> std::io::Result<()> {
        match (setting_name, value) {
            // ---- fill -------------------------------------------------------
            ("color_r", SettingValue::F64(v)) => {
                self.color_r.set(v);
                Ok(())
            }
            ("color_g", SettingValue::F64(v)) => {
                self.color_g.set(v);
                Ok(())
            }
            ("color_b", SettingValue::F64(v)) => {
                self.color_b.set(v);
                Ok(())
            }
            ("color_a", SettingValue::F64(v)) => {
                self.color_a.set(v);
                Ok(())
            }
            ("size", SettingValue::F64(v)) => {
                self.size.set(v);
                Ok(())
            }

            // ---- junctions -------------------------------------------------------
            ("line_cap", SettingValue::String(v)) => {
                *self.line_cap.borrow_mut() = v;
                Ok(())
            }
            ("line_join", SettingValue::String(v)) => {
                *self.line_join.borrow_mut() = v;
                Ok(())
            }

            // ---- mismatched type -------------------------------------------------
            (key, _wrong_type) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("LineSettings::get_value: unknown key '{}'", key),
            )),
        }
    }
}

#[derive(Default, Clone)]
pub struct ArrowSettingsRc {
    color_r: Rc<Cell<f64>>,
    color_g: Rc<Cell<f64>>,
    color_b: Rc<Cell<f64>>,
    color_a: Rc<Cell<f64>>,
    size: Rc<Cell<f64>>,
    line_cap: Rc<RefCell<String>>,
    line_join: Rc<RefCell<String>>,
    arrow_size: Rc<Cell<f64>>,
}

impl ArrowSettingsRc {
    /// Get a setting by name, returning a `SettingValue`.
    pub fn get_value(&self, setting_name: &str) -> SettingValue {
        match setting_name {
            "color_r" => SettingValue::F64(self.color_r.get()),
            "color_g" => SettingValue::F64(self.color_g.get()),
            "color_b" => SettingValue::F64(self.color_b.get()),
            "color_a" => SettingValue::F64(self.color_a.get()),
            "size" => SettingValue::F64(self.size.get()),
            "line_cap" => SettingValue::String(self.line_cap.borrow().to_string()),
            "line_join" => SettingValue::String(self.line_join.borrow().to_string()),
            "arrow_size" => SettingValue::F64(self.arrow_size.get()),
            other => {
                eprintln!("LineSettings::get_value: unknown key '{}'", other);
                // Default – you can change this to whatever makes sense.
                SettingValue::Bool(false)
            }
        }
    }
    /// Set a setting by name, returning a `SettingValue`.
    pub fn set_value(&self, setting_name: &str, value: SettingValue) -> std::io::Result<()> {
        match (setting_name, value) {
            // ---- fill -------------------------------------------------------
            ("color_r", SettingValue::F64(v)) => {
                self.color_r.set(v);
                Ok(())
            }
            ("color_g", SettingValue::F64(v)) => {
                self.color_g.set(v);
                Ok(())
            }
            ("color_b", SettingValue::F64(v)) => {
                self.color_b.set(v);
                Ok(())
            }
            ("color_a", SettingValue::F64(v)) => {
                self.color_a.set(v);
                Ok(())
            }
            ("size", SettingValue::F64(v)) => {
                self.size.set(v);
                Ok(())
            }

            // ---- junctions -------------------------------------------------------
            ("line_cap", SettingValue::String(v)) => {
                *self.line_cap.borrow_mut() = v;
                Ok(())
            }
            ("line_join", SettingValue::String(v)) => {
                *self.line_join.borrow_mut() = v;
                Ok(())
            }

            // ---- mismatched type -------------------------------------------------
            (key, _wrong_type) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("LineSettings::get_value: unknown key '{}'", key),
            )),
        }
    }
}

#[derive(Default, Clone)]
pub struct FreehandSettingsRc {
    color_r: Rc<Cell<f64>>,
    color_g: Rc<Cell<f64>>,
    color_b: Rc<Cell<f64>>,
    color_a: Rc<Cell<f64>>,
    size: Rc<Cell<f64>>,
    tool: Rc<RefCell<String>>,
}

impl FreehandSettingsRc {
    /// Get a setting by name, returning a `SettingValue`.
    pub fn get_value(&self, setting_name: &str) -> SettingValue {
        match setting_name {
            "color_r" => SettingValue::F64(self.color_r.get()),
            "color_g" => SettingValue::F64(self.color_g.get()),
            "color_b" => SettingValue::F64(self.color_b.get()),
            "color_a" => SettingValue::F64(self.color_a.get()),
            "size" => SettingValue::F64(self.size.get()),
            "tool" => SettingValue::String(self.tool.borrow().to_string()),
            other => {
                eprintln!("FreehandSettings::get_value: unknown key '{}'", other);
                // Default – you can change this to whatever makes sense.
                SettingValue::Bool(false)
            }
        }
    }
    /// Set a setting by name, returning a `SettingValue`.
    pub fn set_value(&self, setting_name: &str, value: SettingValue) -> std::io::Result<()> {
        match (setting_name, value) {
            // ---- fill -------------------------------------------------------
            ("color_r", SettingValue::F64(v)) => {
                self.color_r.set(v);
                Ok(())
            }
            ("color_g", SettingValue::F64(v)) => {
                self.color_g.set(v);
                Ok(())
            }
            ("color_b", SettingValue::F64(v)) => {
                self.color_b.set(v);
                Ok(())
            }
            ("color_a", SettingValue::F64(v)) => {
                self.color_a.set(v);
                Ok(())
            }
            ("size", SettingValue::F64(v)) => {
                self.size.set(v);
                Ok(())
            }

            // ---- junctions -------------------------------------------------------
            ("tool", SettingValue::String(v)) => {
                *self.tool.borrow_mut() = v;
                Ok(())
            }

            // ---- mismatched type -------------------------------------------------
            (key, _wrong_type) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("FreehandSettings::get_value: unknown key '{}'", key),
            )),
        }
    }
}

#[derive(Default, Clone)]
pub struct NumberedCircleSettingsRc {
    fill: Rc<Cell<bool>>,
    fill_r: Rc<Cell<f64>>,
    fill_g: Rc<Cell<f64>>,
    fill_b: Rc<Cell<f64>>,
    fill_a: Rc<Cell<f64>>,
    border: Rc<Cell<bool>>,
    border_size: Rc<Cell<f64>>,
    border_r: Rc<Cell<f64>>,
    border_g: Rc<Cell<f64>>,
    border_b: Rc<Cell<f64>>,
    border_a: Rc<Cell<f64>>,
    number: Rc<Cell<i32>>,
    font_size: Rc<Cell<f64>>,
    font_face: Rc<RefCell<String>>,
    font_weight: Rc<RefCell<String>>,
    font_slant: Rc<RefCell<String>>,
    font_r: Rc<Cell<f64>>,
    font_g: Rc<Cell<f64>>,
    font_b: Rc<Cell<f64>>,
    font_a: Rc<Cell<f64>>,
    radius: Rc<Cell<f64>>,
}

impl NumberedCircleSettingsRc {
    /// Get a setting by name, returning a `SettingValue`.
    pub fn get_value(&self, setting_name: &str) -> SettingValue {
        match setting_name {
            "fill" => SettingValue::Bool(self.fill.get()),
            "fill_r" => SettingValue::F64(self.fill_r.get()),
            "fill_g" => SettingValue::F64(self.fill_g.get()),
            "fill_b" => SettingValue::F64(self.fill_b.get()),
            "fill_a" => SettingValue::F64(self.fill_a.get()),
            "border" => SettingValue::Bool(self.border.get()),
            "border_size" => SettingValue::F64(self.border_size.get()),
            "border_r" => SettingValue::F64(self.border_r.get()),
            "border_g" => SettingValue::F64(self.border_g.get()),
            "border_b" => SettingValue::F64(self.border_b.get()),
            "border_a" => SettingValue::F64(self.border_a.get()),
            "number" => SettingValue::I32(self.number.get()),
            "font_size" => SettingValue::F64(self.font_size.get()),
            "font_face" => SettingValue::String(self.font_face.borrow().clone()),
            "font_weight" => SettingValue::String(self.font_weight.borrow().clone()),
            "font_slant" => SettingValue::String(self.font_slant.borrow().clone()),
            "font_r" => SettingValue::F64(self.font_r.get()),
            "font_g" => SettingValue::F64(self.font_g.get()),
            "font_b" => SettingValue::F64(self.font_b.get()),
            "font_a" => SettingValue::F64(self.font_a.get()),
            "radius" => SettingValue::F64(self.radius.get()),
            other => {
                eprintln!("NumberedCircleSettings::get_value: unknown key '{}'", other);
                // Default – you can change this to whatever makes sense.
                SettingValue::Bool(false)
            }
        }
    }
    /// Set a setting by name, returning a `SettingValue`.
    pub fn set_value(&self, setting_name: &str, value: SettingValue) -> std::io::Result<()> {
        match (setting_name, value) {
            // ---- fill -------------------------------------------------------
            ("fill", SettingValue::Bool(v)) => {
                self.fill.set(v);
                Ok(())
            }
            ("fill_r", SettingValue::F64(v)) => {
                self.fill_r.set(v);
                Ok(())
            }
            ("fill_g", SettingValue::F64(v)) => {
                self.fill_g.set(v);
                Ok(())
            }
            ("fill_b", SettingValue::F64(v)) => {
                self.fill_b.set(v);
                Ok(())
            }
            ("fill_a", SettingValue::F64(v)) => {
                self.fill_a.set(v);
                Ok(())
            }

            // ---- border -----------------------------------------------------
            ("border", SettingValue::Bool(v)) => {
                self.border.set(v);
                Ok(())
            }
            ("border_size", SettingValue::F64(v)) => {
                self.border_size.set(v);
                Ok(())
            }
            ("border_r", SettingValue::F64(v)) => {
                self.border_r.set(v);
                Ok(())
            }
            ("border_g", SettingValue::F64(v)) => {
                self.border_g.set(v);
                Ok(())
            }
            ("border_b", SettingValue::F64(v)) => {
                self.border_b.set(v);
                Ok(())
            }
            ("border_a", SettingValue::F64(v)) => {
                self.border_a.set(v);
                Ok(())
            }

            // ---- font -----------------------------------------------------
            ("font_size", SettingValue::F64(v)) => {
                self.font_size.set(v);
                Ok(())
            }
            ("font_face", SettingValue::String(v)) => {
                *self.font_face.borrow_mut() = v;
                Ok(())
            }
            ("font_weight", SettingValue::String(v)) => {
                *self.font_weight.borrow_mut() = v;
                Ok(())
            }
            ("font_slant", SettingValue::String(v)) => {
                *self.font_slant.borrow_mut() = v;
                Ok(())
            }
            ("font_r", SettingValue::F64(v)) => {
                self.font_r.set(v);
                Ok(())
            }
            ("font_g", SettingValue::F64(v)) => {
                self.font_g.set(v);
                Ok(())
            }
            ("font_b", SettingValue::F64(v)) => {
                self.font_b.set(v);
                Ok(())
            }
            ("font_a", SettingValue::F64(v)) => {
                self.font_a.set(v);
                Ok(())
            }

            // ---- number -----------------------------------------------------
            ("number", SettingValue::I32(v)) => {
                self.number.set(v);
                Ok(())
            }

            // ---- radius -----------------------------------------------------
            ("radius", SettingValue::F64(v)) => {
                self.radius.set(v);
                Ok(())
            }

            // ---- mismatched type -------------------------------------------------
            (key, _wrong_type) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("NumberedCircleSettings::get_value: unknown key '{}'", key),
            )),
        }
    }
}
impl SettingsRc {
    pub fn new() -> Self {
        let raw = Settings::new();
        let arc_rc = ArcSettingsRc {
            fill: Rc::new(Cell::new(raw.arc.fill)),
            fill_r: Rc::new(Cell::new(raw.arc.fill_r)),
            fill_g: Rc::new(Cell::new(raw.arc.fill_g)),
            fill_b: Rc::new(Cell::new(raw.arc.fill_b)),
            fill_a: Rc::new(Cell::new(raw.arc.fill_a)),
            border: Rc::new(Cell::new(raw.arc.border)),
            border_size: Rc::new(Cell::new(raw.arc.border_size)),
            border_r: Rc::new(Cell::new(raw.arc.border_r)),
            border_g: Rc::new(Cell::new(raw.arc.border_g)),
            border_b: Rc::new(Cell::new(raw.arc.border_b)),
            border_a: Rc::new(Cell::new(raw.arc.border_a)),
        };

        let rect_rc = RectSettingsRc {
            fill: Rc::new(Cell::new(raw.rect.fill)),
            fill_r: Rc::new(Cell::new(raw.rect.fill_r)),
            fill_g: Rc::new(Cell::new(raw.rect.fill_g)),
            fill_b: Rc::new(Cell::new(raw.rect.fill_b)),
            fill_a: Rc::new(Cell::new(raw.rect.fill_a)),
            border: Rc::new(Cell::new(raw.rect.border)),
            border_size: Rc::new(Cell::new(raw.rect.border_size)),
            border_r: Rc::new(Cell::new(raw.rect.border_r)),
            border_g: Rc::new(Cell::new(raw.rect.border_g)),
            border_b: Rc::new(Cell::new(raw.rect.border_b)),
            border_a: Rc::new(Cell::new(raw.rect.border_a)),
        };

        let line_rc = LineSettingsRc {
            color_r: Rc::new(Cell::new(raw.line.color_r)),
            color_g: Rc::new(Cell::new(raw.line.color_g)),
            color_b: Rc::new(Cell::new(raw.line.color_b)),
            color_a: Rc::new(Cell::new(raw.line.color_a)),
            size: Rc::new(Cell::new(raw.line.size)),
            line_cap: Rc::new(RefCell::new(raw.line.line_cap)),
            line_join: Rc::new(RefCell::new(raw.line.line_join)),
        };

        let arrow_rc = ArrowSettingsRc {
            color_r: Rc::new(Cell::new(raw.arrow.color_r)),
            color_g: Rc::new(Cell::new(raw.arrow.color_g)),
            color_b: Rc::new(Cell::new(raw.arrow.color_b)),
            color_a: Rc::new(Cell::new(raw.arrow.color_a)),
            size: Rc::new(Cell::new(raw.arrow.size)),
            line_cap: Rc::new(RefCell::new(raw.arrow.line_cap)),
            line_join: Rc::new(RefCell::new(raw.arrow.line_join)),
            arrow_size: Rc::new(Cell::new(raw.arrow.arrow_size)),
        };

        let freehand_rc = FreehandSettingsRc {
            color_r: Rc::new(Cell::new(raw.freehand.color_r)),
            color_g: Rc::new(Cell::new(raw.freehand.color_g)),
            color_b: Rc::new(Cell::new(raw.freehand.color_b)),
            color_a: Rc::new(Cell::new(raw.freehand.color_a)),
            size: Rc::new(Cell::new(raw.freehand.size)),
            tool: Rc::new(RefCell::new(raw.freehand.tool)),
        };

        let numbered_circle_rc = NumberedCircleSettingsRc {
            fill: Rc::new(Cell::new(raw.numbered_circle.fill)),
            fill_r: Rc::new(Cell::new(raw.numbered_circle.fill_r)),
            fill_g: Rc::new(Cell::new(raw.numbered_circle.fill_g)),
            fill_b: Rc::new(Cell::new(raw.numbered_circle.fill_b)),
            fill_a: Rc::new(Cell::new(raw.numbered_circle.fill_a)),
            border: Rc::new(Cell::new(raw.numbered_circle.border)),
            border_size: Rc::new(Cell::new(raw.numbered_circle.border_size)),
            border_r: Rc::new(Cell::new(raw.numbered_circle.border_r)),
            border_g: Rc::new(Cell::new(raw.numbered_circle.border_g)),
            border_b: Rc::new(Cell::new(raw.numbered_circle.border_b)),
            border_a: Rc::new(Cell::new(raw.numbered_circle.border_a)),
            number: Rc::new(Cell::new(raw.numbered_circle.number)),
            font_size: Rc::new(Cell::new(raw.numbered_circle.font_size)),
            font_face: Rc::new(RefCell::new(raw.numbered_circle.font_face)),
            font_weight: Rc::new(RefCell::new(raw.numbered_circle.font_weight)),
            font_slant: Rc::new(RefCell::new(raw.numbered_circle.font_slant)),
            font_r: Rc::new(Cell::new(raw.numbered_circle.font_r)),
            font_g: Rc::new(Cell::new(raw.numbered_circle.font_g)),
            font_b: Rc::new(Cell::new(raw.numbered_circle.font_b)),
            font_a: Rc::new(Cell::new(raw.numbered_circle.font_a)),
            radius: Rc::new(Cell::new(raw.numbered_circle.radius)),
        };

        SettingsRc {
            arc: arc_rc,
            rect: rect_rc,
            line: line_rc,
            arrow: arrow_rc,
            freehand: freehand_rc,
            numbered_circle: numbered_circle_rc,
        }
    }

    pub fn hard_copy(&self) -> Settings {
        let ar: ArcSettings = ArcSettings {
            fill: self.arc.fill.get(),
            fill_r: self.arc.fill_r.get(),
            fill_g: self.arc.fill_g.get(),
            fill_b: self.arc.fill_b.get(),
            fill_a: self.arc.fill_a.get(),
            border: self.arc.border.get(),
            border_size: self.arc.border_size.get(),
            border_r: self.arc.border_r.get(),
            border_g: self.arc.border_g.get(),
            border_b: self.arc.border_b.get(),
            border_a: self.arc.border_a.get(),
        };

        let re: RectSettings = RectSettings {
            fill: self.rect.fill.get(),
            fill_r: self.rect.fill_r.get(),
            fill_g: self.rect.fill_g.get(),
            fill_b: self.rect.fill_b.get(),
            fill_a: self.rect.fill_a.get(),
            border: self.rect.border.get(),
            border_size: self.rect.border_size.get(),
            border_r: self.rect.border_r.get(),
            border_g: self.rect.border_g.get(),
            border_b: self.rect.border_b.get(),
            border_a: self.rect.border_a.get(),
        };

        let li = LineSettings {
            color_r: self.line.color_r.get(),
            color_g: self.line.color_g.get(),
            color_b: self.line.color_b.get(),
            color_a: self.line.color_a.get(),
            size: self.line.size.get(),
            line_cap: self.line.line_cap.borrow().to_string(),
            line_join: self.line.line_join.borrow().to_string(),
        };

        let aw = ArrowSettings {
            color_r: self.arrow.color_r.get(),
            color_g: self.arrow.color_g.get(),
            color_b: self.arrow.color_b.get(),
            color_a: self.arrow.color_a.get(),
            size: self.arrow.size.get(),
            line_cap: self.arrow.line_cap.borrow().to_string(),
            line_join: self.arrow.line_join.borrow().to_string(),
            arrow_size: self.arrow.arrow_size.get(),
        };

        let fh = FreehandSettings {
            color_r: self.freehand.color_r.get(),
            color_g: self.freehand.color_g.get(),
            color_b: self.freehand.color_b.get(),
            color_a: self.freehand.color_a.get(),
            size: self.freehand.size.get(),
            tool: self.freehand.tool.borrow().to_string(),
        };

        let nc: NumberedCircleSettings = NumberedCircleSettings {
            fill: self.numbered_circle.fill.get(),
            fill_r: self.numbered_circle.fill_r.get(),
            fill_g: self.numbered_circle.fill_g.get(),
            fill_b: self.numbered_circle.fill_b.get(),
            fill_a: self.numbered_circle.fill_a.get(),
            border: self.numbered_circle.border.get(),
            border_size: self.numbered_circle.border_size.get(),
            border_r: self.numbered_circle.border_r.get(),
            border_g: self.numbered_circle.border_g.get(),
            border_b: self.numbered_circle.border_b.get(),
            border_a: self.numbered_circle.border_a.get(),
            number: self.numbered_circle.number.get(),
            font_size: self.numbered_circle.font_size.get(),
            font_face: self.numbered_circle.font_face.borrow().clone(),
            font_weight: self.numbered_circle.font_weight.borrow().clone(),
            font_slant: self.numbered_circle.font_slant.borrow().clone(),
            font_r: self.numbered_circle.font_r.get(),
            font_g: self.numbered_circle.font_g.get(),
            font_b: self.numbered_circle.font_b.get(),
            font_a: self.numbered_circle.font_a.get(),
            radius: self.numbered_circle.radius.get(),
        };

        Settings {
            arc: ar,
            rect: re,
            line: li,
            arrow: aw,
            freehand: fh,
            numbered_circle: nc,
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        let filename = DRAWING_AREA_SETTINGS_TOML;

        let toml_str = fs::read_to_string(filename).expect("Failed to read Cargo.toml file");

        let data: Settings = toml::from_str(&toml_str).expect("Failed to deserialize Cargo.toml");

        Self {
            arc: data.arc,
            rect: data.rect,
            line: data.line,
            arrow: data.arrow,
            freehand: data.freehand,
            numbered_circle: data.numbered_circle,
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
