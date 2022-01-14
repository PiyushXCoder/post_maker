//! load, save configuration and parse cli args

use clap::{ArgEnum, Parser};
use fltk::dialog;
use fltk_theme::ThemeType;
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub quote_font_ttf: String,
    pub tag_font_ttf: String,
    pub quote_font_ratio: f64,
    pub tag_font_ratio: f64,
    pub color_layer: [u8; 4],
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            quote_font_ttf: String::new(),
            tag_font_ttf: String::new(),
            quote_font_ratio: 215.0,
            tag_font_ratio: 150.0,
            color_layer: [20, 22, 25, 200],
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
            if let Ok(text) = std::fs::read_to_string(&conf) {
                if let Ok(config) = serde_json::from_str::<Self>(&text) {
                    return config;
                }
            }
        }

        let config = Self::default();
        if let Err(_) = std::fs::write(&conf, serde_json::to_string(&config).unwrap()) {
            dialog::message_default("Can't write config!");
            eprintln!("Can't write config!");
        }
        config
    }

    pub(crate) fn save(&self) {
        let conf = match dirs::config_dir() {
            Some(path) => path.join("post_maker.config"),
            None => std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("post_maker.config"),
        };

        if let Err(_) = std::fs::write(&conf, serde_json::to_string(self).unwrap()) {
            dialog::message_default("Can't write config!");
            eprintln!("Can't write config!");
        }
    }
}

pub(crate) fn config() -> Args {
    let args = Args::parse();
    args
}
