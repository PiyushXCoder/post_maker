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

use std::{cell::RefCell, rc::Rc};

use crate::globals;
use fltk::{
    app,
    browser::{Browser, BrowserType},
    button::Button,
    frame::Frame,
    group::Flex,
    image::SvgImage,
    prelude::*,
    window::Window,
};

pub(crate) struct ConfigPicker {
    pub(crate) win: Window,
    pub(crate) selected: Rc<RefCell<Option<String>>>,
    pub(crate) browse: Browser,
    pub(crate) apply_btn: Button,
}

impl ConfigPicker {
    pub(crate) fn new(configs: Vec<String>) -> Self {
        let mut win = Window::new(0, 0, 500, 400, "Configurations").center_screen();
        win.set_icon(Some(
            SvgImage::from_data(globals::ICON.to_str().unwrap()).unwrap(),
        ));

        let mut main_flex = Flex::default().size_of_parent().column();

        main_flex.set_size(
            &Frame::default().with_label("Pick a configutation to use"),
            40,
        );

        // Work area
        let mut browse = Browser::default().with_type(BrowserType::Hold);
        for name in &configs {
            browse.add(&name);
        }
        browse.select(1);

        // Panel
        let top_padding_btn = Frame::default();
        let mut panel_flex = Flex::default().row();
        Frame::default();
        let apply_btn = Button::default().with_label("apply");
        Frame::default();
        panel_flex.set_size(&apply_btn, 100);
        panel_flex.end();
        let bottom_padding_btn = Frame::default();

        main_flex.set_size(&top_padding_btn, 5);
        main_flex.set_size(&panel_flex, 30);
        main_flex.set_size(&bottom_padding_btn, 5);
        main_flex.end();

        win.end();
        win.make_resizable(true);

        let mut config_picker = Self {
            win,
            selected: Rc::new(RefCell::new(browse.selected_text())),
            browse,
            apply_btn,
        };
        config_picker.event();

        config_picker.win.show();
        while config_picker.win.shown() {
            app::wait();
        }
        config_picker
    }

    fn event(&mut self) {
        let mut win = self.win.clone();
        self.apply_btn.set_callback(move |_| {
            win.hide();
        });

        let selected = Rc::clone(&self.selected);
        self.browse.set_callback(move |f| {
            *selected.borrow_mut() = f.selected_text();
        });

        let selected = Rc::clone(&self.selected);
        self.win.set_callback(move |f| {
            *selected.borrow_mut() = None;
            f.hide();
        });
    }
}
