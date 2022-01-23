//! Window to edit configuration

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use fltk::{
    app,
    browser::{Browser, BrowserType},
    button::Button,
    dialog::{self, FileDialogOptions, NativeFileChooser},
    enums::{Align, Font},
    frame::Frame,
    group::Flex,
    image::SvgImage,
    misc::Spinner,
    output::Output,
    prelude::*,
    valuator::ValueInput,
    window::Window,
};

use crate::{
    config::{self, ConfigFile},
    globals,
};

pub(crate) struct ConfigWindow {
    pub(crate) win: Window,
    pub(crate) browse: Browser,
    pub(crate) selected_browse_line: Rc<RefCell<i32>>,
    pub(crate) add_config_btn: Button,
    pub(crate) del_config_btn: Button,
    pub(crate) quote_font_ttf: Output,
    pub(crate) quote_font_ttf_browse: Button,
    pub(crate) subquote_font_ttf: Output,
    pub(crate) subquote_font_ttf_browse: Button,
    pub(crate) tag_font_ttf: Output,
    pub(crate) tag_font_ttf_browse: Button,
    pub(crate) quote_font_ratio: ValueInput,
    pub(crate) subquote_font_ratio: ValueInput,
    pub(crate) tag_font_ratio: ValueInput,
    pub(crate) quote_position_ratio: ValueInput,
    pub(crate) subquote_position_ratio: ValueInput,
    pub(crate) tag_position_ratio: ValueInput,
    pub(crate) image_ratio_width: ValueInput,
    pub(crate) image_ratio_height: ValueInput,
    pub(crate) layer_red: Spinner,
    pub(crate) layer_green: Spinner,
    pub(crate) layer_blue: Spinner,
    pub(crate) layer_alpha: Spinner,
    pub(crate) defaults_btn: Button,
    pub(crate) save_btn: Button,
    pub(crate) cancel_btn: Button,
    pub(crate) configs: Rc<RefCell<HashMap<String, ConfigFile>>>,
    did_save: Rc<RefCell<bool>>,
}

impl ConfigWindow {
    pub(crate) fn new() -> Self {
        let configs = config::get_configs().unwrap_or(HashMap::new());
        let mut win = Window::new(0, 0, 730, 530, "Config").center_screen();
        win.set_icon(Some(
            SvgImage::from_data(globals::ICON.to_str().unwrap()).unwrap(),
        ));
        let mut row = Flex::default().with_size(720, 520).with_pos(5, 5).row();
        let mut config_picker_flex = Flex::default().column();
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
        row.set_size(&config_picker_flex, 200);

        let mut col = Flex::default().column();

        let mut quote_font_ttf_grp = Flex::default().row();
        quote_font_ttf_grp.set_size(
            &Frame::default()
                .with_label("Font for quote (ttf)")
                .with_align(Align::Right | Align::Inside),
            190,
        );
        let quote_font_ttf = Output::default();
        let quote_font_ttf_browse = Button::default().with_label("Pick");
        quote_font_ttf_grp.set_size(&quote_font_ttf_browse, 50);
        quote_font_ttf_grp.end();
        col.set_size(&quote_font_ttf_grp, 30);

        let mut subquote_font_ttf_grp = Flex::default().row();
        subquote_font_ttf_grp.set_size(
            &Frame::default()
                .with_label("Font for Subquote (ttf)")
                .with_align(Align::Right | Align::Inside),
            190,
        );
        let subquote_font_ttf = Output::default();
        let subquote_font_ttf_browse = Button::default().with_label("Pick");
        subquote_font_ttf_grp.set_size(&subquote_font_ttf_browse, 50);
        subquote_font_ttf_grp.end();
        col.set_size(&subquote_font_ttf_grp, 30);

        let mut tag_font_ttf_grp = Flex::default().row();
        tag_font_ttf_grp.set_size(
            &Frame::default()
                .with_label("Font for tag (ttf)")
                .with_align(Align::Right | Align::Inside),
            190,
        );
        let tag_font_ttf = Output::default();
        let tag_font_ttf_browse = Button::default().with_label("Pick");
        tag_font_ttf_grp.set_size(&tag_font_ttf_browse, 50);
        tag_font_ttf_grp.end();
        col.set_size(&tag_font_ttf_grp, 30);

        let mut quote_font_ratio_grp = Flex::default().row();
        quote_font_ratio_grp.set_size(
            &Frame::default()
                .with_label("Quote text size ratio")
                .with_align(Align::Right | Align::Inside),
            190,
        );
        let quote_font_ratio = ValueInput::default();
        quote_font_ratio_grp.end();
        col.set_size(&quote_font_ratio_grp, 30);

        let mut grp = Flex::default().row();
        grp.set_size(&Frame::default(), 190);
        let mut hint = Frame::default()
            .with_label("Font size in image of resolution 4000x5000")
            .with_align(Align::Left | Align::Inside);
        hint.set_label_font(Font::CourierItalic);
        hint.set_label_size(12);
        grp.end();
        col.set_size(&grp, 13);

        let mut subquote_font_ratio_grp = Flex::default().row();
        subquote_font_ratio_grp.set_size(
            &Frame::default()
                .with_label("Subquote text size ratio")
                .with_align(Align::Right | Align::Inside),
            190,
        );
        let subquote_font_ratio = ValueInput::default();
        subquote_font_ratio_grp.end();
        col.set_size(&subquote_font_ratio_grp, 30);

        let mut grp = Flex::default().row();
        grp.set_size(&Frame::default(), 190);
        let mut hint = Frame::default()
            .with_label("Font size in image of resolution 4000x5000")
            .with_align(Align::Left | Align::Inside);
        hint.set_label_font(Font::CourierItalic);
        hint.set_label_size(12);
        grp.end();
        col.set_size(&grp, 13);

        let mut tag_font_ratio_grp = Flex::default().row();
        tag_font_ratio_grp.set_size(
            &Frame::default()
                .with_label("Tag text size ratio")
                .with_align(Align::Right | Align::Inside),
            190,
        );
        let tag_font_ratio = ValueInput::default();
        tag_font_ratio_grp.end();
        col.set_size(&tag_font_ratio_grp, 30);

        let mut grp = Flex::default().row();
        grp.set_size(&Frame::default(), 190);
        let mut hint = Frame::default()
            .with_label("Font size in image of resolution 4000x5000")
            .with_align(Align::Left | Align::Inside);
        hint.set_label_font(Font::CourierItalic);
        hint.set_label_size(12);
        grp.end();
        col.set_size(&grp, 13);

        let mut quote_position_ratio_grp = Flex::default().row();
        quote_position_ratio_grp.set_size(
            &Frame::default()
                .with_label("Quote text position ratio")
                .with_align(Align::Right | Align::Inside),
            190,
        );
        let quote_position_ratio = ValueInput::default();
        quote_position_ratio_grp.end();
        col.set_size(&quote_position_ratio_grp, 30);

        let mut subquote_position_ratio_grp = Flex::default().row();
        subquote_position_ratio_grp.set_size(
            &Frame::default()
                .with_label("Subquote text position ratio")
                .with_align(Align::Right | Align::Inside),
            190,
        );
        let subquote_position_ratio = ValueInput::default();
        subquote_position_ratio_grp.end();
        col.set_size(&subquote_position_ratio_grp, 30);

        let mut tag_position_ratio_grp = Flex::default().row();
        tag_position_ratio_grp.set_size(
            &Frame::default()
                .with_label("Tag text position ratio")
                .with_align(Align::Right | Align::Inside),
            190,
        );
        let tag_position_ratio = ValueInput::default();
        tag_position_ratio_grp.end();
        col.set_size(&tag_position_ratio_grp, 30);

        let mut image_ratio_grp = Flex::default().row();
        image_ratio_grp.set_size(
            &Frame::default()
                .with_label("Image size ratio")
                .with_align(Align::Right | Align::Inside),
            190,
        );
        let image_ratio_width = ValueInput::default();
        image_ratio_grp.set_size(&Frame::default().with_label("x"), 30);
        let image_ratio_height = ValueInput::default();
        image_ratio_grp.end();
        col.set_size(&image_ratio_grp, 30);

        col.set_size(
            &Frame::default().with_label("Default colour shader to use with new images:"),
            30,
        );

        let mut darklayer_grp = Flex::default().row();
        darklayer_grp.set_pad(2);
        darklayer_grp.set_size(&Frame::default().with_label("Red"), 30);
        let mut layer_red = Spinner::default();
        layer_red.set_range(0.0, 255.0);
        // darklayer_flex.set_size(&layer_red, 50);
        darklayer_grp.set_size(&Frame::default().with_label("Green"), 40);
        let mut layer_green = Spinner::default();
        layer_green.set_range(0.0, 255.0);
        // darklayer_flex.set_size(&layer_green, 50);
        darklayer_grp.set_size(&Frame::default().with_label("Blue"), 30);
        let mut layer_blue = Spinner::default();
        layer_blue.set_range(0.0, 255.0);
        // darklayer_flex.set_size(&layer_blue, 50);
        darklayer_grp.set_size(&Frame::default().with_label("Alpha"), 40);
        let mut layer_alpha = Spinner::default();
        layer_alpha.set_range(0.0, 255.0);
        // darklayer_flex.set_size(&layer_alpha, 50);
        darklayer_grp.end();
        col.set_size(&darklayer_grp, 30);

        Frame::default();

        let mut panel_grp = Flex::default().row();
        Frame::default();
        let defaults_btn = Button::default().with_label("Defaults");
        let save_btn = Button::default().with_label("Save");
        let cancel_btn = Button::default().with_label("Cancel");
        panel_grp.set_size(&defaults_btn, 100);
        panel_grp.set_size(&save_btn, 100);
        panel_grp.set_size(&cancel_btn, 100);
        panel_grp.end();

        col.set_size(&panel_grp, 30);

        col.end();
        row.end();
        win.end();
        win.make_modal(true);
        win.make_resizable(true);

        let mut config_window = Self {
            win,
            browse,
            selected_browse_line: Rc::new(RefCell::new(0)),
            add_config_btn,
            del_config_btn,
            quote_font_ttf,
            quote_font_ttf_browse,
            subquote_font_ttf,
            subquote_font_ttf_browse,
            tag_font_ttf,
            tag_font_ttf_browse,
            quote_font_ratio,
            subquote_font_ratio,
            tag_font_ratio,
            quote_position_ratio,
            subquote_position_ratio,
            tag_position_ratio,
            image_ratio_width,
            image_ratio_height,
            layer_red,
            layer_green,
            layer_blue,
            layer_alpha,
            defaults_btn,
            save_btn,
            cancel_btn,
            configs: Rc::new(RefCell::new(configs)),
            did_save: Rc::new(RefCell::new(false)),
        };
        config_window.event();

        config_window
    }

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
        self.quote_font_ttf
            .set_value(config.quote_font_ttf.as_str());
        self.subquote_font_ttf
            .set_value(config.subquote_font_ttf.as_str());
        self.tag_font_ttf.set_value(config.tag_font_ttf.as_str());
        self.quote_font_ratio.set_value(config.quote_font_ratio);
        self.subquote_font_ratio
            .set_value(config.subquote_font_ratio);
        self.tag_font_ratio.set_value(config.tag_font_ratio);
        self.quote_position_ratio
            .set_value(config.quote_position_ratio);
        self.subquote_position_ratio
            .set_value(config.subquote_position_ratio);
        self.tag_position_ratio.set_value(config.tag_position_ratio);
        self.image_ratio_width.set_value(config.image_ratio.0);
        self.image_ratio_height.set_value(config.image_ratio.1);
        self.layer_red.set_value(config.color_layer[0] as f64);
        self.layer_green.set_value(config.color_layer[1] as f64);
        self.layer_blue.set_value(config.color_layer[2] as f64);
        self.layer_alpha.set_value(config.color_layer[3] as f64);
        *self.did_save.borrow_mut() = false;
        drop(config);
        self.win.show();
        while self.win.shown() {
            app::wait();
        }
        *self.did_save.borrow()
    }

    fn event(&mut self) {
        let mut quote_font_ttf = self.quote_font_ttf.clone();
        let mut subquote_font_ttf = self.subquote_font_ttf.clone();
        let mut tag_font_ttf = self.tag_font_ttf.clone();
        let mut quote_font_ratio = self.quote_font_ratio.clone();
        let mut subquote_font_ratio = self.subquote_font_ratio.clone();
        let mut tag_font_ratio = self.tag_font_ratio.clone();
        let mut quote_position_ratio = self.quote_position_ratio.clone();
        let mut subquote_position_ratio = self.subquote_position_ratio.clone();
        let mut tag_position_ratio = self.tag_position_ratio.clone();
        let mut image_ratio_width = self.image_ratio_width.clone();
        let mut image_ratio_height = self.image_ratio_height.clone();
        let mut layer_red = self.layer_red.clone();
        let mut layer_green = self.layer_green.clone();
        let mut layer_blue = self.layer_blue.clone();
        let mut layer_alpha = self.layer_alpha.clone();
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

            if let Some(conf) = configs
                .borrow_mut()
                .get_mut(&browse.selected_text().unwrap())
            {
                conf.quote_font_ttf = quote_font_ttf.value();
                conf.subquote_font_ttf = subquote_font_ttf.value();
                conf.tag_font_ttf = tag_font_ttf.value();
                conf.quote_font_ratio = quote_font_ratio.value();
                conf.subquote_font_ratio = subquote_font_ratio.value();
                conf.tag_font_ratio = tag_font_ratio.value();
                conf.quote_position_ratio = quote_position_ratio.value();
                conf.subquote_position_ratio = subquote_position_ratio.value();
                conf.tag_position_ratio = tag_position_ratio.value();
                conf.image_ratio = (image_ratio_width.value(), image_ratio_height.value());
                conf.color_layer = [
                    layer_red.value() as u8,
                    layer_green.value() as u8,
                    layer_blue.value() as u8,
                    layer_alpha.value() as u8,
                ];
            }

            let conf = ConfigFile::default();
            quote_font_ttf.set_value(&conf.quote_font_ttf);
            subquote_font_ttf.set_value(&conf.subquote_font_ttf);
            tag_font_ttf.set_value(&conf.tag_font_ttf);
            quote_font_ratio.set_value(conf.quote_font_ratio);
            subquote_font_ratio.set_value(conf.subquote_font_ratio);
            tag_font_ratio.set_value(conf.tag_font_ratio);
            quote_position_ratio.set_value(conf.quote_position_ratio);
            subquote_position_ratio.set_value(conf.subquote_position_ratio);
            tag_position_ratio.set_value(conf.tag_position_ratio);
            image_ratio_width.set_value(conf.image_ratio.0);
            image_ratio_height.set_value(conf.image_ratio.1);
            layer_red.set_value(conf.color_layer[0] as f64);
            layer_green.set_value(conf.color_layer[1] as f64);
            layer_blue.set_value(conf.color_layer[2] as f64);
            layer_alpha.set_value(conf.color_layer[3] as f64);
            browse.add(&name);
            configs.borrow_mut().insert(name.clone(), conf);
            browse.select(browse.size());
            *selected_browse_line.borrow_mut() = browse.value();
        });

        let mut quote_font_ttf = self.quote_font_ttf.clone();
        let mut subquote_font_ttf = self.subquote_font_ttf.clone();
        let mut tag_font_ttf = self.tag_font_ttf.clone();
        let mut quote_font_ratio = self.quote_font_ratio.clone();
        let mut subquote_font_ratio = self.subquote_font_ratio.clone();
        let mut tag_font_ratio = self.tag_font_ratio.clone();
        let mut quote_position_ratio = self.quote_position_ratio.clone();
        let mut subquote_position_ratio = self.subquote_position_ratio.clone();
        let mut tag_position_ratio = self.tag_position_ratio.clone();
        let mut image_ratio_width = self.image_ratio_width.clone();
        let mut image_ratio_height = self.image_ratio_height.clone();
        let mut layer_red = self.layer_red.clone();
        let mut layer_green = self.layer_green.clone();
        let mut layer_blue = self.layer_blue.clone();
        let mut layer_alpha = self.layer_alpha.clone();
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
                quote_font_ttf.set_value(&conf.quote_font_ttf);
                subquote_font_ttf.set_value(&conf.subquote_font_ttf);
                tag_font_ttf.set_value(&conf.tag_font_ttf);
                quote_font_ratio.set_value(conf.quote_font_ratio);
                subquote_font_ratio.set_value(conf.subquote_font_ratio);
                tag_font_ratio.set_value(conf.tag_font_ratio);
                quote_position_ratio.set_value(conf.quote_position_ratio);
                subquote_position_ratio.set_value(conf.subquote_position_ratio);
                tag_position_ratio.set_value(conf.tag_position_ratio);
                image_ratio_width.set_value(conf.image_ratio.0);
                image_ratio_height.set_value(conf.image_ratio.1);
                layer_red.set_value(conf.color_layer[0] as f64);
                layer_green.set_value(conf.color_layer[1] as f64);
                layer_blue.set_value(conf.color_layer[2] as f64);
                layer_alpha.set_value(conf.color_layer[3] as f64);
            }
        });

        let mut quote_font_ttf = self.quote_font_ttf.clone();
        let mut subquote_font_ttf = self.subquote_font_ttf.clone();
        let mut tag_font_ttf = self.tag_font_ttf.clone();
        let mut quote_font_ratio = self.quote_font_ratio.clone();
        let mut subquote_font_ratio = self.subquote_font_ratio.clone();
        let mut tag_font_ratio = self.tag_font_ratio.clone();
        let mut quote_position_ratio = self.quote_position_ratio.clone();
        let mut subquote_position_ratio = self.subquote_position_ratio.clone();
        let mut tag_position_ratio = self.tag_position_ratio.clone();
        let mut image_ratio_width = self.image_ratio_width.clone();
        let mut image_ratio_height = self.image_ratio_height.clone();
        let mut layer_red = self.layer_red.clone();
        let mut layer_green = self.layer_green.clone();
        let mut layer_blue = self.layer_blue.clone();
        let mut layer_alpha = self.layer_alpha.clone();
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

            if let Some(conf) = configs
                .borrow_mut()
                .get_mut(&f.text(*selected_browse_line.borrow()).unwrap())
            {
                conf.quote_font_ttf = quote_font_ttf.value();
                conf.subquote_font_ttf = subquote_font_ttf.value();
                conf.tag_font_ttf = tag_font_ttf.value();
                conf.quote_font_ratio = quote_font_ratio.value();
                conf.subquote_font_ratio = subquote_font_ratio.value();
                conf.tag_font_ratio = tag_font_ratio.value();
                conf.quote_position_ratio = quote_position_ratio.value();
                conf.subquote_position_ratio = subquote_position_ratio.value();
                conf.tag_position_ratio = tag_position_ratio.value();
                conf.image_ratio = (image_ratio_width.value(), image_ratio_height.value());
                conf.color_layer = [
                    layer_red.value() as u8,
                    layer_green.value() as u8,
                    layer_blue.value() as u8,
                    layer_alpha.value() as u8,
                ];
            }

            if let Some(conf) = configs.borrow().get(&f.selected_text().unwrap()) {
                quote_font_ttf.set_value(&conf.quote_font_ttf);
                subquote_font_ttf.set_value(&conf.subquote_font_ttf);
                tag_font_ttf.set_value(&conf.tag_font_ttf);
                quote_font_ratio.set_value(conf.quote_font_ratio);
                subquote_font_ratio.set_value(conf.subquote_font_ratio);
                tag_font_ratio.set_value(conf.tag_font_ratio);
                quote_position_ratio.set_value(conf.quote_position_ratio);
                subquote_position_ratio.set_value(conf.subquote_position_ratio);
                tag_position_ratio.set_value(conf.tag_position_ratio);
                image_ratio_width.set_value(conf.image_ratio.0);
                image_ratio_height.set_value(conf.image_ratio.1);
                layer_red.set_value(conf.color_layer[0] as f64);
                layer_green.set_value(conf.color_layer[1] as f64);
                layer_blue.set_value(conf.color_layer[2] as f64);
                layer_alpha.set_value(conf.color_layer[3] as f64);
            }
            *selected_browse_line.borrow_mut() = f.value();
            // println!("browse {:?}", selected_config_name);
        });

        let mut quote_font_ttf = self.quote_font_ttf.clone();
        self.quote_font_ttf_browse.set_callback(move |_| {
            let mut chooser = NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseFile);
            chooser.set_option(FileDialogOptions::UseFilterExt);
            chooser.set_filter("*.ttf");
            chooser.show();
            let path = chooser.filename();
            let path = std::fs::canonicalize(&path).unwrap_or(path);
            quote_font_ttf.set_value(path.to_str().unwrap());
        });

        let mut subquote_font_ttf = self.subquote_font_ttf.clone();
        self.subquote_font_ttf_browse.set_callback(move |_| {
            let mut chooser = NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseFile);
            chooser.set_option(FileDialogOptions::UseFilterExt);
            chooser.set_filter("*.ttf");
            chooser.show();
            let path = chooser.filename();
            let path = std::fs::canonicalize(&path).unwrap_or(path);
            subquote_font_ttf.set_value(path.to_str().unwrap());
        });

        let mut tag_font_ttf = self.tag_font_ttf.clone();
        self.tag_font_ttf_browse.set_callback(move |_| {
            let mut chooser = NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseFile);
            chooser.set_option(FileDialogOptions::UseFilterExt);
            chooser.set_filter("*.ttf");
            chooser.show();
            let path = chooser.filename();
            tag_font_ttf.set_value(path.to_str().unwrap());
        });

        let mut win = self.win.clone();
        self.cancel_btn.set_callback(move |_| {
            win.hide();
        });

        let mut quote_font_ttf = self.quote_font_ttf.clone();
        let mut subquote_font_ttf = self.subquote_font_ttf.clone();
        let mut tag_font_ttf = self.tag_font_ttf.clone();
        let mut quote_font_ratio = self.quote_font_ratio.clone();
        let mut subquote_font_ratio = self.subquote_font_ratio.clone();
        let mut tag_font_ratio = self.tag_font_ratio.clone();
        let mut quote_position_ratio = self.quote_position_ratio.clone();
        let mut subquote_position_ratio = self.subquote_position_ratio.clone();
        let mut tag_position_ratio = self.tag_position_ratio.clone();
        let mut image_ratio_width = self.image_ratio_width.clone();
        let mut image_ratio_height = self.image_ratio_height.clone();
        let mut layer_red = self.layer_red.clone();
        let mut layer_green = self.layer_green.clone();
        let mut layer_blue = self.layer_blue.clone();
        let mut layer_alpha = self.layer_alpha.clone();
        self.defaults_btn.set_callback(move |_| {
            let conf = ConfigFile::default();
            quote_font_ttf.set_value(&conf.quote_font_ttf);
            subquote_font_ttf.set_value(&conf.subquote_font_ttf);
            tag_font_ttf.set_value(&conf.tag_font_ttf);
            quote_font_ratio.set_value(conf.quote_font_ratio);
            subquote_font_ratio.set_value(conf.subquote_font_ratio);
            tag_font_ratio.set_value(conf.tag_font_ratio);
            quote_position_ratio.set_value(conf.quote_position_ratio);
            subquote_position_ratio.set_value(conf.subquote_position_ratio);
            tag_position_ratio.set_value(conf.tag_position_ratio);
            image_ratio_width.set_value(conf.image_ratio.0);
            image_ratio_height.set_value(conf.image_ratio.1);
            layer_red.set_value(conf.color_layer[0] as f64);
            layer_green.set_value(conf.color_layer[1] as f64);
            layer_blue.set_value(conf.color_layer[2] as f64);
            layer_alpha.set_value(conf.color_layer[3] as f64);
        });

        let quote_font_ttf = self.quote_font_ttf.clone();
        let subquote_font_ttf = self.subquote_font_ttf.clone();
        let tag_font_ttf = self.tag_font_ttf.clone();
        let quote_font_ratio = self.quote_font_ratio.clone();
        let subquote_font_ratio = self.subquote_font_ratio.clone();
        let tag_font_ratio = self.tag_font_ratio.clone();
        let quote_position_ratio = self.quote_position_ratio.clone();
        let subquote_position_ratio = self.subquote_position_ratio.clone();
        let tag_position_ratio = self.tag_position_ratio.clone();
        let image_ratio_width = self.image_ratio_width.clone();
        let image_ratio_height = self.image_ratio_height.clone();
        let layer_red = self.layer_red.clone();
        let layer_green = self.layer_green.clone();
        let layer_blue = self.layer_blue.clone();
        let layer_alpha = self.layer_alpha.clone();
        let browse = self.browse.clone();
        let configs = Rc::clone(&self.configs);
        let did_save = Rc::clone(&self.did_save);
        let mut win = self.win.clone();
        self.save_btn.set_callback(move |_| {
            if let Some(conf) = configs
                .borrow_mut()
                .get_mut(&browse.selected_text().unwrap())
            {
                conf.quote_font_ttf = quote_font_ttf.value();
                conf.subquote_font_ttf = subquote_font_ttf.value();
                conf.tag_font_ttf = tag_font_ttf.value();
                conf.quote_font_ratio = quote_font_ratio.value();
                conf.subquote_font_ratio = subquote_font_ratio.value();
                conf.tag_font_ratio = tag_font_ratio.value();
                conf.quote_position_ratio = quote_position_ratio.value();
                conf.subquote_position_ratio = subquote_position_ratio.value();
                conf.tag_position_ratio = tag_position_ratio.value();
                conf.image_ratio = (image_ratio_width.value(), image_ratio_height.value());
                conf.color_layer = [
                    layer_red.value() as u8,
                    layer_green.value() as u8,
                    layer_blue.value() as u8,
                    layer_alpha.value() as u8,
                ];
            }

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
