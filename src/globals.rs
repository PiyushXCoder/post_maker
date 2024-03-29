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

use crate::{config, result_ext::ResultExt};
use lazy_static::lazy_static;
use rusttype::Font;
use std::{ffi::OsString, io::Read, sync::RwLock};

lazy_static! {
    /// Theme for the GUI
    pub(crate) static ref THEME: config::Themes =
        config::args().theme.unwrap_or(config::Themes::System);

    /// Name of config to use
    pub(crate) static ref CONFIG_NAME: RwLock<String> = RwLock::new("default".to_owned());

    /// Loaded configuration
    pub(crate) static ref CONFIG: RwLock<config::ConfigFile> =
        RwLock::new(config::ConfigFile::load());

    /// Main Sender
    pub(crate) static ref MAIN_SENDER: RwLock<Option<fltk::app::Sender<crate::AppMessage>>> = RwLock::new(None);

    /// TTF Font for Quote
    pub(crate) static ref FONT_QUOTE: Font<'static> = load_font(rw_read!(CONFIG).quote_font.as_str());

    /// TTF Font for Subquote
    pub(crate) static ref FONT_SUBQUOTE: Font<'static> = load_font(rw_read!(CONFIG).subquote_font.as_str());

    /// TTF Font for Subquote 2
    pub(crate) static ref FONT_SUBQUOTE2: Font<'static> = load_font(rw_read!(CONFIG).subquote2_font.as_str());

    /// TTF Font for Tag
    pub(crate) static ref FONT_TAG: Font<'static> = load_font(rw_read!(CONFIG).tag_font.as_str());

    /// TTF Font for Tag 2
    pub(crate) static ref FONT_TAG2: Font<'static> = load_font(rw_read!(CONFIG).tag2_font.as_str());

    /// Image to use for Window
    pub(crate) static ref ICON: OsString = include_str!("../assets/icon.svg").into();

    /// Image to use for About
    pub(crate) static ref ICON_WITH_TEXT: OsString = include_str!("../assets/icon_with_text.svg").into();

    /// Image to use for Reload Button
    pub(crate) static ref RELOAD_ICON: OsString = {
        let img = include_str!("../assets/reload.svg");
        if *THEME == config::Themes::Dark || *THEME == config::Themes::HighContrast {
            return img.replace("fill=\"black\"", "fill=\"white\"").into();
        }
        img.into()
    };
}

fn load_font(path: &str) -> Font<'static> {
    let mut buffer = Vec::new();
    if let Ok(mut file) = std::fs::File::open(path) {
        if let Ok(_) = file.read_to_end(&mut buffer) {
            if let Some(out) = rusttype::Font::try_from_vec(buffer) {
                return out;
            }
        }
    }
    rusttype::Font::try_from_vec(include_bytes!("../assets/OpenSans-Regular.ttf").to_vec()).unwrap()
}
