use crate::config_window::ConfigWindow;
use crate::crop_window::CropWindow;
use crate::draw_thread::*;
use crate::utils::ImageProperties;
use fltk::{
    app,
    button::Button,
    dialog::NativeFileChooser,
    draw as dr, enums,
    enums::Shortcut,
    frame::Frame,
    group::Flex,
    input::{Input, MultilineInput},
    menu,
    misc::Spinner,
    prelude::*,
    window::Window,
};
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
    pub(crate) tag: Input,
    pub(crate) layer_red: Spinner,
    pub(crate) layer_green: Spinner,
    pub(crate) layer_blue: Spinner,
    pub(crate) layer_alpha: Spinner,
    pub(crate) quote_position: Spinner,
    pub(crate) tag_position: Spinner,
    pub(crate) crop_btn: Button,
    pub(crate) status: Frame,
    pub(crate) page: Page,
    pub(crate) draw_buff: Arc<RwLock<Vec<u8>>>,
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
        draw_buff: Arc<RwLock<Vec<u8>>>,
    ) -> Self {
        let mut win = Window::new(0, 0, 1000, 600, "Post Maker").center_screen();

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
        toolbar_flex.end();
        main_flex.set_size(&toolbar_flex, 30);

        let mut workspace_flex = Flex::default().row();
        // Controls
        let mut controls_flex = Flex::default().column();
        controls_flex.set_size(
            &Frame::default()
                .with_label("Quote:")
                .with_align(enums::Align::Left | enums::Align::Inside),
            20,
        );
        let quote = MultilineInput::default();
        controls_flex.set_size(&quote, 90);
        controls_flex.set_size(
            &Frame::default()
                .with_label("Tag:")
                .with_align(enums::Align::Left | enums::Align::Inside),
            20,
        );
        let tag = Input::default();
        controls_flex.set_size(&tag, 30);
        controls_flex.set_size(
            &Frame::default()
                .with_label("Dark Layer (RGBA):")
                .with_align(enums::Align::Left | enums::Align::Inside),
            20,
        );
        let mut darklayer_flex = Flex::default().row();
        darklayer_flex.set_pad(2);
        darklayer_flex.set_size(&Frame::default().with_label("Red"), 30);
        let mut layer_red = Spinner::default();
        layer_red.set_range(0.0, 255.0);
        darklayer_flex.set_size(&layer_red, 50);
        darklayer_flex.set_size(&Frame::default().with_label("Green"), 40);
        let mut layer_green = Spinner::default();
        layer_green.set_range(0.0, 255.0);
        darklayer_flex.set_size(&layer_green, 50);
        darklayer_flex.set_size(&Frame::default().with_label("Blue"), 30);
        let mut layer_blue = Spinner::default();
        layer_blue.set_range(0.0, 255.0);
        darklayer_flex.set_size(&layer_blue, 50);
        darklayer_flex.set_size(&Frame::default().with_label("Alpha"), 40);
        let mut layer_alpha = Spinner::default();
        layer_alpha.set_range(0.0, 255.0);
        darklayer_flex.set_size(&layer_alpha, 50);
        darklayer_flex.end();
        controls_flex.set_size(&darklayer_flex, 30);

        let quote_position_flex = Flex::default().row();
        Frame::default()
            .with_label("Quote Position:")
            .with_align(enums::Align::Left | enums::Align::Inside);
        let quote_position = fltk::misc::Spinner::default();
        quote_position_flex.end();
        controls_flex.set_size(&quote_position_flex, 30);

        let tag_position_flex = Flex::default().row();
        Frame::default()
            .with_label("Tag Position:")
            .with_align(enums::Align::Left | enums::Align::Inside);
        let tag_position = fltk::misc::Spinner::default();
        tag_position_flex.end();
        controls_flex.set_size(&tag_position_flex, 30);

        let mut actions_flex = Flex::default().row();
        Frame::default();
        let crop_btn = Button::default().with_label("Crop");
        actions_flex.set_size(&crop_btn, 100);
        Frame::default();
        actions_flex.end();
        controls_flex.set_size(&actions_flex, 30);

        Frame::default();

        let status = Frame::default();
        controls_flex.set_size(&status, 30);

        controls_flex.end();
        workspace_flex.set_size(&controls_flex, 360);

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
            tag,
            layer_red,
            layer_green,
            layer_blue,
            layer_alpha,
            quote_position,
            tag_position,
            crop_btn,
            status,
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
        let mut win = self.win.clone();
        self.menubar.add(
            "&File/Open Folder...\t",
            Shortcut::Ctrl | 'o',
            menu::MenuFlag::Normal,
            move |_| {
                win.redraw();
                let mut chooser = NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseDir);
                chooser.set_option(fltk::dialog::FileDialogOptions::NewFolder);
                chooser.show();
                let path = chooser.filename();
                if !path.exists() {
                    win.activate();
                    return;
                }
                let expost_dir = path.join("export");
                if !expost_dir.exists() {
                    if let Err(_) = fs::create_dir(expost_dir) {
                        fltk::dialog::message_default("Failed: Readonly folder!");
                        return;
                    }
                }
                let files = fs::read_dir(&path).unwrap();
                let mut text = String::new();
                for file in files {
                    let file = file.unwrap();
                    let path = file.path();
                    if path.extension() == Some(OsStr::new("jpg")) {
                        text = format!("{}|{}", text, path.to_str().unwrap());
                    }
                }
                if text.len() == 0 {
                    win.activate();
                    return;
                }
                file_choice.clear();
                file_choice.add_choice(&text[1..]);
                file_choice.set_value(0);
                sender.send(DrawMessage::Open).unwrap();
            },
        );

        let sender = self.sender.clone();
        self.menubar.add(
            "&File/Save...\t",
            Shortcut::Ctrl | 's',
            menu::MenuFlag::Normal,
            move |_| sender.send(DrawMessage::Save).unwrap(),
        );

        let mut config_window = ConfigWindow::new();
        let sender = self.sender.clone();
        self.menubar.add(
            "&Edit/Configure...\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            move |_| {
                if config_window.show() {
                    sender.send(DrawMessage::Recalc).unwrap();
                }
            },
        );
    }

    fn draw(&mut self) {
        let buff = Arc::clone(&self.draw_buff);
        let properties = Arc::clone(&self.properties);
        self.page.image.draw(move |f| {
            let (width, height) = properties.read().unwrap().dimension;
            let image = &*buff.read().unwrap();
            dr::draw_image(
                &image,
                f.x(),
                f.y(),
                width as i32,
                height as i32,
                enums::ColorDepth::Rgb8,
            )
            .unwrap();
        })
    }

    fn events(&mut self) {
        let sender = self.sender.clone();
        self.save_btn
            .set_callback(move |_| sender.send(DrawMessage::Save).unwrap());

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
                sender.send(DrawMessage::Recalc).unwrap();
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
                sender.send(DrawMessage::Recalc).unwrap();
                sender.send(DrawMessage::Flush).unwrap();
                image.redraw();
            }
            true
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.quote_position.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.quote_position = f.value();
            prop.is_saved = false;
            sender.send(DrawMessage::Recalc).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.tag_position.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.tag_position = f.value();
            prop.is_saved = false;
            sender.send(DrawMessage::Recalc).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.layer_red.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.rgba[0] = f.value() as u8;
            prop.is_saved = false;
            sender.send(DrawMessage::Recalc).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.layer_green.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.rgba[1] = f.value() as u8;
            prop.is_saved = false;
            sender.send(DrawMessage::Recalc).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });

        let mut image = self.page.image.clone();
        let properties = Arc::clone(&self.properties);
        let sender = self.sender.clone();
        self.layer_blue.set_callback(move |f| {
            let mut prop = properties.write().unwrap();
            prop.rgba[2] = f.value() as u8;
            prop.is_saved = false;
            sender.send(DrawMessage::Recalc).unwrap();
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
            sender.send(DrawMessage::Recalc).unwrap();
            sender.send(DrawMessage::Flush).unwrap();
            image.redraw();
        });
    }
}
