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

//! Window to edit configuration

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use fltk::{
    app,
    browser::{Browser, BrowserType},
    button::{Button, RadioRoundButton},
    dialog::{self, FileDialogOptions, NativeFileChooser},
    enums::{self, Align, Event, Font},
    frame::Frame,
    group::{Flex, Scroll},
    image::SvgImage,
    output::Output,
    prelude::*,
    valuator::ValueInput,
    window::Window,
};

use crate::{
    config::{self, ConfigFile},
    globals, utils,
};

pub(crate) struct ConfigWindow {
    pub(crate) win: Window,
    pub(crate) browse: Browser,
    pub(crate) selected_browse_line: Rc<RefCell<i32>>,
    pub(crate) add_config_btn: Button,
    pub(crate) del_config_btn: Button,
    pub(crate) quote_font: Output,
    pub(crate) quote_font_browse: Button,
    pub(crate) subquote_font: Output,
    pub(crate) subquote_font_browse: Button,
    pub(crate) subquote2_font: Output,
    pub(crate) subquote2_font_browse: Button,
    pub(crate) tag_font: Output,
    pub(crate) tag_font_browse: Button,
    pub(crate) tag2_font: Output,
    pub(crate) tag2_font_browse: Button,
    pub(crate) quote_font_ratio: ValueInput,
    pub(crate) subquote_font_ratio: ValueInput,
    pub(crate) subquote2_font_ratio: ValueInput,
    pub(crate) tag_font_ratio: ValueInput,
    pub(crate) tag2_font_ratio: ValueInput,
    pub(crate) quote_position_ratio: ValueInput,
    pub(crate) subquote_position_ratio: ValueInput,
    pub(crate) subquote2_position_ratio: ValueInput,
    pub(crate) tag_position_ratio: ValueInput,
    pub(crate) tag2_position_ratio: ValueInput,
    pub(crate) image_ratio_width: ValueInput,
    pub(crate) image_ratio_height: ValueInput,
    /// RGB value of top translucent layer
    pub(crate) translucent_layer_rgb: Button,
    /// opacity value of top translucent layer
    pub(crate) translucent_layer_alpha: ValueInput,
    pub(crate) png_format: RadioRoundButton,
    pub(crate) jpeg_format: RadioRoundButton,
    pub(crate) defaults_btn: Button,
    pub(crate) save_btn: Button,
    pub(crate) cancel_btn: Button,
    pub(crate) configs: Rc<RefCell<HashMap<String, ConfigFile>>>,
    did_save: Rc<RefCell<bool>>,
}

impl ConfigWindow {
    pub(crate) fn new() -> Self {
        let configs = config::get_configs().unwrap_or(HashMap::new());
        let mut win = Window::new(0, 0, 900, 600, "Config").center_screen();
        win.set_icon(Some(
            SvgImage::from_data(globals::ICON.to_str().unwrap()).unwrap(),
        ));

        // Config picking area
        let mut config_picker_flex = Flex::default()
            .with_size(200, win.height() - 50)
            .with_pos(5, 5)
            .column();
        // Picker
        let browse = Browser::default().with_type(BrowserType::Hold);

        // Panel
        let top_padding_btn = Frame::default();
        let mut panel_flex = Flex::default().row();
        Frame::default();
        let add_config_btn = Button::default().with_label("add");
        let del_config_btn = Button::default().with_label("delete");
        Frame::default();
        panel_flex.set_size(&add_config_btn, 50);
        panel_flex.set_size(&del_config_btn, 50);
        panel_flex.end();
        let bottom_padding_btn = Frame::default();

        config_picker_flex.set_size(&top_padding_btn, 5);
        config_picker_flex.set_size(&panel_flex, 30);
        config_picker_flex.set_size(&bottom_padding_btn, 5);
        config_picker_flex.end();

        // Bottom Panel
        let mut panel_grp = Flex::default()
            .with_size(win.width() - 10, 30)
            .with_pos(5, win.height() - 40)
            .row();
        Frame::default();
        let defaults_btn = Button::default().with_label("Defaults");
        let save_btn = Button::default().with_label("Save");
        let cancel_btn = Button::default().with_label("Cancel");
        panel_grp.set_size(&defaults_btn, 100);
        panel_grp.set_size(&save_btn, 100);
        panel_grp.set_size(&cancel_btn, 100);
        panel_grp.end();

        // Rest everything
        let mut scroll = Scroll::default()
            .with_size(win.width() - 210, win.height() - 50)
            .with_pos(205, 5);

        let mut col = Flex::default()
            .with_size(scroll.width() - 35, 700)
            .column()
            .with_pos(100, 0);

        let mut label = Frame::default().with_label("Fonts:");
        label.set_label_font(enums::Font::HelveticaBold);
        col.set_size(&label, 30);
        // Fonts Group
        let row_grp = Flex::default().row();
        // column 1
        let mut col_grp = Flex::default().column();
        let mut quote_font_grp = Flex::default().row();
        quote_font_grp.set_size(
            &Frame::default()
                .with_label("Quote")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let quote_font = Output::default();
        let quote_font_browse = Button::default().with_label("Pick");
        quote_font_grp.set_size(&quote_font_browse, 50);
        quote_font_grp.end();
        col_grp.set_size(&quote_font_grp, 30);

        let mut subquote_font_grp = Flex::default().row();
        subquote_font_grp.set_size(
            &Frame::default()
                .with_label("Subquote")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let subquote_font = Output::default();
        let subquote_font_browse = Button::default().with_label("Pick");
        subquote_font_grp.set_size(&subquote_font_browse, 50);
        subquote_font_grp.end();
        col_grp.set_size(&subquote_font_grp, 30);

        let mut subquote2_font_grp = Flex::default().row();
        subquote2_font_grp.set_size(
            &Frame::default()
                .with_label("Subquote 2")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let subquote2_font = Output::default();
        let subquote2_font_browse = Button::default().with_label("Pick");
        subquote2_font_grp.set_size(&subquote2_font_browse, 50);
        subquote2_font_grp.end();
        col_grp.set_size(&subquote2_font_grp, 30);
        col_grp.end();

        // column 2
        let mut col_grp = Flex::default().column();
        let mut tag_font_grp = Flex::default().row();
        tag_font_grp.set_size(
            &Frame::default()
                .with_label("Tag")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let tag_font = Output::default();
        let tag_font_browse = Button::default().with_label("Pick");
        tag_font_grp.set_size(&tag_font_browse, 50);
        tag_font_grp.end();
        col_grp.set_size(&tag_font_grp, 30);

        let mut tag2_font_grp = Flex::default().row();
        tag2_font_grp.set_size(
            &Frame::default()
                .with_label("Tag 2")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let tag2_font = Output::default();
        let tag2_font_browse = Button::default().with_label("Pick");
        tag2_font_grp.set_size(&tag2_font_browse, 50);
        tag2_font_grp.end();
        col_grp.set_size(&tag2_font_grp, 30);
        col_grp.end();
        row_grp.end();
        col.set_size(&row_grp, 110);

        let mut label = Frame::default().with_label("Ratio of size of text:");
        label.set_label_font(enums::Font::HelveticaBold);
        col.set_size(&label, 15);
        let mut hint = Frame::default().with_label("Font size in image of height 4000 pixels");
        hint.set_label_font(Font::CourierItalic);
        hint.set_label_size(12);
        col.set_size(&hint, 20);
        // Size Ratio Group
        let row_grp = Flex::default().row();
        // column 1
        let mut col_grp = Flex::default().column();
        let mut quote_font_ratio_grp = Flex::default().row();
        quote_font_ratio_grp.set_size(
            &Frame::default()
                .with_label("Quote")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let quote_font_ratio = ValueInput::default();
        quote_font_ratio_grp.end();
        col_grp.set_size(&quote_font_ratio_grp, 30);

        let mut subquote_font_ratio_grp = Flex::default().row();
        subquote_font_ratio_grp.set_size(
            &Frame::default()
                .with_label("Subquote")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let subquote_font_ratio = ValueInput::default();
        subquote_font_ratio_grp.end();
        col_grp.set_size(&subquote_font_ratio_grp, 30);

        let mut subquote2_font_ratio_grp = Flex::default().row();
        subquote2_font_ratio_grp.set_size(
            &Frame::default()
                .with_label("Subquote 2")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let subquote2_font_ratio = ValueInput::default();
        subquote2_font_ratio_grp.end();
        col_grp.set_size(&subquote2_font_ratio_grp, 30);
        col_grp.end();

        // column 2
        let mut col_grp = Flex::default().column();
        let mut tag_font_ratio_grp = Flex::default().row();
        tag_font_ratio_grp.set_size(
            &Frame::default()
                .with_label("Tag")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let tag_font_ratio = ValueInput::default();
        tag_font_ratio_grp.end();
        col_grp.set_size(&tag_font_ratio_grp, 30);

        let mut tag2_font_ratio_grp = Flex::default().row();
        tag2_font_ratio_grp.set_size(
            &Frame::default()
                .with_label("Tag 2")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let tag2_font_ratio = ValueInput::default();
        tag2_font_ratio_grp.end();
        col_grp.set_size(&tag2_font_ratio_grp, 30);

        col_grp.end();
        row_grp.end();
        col.set_size(&row_grp, 110);

        let mut label = Frame::default().with_label("Position percentage of text:");
        label.set_label_font(enums::Font::HelveticaBold);
        col.set_size(&label, 15);
        let mut hint =
            Frame::default().with_label("Percentage of height at which text to be place");
        hint.set_label_font(Font::CourierItalic);
        hint.set_label_size(12);
        col.set_size(&hint, 20);
        // Size Ratio Group
        let row_grp = Flex::default().row();
        // column 1
        let mut col_grp = Flex::default().column();
        let mut quote_position_ratio_grp = Flex::default().row();
        quote_position_ratio_grp.set_size(
            &Frame::default()
                .with_label("Quote")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let quote_position_ratio = ValueInput::default();
        quote_position_ratio_grp.end();
        col_grp.set_size(&quote_position_ratio_grp, 30);

        let mut subquote_position_ratio_grp = Flex::default().row();
        subquote_position_ratio_grp.set_size(
            &Frame::default()
                .with_label("Subquote")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let subquote_position_ratio = ValueInput::default();
        subquote_position_ratio_grp.end();
        col_grp.set_size(&subquote_position_ratio_grp, 30);

        let mut subquote2_position_ratio_grp = Flex::default().row();
        subquote2_position_ratio_grp.set_size(
            &Frame::default()
                .with_label("Subquote 2")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let subquote2_position_ratio = ValueInput::default();
        subquote2_position_ratio_grp.end();
        col_grp.set_size(&subquote2_position_ratio_grp, 30);
        col_grp.end();

        // column 2
        let mut col_grp = Flex::default().column();
        let mut tag_position_ratio_grp = Flex::default().row();
        tag_position_ratio_grp.set_size(
            &Frame::default()
                .with_label("Tag")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let tag_position_ratio = ValueInput::default();
        tag_position_ratio_grp.end();
        col_grp.set_size(&tag_position_ratio_grp, 30);

        let mut tag2_position_ratio_grp = Flex::default().row();
        tag2_position_ratio_grp.set_size(
            &Frame::default()
                .with_label("Tag 2")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let tag2_position_ratio = ValueInput::default();
        tag2_position_ratio_grp.end();
        col_grp.set_size(&tag2_position_ratio_grp, 30);
        col_grp.end();
        row_grp.end();
        col.set_size(&row_grp, 100);

        let mut label = Frame::default().with_label("Image:");
        label.set_label_font(enums::Font::HelveticaBold);
        col.set_size(&label, 30);

        let mut image_ratio_grp = Flex::default().row();
        image_ratio_grp.set_size(
            &Frame::default()
                .with_label("Image size ratio")
                .with_align(Align::Right | Align::Inside),
            130,
        );
        let image_ratio_width = ValueInput::default();
        image_ratio_grp.set_size(&Frame::default().with_label("x"), 30);
        let image_ratio_height = ValueInput::default();
        image_ratio_grp.end();
        col.set_size(&image_ratio_grp, 30);

        let mut label = Frame::default().with_label("Colour for dark layer:");
        label.set_label_font(enums::Font::HelveticaBold);
        col.set_size(&label, 15);
        let mut hint =
            Frame::default().with_label("Alpha should be between (0-255), Alpha mean opacity");
        hint.set_label_font(Font::CourierItalic);
        hint.set_label_size(12);
        col.set_size(&hint, 20);

        let mut translucent_layer_flex = Flex::default().row();
        translucent_layer_flex.set_size(&Frame::default(), 20);
        translucent_layer_flex.set_size(&Frame::default().with_label("Colour"), 50);
        let mut translucent_layer_rgb = Button::default();
        translucent_layer_rgb.set_frame(enums::FrameType::BorderBox);

        translucent_layer_flex.set_size(&Frame::default().with_label("Alpha"), 50);
        let translucent_layer_alpha = ValueInput::default();
        translucent_layer_flex.end();
        col.set_size(&translucent_layer_flex, 30);

        let mut label = Frame::default().with_label("Export Format:");
        label.set_label_font(enums::Font::HelveticaBold);
        col.set_size(&label, 15);

        let mut hint = Frame::default().with_label("Image format to export image");
        hint.set_label_font(Font::CourierItalic);
        hint.set_label_size(12);
        col.set_size(&hint, 20);

        let mut image_format_flex = Flex::default().row();
        image_format_flex.set_size(&Frame::default(), 20);
        let mut png_format = RadioRoundButton::default().with_label("Png");
        png_format.set_value(true);
        let jpeg_format = RadioRoundButton::default().with_label("Jpeg");
        image_format_flex.end();
        col.set_size(&image_format_flex, 30);

        Frame::default();
        col.end();

        scroll.end();
        scroll.make_resizable(true);
        scroll.scroll_to(-1 * col.x() - 5, -1 * col.y() - 5);

        win.end();
        win.make_modal(true);

        let mut config_window = Self {
            win,
            browse,
            selected_browse_line: Rc::new(RefCell::new(0)),
            add_config_btn,
            del_config_btn,
            quote_font,
            quote_font_browse,
            subquote_font,
            subquote_font_browse,
            subquote2_font,
            subquote2_font_browse,
            tag_font,
            tag_font_browse,
            tag2_font,
            tag2_font_browse,
            quote_font_ratio,
            subquote_font_ratio,
            subquote2_font_ratio,
            tag_font_ratio,
            tag2_font_ratio,
            quote_position_ratio,
            subquote_position_ratio,
            subquote2_position_ratio,
            tag_position_ratio,
            tag2_position_ratio,
            image_ratio_width,
            image_ratio_height,
            translucent_layer_rgb,
            translucent_layer_alpha,
            png_format,
            jpeg_format,
            defaults_btn,
            save_btn,
            cancel_btn,
            configs: Rc::new(RefCell::new(configs)),
            did_save: Rc::new(RefCell::new(false)),
        };
        config_window.event();

        config_window
    }

    // Show to edit config
    pub(crate) fn show(&mut self) -> bool {
        let config_name = &*globals::CONFIG_NAME.read().unwrap();
        self.browse.clear();
        for (idx, name) in self.configs.borrow().keys().enumerate() {
            self.browse.add(name);
            if name == config_name {
                self.browse.select(idx as i32 + 1);
            }
        }
        *self.selected_browse_line.borrow_mut() = self.browse.value();
        let config = globals::CONFIG.read().unwrap();
        self.quote_font.set_value(config.quote_font.as_str());
        self.subquote_font.set_value(config.subquote_font.as_str());
        self.subquote2_font
            .set_value(config.subquote2_font.as_str());
        self.tag_font.set_value(config.tag_font.as_str());
        self.tag2_font.set_value(config.tag2_font.as_str());
        self.quote_font_ratio.set_value(config.quote_font_ratio);
        self.subquote_font_ratio
            .set_value(config.subquote_font_ratio);
        self.subquote2_font_ratio
            .set_value(config.subquote2_font_ratio);
        self.tag_font_ratio.set_value(config.tag_font_ratio);
        self.tag2_font_ratio.set_value(config.tag2_font_ratio);
        self.quote_position_ratio
            .set_value(config.quote_position_ratio);
        self.subquote_position_ratio
            .set_value(config.subquote_position_ratio);
        self.subquote2_position_ratio
            .set_value(config.subquote2_position_ratio);
        self.tag_position_ratio.set_value(config.tag_position_ratio);
        self.tag2_position_ratio
            .set_value(config.tag2_position_ratio);
        self.image_ratio_width.set_value(config.image_ratio.0);
        self.image_ratio_height.set_value(config.image_ratio.1);
        utils::set_color_btn_rgba(config.color_layer, &mut self.translucent_layer_rgb);
        self.translucent_layer_alpha
            .set_value(config.color_layer[3] as f64);
        *self.did_save.borrow_mut() = false;
        drop(config);
        self.win.show();
        while self.win.shown() {
            app::wait();
        }
        *self.did_save.borrow()
    }

    /// Set callbacks of elements
    fn event(&mut self) {
        // Add new Config Button
        let mut quote_font = self.quote_font.clone();
        let mut subquote_font = self.subquote_font.clone();
        let mut subquote2_font = self.subquote2_font.clone();
        let mut tag_font = self.tag_font.clone();
        let mut tag2_font = self.tag2_font.clone();
        let mut quote_font_ratio = self.quote_font_ratio.clone();
        let mut subquote_font_ratio = self.subquote_font_ratio.clone();
        let mut subquote2_font_ratio = self.subquote2_font_ratio.clone();
        let mut tag_font_ratio = self.tag_font_ratio.clone();
        let mut tag2_font_ratio = self.tag2_font_ratio.clone();
        let mut quote_position_ratio = self.quote_position_ratio.clone();
        let mut subquote_position_ratio = self.subquote_position_ratio.clone();
        let mut subquote2_position_ratio = self.subquote2_position_ratio.clone();
        let mut tag_position_ratio = self.tag_position_ratio.clone();
        let mut tag2_position_ratio = self.tag2_position_ratio.clone();
        let mut image_ratio_width = self.image_ratio_width.clone();
        let mut image_ratio_height = self.image_ratio_height.clone();
        let mut layer_rgb = self.translucent_layer_rgb.clone();
        let mut layer_alpha = self.translucent_layer_alpha.clone();
        let mut browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        let selected_browse_line = Rc::clone(&self.selected_browse_line);
        self.add_config_btn.set_callback(move |_| {
            let name = loop {
                let name = dialog::input_default("Enter new config's name", "");
                match name {
                    Some(name) => {
                        let name = name.trim();
                        if name == "" {
                            dialog::alert_default("Name is empty!");
                        } else if !configs.borrow().contains_key(name) {
                            break name.to_owned();
                        } else {
                            dialog::alert_default("Name is already used!");
                        }
                    }
                    None => {
                        return;
                    }
                }
            };

            let conf = ConfigFile::default();
            quote_font.set_value(&conf.quote_font);
            subquote_font.set_value(&conf.subquote_font);
            subquote2_font.set_value(&conf.subquote2_font);
            tag_font.set_value(&conf.tag_font);
            tag2_font.set_value(&conf.tag2_font);
            quote_font_ratio.set_value(conf.quote_font_ratio);
            subquote_font_ratio.set_value(conf.subquote_font_ratio);
            subquote2_font_ratio.set_value(conf.subquote2_font_ratio);
            tag_font_ratio.set_value(conf.tag_font_ratio);
            tag2_font_ratio.set_value(conf.tag2_font_ratio);
            quote_position_ratio.set_value(conf.quote_position_ratio);
            subquote_position_ratio.set_value(conf.subquote_position_ratio);
            subquote2_position_ratio.set_value(conf.subquote2_position_ratio);
            tag_position_ratio.set_value(conf.tag_position_ratio);
            tag2_position_ratio.set_value(conf.tag2_position_ratio);
            image_ratio_width.set_value(conf.image_ratio.0);
            image_ratio_height.set_value(conf.image_ratio.1);
            utils::set_color_btn_rgba(conf.color_layer, &mut layer_rgb);
            layer_alpha.set_value(conf.color_layer[3] as f64);
            browse.add(&name);
            configs.borrow_mut().insert(name.clone(), conf);
            browse.select(browse.size());
            *selected_browse_line.borrow_mut() = browse.value();
            layer_rgb.redraw();
        });

        // Delete selected Config Button
        let mut quote_font = self.quote_font.clone();
        let mut subquote_font = self.subquote_font.clone();
        let mut subquote2_font = self.subquote2_font.clone();
        let mut tag_font = self.tag_font.clone();
        let mut tag2_font = self.tag2_font.clone();
        let mut quote_font_ratio = self.quote_font_ratio.clone();
        let mut subquote_font_ratio = self.subquote_font_ratio.clone();
        let mut subquote2_font_ratio = self.subquote2_font_ratio.clone();
        let mut tag_font_ratio = self.tag_font_ratio.clone();
        let mut tag2_font_ratio = self.tag2_font_ratio.clone();
        let mut quote_position_ratio = self.quote_position_ratio.clone();
        let mut subquote_position_ratio = self.subquote_position_ratio.clone();
        let mut subquote2_position_ratio = self.subquote2_position_ratio.clone();
        let mut tag_position_ratio = self.tag_position_ratio.clone();
        let mut tag2_position_ratio = self.tag2_position_ratio.clone();
        let mut image_ratio_width = self.image_ratio_width.clone();
        let mut image_ratio_height = self.image_ratio_height.clone();
        let mut layer_rgb = self.translucent_layer_rgb.clone();
        let mut layer_alpha = self.translucent_layer_alpha.clone();
        let mut browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        let selected_browse_line = Rc::clone(&self.selected_browse_line);
        self.del_config_btn.set_callback(move |_| {
            let ch = dialog::choice_default("Do you want to delete??", "Yes", "No", "");
            if ch == 1 {
                return;
            }
            if browse.size() == 1 {
                dialog::alert_default("Atleast one config should exist!");
                return;
            }
            let line = browse.value();
            configs
                .borrow_mut()
                .remove(&browse.selected_text().unwrap());
            browse.remove(browse.value());

            let line = if browse.size() < line { line - 1 } else { line };
            browse.select(line);
            *selected_browse_line.borrow_mut() = browse.value();

            if let Some(conf) = configs.borrow().get(&browse.selected_text().unwrap()) {
                quote_font.set_value(&conf.quote_font);
                subquote_font.set_value(&conf.subquote_font);
                subquote2_font.set_value(&conf.subquote2_font);
                tag_font.set_value(&conf.tag_font);
                tag2_font.set_value(&conf.tag2_font);
                quote_font_ratio.set_value(conf.quote_font_ratio);
                subquote_font_ratio.set_value(conf.subquote_font_ratio);
                subquote2_font_ratio.set_value(conf.subquote2_font_ratio);
                tag_font_ratio.set_value(conf.tag_font_ratio);
                tag2_font_ratio.set_value(conf.tag2_font_ratio);
                quote_position_ratio.set_value(conf.quote_position_ratio);
                subquote_position_ratio.set_value(conf.subquote_position_ratio);
                subquote2_position_ratio.set_value(conf.subquote2_position_ratio);
                tag_position_ratio.set_value(conf.tag_position_ratio);
                tag2_position_ratio.set_value(conf.tag2_position_ratio);
                image_ratio_width.set_value(conf.image_ratio.0);
                image_ratio_height.set_value(conf.image_ratio.1);
                utils::set_color_btn_rgba(conf.color_layer, &mut layer_rgb);
                layer_alpha.set_value(conf.color_layer[3] as f64);
                layer_rgb.redraw();
            }
        });

        // Browse Config List
        let mut quote_font = self.quote_font.clone();
        let mut subquote_font = self.subquote_font.clone();
        let mut subquote2_font = self.subquote2_font.clone();
        let mut tag_font = self.tag_font.clone();
        let mut tag2_font = self.tag2_font.clone();
        let mut quote_font_ratio = self.quote_font_ratio.clone();
        let mut subquote_font_ratio = self.subquote_font_ratio.clone();
        let mut subquote2_font_ratio = self.subquote2_font_ratio.clone();
        let mut tag_font_ratio = self.tag_font_ratio.clone();
        let mut tag2_font_ratio = self.tag2_font_ratio.clone();
        let mut quote_position_ratio = self.quote_position_ratio.clone();
        let mut subquote_position_ratio = self.subquote_position_ratio.clone();
        let mut subquote2_position_ratio = self.subquote2_position_ratio.clone();
        let mut tag_position_ratio = self.tag_position_ratio.clone();
        let mut tag2_position_ratio = self.tag2_position_ratio.clone();
        let mut image_ratio_width = self.image_ratio_width.clone();
        let mut image_ratio_height = self.image_ratio_height.clone();
        let mut layer_rgb = self.translucent_layer_rgb.clone();
        let mut layer_alpha = self.translucent_layer_alpha.clone();
        let configs = Rc::clone(&self.configs);
        let selected_browse_line = Rc::clone(&self.selected_browse_line);
        self.browse.set_callback(move |f| {
            if f.value() == 0 {
                f.select(*selected_browse_line.borrow());
                return;
            }

            if *selected_browse_line.borrow() == f.value() {
                return;
            }

            if let Some(conf) = configs.borrow().get(&f.selected_text().unwrap()) {
                quote_font.set_value(&conf.quote_font);
                subquote_font.set_value(&conf.subquote_font);
                subquote2_font.set_value(&conf.subquote2_font);
                tag_font.set_value(&conf.tag_font);
                tag2_font.set_value(&conf.tag2_font);
                quote_font_ratio.set_value(conf.quote_font_ratio);
                subquote_font_ratio.set_value(conf.subquote_font_ratio);
                subquote2_font_ratio.set_value(conf.subquote2_font_ratio);
                tag_font_ratio.set_value(conf.tag_font_ratio);
                tag2_font_ratio.set_value(conf.tag2_font_ratio);
                quote_position_ratio.set_value(conf.quote_position_ratio);
                subquote_position_ratio.set_value(conf.subquote_position_ratio);
                subquote2_position_ratio.set_value(conf.subquote2_position_ratio);
                tag_position_ratio.set_value(conf.tag_position_ratio);
                tag2_position_ratio.set_value(conf.tag2_position_ratio);
                image_ratio_width.set_value(conf.image_ratio.0);
                image_ratio_height.set_value(conf.image_ratio.1);
                utils::set_color_btn_rgba(conf.color_layer, &mut layer_rgb);
                layer_alpha.set_value(conf.color_layer[3] as f64);
                layer_rgb.redraw();
            }
            *selected_browse_line.borrow_mut() = f.value();
        });

        // Browse for Quote Font
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        let mut quote_font = self.quote_font.clone();
        self.quote_font_browse.set_callback(move |_| {
            let mut chooser = NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseFile);
            chooser.set_option(FileDialogOptions::UseFilterExt);
            chooser.set_filter("*.ttf");
            chooser.show();
            let path = chooser.filename();
            let path = std::fs::canonicalize(&path).unwrap_or(path);
            let path = path.to_str().unwrap();
            quote_font.set_value(path);
            if let Some(conf) = configs
                .borrow_mut()
                .get_mut(&browse.selected_text().unwrap())
            {
                conf.quote_font = path.to_owned();
            }
        });

        // Browse for Subquote Font
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        let mut subquote_font = self.subquote_font.clone();
        self.subquote_font_browse.set_callback(move |_| {
            let mut chooser = NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseFile);
            chooser.set_option(FileDialogOptions::UseFilterExt);
            chooser.set_filter("*.ttf");
            chooser.show();
            let path = chooser.filename();
            let path = std::fs::canonicalize(&path).unwrap_or(path);
            let path = path.to_str().unwrap();
            subquote_font.set_value(path);
            if let Some(conf) = configs
                .borrow_mut()
                .get_mut(&browse.selected_text().unwrap())
            {
                conf.subquote_font = path.to_owned();
            }
        });

        // Browse for Subquote2 Font
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        let mut subquote2_font = self.subquote2_font.clone();
        self.subquote2_font_browse.set_callback(move |_| {
            let mut chooser = NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseFile);
            chooser.set_option(FileDialogOptions::UseFilterExt);
            chooser.set_filter("*.ttf");
            chooser.show();
            let path = chooser.filename();
            let path = std::fs::canonicalize(&path).unwrap_or(path);
            let path = path.to_str().unwrap();
            subquote2_font.set_value(path);
            if let Some(conf) = configs
                .borrow_mut()
                .get_mut(&browse.selected_text().unwrap())
            {
                conf.subquote2_font = path.to_owned();
            }
        });

        // Browse for Tag Font
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        let mut tag_font = self.tag_font.clone();
        self.tag_font_browse.set_callback(move |_| {
            let mut chooser = NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseFile);
            chooser.set_option(FileDialogOptions::UseFilterExt);
            chooser.set_filter("*.ttf");
            chooser.show();
            let path = chooser.filename();
            let path = std::fs::canonicalize(&path).unwrap_or(path);
            let path = path.to_str().unwrap();
            tag_font.set_value(path);
            if let Some(conf) = configs
                .borrow_mut()
                .get_mut(&browse.selected_text().unwrap())
            {
                conf.tag_font = path.to_owned();
            }
        });

        // Browse for Tag2 Font
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        let mut tag2_font = self.tag2_font.clone();
        self.tag2_font_browse.set_callback(move |_| {
            let mut chooser = NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseFile);
            chooser.set_option(FileDialogOptions::UseFilterExt);
            chooser.set_filter("*.ttf");
            chooser.show();
            let path = chooser.filename();
            let path = std::fs::canonicalize(&path).unwrap_or(path);
            let path = path.to_str().unwrap();
            tag2_font.set_value(path);
            if let Some(conf) = configs
                .borrow_mut()
                .get_mut(&browse.selected_text().unwrap())
            {
                conf.tag2_font = path.to_owned();
            }
        });

        let mut win = self.win.clone();
        self.cancel_btn.set_callback(move |_| {
            win.hide();
        });

        // Quote font size ratio
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.quote_font_ratio.handle(move |f, ev| {
            if ev == Event::KeyUp {
                if let Some(conf) = configs
                    .borrow_mut()
                    .get_mut(&browse.selected_text().unwrap())
                {
                    conf.quote_font_ratio = f.value();
                }
            }
            true
        });

        // Subquote font size ratio
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.subquote_font_ratio.handle(move |f, ev| {
            if ev == Event::KeyUp {
                if let Some(conf) = configs
                    .borrow_mut()
                    .get_mut(&browse.selected_text().unwrap())
                {
                    conf.subquote_font_ratio = f.value();
                }
            }
            true
        });

        // Subquote2 font size ratio
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.subquote2_font_ratio.handle(move |f, ev| {
            if ev == Event::KeyUp {
                if let Some(conf) = configs
                    .borrow_mut()
                    .get_mut(&browse.selected_text().unwrap())
                {
                    conf.subquote2_font_ratio = f.value();
                }
            }
            true
        });

        // Tag font size ratio
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.tag_font_ratio.handle(move |f, ev| {
            if ev == Event::KeyUp {
                if let Some(conf) = configs
                    .borrow_mut()
                    .get_mut(&browse.selected_text().unwrap())
                {
                    conf.tag_font_ratio = f.value();
                }
            }
            true
        });

        // Tag2 font size ratio
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.tag2_font_ratio.handle(move |f, ev| {
            if ev == Event::KeyUp {
                if let Some(conf) = configs
                    .borrow_mut()
                    .get_mut(&browse.selected_text().unwrap())
                {
                    conf.tag2_font_ratio = f.value();
                }
            }
            true
        });

        // Quote position ratio
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.quote_position_ratio.handle(move |f, ev| {
            if ev == Event::KeyUp {
                if let Some(conf) = configs
                    .borrow_mut()
                    .get_mut(&browse.selected_text().unwrap())
                {
                    conf.quote_position_ratio = f.value();
                }
            }
            true
        });

        // Subquote position ratio
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.subquote_position_ratio.handle(move |f, ev| {
            if ev == Event::KeyUp {
                if let Some(conf) = configs
                    .borrow_mut()
                    .get_mut(&browse.selected_text().unwrap())
                {
                    conf.subquote_position_ratio = f.value();
                }
            }
            true
        });

        // Subquote2 position ratio
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.subquote2_position_ratio.handle(move |f, ev| {
            if ev == Event::KeyUp {
                if let Some(conf) = configs
                    .borrow_mut()
                    .get_mut(&browse.selected_text().unwrap())
                {
                    conf.subquote2_position_ratio = f.value();
                }
            }
            true
        });

        // Tag position ratio
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.tag_position_ratio.handle(move |f, ev| {
            if ev == Event::KeyUp {
                if let Some(conf) = configs
                    .borrow_mut()
                    .get_mut(&browse.selected_text().unwrap())
                {
                    conf.tag_position_ratio = f.value();
                }
            }
            true
        });

        // Tag2 position ratio
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.tag2_position_ratio.handle(move |f, ev| {
            if ev == Event::KeyUp {
                if let Some(conf) = configs
                    .borrow_mut()
                    .get_mut(&browse.selected_text().unwrap())
                {
                    conf.tag2_position_ratio = f.value();
                }
            }
            true
        });

        // Image Ratio Width
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.image_ratio_width.handle(move |f, ev| {
            if ev == Event::KeyUp {
                if let Some(conf) = configs
                    .borrow_mut()
                    .get_mut(&browse.selected_text().unwrap())
                {
                    conf.image_ratio.0 = f.value();
                }
            }
            true
        });

        // Image Ratio Height
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.image_ratio_height.handle(move |f, ev| {
            if ev == Event::KeyUp {
                if let Some(conf) = configs
                    .borrow_mut()
                    .get_mut(&browse.selected_text().unwrap())
                {
                    conf.image_ratio.1 = f.value();
                }
            }
            true
        });

        // Translucent Layer RGB
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.translucent_layer_rgb.set_callback(move |mut f| {
            if let Some(conf) = configs
                .borrow_mut()
                .get_mut(&browse.selected_text().unwrap())
            {
                let (r, g, b) = dialog::color_chooser_with_default(
                    "Pick a colour",
                    dialog::ColorMode::Byte,
                    (
                        conf.color_layer[0],
                        conf.color_layer[1],
                        conf.color_layer[2],
                    ),
                );
                conf.color_layer = [r, g, b, conf.color_layer[3]];
                utils::set_color_btn_rgba(conf.color_layer, &mut f);
                f.redraw();
            }
        });

        // Translucent Layer Opacity
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.translucent_layer_alpha.handle(move |f, ev| {
            if ev == Event::KeyUp {
                if f.value() > 255.0 {
                    f.set_value(255.0);
                } else if f.value() < 0.0 {
                    f.set_value(0.0);
                }

                if let Some(conf) = configs
                    .borrow_mut()
                    .get_mut(&browse.selected_text().unwrap())
                {
                    conf.color_layer[3] = f.value() as u8;
                }
            }
            true
        });

        // Png Image format
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.png_format.set_callback(move |_| {
            if let Some(conf) = configs
                .borrow_mut()
                .get_mut(&browse.selected_text().unwrap())
            {
                conf.image_format = "png".to_owned();
            }
        });

        // Jpeg Image format
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        self.jpeg_format.set_callback(move |_| {
            if let Some(conf) = configs
                .borrow_mut()
                .get_mut(&browse.selected_text().unwrap())
            {
                conf.image_format = "jpeg".to_owned();
            }
        });

        // Reset to default configuation button
        let mut quote_font = self.quote_font.clone();
        let mut subquote_font = self.subquote_font.clone();
        let mut subquote2_font = self.subquote2_font.clone();
        let mut tag_font = self.tag_font.clone();
        let mut tag2_font = self.tag2_font.clone();
        let mut quote_font_ratio = self.quote_font_ratio.clone();
        let mut subquote_font_ratio = self.subquote_font_ratio.clone();
        let mut subquote2_font_ratio = self.subquote2_font_ratio.clone();
        let mut tag_font_ratio = self.tag_font_ratio.clone();
        let mut tag2_font_ratio = self.tag2_font_ratio.clone();
        let mut quote_position_ratio = self.quote_position_ratio.clone();
        let mut subquote_position_ratio = self.subquote_position_ratio.clone();
        let mut subquote2_position_ratio = self.subquote2_position_ratio.clone();
        let mut tag_position_ratio = self.tag_position_ratio.clone();
        let mut tag2_position_ratio = self.tag2_position_ratio.clone();
        let mut image_ratio_width = self.image_ratio_width.clone();
        let mut image_ratio_height = self.image_ratio_height.clone();
        let mut layer_rgb = self.translucent_layer_rgb.clone();
        let mut layer_alpha = self.translucent_layer_alpha.clone();
        let mut png_format = self.png_format.clone();
        let mut jpeg_format = self.jpeg_format.clone();
        let configs = Rc::clone(&self.configs);
        let browse = self.browse.clone();
        self.defaults_btn.set_callback(move |_| {
            let conf = ConfigFile::default();
            quote_font.set_value(&conf.quote_font);
            subquote_font.set_value(&conf.subquote_font);
            subquote2_font.set_value(&conf.subquote2_font);
            tag_font.set_value(&conf.tag_font);
            tag2_font.set_value(&conf.tag2_font);
            quote_font_ratio.set_value(conf.quote_font_ratio);
            subquote_font_ratio.set_value(conf.subquote_font_ratio);
            subquote2_font_ratio.set_value(conf.subquote2_font_ratio);
            tag_font_ratio.set_value(conf.tag_font_ratio);
            tag2_font_ratio.set_value(conf.tag2_font_ratio);
            quote_position_ratio.set_value(conf.quote_position_ratio);
            subquote_position_ratio.set_value(conf.subquote_position_ratio);
            subquote2_position_ratio.set_value(conf.subquote2_position_ratio);
            tag_position_ratio.set_value(conf.tag_position_ratio);
            tag2_position_ratio.set_value(conf.tag2_position_ratio);
            image_ratio_width.set_value(conf.image_ratio.0);
            image_ratio_height.set_value(conf.image_ratio.1);
            utils::set_color_btn_rgba(conf.color_layer, &mut layer_rgb);
            layer_rgb.redraw();
            layer_alpha.set_value(conf.color_layer[3] as f64);
            png_format.set_value(true);
            jpeg_format.set_value(false);
            configs
                .borrow_mut()
                .insert(browse.selected_text().unwrap(), conf);
        });

        // Save Button
        let configs = Rc::clone(&self.configs);
        let did_save = Rc::clone(&self.did_save);
        let mut win = self.win.clone();
        self.save_btn.set_callback(move |_| {
            config::save_configs((*configs.borrow()).clone());

            if let Some(c) = configs.borrow().get(&*globals::CONFIG_NAME.read().unwrap()) {
                *globals::CONFIG.write().unwrap() = c.to_owned();
            }
            *did_save.borrow_mut() = true;
            win.hide();
            dialog::message_default("Re-open Post Maker to see changes properly!")
        });
    }
}
