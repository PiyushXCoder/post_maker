use crate::config;
use lazy_static::lazy_static;
use rusttype::Font;
use std::{ffi::OsString, io::Read, sync::RwLock};

lazy_static! {
    pub(crate) static ref THEME: config::Themes =
        config::config().theme.unwrap_or(config::Themes::System);
    pub(crate) static ref CONFIG_NAME: RwLock<String> = RwLock::new("default".to_owned());
    pub(crate) static ref CONFIG: RwLock<config::ConfigFile> =
        RwLock::new(config::ConfigFile::load());
    pub(crate) static ref FONT_QUOTE: Font<'static> = {
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
    pub(crate) static ref FONT_SUBQUOTE: Font<'static> = {
        let mut buffer = Vec::new();
        if let Ok(mut file) = std::fs::File::open(CONFIG.read().unwrap().subquote_font_ttf.as_str())
        {
            if let Ok(_) = file.read_to_end(&mut buffer) {
                if let Some(out) = rusttype::Font::try_from_vec(buffer) {
                    return out;
                }
            }
        }
        rusttype::Font::try_from_vec(include_bytes!("../Rajdhani-Regular.ttf").to_vec()).unwrap()
    };
    pub(crate) static ref FONT_SUBQUOTE2: Font<'static> = {
        let mut buffer = Vec::new();
        if let Ok(mut file) =
            std::fs::File::open(CONFIG.read().unwrap().subquote2_font_ttf.as_str())
        {
            if let Ok(_) = file.read_to_end(&mut buffer) {
                if let Some(out) = rusttype::Font::try_from_vec(buffer) {
                    return out;
                }
            }
        }
        rusttype::Font::try_from_vec(include_bytes!("../Rajdhani-Regular.ttf").to_vec()).unwrap()
    };
    pub(crate) static ref FONT_TAG: Font<'static> = {
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
    pub(crate) static ref FONT_TAG2: Font<'static> = {
        let mut buffer = Vec::new();
        if let Ok(mut file) = std::fs::File::open(&CONFIG.read().unwrap().tag2_font_ttf.as_str()) {
            if let Ok(_) = file.read_to_end(&mut buffer) {
                if let Some(out) = rusttype::Font::try_from_vec(buffer) {
                    return out;
                }
            }
        }
        rusttype::Font::try_from_vec(include_bytes!("../Kalam-Regular.ttf").to_vec()).unwrap()
    };
    pub(crate) static ref ICON: OsString = include_str!("../icon.svg").into();
    pub(crate) static ref RELOAD_ICON: OsString = {
        let img = include_str!("../reload.svg");
        if *THEME == config::Themes::Dark || *THEME == config::Themes::HighContrast {
            return img.replace("fill=\"black\"", "fill=\"white\"").into();
        }
        img.into()
    };
}
