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

//! Main window where you do all editing
use crate::{
    about_window::About,
    config_window::ConfigWindow,
    crop_window::CropWindow,
    dialog,
    draw_thread::*,
    export_all_window::ExportAllWindow,
    globals,
    result_ext::ResultExt,
    utils::{self, ImageInfo, ImageProperties, ImageType},
};
use fltk::{
    button::Button,
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
use std::{
    ffi::OsStr,
    fs,
    path::PathBuf,
    process::Command,
    sync::Arc,
    sync::{mpsc, RwLock},
};

pub(crate) struct MainWindow {
    pub(crate) win: Window,
    pub(crate) menubar: menu::SysMenuBar,
    pub(crate) back_btn: Button,
    pub(crate) next_btn: Button,
    pub(crate) save_btn: Button,
    /// To choose the file which is being edited in directory
    pub(crate) file_choice: menu::Choice,
    pub(crate) quote: MultilineInput,
    pub(crate) subquote: MultilineInput,
    pub(crate) subquote2: MultilineInput,
    pub(crate) tag: Input,
    pub(crate) tag2: Input,
    /// RGB value of top translucent layer
    pub(crate) translucent_layer_rgb: Button,
    /// opacity value of top translucent layer
    pub(crate) translucent_layer_alpha: Spinner,
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
    pub(crate) reset_translucent_layer_btn: Button,
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
    pub(crate) images_list: Arc<RwLock<Vec<ImageInfo>>>,
    pub(crate) draw_buff: Arc<RwLock<Option<Vec<u8>>>>,
    pub(crate) properties: Arc<RwLock<ImageProperties>>,
    pub(crate) sender: mpsc::Sender<DrawMessage>,
}

/// Contains the elements to draw page in mid of workspace
#[derive(Clone)]
pub(crate) struct Page {
    pub(crate) image: Frame,
    pub(crate) row_flex: Flex,
    pub(crate) col_flex: Flex,
}

impl MainWindow {
    pub(crate) fn new(draw_buff: Arc<RwLock<Option<Vec<u8>>>>) -> Self {
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
        let mut translucent_layer_head_flex = Flex::default().row();
        Frame::default()
            .with_label("Translucent Layer:")
            .with_align(enums::Align::Left | enums::Align::Inside);
        let mut reset_darklayer_btn = Button::default();
        reset_darklayer_btn.set_image(Some(reload_image.clone()));
        translucent_layer_head_flex.set_size(&reset_darklayer_btn, 30);
        translucent_layer_head_flex.end();
        right_controls_flex.set_size(&translucent_layer_head_flex, 30);

        let mut translucent_layer_flex = Flex::default().row();
        translucent_layer_flex.set_pad(2);
        translucent_layer_flex.set_size(&Frame::default().with_label("Colour"), 50);
        let mut translucent_layer_rgb = Button::default();
        translucent_layer_rgb.set_frame(enums::FrameType::BorderBox);

        translucent_layer_flex.set_size(&Frame::default().with_label("Alpha"), 50);
        let mut translucent_layer_alpha = Spinner::default();
        translucent_layer_alpha.set_range(0.0, 255.0);
        translucent_layer_flex.end();
        right_controls_flex.set_size(&translucent_layer_flex, 30);

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

        let properties = Arc::new(RwLock::new(ImageProperties::default()));
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
            translucent_layer_rgb,
            translucent_layer_alpha,
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
            reset_translucent_layer_btn: reset_darklayer_btn,
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
            images_list: Arc::new(RwLock::new(vec![])),
            draw_buff,
            properties: Arc::clone(&properties),
            page: Page {
                image: img_view,
                row_flex: center_row_flex,
                col_flex: center_col_flex,
            },
            sender: rx,
        };

        if let Some(a) = &*rw_read!(globals::MAIN_SENDER) {
            spawn_image_thread(tx, a.to_owned(), Arc::clone(&properties), &main_win);
        }
        main_win.menu();
        main_win.draw();
        main_win.events();
        main_win
    }

    /// Set menubar in window
    fn menu(&mut self) {
        let mut file_choice = self.file_choice.clone();
        let sender = self.sender.clone();
        let imgs = Arc::clone(&self.images_list);
        let mut win = self.win.clone();
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
                win.set_label(&format!(
                    "{} - Post Maker",
                    path.file_name()
                        .unwrap_or(OsStr::new("Unknown"))
                        .to_string_lossy()
                ));
                load_dir(&path, Arc::clone(&imgs), &mut file_choice, &sender);
            },
        );

        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.menubar.add(
            "&File/Save Image...\t",
            Shortcut::Ctrl | 's',
            menu::MenuFlag::Normal,
            move |_| {
                let mut prop = rw_write!(properties);
                prop.is_saved = true;
                sender.send_it(DrawMessage::Save);
            },
        );

        let sender = self.sender.clone();
        self.menubar.add(
            "&Actions/Show Details...\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                sender.send_it(DrawMessage::ShowImagesDetails);
            },
        );

        let properties = Arc::clone(&self.properties);
        self.menubar.add(
            "&Actions/Open Exports Folder...\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                let props = rw_read!(properties);
                if let Some(prop) = &props.image_info {
                    let export = prop.path.parent().unwrap().join("export");
                    if export.exists() {
                        if cfg!(windows) {
                            Command::new("explorer")
                                .arg(export.to_str().unwrap_or_default())
                                .spawn()
                                .warn_log("Failed top spawn command");
                        } else if cfg!(unix) {
                            Command::new("xdg-open")
                                .arg(export.to_str().unwrap_or_default())
                                .spawn()
                                .warn_log("Failed top spawn command");
                        } else if cfg!(macos) {
                            Command::new("open")
                                .arg(export.to_str().unwrap_or_default())
                                .spawn()
                                .warn_log("Failed top spawn command");
                        } else {
                            dialog::alert_default("Unknown Operating System")
                        }
                    }
                }
            },
        );

        let mut win = self.win.clone();
        let mut export_all = ExportAllWindow::new(Arc::clone(&self.images_list));
        self.menubar.add(
            "&Actions/Export All with Quotes...\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                export_all.export();
                win.redraw();
                fltk::app::awake();
            },
        );

        let properties = Arc::clone(&self.properties);
        let mut win = self.win.clone();
        self.menubar.add(
            "&Actions/Delete Exports...\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                win.deactivate();
                if dialog::choice_default("Do you want to remove exports?", "Yes", "No") == 0 {
                    let props = rw_read!(properties);
                    if let Some(prop) = &props.image_info {
                        let export = prop.path.parent().unwrap().join("export");
                        if export.exists() {
                            fs::remove_dir_all(&export)
                                .warn_log("Failed to remove export directory");
                        }
                    }
                }
                win.activate();
                win.redraw();
                fltk::app::awake();
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
                    sender.send_it(DrawMessage::RedrawToBuffer);
                    sender.send_it(DrawMessage::Flush);
                    image.redraw();
                }
            },
        );

        let mut about_win = About::new();
        self.menubar.add(
            "&Help/About...\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                about_win.show();
            },
        );
    }

    /// Set drawing in window
    fn draw(&mut self) {
        let buff = Arc::clone(&self.draw_buff);
        let properties = Arc::clone(&self.properties);
        self.page.image.draw(move |f| {
            let (width, height) = rw_read!(properties).dimension;
            if let Some(image) = &*rw_read!(buff) {
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

    /// Set callbacks of elements
    fn events(&mut self) {
        // Resest Button for FileChoice
        let mut file_choice = self.file_choice.clone();
        let sender = self.sender.clone();
        let imgs = Arc::clone(&self.images_list);
        self.reset_file_choice.set_callback(move |_| {
            let path = match rw_read!(imgs).first() {
                Some(image_info) => image_info.path.parent().unwrap().to_path_buf(),
                None => return,
            };
            load_dir(&path, Arc::clone(&imgs), &mut file_choice, &sender);
        });

        // Reset Button for Translucent Layer
        let mut layer_rgb = self.translucent_layer_rgb.clone();
        let mut layer_alpha = self.translucent_layer_alpha.clone();
        let mut image = self.page.image.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.reset_translucent_layer_btn.set_callback(move |_| {
            let mut prop = rw_write!(properties);
            let color = rw_read!(globals::CONFIG).color_layer;
            prop.translucent_layer_color = color;
            prop.is_saved = false;
            utils::set_color_btn_rgba(color, &mut layer_rgb);
            layer_alpha.set_value(color[3] as f64);
            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Reset Button for Quote Input
        let mut quote_position = self.quote_position.clone();
        let mut quote_position_slider = self.quote_position_slider.clone();
        let mut image = self.page.image.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.reset_quote_position_btn.set_callback(move |_| {
            let mut prop = rw_write!(properties);
            let height = prop.original_dimension.1;
            let pos = height * rw_read!(globals::CONFIG).quote_position_ratio;
            prop.quote_position = pos;
            prop.is_saved = false;
            quote_position.set_value(pos);
            quote_position_slider.set_value(pos);

            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Reset Button for Subquote Input
        let mut subquote_position = self.subquote_position.clone();
        let mut subquote_position_slider = self.subquote_position_slider.clone();
        let mut image = self.page.image.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.reset_subquote_position_btn.set_callback(move |_| {
            let mut prop = rw_write!(properties);
            let height = prop.original_dimension.1;
            let pos = height * rw_read!(globals::CONFIG).subquote_position_ratio;
            prop.subquote_position = pos;
            prop.is_saved = false;
            subquote_position.set_value(pos);
            subquote_position_slider.set_value(pos);

            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Reset Button for Subquotes2 Input
        let mut subquote2_position = self.subquote2_position.clone();
        let mut subquote2_position_slider = self.subquote2_position_slider.clone();
        let mut image = self.page.image.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.reset_subquote2_position_btn.set_callback(move |_| {
            let mut prop = rw_write!(properties);
            let height = prop.original_dimension.1;
            let pos = height * rw_read!(globals::CONFIG).subquote2_position_ratio;
            prop.subquote2_position = pos;
            prop.is_saved = false;
            subquote2_position.set_value(pos);
            subquote2_position_slider.set_value(pos);

            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Reset Button for Tag Input
        let mut tag_position = self.tag_position.clone();
        let mut tag_position_slider = self.tag_position_slider.clone();
        let mut image = self.page.image.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.reset_tag_position_btn.set_callback(move |_| {
            let mut prop = rw_write!(properties);
            let height = prop.original_dimension.1;
            let pos = height * rw_read!(globals::CONFIG).tag_position_ratio;
            prop.tag_position = pos;
            prop.is_saved = false;
            tag_position.set_value(pos);
            tag_position_slider.set_value(pos);

            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Reset Button for Tag2 Input
        let mut tag2_position = self.tag2_position.clone();
        let mut tag2_position_slider = self.tag2_position_slider.clone();
        let mut image = self.page.image.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.reset_tag2_position_btn.set_callback(move |_| {
            let mut prop = rw_write!(properties);
            let height = prop.original_dimension.1;
            let pos = height * rw_read!(globals::CONFIG).tag2_position_ratio;
            prop.tag2_position = pos;
            prop.is_saved = false;
            tag2_position.set_value(pos);
            tag2_position_slider.set_value(pos);

            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Save Button
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.save_btn.set_callback(move |_| {
            let mut prop = rw_write!(properties);
            prop.is_saved = true;
            sender.send_it(DrawMessage::Save);
        });

        // Clone Button
        let mut image = self.page.image.clone();
        let mut file_choice = self.file_choice.clone();
        let sender = self.sender.clone();
        self.clone_btn.set_callback(move |_| {
            let ch = dialog::choice_default("Do you want to clone??", "Yes", "No");
            if ch == 0 {
                sender.send_it(DrawMessage::Clone);
                sender.send_it(DrawMessage::Open);
                sender.send_it(DrawMessage::CheckImage);
                image.redraw();
                file_choice.redraw();
            }
        });

        // Delete Button
        let mut image = self.page.image.clone();
        let mut file_choice = self.file_choice.clone();
        let sender = self.sender.clone();
        self.delete_btn.set_callback(move |_| {
            let ch = dialog::choice_default("Do you want to delete??", "Yes", "No");
            if ch == 0 {
                sender.send_it(DrawMessage::Delete);
                sender.send_it(DrawMessage::Open);
                sender.send_it(DrawMessage::CheckImage);
                image.redraw();
                file_choice.redraw();
            }
        });

        // Crop Button
        let properties = Arc::clone(&self.properties);
        let mut crop_win = CropWindow::new();
        let sender = self.sender.clone();
        self.crop_btn.set_callback(move |_| {
            let mut prop = rw_write!(properties);
            if let Some(image_info) = &prop.image_info {
                if let Some((x, y)) = crop_win.load_to_crop(&image_info, prop.crop_position) {
                    sender.send_it(DrawMessage::ChangeCrop((x, y)));
                    prop.is_saved = false;
                }
            }
        });

        // Next Image Button
        let mut file_choice = self.file_choice.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.next_btn.set_callback(move |_| {
            let prop = rw_read!(properties);
            if !prop.is_saved {
                let save = fltk::dialog::choice_default("Save?", "yes", "no", "cancel");
                match save {
                    0 => sender.send_it(DrawMessage::Save),
                    1 => {}
                    _ => return,
                }
            }

            if file_choice.value() == file_choice.size() - 2 {
                file_choice.set_value(0);
            } else {
                file_choice.set_value(file_choice.value() + 1);
            }
            sender.send_it(DrawMessage::Open);
            sender.send_it(DrawMessage::CheckImage);
        });

        // Back Image Button
        let mut file_choice = self.file_choice.clone();
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.back_btn.set_callback(move |_| {
            let prop = rw_read!(properties);
            if !prop.is_saved {
                let save = fltk::dialog::choice_default("Save?", "yes", "no", "cancel");
                match save {
                    0 => sender.send_it(DrawMessage::Save),
                    1 => {}
                    _ => return,
                }
            }

            if file_choice.value() == 0 {
                file_choice.set_value(file_choice.size() - 2);
            } else {
                file_choice.set_value(file_choice.value() - 1);
            }
            sender.send_it(DrawMessage::Open);
            sender.send_it(DrawMessage::CheckImage);
        });

        // File Choice
        let sender = self.sender.clone();
        let properties = Arc::clone(&self.properties);
        self.file_choice.set_callback(move |_| {
            let prop = rw_read!(properties);
            if !prop.is_saved {
                let save = fltk::dialog::choice_default("Save?", "yes", "no", "cancel");
                match save {
                    0 => sender.send_it(DrawMessage::Save),
                    1 => {}
                    _ => return,
                }
            }
            sender.send_it(DrawMessage::Open);
            sender.send_it(DrawMessage::CheckImage);
        });

        // Quote Input
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.quote.handle(move |f, ev| {
            if ev == enums::Event::KeyUp {
                let mut prop = rw_write!(properties);
                prop.quote = f.value();
                prop.is_saved = false;
                sender.send_it(DrawMessage::RedrawToBuffer);
                sender.send_it(DrawMessage::Flush);
                image.redraw();
            }
            true
        });

        // Subquote Input
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.subquote.handle(move |f, ev| {
            if ev == enums::Event::KeyUp {
                let mut prop = rw_write!(properties);
                prop.subquote = f.value();
                prop.is_saved = false;
                sender.send_it(DrawMessage::RedrawToBuffer);
                sender.send_it(DrawMessage::Flush);
                image.redraw();
            }
            true
        });

        // Subquote2 Input
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.subquote2.handle(move |f, ev| {
            if ev == enums::Event::KeyUp {
                let mut prop = rw_write!(properties);
                prop.subquote2 = f.value();
                prop.is_saved = false;
                sender.send_it(DrawMessage::RedrawToBuffer);
                sender.send_it(DrawMessage::Flush);
                image.redraw();
            }
            true
        });

        // Tag Input
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.tag.handle(move |f, ev| {
            if ev == enums::Event::KeyUp {
                let mut prop = rw_write!(properties);
                prop.tag = f.value();
                prop.is_saved = false;
                sender.send_it(DrawMessage::RedrawToBuffer);
                sender.send_it(DrawMessage::Flush);
                image.redraw();
            }
            true
        });

        // Tag2 Input
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.tag2.handle(move |f, ev| {
            if ev == enums::Event::KeyUp {
                let mut prop = rw_write!(properties);
                prop.tag2 = f.value();
                prop.is_saved = false;
                sender.send_it(DrawMessage::RedrawToBuffer);
                sender.send_it(DrawMessage::Flush);
                image.redraw();
            }
            true
        });

        // Quote Position Input
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut quote_position_slider = self.quote_position_slider.clone();
        self.quote_position.set_callback(move |f| {
            let mut prop = rw_write!(properties);
            prop.quote_position = f.value();
            quote_position_slider.set_value(f.value());
            prop.is_saved = false;
            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Quote Position Slider
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut quote_position = self.quote_position.clone();
        self.quote_position_slider.set_callback(move |f| {
            let mut prop = rw_write!(properties);
            prop.quote_position = f.value();
            quote_position.set_value(f.value());
            prop.is_saved = false;
            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Subquote Position Input
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut subquote_position_slider = self.subquote_position_slider.clone();
        self.subquote_position.set_callback(move |f| {
            let mut prop = rw_write!(properties);
            prop.subquote_position = f.value();
            subquote_position_slider.set_value(f.value());
            prop.is_saved = false;
            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Subquote Position Slider
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut subquote_position = self.subquote_position.clone();
        self.subquote_position_slider.set_callback(move |f| {
            let mut prop = rw_write!(properties);
            prop.subquote_position = f.value();
            subquote_position.set_value(f.value());
            prop.is_saved = false;
            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Subquote2 Position Input
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut subquote2_position_slider = self.subquote2_position_slider.clone();
        self.subquote2_position.set_callback(move |f| {
            let mut prop = rw_write!(properties);
            prop.subquote2_position = f.value();
            subquote2_position_slider.set_value(f.value());
            prop.is_saved = false;
            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Subquote2 Position Slider
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut subquote2_position = self.subquote2_position.clone();
        self.subquote2_position_slider.set_callback(move |f| {
            let mut prop = rw_write!(properties);
            prop.subquote2_position = f.value();
            subquote2_position.set_value(f.value());
            prop.is_saved = false;
            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Tag Position Input
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut tag_position_slider = self.tag_position_slider.clone();
        self.tag_position.set_callback(move |f| {
            let mut prop = rw_write!(properties);
            prop.tag_position = f.value();
            tag_position_slider.set_value(f.value());
            prop.is_saved = false;
            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Tag Position Slider
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut tag_position = self.tag_position.clone();
        self.tag_position_slider.set_callback(move |f| {
            let mut prop = rw_write!(properties);
            prop.tag_position = f.value();
            tag_position.set_value(f.value());
            prop.is_saved = false;
            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Tag2 Position Input
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut tag2_position_slider = self.tag2_position_slider.clone();
        self.tag2_position.set_callback(move |f| {
            let mut prop = rw_write!(properties);
            prop.tag2_position = f.value();
            tag2_position_slider.set_value(f.value());
            prop.is_saved = false;
            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Tag2 Position Slider
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        let mut tag2_position = self.tag2_position.clone();
        self.tag2_position_slider.set_callback(move |f| {
            let mut prop = rw_write!(properties);
            prop.tag2_position = f.value();
            tag2_position.set_value(f.value());
            prop.is_saved = false;
            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Translucent Layer RGB
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.translucent_layer_rgb.set_callback(move |mut f| {
            let mut prop = rw_write!(properties);
            let (r, g, b) = dialog::color_chooser_with_default(
                "Pick a colour",
                dialog::ColorMode::Byte,
                (
                    prop.translucent_layer_color[0],
                    prop.translucent_layer_color[1],
                    prop.translucent_layer_color[2],
                ),
            );
            prop.translucent_layer_color = [r, g, b, prop.translucent_layer_color[3]];
            utils::set_color_btn_rgba(prop.translucent_layer_color, &mut f);
            f.redraw();
            prop.is_saved = false;
            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });

        // Translucent Layer Opacity
        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.translucent_layer_alpha.set_callback(move |f| {
            let mut prop = rw_write!(properties);
            prop.translucent_layer_color[3] = f.value() as u8;
            prop.is_saved = false;
            sender.send_it(DrawMessage::RedrawToBuffer);
            sender.send_it(DrawMessage::Flush);
            image.redraw();
        });
    }
}

/// Load all iamges in a directory
fn load_dir(
    path: &PathBuf,
    imgs: Arc<RwLock<Vec<ImageInfo>>>,
    file_choice: &mut menu::Choice,
    sender: &mpsc::Sender<DrawMessage>,
) {
    let mut files = fs::read_dir(path)
        .unwrap()
        .map(|r| r.unwrap())
        .collect::<Vec<fs::DirEntry>>();
    files.sort_by_key(|i| i.file_name());
    let mut text = String::new();
    let mut imgs_b = rw_write!(imgs);
    *imgs_b = vec![];
    for file in files {
        let path = file.path();
        if let Ok(Some(ty)) = infer::get_from_path(&path) {
            let mime = ty.mime_type();
            match ImageType::from_mime(mime) {
                ImageType::None => (),
                _ => {
                    text = format!("{}|{}", text, path.file_name().unwrap().to_str().unwrap());
                    imgs_b.push(ImageInfo {
                        path,
                        image_type: ImageType::from_mime(mime),
                    });
                }
            }
        }
    }
    if text.len() == 0 {
        return;
    }
    file_choice.clear();
    file_choice.add_choice(&text[1..]);
    file_choice.set_value(0);
    sender.send_it(DrawMessage::Open);
    sender.send_it(DrawMessage::CheckImage);
}

trait SenderExt {
    fn send_it(&self, a: DrawMessage);
}

impl SenderExt for mpsc::Sender<DrawMessage> {
    fn send_it(&self, a: DrawMessage) {
        self.send(a).expect_log("Program panic!");
    }
}
