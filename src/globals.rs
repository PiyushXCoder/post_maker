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
        rusttype::Font::try_from_vec(
            include_bytes!("../assets/fonts/ReenieBeanie-Regular.ttf").to_vec(),
        )
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
        rusttype::Font::try_from_vec(
            include_bytes!("../assets/fonts/ReenieBeanie-Regular.ttf").to_vec(),
        )
        .unwrap()
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
        rusttype::Font::try_from_vec(
            include_bytes!("../assets/fonts/Rajdhani-Regular.ttf").to_vec(),
        )
        .unwrap()
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
        rusttype::Font::try_from_vec(include_bytes!("../assets/fonts/Kalam-Regular.ttf").to_vec())
            .unwrap()
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
        rusttype::Font::try_from_vec(
            include_bytes!("../assets/fonts/Rajdhani-Regular.ttf").to_vec(),
        )
        .unwrap()
    };
    pub(crate) static ref ICON: OsString = include_str!("../assets/icon.svg").into();
    pub(crate) static ref RELOAD_ICON: OsString = {
        let img = include_str!("../assets/reload.svg");
        if *THEME == config::Themes::Dark || *THEME == config::Themes::HighContrast {
            return img.replace("fill=\"black\"", "fill=\"white\"").into();
        }
        img.into()
    };
}
