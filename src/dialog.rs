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

pub(crate) use fltk::dialog::{color_chooser_with_default, ColorMode};
use fltk::{app::get_mouse, dialog};

pub(crate) fn input_default(txt: &str, deflt: &str) -> Option<String> {
    let (x, y) = get_mouse();
    dialog::input(x, y, txt, deflt)
}

pub(crate) fn alert_default(txt: &str) {
    let (x, y) = get_mouse();
    dialog::alert(x, y, txt)
}

pub(crate) fn message_default(txt: &str) {
    let (x, y) = get_mouse();
    dialog::message(x, y, txt)
}

pub(crate) fn choice_default(txt: &str, b0: &str, b1: &str) -> i32 {
    let (x, y) = get_mouse();
    dialog::choice2(x, y, txt, b0, b1, "").unwrap_or(-1)
}
