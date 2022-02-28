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

//! About Window
use crate::{config, globals};
use fltk::{
    app,
    button::Button,
    dialog,
    enums::{self, Align, Event},
    frame::Frame,
    group::Flex,
    image::SvgImage,
    prelude::*,
    window::Window,
};

pub(crate) struct About {
    pub(crate) win: Window,
    pub(crate) repo_link: Frame,
    pub(crate) dev_link: Frame,
    pub(crate) license_link: Frame,
    pub(crate) close_btn: Button,
}

impl About {
    pub(crate) fn new() -> Self {
        let mut win = Window::new(0, 0, 400, 500, "About Us").center_screen();
        win.set_icon(Some(
            SvgImage::from_data(globals::ICON.to_str().unwrap()).unwrap(),
        ));

        let link_color = if *globals::THEME == config::Themes::Dark
            || *globals::THEME == config::Themes::HighContrast
        {
            enums::Color::rgb_color(111, 190, 255)
        } else {
            enums::Color::rgb_color(16, 71, 151)
        };

        let mut main_flex = Flex::default().with_size(390, 490).with_pos(5, 5).column();

        let mut icon = Frame::default();
        let mut img = SvgImage::from_data(globals::ICON_WITH_TEXT.to_str().unwrap()).unwrap();
        img.scale(200, 200, true, true);
        icon.set_image(Some(img));

        let mut description =
            Frame::default().with_label(&textwrap::fill(env!("CARGO_PKG_DESCRIPTION"), 50));
        description.set_label_size(14);
        main_flex.set_size(&description, 50);

        main_flex.set_size(
            &Frame::default().with_label(&format!("Version: {}", env!("CARGO_PKG_VERSION"))),
            30,
        );

        let mut grp = Flex::default().row();
        let mut git = Frame::default()
            .with_label("Git:")
            .with_align(Align::Right | Align::Inside);
        git.set_label_size(13);
        grp.set_size(&git, 60);

        let mut repo_link = Frame::default()
            .with_label(env!("CARGO_PKG_REPOSITORY"))
            .with_align(Align::Left | Align::Inside);
        repo_link.set_label_color(link_color.clone());
        repo_link.set_label_size(13);
        grp.end();
        main_flex.set_size(&grp, 30);

        let mut text = Frame::default()
            .with_label("Developed with <3 by PiyushXCoder")
            .with_align(Align::Bottom | Align::Inside);
        text.set_label_size(13);
        main_flex.set_size(&text, 20);

        let mut dev_link = Frame::default()
            .with_label("https://piyushxcoder.in")
            .with_align(Align::Top | Align::Inside);
        dev_link.set_label_color(link_color.clone());
        dev_link.set_label_size(13);
        main_flex.set_size(&dev_link, 20);

        let mut license = Frame::default()
            .with_label("This program comes with absolutely no warrant.See the")
            .with_align(Align::Bottom | Align::Inside);
        license.set_label_size(13);
        main_flex.set_size(&license, 25);

        let mut license_link =
            Frame::default().with_label("GNU General Public License, version 3 or later");
        license_link.set_label_color(link_color.clone());
        license_link.set_label_size(13);
        main_flex.set_size(&license_link, 13);

        let mut license = Frame::default()
            .with_label("for details.")
            .with_align(Align::Top | Align::Inside);
        license.set_label_size(13);
        main_flex.set_size(&license, 25);

        // Panel
        let mut panel_flex = Flex::default().row();
        Frame::default();
        let close_btn = Button::default().with_label("Close");
        Frame::default();
        panel_flex.set_size(&close_btn, 100);
        panel_flex.end();
        main_flex.set_size(&panel_flex, 30);

        main_flex.set_size(&Frame::default(), 5);

        main_flex.end();

        win.end();
        win.make_modal(true);

        let mut about = Self {
            win,
            repo_link,
            dev_link,
            license_link,
            close_btn,
        };
        about.event();

        about
    }

    pub(crate) fn show(&mut self) {
        self.win.show();
        while self.win.shown() {
            app::wait();
        }
    }

    // Set callbacks of elements
    fn event(&mut self) {
        // Repository Link
        self.repo_link.handle(|_, ev| {
            if ev == Event::Push {
                if let Err(e) = webbrowser::open(env!("CARGO_PKG_REPOSITORY")) {
                    dialog::alert_default("Failed to open the link!");
                    warn!("Failed to open the link!\n{:?}", e);
                }
            }
            true
        });

        // Developer's Link
        self.dev_link.handle(|_, ev| {
            if ev == Event::Push {
                if let Err(e) = webbrowser::open("https://piyushxcoder.in") {
                    dialog::alert_default("Failed to open the link!");
                    warn!("Failed to open the link!\n{:?}", e);
                }
            }
            true
        });

        // License Link
        self.license_link.handle(|_, ev| {
            if ev == Event::Push {
                if let Err(e) = webbrowser::open("https://www.gnu.org/licenses/gpl-3.0.html") {
                    dialog::alert_default("Failed to open the link!");
                    warn!("Failed to open the link!\n{:?}", e);
                }
            }
            true
        });

        // Close Button
        let mut win = self.win.clone();
        self.close_btn.set_callback(move |_| {
            win.hide();
        });
    }
}
