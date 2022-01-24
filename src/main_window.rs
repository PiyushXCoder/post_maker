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

use crate::crop_window::CropWindow;
use crate::draw_thread::*;
use crate::utils;
use crate::utils::ImageProperties;
use crate::{config_window::ConfigWindow, globals};
use fltk::{
    app,
    button::Button,
    dialog,
    dialog::NativeFileChooser,
    draw as dr, enums,
    enums::Shortcut,
    frame::Frame,
    group::Flex,
    image::SvgImage,
    input::{Input, MultilineInput},
    menu,
    misc::Spinner,
    prelude::*,
    valuator::{Slider, SliderType},
    window::Window,
};
use std::path::PathBuf;
use std::sync::{mpsc, RwLock};
use std::{ffi::OsStr, fs, sync::Arc};

pub(crate) struct MainWindow {
    pub(crate) win: Window,
    pub(crate) menubar: menu::SysMenuBar,
    pub(crate) back_btn: Button,
    pub(crate) next_btn: Button,
    pub(crate) save_btn: Button,
    pub(crate) file_choice: menu::Choice,
    pub(crate) quote: MultilineInput,
    pub(crate) subquote: MultilineInput,
    pub(crate) subquote2: MultilineInput,
    pub(crate) tag: Input,
    pub(crate) tag2: Input,
    pub(crate) layer_rgb: Button,
    pub(crate) layer_alpha: Spinner,
    pub(crate) quote_position: Spinner,
    pub(crate) subquote_position: Spinner,
    pub(crate) subquote2_position: Spinner,
    pub(crate) tag_position: Spinner,
    pub(crate) tag2_position: Spinner,
    pub(crate) quote_position_slider: Slider,
    pub(crate) subquote_position_slider: Slider,
    pub(crate) subquote2_position_slider: Slider,
    pub(crate) tag_position_slider: Slider,
    pub(crate) tag2_position_slider: Slider,
    pub(crate) reset_darklayer_btn: Button,
    pub(crate) reset_quote_position_btn: Button,
    pub(crate) reset_subquote_position_btn: Button,
    pub(crate) reset_subquote2_position_btn: Button,
    pub(crate) reset_tag_position_btn: Button,
    pub(crate) reset_tag2_position_btn: Button,
    pub(crate) reset_file_choice: Button,
    pub(crate) crop_btn: Button,
    pub(crate) clone_btn: Button,
    pub(crate) delete_btn: Button,
    pub(crate) status: Frame,
    pub(crate) count: Frame,
    pub(crate) dimension: Frame,
    pub(crate) page: Page,
    pub(crate) images_path: Arc<RwLock<Vec<PathBuf>>>,
    pub(crate) draw_buff: Arc<RwLock<Option<Vec<u8>>>>,
    pub(crate) properties: Arc<RwLock<ImageProperties>>,
    pub(crate) sender: mpsc::Sender<DrawMessage>,
}

#[derive(Clone)]
pub(crate) struct Page {
    pub(crate) image: Frame,
    pub(crate) row_flex: Flex,
    pub(crate) col_flex: Flex,
}

impl MainWindow {
    pub(crate) fn new(
        sender: app::Sender<crate::AppMessage>,
        draw_buff: Arc<RwLock<Option<Vec<u8>>>>,
    ) -> Self {
        let mut win = Window::new(0, 0, 1100, 700, "Post Maker").center_screen();
        win.set_icon(Some(
            SvgImage::from_data(globals::ICON.to_str().unwrap()).unwrap(),
        ));

        let reload_image = SvgImage::from_data(globals::RELOAD_ICON.to_str().unwrap()).unwrap();

        let mut main_flex = Flex::default().size_of_parent().column();
        let menubar = menu::SysMenuBar::default();
        main_flex.set_size(&menubar, 30);

        let mut toolbar_flex = Flex::default().row();
        let back_btn = Button::default().with_label("Back");
        toolbar_flex.set_size(&back_btn, 50);
        let next_btn = Button::default().with_label("Next");
        toolbar_flex.set_size(&next_btn, 50);
        let save_btn = Button::default().with_label("Save");
        toolbar_flex.set_size(&save_btn, 50);
        let file_choice = menu::Choice::default();
        let mut reset_file_choice = Button::default();
        reset_file_choice.set_image(Some(reload_image.clone()));
        toolbar_flex.set_size(&reset_file_choice, 30);
        toolbar_flex.end();
        main_flex.set_size(&toolbar_flex, 30);

        let mut workspace_flex = Flex::default().row();
        // Controls Left
        let mut left_controls_flex = Flex::default().column();
        left_controls_flex.set_size(
            &Frame::default()
                .with_label("Quote:")
                .with_align(enums::Align::Left | enums::Align::Inside),
            25,
        );
        let quote = MultilineInput::default();
        left_controls_flex.set_size(&quote, 70);

        left_controls_flex.set_size(
            &Frame::default()
                .with_label("Subquote:")
                .with_align(enums::Align::Left | enums::Align::Inside),
            25,
        );
        let subquote = MultilineInput::default();
        left_controls_flex.set_size(&subquote, 70);

        left_controls_flex.set_size(
            &Frame::default()
                .with_label("Subquote 2:")
                .with_align(enums::Align::Left | enums::Align::Inside),
            25,
        );
        let subquote2 = MultilineInput::default();
        left_controls_flex.set_size(&subquote2, 70);

        left_controls_flex.set_size(
            &Frame::default()
                .with_label("Tag:")
                .with_align(enums::Align::Left | enums::Align::Inside),
            25,
        );
        let tag = Input::default();
        left_controls_flex.set_size(&tag, 30);

        left_controls_flex.set_size(
            &Frame::default()
                .with_label("Tag 2:")
                .with_align(enums::Align::Left | enums::Align::Inside),
            25,
        );
        let tag2 = Input::default();
        left_controls_flex.set_size(&tag2, 30);

        let mut actions_flex = Flex::default().row();
        Frame::default();
        let delete_btn = Button::default().with_label("Delete");
        actions_flex.set_size(&delete_btn, 100);
        let clone_btn = Button::default().with_label("Clone");
        actions_flex.set_size(&clone_btn, 100);
        let crop_btn = Button::default().with_label("Crop");
        actions_flex.set_size(&crop_btn, 100);
        Frame::default();
        actions_flex.end();
        left_controls_flex.set_size(&actions_flex, 30);

        Frame::default();

        let info_flex = Flex::default().row();
        let count = Frame::default().with_align(enums::Align::Left | enums::Align::Inside);
        let status = Frame::default();
        let dimension = Frame::default().with_align(enums::Align::Right | enums::Align::Inside);
        info_flex.end();
        left_controls_flex.set_size(&info_flex, 30);

        left_controls_flex.end();
        workspace_flex.set_size(&left_controls_flex, 310);

        // Page
        let mut center_row_flex = Flex::default().row();
        Frame::default();
        let mut center_col_flex = Flex::default().column();
        Frame::default();
        let img_view = Frame::default();
        center_col_flex.set_size(&img_view, 500);
        Frame::default();
        center_col_flex.end();
        Frame::default();
        center_row_flex.set_size(&center_col_flex, 400);
        center_row_flex.end();

        // Controls right
        let mut right_controls_flex = Flex::default().column();
        let mut darklayer_head_flex = Flex::default().row();
        Frame::default()
            .with_label("Dark Layer (RGBA):")
            .with_align(enums::Align::Left | enums::Align::Inside);
        let mut reset_darklayer_btn = Button::default();
        reset_darklayer_btn.set_image(Some(reload_image.clone()));
        darklayer_head_flex.set_size(&reset_darklayer_btn, 30);
        darklayer_head_flex.end();
        right_controls_flex.set_size(&darklayer_head_flex, 30);

        let mut darklayer_flex = Flex::default().row();
        darklayer_flex.set_pad(2);
        darklayer_flex.set_size(&Frame::default().with_label("Colour"), 50);
        let mut layer_rgb = Button::default();
        layer_rgb.set_frame(enums::FrameType::BorderBox);

        darklayer_flex.set_size(&Frame::default().with_label("Alpha"), 50);
        let mut layer_alpha = Spinner::default();
        layer_alpha.set_range(0.0, 255.0);
        darklayer_flex.end();
        right_controls_flex.set_size(&darklayer_flex, 30);

        let mut quote_position_flex = Flex::default().row();
        quote_position_flex.set_size(
            &Frame::default()
                .with_label("Quote Position")
                .with_align(enums::Align::Left | enums::Align::Inside),
            140,
        );
        let quote_position = fltk::misc::Spinner::default();
        let mut reset_quote_position_btn = Button::default();
        reset_quote_position_btn.set_image(Some(reload_image.clone()));
        quote_position_flex.set_size(&reset_quote_position_btn, 30);
        quote_position_flex.end();
        right_controls_flex.set_size(&quote_position_flex, 30);

        let mut quote_position_slider = Slider::default().with_type(SliderType::HorizontalNice);
        quote_position_slider.set_step(1.0, 1);
        quote_position_slider.set_frame(enums::FrameType::NoBox);
        right_controls_flex.set_size(&quote_position_slider, 30);

        let mut subquote_position_flex = Flex::default().row();
        subquote_position_flex.set_size(
            &Frame::default()
                .with_label("Subquote Position")
                .with_align(enums::Align::Left | enums::Align::Inside),
            140,
        );
        let subquote_position = fltk::misc::Spinner::default();
        let mut reset_subquote_position_btn = Button::default();
        reset_subquote_position_btn.set_image(Some(reload_image.clone()));
        subquote_position_flex.set_size(&reset_subquote_position_btn, 30);
        subquote_position_flex.end();
        right_controls_flex.set_size(&subquote_position_flex, 30);

        let mut subquote_position_slider = Slider::default().with_type(SliderType::HorizontalNice);
        subquote_position_slider.set_step(1.0, 1);
        subquote_position_slider.set_frame(enums::FrameType::NoBox);
        right_controls_flex.set_size(&subquote_position_slider, 30);

        let mut subquote2_position_flex = Flex::default().row();
        subquote2_position_flex.set_size(
            &Frame::default()
                .with_label("Subquote 2 Position")
                .with_align(enums::Align::Left | enums::Align::Inside),
            140,
        );
        let subquote2_position = fltk::misc::Spinner::default();
        let mut reset_subquote2_position_btn = Button::default();
        reset_subquote2_position_btn.set_image(Some(reload_image.clone()));
        subquote2_position_flex.set_size(&reset_subquote2_position_btn, 30);
        subquote2_position_flex.end();
        right_controls_flex.set_size(&subquote2_position_flex, 30);

        let mut subquote2_position_slider = Slider::default().with_type(SliderType::HorizontalNice);
        subquote2_position_slider.set_step(1.0, 1);
        subquote2_position_slider.set_frame(enums::FrameType::NoBox);
        right_controls_flex.set_size(&subquote2_position_slider, 30);

        let mut tag_position_flex = Flex::default().row();
        tag_position_flex.set_size(
            &Frame::default()
                .with_label("Tag Position")
                .with_align(enums::Align::Left | enums::Align::Inside),
            140,
        );
        let tag_position = fltk::misc::Spinner::default();
        let mut reset_tag_position_btn = Button::default();
        reset_tag_position_btn.set_image(Some(reload_image.clone()));
        tag_position_flex.set_size(&reset_tag_position_btn, 30);
        tag_position_flex.end();
        right_controls_flex.set_size(&tag_position_flex, 30);

        let mut tag_position_slider = Slider::default().with_type(SliderType::HorizontalNice);
        tag_position_slider.set_step(1.0, 1);
        tag_position_slider.set_frame(enums::FrameType::NoBox);
        right_controls_flex.set_size(&tag_position_slider, 30);

        let mut tag2_position_flex = Flex::default().row();
        tag2_position_flex.set_size(
            &Frame::default()
                .with_label("Tag 2 Position")
                .with_align(enums::Align::Left | enums::Align::Inside),
            140,
        );
        let tag2_position = fltk::misc::Spinner::default();
        let mut reset_tag2_position_btn = Button::default();
        reset_tag2_position_btn.set_image(Some(reload_image.clone()));
        tag2_position_flex.set_size(&reset_tag2_position_btn, 30);
        tag2_position_flex.end();
        right_controls_flex.set_size(&tag2_position_flex, 30);

        let mut tag2_position_slider = Slider::default().with_type(SliderType::HorizontalNice);
        tag2_position_slider.set_step(1.0, 1);
        tag2_position_slider.set_frame(enums::FrameType::NoBox);
        right_controls_flex.set_size(&tag2_position_slider, 30);

        Frame::default();
        right_controls_flex.end();
        workspace_flex.set_size(&right_controls_flex, 270);

        workspace_flex.end();

        main_flex.end();

        win.end();
        win.make_resizable(true);
        win.show();

        let properties = Arc::new(RwLock::new(ImageProperties::new()));
        let (rx, tx) = std::sync::mpsc::channel();
        let mut main_win = Self {
            win,
            menubar,
            back_btn,
            next_btn,
            save_btn,
            file_choice,
            quote,
            subquote,
            subquote2,
            tag,
            tag2,
            layer_rgb,
            layer_alpha,
            quote_position,
            subquote_position,
            subquote2_position,
            tag_position,
            tag2_position,
            quote_position_slider,
            subquote_position_slider,
            subquote2_position_slider,
            tag_position_slider,
            tag2_position_slider,
            reset_darklayer_btn,
            reset_quote_position_btn,
            reset_subquote_position_btn,
            reset_subquote2_position_btn,
            reset_tag_position_btn,
            reset_tag2_position_btn,
            reset_file_choice,
            crop_btn,
            clone_btn,
            delete_btn,
            status,
            count,
            dimension,
            images_path: Arc::new(RwLock::new(vec![])),
            draw_buff,
            properties: Arc::clone(&properties),
            page: Page {
                image: img_view,
                row_flex: center_row_flex,
                col_flex: center_col_flex,
            },
            sender: rx,
        };
        spawn_image_thread(tx, sender, Arc::clone(&properties), &main_win);
        main_win.menu();
        main_win.draw();
        main_win.events();
        main_win
    }

    fn menu(&mut self) {
        let mut file_choice = self.file_choice.clone();
        let sender = self.sender.clone();
        let imgs = Arc::clone(&self.images_path);
        self.menubar.add(
            "&File/Open Folder...\t",
            Shortcut::Ctrl | 'o',
            menu::MenuFlag::Normal,
            move |_| {
                let mut chooser = NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseDir);
                chooser.set_option(fltk::dialog::FileDialogOptions::NewFolder);
                chooser.show();
                let path = chooser.filename();
                let path = fs::canonicalize(&path).unwrap_or(path);
                if !path.exists() {
                    return;
                }
                let expost_dir = path.join("export");
                if !expost_dir.exists() {
                    if let Err(e) = fs::create_dir(expost_dir) {
                        fltk::dialog::alert_default("Failed to create export folder!");
                        warn!("Failed to create export folder!\n{:?}", e);
                        return;
                    }
                }
                load_dir(&path, Arc::clone(&imgs), &mut file_choice, &sender);
            },
        );

        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.menubar.add(
            "&File/Save...\t",
            Shortcut::Ctrl | 's',
            menu::MenuFlag::Normal,
            move |_| {
                let mut prop = properties.write().unwrap();
                prop.is_saved = true;
                sender.send(DrawMessage::Save).unwrap();
            },
        );

        let mut config_window = ConfigWindow::new();
        let sender = self.sender.clone();
        let mut image = self.page.image.clone();
        self.menubar.add(
            "&Edit/Configure...\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                if config_window.show() {
                    sender.send(DrawMessage::RedrawToBuffer).unwrap();
                    sender.send(DrawMessage::Flush).unwrap();
                    image.redraw();
                }
            },
        );

        self.menubar.add(
            "&Help/About...\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                dialog::alert_default(
                    "Created with <3 by PiyushXCoder\nhttps://github.com/PiyushXCoder",
                );
            },
        );
    }

    fn draw(&mut self) {
        let buff = Arc::clone(&self.draw_buff);
        let properties = Arc::clone(&self.properties);
        self.page.image.draw(move |f| {
            let (width, height) = properties.read().unwrap().dimension;
            if let Some(image) = &*buff.read().unwrap() {
                dr::draw_image(
                    &image,
                    f.x(),
                    f.y(),
                    width as i32,
                    height as i32,
                    enums::ColorDepth::Rgb8,
                )
                .unwrap();
            }
        })
    }

    fn events(&mut self) {
        let mut file_choice = self.file_choice.clone();
        let sender = self.sender.clone();
        let imgs = Arc::clone(&self.images_path);
        self.reset_file_choice.set_callback(move |_| {
            let path = match imgs.read().unwrap().first() {
                Some(path) => path.parent().unwrap().to_path_buf(),
                None => return,
            };
            load_dir(&path, Arc::clone(&imgs), &mut file_choice, &sender);
        });

        let mut layer_rgb = self.layer_rgb.clone();
        let mut layer_alpha = self.layer_alpha.clone();
        let mut image = self.page.image.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.reset_darklayer_btn.set_callback(move |_| {
            let mut prop = properties.write().unwrap();
            let color = globals::CONFIG.read().unwrap().color_layer;
            prop.rgba = color;
            prop.is_saved = false;
            utils::set_color_btn_rgba(color, &mut layer_rgb);
            layer_alpha.set_value(color[3] as f64);
            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut quote_position = self.quote_position.clone();
        let mut quote_position_slider = self.quote_position_slider.clone();
        let mut image = self.page.image.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.reset_quote_position_btn.set_callback(move |_| {
            let mut prop = properties.write().unwrap();
            let height = prop.original_dimension.1;
            let pos = height * globals::CONFIG.read().unwrap().quote_position_ratio;
            prop.quote_position = pos;
            prop.is_saved = false;
            quote_position.set_value(pos);
            quote_position_slider.set_value(pos);

            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut subquote_position = self.subquote_position.clone();
        let mut subquote_position_slider = self.subquote_position_slider.clone();
        let mut image = self.page.image.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.reset_subquote_position_btn.set_callback(move |_| {
            let mut prop = properties.write().unwrap();
            let height = prop.original_dimension.1;
            let pos = height * globals::CONFIG.read().unwrap().subquote_position_ratio;
            prop.subquote_position = pos;
            prop.is_saved = false;
            subquote_position.set_value(pos);
            subquote_position_slider.set_value(pos);

            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut subquote2_position = self.subquote2_position.clone();
        let mut subquote2_position_slider = self.subquote2_position_slider.clone();
        let mut image = self.page.image.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.reset_subquote2_position_btn.set_callback(move |_| {
            let mut prop = properties.write().unwrap();
            let height = prop.original_dimension.1;
            let pos = height * globals::CONFIG.read().unwrap().subquote2_position_ratio;
            prop.subquote2_position = pos;
            prop.is_saved = false;
            subquote2_position.set_value(pos);
            subquote2_position_slider.set_value(pos);

            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut tag_position = self.tag_position.clone();
        let mut tag_position_slider = self.tag_position_slider.clone();
        let mut image = self.page.image.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.reset_tag_position_btn.set_callback(move |_| {
            let mut prop = properties.write().unwrap();
            let height = prop.original_dimension.1;
            let pos = height * globals::CONFIG.read().unwrap().tag_position_ratio;
            prop.tag_position = pos;
            prop.is_saved = false;
            tag_position.set_value(pos);
            tag_position_slider.set_value(pos);

            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut tag2_position = self.tag2_position.clone();
        let mut tag2_position_slider = self.tag2_position_slider.clone();
        let mut image = self.page.image.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.reset_tag2_position_btn.set_callback(move |_| {
            let mut prop = properties.write().unwrap();
            let height = prop.original_dimension.1;
            let pos = height * globals::CONFIG.read().unwrap().tag2_position_ratio;
            prop.tag2_position = pos;
            prop.is_saved = false;
            tag2_position.set_value(pos);
            tag2_position_slider.set_value(pos);

            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.save_btn.set_callback(move |_| {
            let mut prop = properties.write().unwrap();
            prop.is_saved = true;
            sender.send(DrawMessage::Save).unwrap()
        });

        let mut image = self.page.image.clone();
        let mut file_choice = self.file_choice.clone();
        let sender = self.sender.clone();
        self.clone_btn.set_callback(move |_| {
            let ch = dialog::choice_default("Do you want to clone??", "Yes", "No", "");
            if ch == 0 {
                sender.send(DrawMessage::Clone).unwrap();
                sender.send(DrawMessage::Open).unwrap();
                image.redraw();
                file_choice.redraw();
            }
        });

        let mut image = self.page.image.clone();
        let mut file_choice = self.file_choice.clone();
        let sender = self.sender.clone();
        self.delete_btn.set_callback(move |_| {
            let ch = dialog::choice_default("Do you want to delete??", "Yes", "No", "");
            if ch == 0 {
                sender.send(DrawMessage::Delete).unwrap();
                sender.send(DrawMessage::Open).unwrap();
                image.redraw();
                file_choice.redraw();
            }
        });

        let properties = Arc::clone(&self.properties);
        let mut crop_win = CropWindow::new();
        let sender = self.sender.clone();
        self.crop_btn.set_callback(move |_| {
            let mut prop = properties.write().unwrap();
            if let Some(path) = &prop.path {
                if let Some((x, y)) = crop_win.load_to_crop(path, prop.crop_position) {
                    sender.send(DrawMessage::ChangeCrop((x, y))).unwrap();
                    prop.is_saved = false;
                }
            }
        });

        let mut file_choice = self.file_choice.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.next_btn.set_callback(move |_| {
            let prop = properties.read().unwrap();
            if !prop.is_saved {
                let save = fltk::dialog::choice_default("Save?", "yes", "no", "cancel");
                match save {
                    0 => sender.send(DrawMessage::Save).unwrap(),
                    1 => {}
                    _ => return,
                }
            }

            if file_choice.value() == file_choice.size() - 2 {
                file_choice.set_value(0);
            } else {
                file_choice.set_value(file_choice.value() + 1);
            }
            sender.send(DrawMessage::Open).unwrap();
        });

        let mut file_choice = self.file_choice.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.back_btn.set_callback(move |_| {
            let prop = properties.read().unwrap();
            if !prop.is_saved {
                let save = fltk::dialog::choice_default("Save?", "yes", "no", "cancel");
                match save {
                    0 => sender.send(DrawMessage::Save).unwrap(),
                    1 => {}
                    _ => return,
                }
            }

            if file_choice.value() == 0 {
                file_choice.set_value(file_choice.size() - 2);
            } else {
                file_choice.set_value(file_choice.value() - 1);
            }
            sender.send(DrawMessage::Open).unwrap();
        });

        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.file_choice.set_callback(move |_| {
            let prop = properties.read().unwrap();
            if !prop.is_saved {
                let save = fltk::dialog::choice_default("Save?", "yes", "no", "cancel");
                match save {
                    0 => sender.send(DrawMessage::Save).unwrap(),
                    1 => {}
                    _ => return,
                }
            }
            sender.send(DrawMessage::Open).unwrap();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.quote.handle(move |f, ev| {
            if ev == enums::Event::KeyUp {
                let mut prop = properties.write().unwrap();
                prop.quote = f.value();
                prop.is_saved = false;
                sender.send(DrawMessage::RedrawToBuffer).unwrap();
                sender.send(DrawMessage::Flush).unwrap();
                image.redraw();
            }
            true
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.subquote.handle(move |f, ev| {
            if ev == enums::Event::KeyUp {
                let mut prop = properties.write().unwrap();
                prop.subquote = f.value();
                prop.is_saved = false;
                sender.send(DrawMessage::RedrawToBuffer).unwrap();
                sender.send(DrawMessage::Flush).unwrap();
                image.redraw();
            }
            true
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.subquote2.handle(move |f, ev| {
            if ev == enums::Event::KeyUp {
                let mut prop = properties.write().unwrap();
                prop.subquote2 = f.value();
                prop.is_saved = false;
                sender.send(DrawMessage::RedrawToBuffer).unwrap();
                sender.send(DrawMessage::Flush).unwrap();
                image.redraw();
            }
            true
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.tag.handle(move |f, ev| {
            if ev == enums::Event::KeyUp {
                let mut prop = properties.write().unwrap();
                prop.tag = f.value();
                prop.is_saved = false;
                sender.send(DrawMessage::RedrawToBuffer).unwrap();
                sender.send(DrawMessage::Flush).unwrap();
                image.redraw();
            }
            true
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.tag2.handle(move |f, ev| {
            if ev == enums::Event::KeyUp {
                let mut prop = properties.write().unwrap();
                prop.tag2 = f.value();
                prop.is_saved = false;
                sender.send(DrawMessage::RedrawToBuffer).unwrap();
                sender.send(DrawMessage::Flush).unwrap();
                image.redraw();
            }
            true
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut quote_position_slider = self.quote_position_slider.clone();
        self.quote_position.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.quote_position = f.value();
            quote_position_slider.set_value(f.value());
            prop.is_saved = false;
            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut quote_position = self.quote_position.clone();
        self.quote_position_slider.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.quote_position = f.value();
            quote_position.set_value(f.value());
            prop.is_saved = false;
            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut subquote_position_slider = self.subquote_position_slider.clone();
        self.subquote_position.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.subquote_position = f.value();
            subquote_position_slider.set_value(f.value());
            prop.is_saved = false;
            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut subquote_position = self.subquote_position.clone();
        self.subquote_position_slider.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.subquote_position = f.value();
            subquote_position.set_value(f.value());
            prop.is_saved = false;
            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut subquote2_position_slider = self.subquote2_position_slider.clone();
        self.subquote2_position.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.subquote2_position = f.value();
            subquote2_position_slider.set_value(f.value());
            prop.is_saved = false;
            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut subquote2_position = self.subquote2_position.clone();
        self.subquote2_position_slider.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.subquote2_position = f.value();
            subquote2_position.set_value(f.value());
            prop.is_saved = false;
            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut tag_position_slider = self.tag_position_slider.clone();
        self.tag_position.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.tag_position = f.value();
            tag_position_slider.set_value(f.value());
            prop.is_saved = false;
            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut tag_position = self.tag_position.clone();
        self.tag_position_slider.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.tag_position = f.value();
            tag_position.set_value(f.value());
            prop.is_saved = false;
            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut tag2_position_slider = self.tag2_position_slider.clone();
        self.tag2_position.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.tag2_position = f.value();
            tag2_position_slider.set_value(f.value());
            prop.is_saved = false;
            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut tag2_position = self.tag2_position.clone();
        self.tag2_position_slider.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.tag2_position = f.value();
            tag2_position.set_value(f.value());
            prop.is_saved = false;
            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.layer_rgb.set_callback(move |mut f| {
            let mut prop = properties.write().unwrap();
            let (r, g, b) = dialog::color_chooser_with_default(
                "Pick a colour",
                dialog::ColorMode::Byte,
                (prop.rgba[0], prop.rgba[1], prop.rgba[2]),
            );
            prop.rgba = [r, g, b, prop.rgba[3]];
            utils::set_color_btn_rgba(prop.rgba, &mut f);
            f.redraw();
            prop.is_saved = false;
            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.layer_alpha.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.rgba[3] = f.value() as u8;
            prop.is_saved = false;
            sender.send(DrawMessage::RedrawToBuffer).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });
    }
}

fn load_dir(
    path: &PathBuf,
    imgs: Arc<RwLock<Vec<PathBuf>>>,
    file_choice: &mut menu::Choice,
    sender: &mpsc::Sender<DrawMessage>,
) {
    let mut files = fs::read_dir(path)
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<fs::DirEntry>>();
    files.sort_by_key(|i| i.file_name());
    let mut text = String::new();
    let mut imgs_b = imgs.write().unwrap();
    *imgs_b = vec![];
    for file in files {
        let path = file.path();
        if path.extension() == Some(OsStr::new("jpg"))
            || path.extension() == Some(OsStr::new("png"))
        {
            text = format!("{}|{}", text, path.file_name().unwrap().to_str().unwrap());
            imgs_b.push(path);
        }
    }
    if text.len() == 0 {
        return;
    }
    file_choice.clear();
    file_choice.add_choice(&text[1..]);
    file_choice.set_value(0);
    sender.send(DrawMessage::Open).unwrap();
}
