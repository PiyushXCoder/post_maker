/*
    This file is part of Post Maker.
    Post Maker is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
    Post Maker is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with Post Maker.  If not, see <https://www.gnu.org/licenses/>
*/

//! load, save configuration and parse cli args
use crate::{
    config_picker::ConfigPicker, dialog, globals, result_ext::ResultExt, utils::ImageType,
};
use clap::{ArgEnum, Parser};
use fltk_theme::ThemeType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, path::PathBuf};

lazy_static! {
    /// Directory where all Configurations are present
    static ref CONFIG_DIR: PathBuf = {
        let dir = match dirs::config_dir() {
            Some(path) => path,
            None => std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .to_owned(),
        }
        .join("post_maker");
        if !dir.exists() {
            if let Err(e) = std::fs::create_dir(&dir) {
                dialog::alert_default("Failed to create config dir!");
                panic!("Failed to create config dir!\n{:?}", e);
            }
        }
        dir
    };

    /// Configuration File
    static ref CONFIG_FILE: PathBuf = CONFIG_DIR.join("post_maker.config");

    /// Log File
    static ref LOG_FILE: PathBuf = CONFIG_DIR.join("post_maker.log");
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
pub(crate) enum Themes {
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

pub(crate) fn args() -> Args {
    let args = Args::parse();
    args
}

/// Configuation file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ConfigFile {
    pub(crate) quote_font: String,
    pub(crate) subquote_font: String,
    pub(crate) subquote2_font: String,
    pub(crate) tag_font: String,
    pub(crate) tag2_font: String,
    pub(crate) quote_font_ratio: f64,
    pub(crate) subquote_font_ratio: f64,
    pub(crate) subquote2_font_ratio: f64,
    pub(crate) tag_font_ratio: f64,
    pub(crate) tag2_font_ratio: f64,
    pub(crate) quote_position_ratio: f64,
    pub(crate) subquote_position_ratio: f64,
    pub(crate) subquote2_position_ratio: f64,
    pub(crate) tag_position_ratio: f64,
    pub(crate) tag2_position_ratio: f64,
    pub(crate) image_ratio: (f64, f64),
    pub(crate) color_layer: [u8; 4],
    pub(crate) minimum_width_limit: f64, // for export of image
    pub(crate) maximum_width_limit: f64, // for export of image
    pub(crate) image_format: ImageType,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            quote_font: String::new(),
            subquote_font: String::new(),
            subquote2_font: String::new(),
            tag_font: String::new(),
            tag2_font: String::new(),
            quote_font_ratio: 250.0,
            subquote_font_ratio: 230.0,
            subquote2_font_ratio: 230.0,
            tag_font_ratio: 150.0,
            tag2_font_ratio: 150.0,
            quote_position_ratio: 0.7,
            subquote_position_ratio: 0.8,
            subquote2_position_ratio: 0.9,
            tag_position_ratio: 0.5,
            tag2_position_ratio: 0.95,
            image_ratio: (4.0, 5.0),
            color_layer: [20, 22, 25, 197],
            minimum_width_limit: 650.0,
            maximum_width_limit: 1080.0,
            image_format: ImageType::Png,
        }
    }
}

impl ConfigFile {
    pub(crate) fn load() -> Self {
        if CONFIG_FILE.exists() {
            let map = get_configs();

            let map = match map {
                Some(m) => m,
                None => HashMap::new(),
            };

            let default_config = (&*rw_read!(globals::CONFIG_NAME)).to_string();
            let config_name = if (map.len() > 1 || !map.contains_key(&default_config))
                && map.len() != 0
            {
                let picked = ConfigPicker::new(map.keys().map(|a| a.to_owned()).collect()).selected;
                let picked = picked.borrow();
                match &*picked {
                    Some(v) => v.to_owned(),
                    None => std::process::exit(0),
                }
            } else {
                default_config
            };

            if let Some(config) = map.get(&config_name) {
                *rw_write!(globals::CONFIG_NAME) = config_name;
                return config.to_owned();
            }
        }

        let config = Self::default();
        let mut configs = HashMap::new();
        configs.insert(
            (&*rw_read!(globals::CONFIG_NAME)).to_owned(),
            config.clone(),
        );
        save_configs(configs);
        config
    }
}

/// Get parsed configs from file
pub(crate) fn get_configs() -> Option<HashMap<String, ConfigFile>> {
    match std::fs::read_to_string(&*CONFIG_FILE) {
        Ok(r) => serde_json::from_str::<HashMap<String, ConfigFile>>(&r).ok(),
        Err(_) => None,
    }
}

/// Save configs
pub(crate) fn save_configs(configs: HashMap<String, ConfigFile>) {
    std::fs::write(&*CONFIG_FILE, serde_json::to_string(&configs).unwrap())
        .expect_log("Can't write config!");
}

pub(crate) fn log_file() -> File {
    match File::create(&*LOG_FILE) {
        Ok(f) => f,
        Err(e) => {
            dialog::alert_default("Can't open log file!");
            panic!("{:?}", e);
        }
    }
}
