use crate::utils::ImageContainer;
use crate::{properties::Properties, utils};
use fltk::{
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
use image::GenericImageView;
use std::io::Read;
use std::{cell::RefCell, ffi::OsStr, fs, path::Path, rc::Rc};

pub(crate) struct MainWindow {
    pub(crate) win: Window,
    menubar: menu::SysMenuBar,
    back_btn: Button,
    next_btn: Button,
    save_btn: Button,
    file_choice: menu::Choice,
    quote: MultilineInput,
    tag: Input,
    layer_red: Spinner,
    layer_green: Spinner,
    layer_blue: Spinner,
    layer_alpha: Spinner,
    quote_position: Spinner,
    tag_position: Spinner,
    crop_btn: Button,
    reset_btn: Button,
    page: Page,
    container: Rc<RefCell<Option<ImageContainer>>>,
}

#[derive(Clone)]
pub(crate) struct Page {
    image: Frame,
    row_flex: Flex,
    col_flex: Flex,
}

impl MainWindow {
    pub(crate) fn new(container: Rc<RefCell<Option<ImageContainer>>>) -> Self {
        let color = [25, 29, 34, 190];

        let mut win = Window::default()
            .with_size(1000, 600)
            .with_label("Post Maker");

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
        layer_red.set_value(color[0] as f64);
        darklayer_flex.set_size(&layer_red, 50);
        darklayer_flex.set_size(&Frame::default().with_label("Green"), 40);
        let mut layer_green = Spinner::default();
        layer_green.set_range(0.0, 255.0);
        layer_green.set_value(color[1] as f64);
        darklayer_flex.set_size(&layer_green, 50);
        darklayer_flex.set_size(&Frame::default().with_label("Blue"), 30);
        let mut layer_blue = Spinner::default();
        layer_blue.set_range(0.0, 255.0);
        layer_blue.set_value(color[2] as f64);
        darklayer_flex.set_size(&layer_blue, 50);
        darklayer_flex.set_size(&Frame::default().with_label("Alpha"), 40);
        let mut layer_alpha = Spinner::default();
        layer_alpha.set_range(0.0, 255.0);
        layer_alpha.set_value(color[3] as f64);
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
        let reset_btn = Button::default().with_label("Reset Values");
        actions_flex.set_size(&reset_btn, 100);
        Frame::default();
        actions_flex.end();
        controls_flex.set_size(&actions_flex, 30);

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
            reset_btn,
            container,
            page: Page {
                image: img_view,
                row_flex: center_row_flex,
                col_flex: center_col_flex,
            },
        };
        main_win.menu();
        main_win.draw();
        main_win.events();
        main_win
    }

    fn menu(&mut self) {
        let mut file_choice = self.file_choice.clone();
        let mut quote = self.quote.clone();
        let mut tag = self.tag.clone();
        let mut layer_red = self.layer_red.clone();
        let mut layer_green = self.layer_green.clone();
        let mut layer_blue = self.layer_blue.clone();
        let mut layer_alpha = self.layer_alpha.clone();
        let mut quote_position = self.quote_position.clone();
        let mut tag_position = self.tag_position.clone();
        let mut page = self.page.clone();
        let container = Rc::clone(&self.container);
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
                if !path.exists() {
                    return;
                }
                let files = fs::read_dir(path).unwrap();
                let mut text = String::new();
                for file in files {
                    let file = file.unwrap();
                    let path = file.path();
                    if path.extension() == Some(OsStr::new("jpg")) {
                        text = format!("{}|{}", text, path.to_str().unwrap());
                    }
                }
                if text.len() == 0 {
                    return;
                }
                file_choice.clear();
                file_choice.add_choice(&text[1..]);
                file_choice.set_value(0);

                load_image(
                    &mut file_choice,
                    &mut quote,
                    &mut tag,
                    &mut layer_red,
                    &mut layer_green,
                    &mut layer_blue,
                    &mut layer_alpha,
                    &mut quote_position,
                    &mut tag_position,
                    &mut page,
                    &container,
                );
                win.redraw();
            },
        );

        self.menubar.add(
            "&File/Save...\t",
            Shortcut::Ctrl | 's',
            menu::MenuFlag::Normal,
            |_| {},
        );

        self.menubar.add(
            "&Edit/Configure...\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| {
                println!("wow");
            },
        );
    }

    fn draw(&mut self) {
        let mut buffer = Vec::new();
        fs::File::open("ReenieBeanie-Regular.ttf")
            .unwrap()
            .read_to_end(&mut buffer)
            .unwrap();
        let font = rusttype::Font::try_from_vec(buffer).unwrap();

        let container = Rc::clone(&self.container);
        let quote = self.quote.clone();
        self.page.image.draw(move |f| {
            if let Some(cont) = &*container.borrow() {
                let image = cont.image.as_rgb8().unwrap();
                dr::draw_image(
                    image.as_raw(),
                    f.x(),
                    f.y(),
                    image.width() as i32,
                    image.height() as i32,
                    enums::ColorDepth::Rgb8,
                )
                .unwrap();

                dr::set_color_rgb(255, 255, 255);

                let size = utils::quote_from_height(image.height());
                dr::set_font(enums::Font::Times, size as i32);

                let (text_width, text_height) = utils::measure_line(
                    &font,
                    &quote.value(),
                    rusttype::Scale::uniform(size as f32),
                );

                dr::draw_text(
                    &quote.value(),
                    f.x() + image.width() as i32 / 2 - text_width as i32 / 2,
                    f.y() + image.height() as i32 / 2 - text_height as i32 / 2,
                );
            }
        })
    }

    fn events(&mut self) {
        let mut image = self.page.image.clone();
        self.quote.handle(move |_, ev| {
            if ev == enums::Event::KeyUp {
                image.redraw();
            }
            true
        });
    }
}

fn load_image(
    file_choice: &mut menu::Choice,
    quote: &mut MultilineInput,
    tag: &mut Input,
    layer_red: &mut Spinner,
    layer_green: &mut Spinner,
    layer_blue: &mut Spinner,
    layer_alpha: &mut Spinner,
    quote_position: &mut Spinner,
    tag_position: &mut Spinner,
    page: &mut Page,
    container: &Rc<RefCell<Option<ImageContainer>>>,
) {
    let file: String = match file_choice.choice() {
        Some(val) => val,
        None => return,
    };

    *container.borrow_mut() = Some(ImageContainer::new(&file));

    let file = Path::new(&file);
    let conf = file.with_extension("conf");

    let mut use_defaults = true;
    if conf.exists() {
        let read = fs::read_to_string(&conf).unwrap();
        if let Ok(prop) = serde_json::from_str::<Properties>(&read) {
            if let Some(cont) = &mut *container.borrow_mut() {
                layer_red.set_value(prop.rgba[0] as f64);
                layer_green.set_value(prop.rgba[1] as f64);
                layer_blue.set_value(prop.rgba[2] as f64);
                layer_alpha.set_value(prop.rgba[3] as f64);
                quote.set_value(&prop.quote);
                tag.set_value(&prop.tag);
                quote_position.set_value(prop.quote_position as f64);
                tag_position.set_value(prop.tag_position as f64);
                cont.apply_crop_pos(prop.crop_position.0, prop.crop_position.1);

                cont.quote = prop.quote;
                cont.tag = prop.tag;
                cont.quote_position = prop.quote_position;
                cont.tag_position = prop.quote_position;
                cont.rgba = prop.rgba;
            }
            use_defaults = false;
        }
    }

    if use_defaults {
        if let Some(cont) = &mut *container.borrow_mut() {
            quote.set_value("");
            tag.set_value("");
            quote_position.set_value(cont.quote_position as f64);
            tag_position.set_value(cont.tag_position as f64);
            cont.apply_crop();

            cont.rgba = [
                layer_red.value() as u8,
                layer_green.value() as u8,
                layer_blue.value() as u8,
                layer_alpha.value() as u8,
            ];
        }
    }

    if let Some(cont) = &mut *container.borrow_mut() {
        cont.apply_layer();
        let (width, height) = cont.image.dimensions();
        page.row_flex.set_size(&page.col_flex, width as i32);
        page.col_flex.set_size(&page.image, height as i32);
        page.row_flex.recalc();
        page.col_flex.recalc();
    }
}
