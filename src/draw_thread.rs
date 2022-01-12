use crate::{
    main_window::{MainWindow, Page},
    AppMessage,
};
use fltk::{
    app,
    input::{Input, MultilineInput},
    menu,
    misc::Spinner,
    prelude::*,
};

use std::{
    fs,
    path::Path,
    sync::{mpsc, Arc, RwLock},
};

use crate::utils::{ImageContainer, ImageProperties};

#[derive(Debug, Clone)]
pub(crate) enum DrawMessage {
    Open,
    Recalc,
    Flush,
}

pub(crate) fn spawn_image_thread(
    reciver: mpsc::Receiver<DrawMessage>,
    app_sender: app::Sender<crate::AppMessage>,
    properties: Arc<RwLock<ImageProperties>>,
    main_win: &MainWindow,
) {
    let mut file_choice = main_win.file_choice.clone();
    let mut quote = main_win.quote.clone();
    let mut tag = main_win.tag.clone();
    let mut layer_red = main_win.layer_red.clone();
    let mut layer_green = main_win.layer_green.clone();
    let mut layer_blue = main_win.layer_blue.clone();
    let mut layer_alpha = main_win.layer_alpha.clone();
    let mut quote_position = main_win.quote_position.clone();
    let mut tag_position = main_win.tag_position.clone();
    let mut page = main_win.page.clone();
    let mut status = main_win.status.clone();

    let mut _container: Option<ImageContainer> = None;
    std::thread::spawn(move || loop {
        if let Ok(val) = reciver.recv() {
            match val {
                DrawMessage::Open => {
                    status.set_label("Loading...");
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
                        &app_sender,
                        &properties,
                        &mut _container,
                    );
                    status.set_label("");
                }
                DrawMessage::Recalc => {
                    if let Some(cont) = &mut _container {
                        cont.recalc();
                    }
                }
                DrawMessage::Flush => {
                    flush_buffer(&app_sender, &mut _container);
                }
            }
        }
    });
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
    app_sender: &app::Sender<crate::AppMessage>,
    properties: &Arc<RwLock<ImageProperties>>,
    container: &mut Option<ImageContainer>,
) {
    let file: String = match file_choice.choice() {
        Some(val) => val,
        None => return,
    };

    *container = Some(ImageContainer::new(&file, Arc::clone(properties)));

    if let Some(cont) = container {
        quote.set_value("");
        tag.set_value("");

        let file = Path::new(&file);
        let conf = file.with_extension("conf");

        let properties = Arc::clone(&cont.properties);
        let mut use_defaults = true;
        if conf.exists() {
            let read = fs::read_to_string(&conf).unwrap();
            if let Ok(saved_prop) = serde_json::from_str::<ImageProperties>(&read) {
                let mut prop = properties.write().unwrap();
                layer_red.set_value(saved_prop.rgba[0] as f64);
                layer_green.set_value(saved_prop.rgba[1] as f64);
                layer_blue.set_value(saved_prop.rgba[2] as f64);
                layer_alpha.set_value(saved_prop.rgba[3] as f64);
                quote.set_value(&saved_prop.quote);
                tag.set_value(&saved_prop.tag);
                quote_position.set_range(0.0, prop.original_dimension.1 as f64);
                quote_position.set_value(saved_prop.quote_position as f64);
                tag_position.set_range(0.0, prop.original_dimension.1 as f64);
                tag_position.set_value(saved_prop.tag_position as f64);

                prop.quote = saved_prop.quote;
                prop.tag = saved_prop.tag;
                prop.quote_position = saved_prop.quote_position;
                prop.tag_position = saved_prop.quote_position;
                prop.rgba = saved_prop.rgba;
                use_defaults = false;

                drop(prop);
                if let Some((x, y)) = saved_prop.crop_position {
                    cont.apply_crop_pos(x, y);
                }
            }
        }

        if use_defaults {
            let mut prop = properties.write().unwrap();
            prop.quote = "".to_owned();
            prop.tag = "".to_owned();

            quote_position.set_range(0.0, prop.original_dimension.1 as f64);
            quote_position.set_value(prop.quote_position as f64);
            tag_position.set_range(0.0, prop.original_dimension.1 as f64);
            tag_position.set_value(prop.tag_position as f64);

            prop.rgba = [
                layer_red.value() as u8,
                layer_green.value() as u8,
                layer_blue.value() as u8,
                layer_alpha.value() as u8,
            ];
            drop(prop);
            cont.apply_crop();
        }

        cont.apply_scale();
        let prop = properties.read().unwrap();
        let (width, height) = prop.dimension;
        page.col_flex.set_size(&page.image, height as i32);
        page.row_flex.set_size(&page.col_flex, width as i32);
        page.col_flex.recalc();
        page.row_flex.recalc();
        cont.recalc();
    }
    flush_buffer(&app_sender, &container);
}

fn flush_buffer(app_sender: &app::Sender<crate::AppMessage>, container: &Option<ImageContainer>) {
    if let Some(cont) = container {
        app_sender.send(AppMessage::RedrawMainWindowImage(
            cont.buffer.as_rgb8().unwrap().as_raw().to_owned(),
        ));
    }
}
