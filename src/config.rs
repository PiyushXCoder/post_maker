//! load, save configuration and parse cli args

use std::collections::HashMap;

use clap::{ArgEnum, Parser};
use fltk::dialog;
use fltk_theme::ThemeType;
use serde::{Deserialize, Serialize};

use crate::globals;

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
    pub tag_font_ttf: String,
    pub quote_font_ratio: f64,
    pub tag_font_ratio: f64,
    pub quote_position_ratio: f64,
    pub tag_position_ratio: f64,
    pub image_ratio: (f64, f64),
    pub color_layer: [u8; 4],
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            quote_font_ttf: String::new(),
            tag_font_ttf: String::new(),
            quote_font_ratio: 230.0,
            tag_font_ratio: 150.0,
            quote_position_ratio: 0.7,
            tag_position_ratio: 0.5,
            image_ratio: (4.0, 4.0),
            color_layer: [20, 22, 25, 197],
        }
    }
}

impl ConfigFile {
    pub(crate) fn load() -> Self {
        let conf = match dirs::config_dir() {
            Some(path) => path.join("post_maker.config"),
            None => std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("post_maker.config"),
        };

        if conf.exists() {
            let map = match std::fs::read_to_string(&conf) {
                Ok(r) => serde_json::from_str::<HashMap<String, Self>>(&r).ok(),
                Err(_) => None,
            };

            let map = match map {
                Some(m) => m,
                None => HashMap::new(),
            };

            if let Some(config) = map.get(&*globals::CONFIG_NAME.read().unwrap()) {
                return config.to_owned();
            }
        }

        let config = Self::default();
        config.save();
        config
    }

    pub(crate) fn save(&self) {
        let config_name = &*globals::CONFIG_NAME.read().unwrap();
        let conf = match dirs::config_dir() {
            Some(path) => path.join("post_maker.config"),
            None => std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("post_maker.config"),
        };

        let map = match std::fs::read_to_string(&conf) {
            Ok(r) => serde_json::from_str::<HashMap<String, Self>>(&r).ok(),
            Err(_) => None,
        };

        let mut map = match map {
            Some(m) => m,
            None => HashMap::new(),
        };

        map.insert(config_name.to_owned(), (*self).clone());

        if let Err(_) = std::fs::write(&conf, serde_json::to_string(&map).unwrap()) {
            dialog::message_default("Can't write config!");
            eprintln!("Can't write config!");
        }
    }
}

pub(crate) fn config() -> Args {
    let args = Args::parse();
    args
}
