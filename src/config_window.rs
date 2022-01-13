use std::{cell::RefCell, rc::Rc};

use fltk::{
    app,
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

use crate::{config::ConfigFile, globals};

pub(crate) struct ConfigWindow {
    pub(crate) win: Window,
    pub(crate) quote_font_ttf: Output,
    pub(crate) quote_font_ttf_browse: Button,
    pub(crate) tag_font_ttf: Output,
    pub(crate) tag_font_ttf_browse: Button,
    pub(crate) quote_font_ratio: ValueInput,
    pub(crate) tag_font_ratio: ValueInput,
    pub(crate) layer_red: Spinner,
    pub(crate) layer_green: Spinner,
    pub(crate) layer_blue: Spinner,
    pub(crate) layer_alpha: Spinner,
    pub(crate) defaults_btn: Button,
    pub(crate) save_btn: Button,
    pub(crate) cancel_btn: Button,
    did_save: Rc<RefCell<bool>>,
}

impl ConfigWindow {
    pub(crate) fn new() -> Self {
        let mut win = Window::new(0, 0, 500, 300, "Config").center_screen();
        if let Ok(image) = SvgImage::from_data(&globals::ICON) {
            win.set_icon(Some(image));
        }

        let mut col = Flex::default().with_size(490, 290).with_pos(5, 5).column();

        let mut quote_font_ttf_grp = Flex::default().row();
        quote_font_ttf_grp.set_size(
            &Frame::default()
                .with_label("Font for quote (ttf)")
                .with_align(Align::Right | Align::Inside),
            160,
        );
        let quote_font_ttf = Output::default();
        let quote_font_ttf_browse = Button::default().with_label("Pick");
        quote_font_ttf_grp.set_size(&quote_font_ttf_browse, 50);
        quote_font_ttf_grp.end();
        col.set_size(&quote_font_ttf_grp, 30);

        let mut tag_font_ttf_grp = Flex::default().row();
        tag_font_ttf_grp.set_size(
            &Frame::default()
                .with_label("Font for tag (ttf)")
                .with_align(Align::Right | Align::Inside),
            160,
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
            160,
        );
        let quote_font_ratio = ValueInput::default();
        quote_font_ratio_grp.end();
        col.set_size(&quote_font_ratio_grp, 30);

        let mut grp = Flex::default().row();
        grp.set_size(&Frame::default(), 160);
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
            160,
        );
        let tag_font_ratio = ValueInput::default();
        tag_font_ratio_grp.end();
        col.set_size(&tag_font_ratio_grp, 30);

        let mut grp = Flex::default().row();
        grp.set_size(&Frame::default(), 160);
        let mut hint = Frame::default()
            .with_label("Font size in image of resolution 4000x5000")
            .with_align(Align::Left | Align::Inside);
        hint.set_label_font(Font::CourierItalic);
        hint.set_label_size(12);
        grp.end();
        col.set_size(&grp, 13);

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
        win.end();
        win.make_modal(true);
        win.make_resizable(true);

        let mut config_window = Self {
            win,
            quote_font_ttf,
            quote_font_ttf_browse,
            tag_font_ttf,
            tag_font_ttf_browse,
            quote_font_ratio,
            tag_font_ratio,
            layer_red,
            layer_green,
            layer_blue,
            layer_alpha,
            defaults_btn,
            save_btn,
            cancel_btn,
            did_save: Rc::new(RefCell::new(false)),
        };
        config_window.event();

        config_window
    }

    pub(crate) fn show(&mut self) -> bool {
        let glob = globals::CONFIG.read().unwrap();
        self.quote_font_ttf.set_value(glob.quote_font_ttf.as_str());
        self.tag_font_ttf.set_value(glob.tag_font_ttf.as_str());
        self.quote_font_ratio.set_value(glob.quote_font_ratio);
        self.tag_font_ratio.set_value(glob.tag_font_ratio);
        self.layer_red.set_value(glob.color_layer[0] as f64);
        self.layer_green.set_value(glob.color_layer[1] as f64);
        self.layer_blue.set_value(glob.color_layer[2] as f64);
        self.layer_alpha.set_value(glob.color_layer[3] as f64);
        *self.did_save.borrow_mut() = false;
        drop(glob);
        self.win.show();
        while self.win.shown() {
            app::wait();
        }
        *self.did_save.borrow()
    }

    fn event(&mut self) {
        let mut quote_font_ttf = self.quote_font_ttf.clone();
        self.quote_font_ttf_browse.set_callback(move |_| {
            let mut chooser = NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseFile);
            chooser.set_option(FileDialogOptions::UseFilterExt);
            chooser.set_filter("*.ttf");
            chooser.show();
            let path = chooser.filename();
            quote_font_ttf.set_value(path.to_str().unwrap());
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
        let mut tag_font_ttf = self.tag_font_ttf.clone();
        let mut quote_font_ratio = self.quote_font_ratio.clone();
        let mut tag_font_ratio = self.tag_font_ratio.clone();
        let mut layer_red = self.layer_red.clone();
        let mut layer_green = self.layer_green.clone();
        let mut layer_blue = self.layer_blue.clone();
        let mut layer_alpha = self.layer_alpha.clone();
        self.defaults_btn.set_callback(move |_| {
            let conf = ConfigFile::default();
            quote_font_ttf.set_value(&conf.quote_font_ttf);
            tag_font_ttf.set_value(&conf.tag_font_ttf);
            quote_font_ratio.set_value(conf.quote_font_ratio);
            tag_font_ratio.set_value(conf.tag_font_ratio);
            layer_red.set_value(conf.color_layer[0] as f64);
            layer_green.set_value(conf.color_layer[1] as f64);
            layer_blue.set_value(conf.color_layer[2] as f64);
            layer_alpha.set_value(conf.color_layer[3] as f64);
        });

        let mut win = self.win.clone();
        let quote_font_ttf = self.quote_font_ttf.clone();
        let tag_font_ttf = self.tag_font_ttf.clone();
        let quote_font_ratio = self.quote_font_ratio.clone();
        let tag_font_ratio = self.tag_font_ratio.clone();
        let layer_red = self.layer_red.clone();
        let layer_green = self.layer_green.clone();
        let layer_blue = self.layer_blue.clone();
        let layer_alpha = self.layer_alpha.clone();
        let did_save = Rc::clone(&self.did_save);
        self.save_btn.set_callback(move |_| {
            let conf = ConfigFile {
                quote_font_ttf: quote_font_ttf.value(),
                tag_font_ttf: tag_font_ttf.value(),
                quote_font_ratio: quote_font_ratio.value(),
                tag_font_ratio: tag_font_ratio.value(),
                color_layer: [
                    layer_red.value() as u8,
                    layer_green.value() as u8,
                    layer_blue.value() as u8,
                    layer_alpha.value() as u8,
                ],
            };

            conf.save();
            *globals::CONFIG.write().unwrap() = conf;
            *did_save.borrow_mut() = true;
            win.hide();
            dialog::message_default("Re-open Post Maker to see changes!")
        });
    }
}
