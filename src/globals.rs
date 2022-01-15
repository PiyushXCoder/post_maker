use crate::config;
use lazy_static::lazy_static;
use rusttype::Font;
use std::{ffi::OsString, io::Read, sync::RwLock};

lazy_static! {
    pub static ref THEME: config::Themes = config::config().theme.unwrap_or(config::Themes::System);
    pub static ref CONFIG: RwLock<config::ConfigFile> = RwLock::new(config::ConfigFile::load());
    pub static ref FONT_QUOTE: Font<'static> = {
        let mut buffer = Vec::new();
        if let Ok(mut file) = std::fs::File::open(CONFIG.read().unwrap().quote_font_ttf.as_str()) {
            if let Ok(_) = file.read_to_end(&mut buffer) {
                if let Some(out) = rusttype::Font::try_from_vec(buffer) {
                    return out;
                }
            }
        }
        rusttype::Font::try_from_vec(include_bytes!("../ReenieBeanie-Regular.ttf").to_vec())
            .unwrap()
    };
    pub static ref FONT_TAG: Font<'static> = {
        let mut buffer = Vec::new();
        if let Ok(mut file) = std::fs::File::open(&CONFIG.read().unwrap().tag_font_ttf.as_str()) {
            if let Ok(_) = file.read_to_end(&mut buffer) {
                if let Some(out) = rusttype::Font::try_from_vec(buffer) {
                    return out;
                }
            }
        }
        rusttype::Font::try_from_vec(include_bytes!("../Kalam-Regular.ttf").to_vec()).unwrap()
    };
    pub static ref ICON: OsString = include_str!("../icon.svg").into();
    pub static ref RELOAD_ICON: OsString = {
        let img = include_str!("../reload.svg");
        if *THEME == config::Themes::Dark || *THEME == config::Themes::HighContrast {
            return img.replace("fill=\"black\"", "fill=\"white\"").into();
        }
        img.into()
    };
}
