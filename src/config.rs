//! load, save configuration and parse cli args

use crate::{config_picker::ConfigPicker, globals};
use clap::{ArgEnum, Parser};
use fltk::dialog;
use fltk_theme::ThemeType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

lazy_static! {
    pub static ref CONFIG_PATH: PathBuf = {
        match dirs::config_dir() {
            Some(path) => path.join("post_maker.config"),
            None => std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("post_maker.config"),
        }
    };
}

/// Simple program calculate size of stuff in quote image
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub(crate) struct Args {
    /// Theme to use for gui
    #[clap(short, long, arg_enum)]
    pub(crate) theme: Option<Themes>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Themes {
    Classic,
    /// Windows 7
    Aero,
    /// Windows 8
    Metro,
    /// Classic MacOS
    AquaClassic,
    /// Xfce
    Greybird,
    /// Windows 2000
    Blue,
    /// Dark
    Dark,
    /// High Contrast
    HighContrast,
    /// Get from System
    System,
}

impl Into<ThemeType> for Themes {
    fn into(self) -> ThemeType {
        match self {
            Self::Classic => ThemeType::Classic,
            Self::Aero => ThemeType::Aero,
            Self::Metro => ThemeType::Metro,
            Self::AquaClassic => ThemeType::AquaClassic,
            Self::Greybird => ThemeType::Greybird,
            Self::Blue => ThemeType::Blue,
            Self::Dark => ThemeType::Dark,
            Self::HighContrast => ThemeType::HighContrast,
            Self::System => {
                if cfg!(windows) {
                    ThemeType::Metro
                } else if cfg!(unix) {
                    ThemeType::Greybird
                } else {
                    ThemeType::Classic
                }
            }
        }
    }
}

/// Configuation file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    pub quote_font_ttf: String,
    pub subquote_font_ttf: String,
    pub tag_font_ttf: String,
    pub quote_font_ratio: f64,
    pub subquote_font_ratio: f64,
    pub tag_font_ratio: f64,
    pub quote_position_ratio: f64,
    pub subquote_position_ratio: f64,
    pub tag_position_ratio: f64,
    pub image_ratio: (f64, f64),
    pub color_layer: [u8; 4],
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            quote_font_ttf: String::new(),
            subquote_font_ttf: String::new(),
            tag_font_ttf: String::new(),
            quote_font_ratio: 230.0,
            subquote_font_ratio: 230.0,
            tag_font_ratio: 150.0,
            quote_position_ratio: 0.7,
            subquote_position_ratio: 0.8,
            tag_position_ratio: 0.5,
            image_ratio: (4.0, 5.0),
            color_layer: [20, 22, 25, 197],
        }
    }
}

impl ConfigFile {
    pub(crate) fn load() -> Self {
        // config_picker::ConfigPicker::new();
        if CONFIG_PATH.exists() {
            let map = get_configs();

            let map = match map {
                Some(m) => m,
                None => HashMap::new(),
            };

            let default_config = (&*globals::CONFIG_NAME.read().unwrap()).to_string();
            let config_name =
                if map.len() > 1 && map.len() != 0 || !map.contains_key(&default_config) {
                    ConfigPicker::new(map.keys().map(|a| a.to_owned()).collect())
                        .selected()
                        .unwrap_or(default_config)
                } else {
                    default_config
                };

            if let Some(config) = map.get(&config_name) {
                *globals::CONFIG_NAME.write().unwrap() = config_name;
                return config.to_owned();
            }
        }

        let config = Self::default();
        let mut configs = HashMap::new();
        configs.insert(
            (&*globals::CONFIG_NAME.read().unwrap()).to_owned(),
            config.clone(),
        );
        save_configs(configs);
        config
    }
}

pub(crate) fn get_configs() -> Option<HashMap<String, ConfigFile>> {
    match std::fs::read_to_string(&*CONFIG_PATH) {
        Ok(r) => serde_json::from_str::<HashMap<String, ConfigFile>>(&r).ok(),
        Err(_) => None,
    }
}

pub(crate) fn save_configs(configs: HashMap<String, ConfigFile>) {
    if let Err(_) = std::fs::write(&*CONFIG_PATH, serde_json::to_string(&configs).unwrap()) {
        dialog::alert_default("Can't write config!");
        eprintln!("Can't write config!");
    }
}

pub(crate) fn config() -> Args {
    let args = Args::parse();
    args
}
