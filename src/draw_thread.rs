//! Thread to manage drawing in background

use crate::{
    globals,
    main_window::{MainWindow, Page},
    utils, AppMessage,
};
use fltk::{
    app,
    button::Button,
    dialog, enums,
    frame::Frame,
    input::{Input, MultilineInput},
    menu,
    misc::Spinner,
    prelude::*,
    valuator::Slider,
};

use std::{
    fs,
    path::{Path, PathBuf},
    sync::{mpsc, Arc, RwLock},
};

use crate::utils::{ImageContainer, ImageProperties};

#[derive(Debug, Clone)]
pub(crate) enum DrawMessage {
    /// Open file or cropped file
    Open,
    /// Load file with specific cropped size
    ChangeCrop((f64, f64)),
    /// Recalculate and draw on buffer image in Container
    Recalc,
    /// Flush buffer to u8 vector present in main, to draw on screen
    Flush,
    /// Save to file
    Save,
    /// Clone file
    Clone,
    /// Delete file
    Delete,
}

pub(crate) fn spawn_image_thread(
    reciver: mpsc::Receiver<DrawMessage>,
    app_sender: app::Sender<crate::AppMessage>,
    properties: Arc<RwLock<ImageProperties>>,
    main_win: &MainWindow,
) {
    let mut win = main_win.win.clone();
    let mut file_choice = main_win.file_choice.clone();
    let mut quote = main_win.quote.clone();
    let mut subquote = main_win.subquote.clone();
    let mut subquote2 = main_win.subquote2.clone();
    let mut tag = main_win.tag.clone();
    let mut tag2 = main_win.tag2.clone();
    let mut layer_rgb = main_win.layer_rgb.clone();
    let mut layer_alpha = main_win.layer_alpha.clone();
    let mut quote_position = main_win.quote_position.clone();
    let mut subquote_position = main_win.subquote_position.clone();
    let mut subquote2_position = main_win.subquote2_position.clone();
    let mut tag_position = main_win.tag_position.clone();
    let mut tag2_position = main_win.tag2_position.clone();
    let mut quote_position_slider = main_win.quote_position_slider.clone();
    let mut subquote_position_slider = main_win.subquote_position_slider.clone();
    let mut subquote2_position_slider = main_win.subquote2_position_slider.clone();
    let mut tag_position_slider = main_win.tag_position_slider.clone();
    let mut tag2_position_slider = main_win.tag2_position_slider.clone();
    let mut page = main_win.page.clone();
    let mut status = main_win.status.clone();
    let mut count = main_win.count.clone();
    let mut dimension = main_win.dimension.clone();
    let images_path = Arc::clone(&main_win.images_path);

    let mut _container: Option<ImageContainer> = None;
    std::thread::spawn(move || loop {
        if let Ok(val) = reciver.recv() {
            match val {
                DrawMessage::Open => {
                    status.set_label("Loading...");
                    load_image(
                        &mut file_choice,
                        Arc::clone(&images_path),
                        None,
                        &mut quote,
                        &mut subquote,
                        &mut subquote2,
                        &mut tag,
                        &mut tag2,
                        &mut layer_rgb,
                        &mut layer_alpha,
                        &mut quote_position,
                        &mut subquote_position,
                        &mut subquote2_position,
                        &mut tag_position,
                        &mut tag2_position,
                        &mut quote_position_slider,
                        &mut subquote_position_slider,
                        &mut subquote2_position_slider,
                        &mut tag_position_slider,
                        &mut tag2_position_slider,
                        &mut page,
                        &mut count,
                        &mut dimension,
                        &app_sender,
                        &properties,
                        &mut _container,
                    );
                    status.set_label("");
                    status.set_label("");
                }
                DrawMessage::ChangeCrop((x, y)) => {
                    status.set_label("Loading...");
                    load_image(
                        &mut file_choice,
                        Arc::clone(&images_path),
                        Some((x, y)),
                        &mut quote,
                        &mut subquote,
                        &mut subquote2,
                        &mut tag,
                        &mut tag2,
                        &mut layer_rgb,
                        &mut layer_alpha,
                        &mut quote_position,
                        &mut subquote_position,
                        &mut subquote2_position,
                        &mut tag_position,
                        &mut tag2_position,
                        &mut quote_position_slider,
                        &mut subquote_position_slider,
                        &mut subquote2_position_slider,
                        &mut tag_position_slider,
                        &mut tag2_position_slider,
                        &mut page,
                        &mut count,
                        &mut dimension,
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
                DrawMessage::Save => {
                    if let Some(cont) = &mut _container {
                        status.set_label("Saving...");
                        win.deactivate();
                        cont.save();
                        win.activate();
                        status.set_label("");
                    }
                }
                DrawMessage::Clone => {
                    if let Some(cont) = &mut _container {
                        status.set_label("Cloning...");
                        win.deactivate();
                        if let Some(path) = cont.clone_img() {
                            let idx = file_choice.value();
                            let mut imgs = images_path.write().unwrap();
                            imgs.insert(idx as usize, path.clone());
                            file_choice.insert(
                                idx,
                                path.file_name().unwrap().to_str().unwrap(),
                                enums::Shortcut::None,
                                menu::MenuFlag::Normal,
                                |a| a.do_callback(),
                            );
                            file_choice.set_value(idx);
                        }
                        status.set_label("");
                        win.activate();
                    }
                }
                DrawMessage::Delete => {
                    if let Some(cont) = &mut _container {
                        status.set_label("Deleting...");
                        win.deactivate();
                        cont.delete();
                        let mut imgs = images_path.write().unwrap();
                        imgs.remove(file_choice.value() as usize);
                        file_choice.remove(file_choice.value());
                        if file_choice.value() != imgs.len() as i32 {
                            file_choice.set_value(file_choice.value());
                        } else {
                            file_choice.set_value(file_choice.value() - 1);
                        }
                        status.set_label("");
                        win.activate();
                    }
                }
            }
        }
    });
}

fn load_image(
    file_choice: &mut menu::Choice,
    images_path: Arc<RwLock<Vec<PathBuf>>>,
    crop: Option<(f64, f64)>,
    quote: &mut MultilineInput,
    subquote: &mut MultilineInput,
    subquote2: &mut MultilineInput,
    tag: &mut Input,
    tag2: &mut Input,
    layer_rgb: &mut Button,
    layer_alpha: &mut Spinner,
    quote_position: &mut Spinner,
    subquote_position: &mut Spinner,
    subquote2_position: &mut Spinner,
    tag_position: &mut Spinner,
    tag2_position: &mut Spinner,
    quote_position_slider: &mut Slider,
    subquote_position_slider: &mut Slider,
    subquote2_position_slider: &mut Slider,
    tag_position_slider: &mut Slider,
    tag2_position_slider: &mut Slider,
    page: &mut Page,
    count: &mut Frame,
    dimension: &mut Frame,
    app_sender: &app::Sender<crate::AppMessage>,
    properties: &Arc<RwLock<ImageProperties>>,
    container: &mut Option<ImageContainer>,
) {
    let imgs = images_path.read().unwrap();
    if imgs.len() == 0 {
        *container = None;
        flush_buffer(app_sender, container);
        return;
    }
    count.set_label(&format!("[{}/{}]", file_choice.value() + 1, imgs.len()));
    let file = imgs.get(file_choice.value() as usize).unwrap();

    *container = Some(ImageContainer::new(&file, Arc::clone(properties)));

    if let Some(cont) = container {
        let file = Path::new(&file);
        let conf = file.with_extension("conf");

        let properties = Arc::clone(&cont.properties);
        let mut use_defaults = true;
        if conf.exists() {
            let mut prop = properties.write().unwrap();
            match fs::read_to_string(&conf) {
                Ok(read) => {
                    if let Ok(saved_prop) = serde_json::from_str::<ImageProperties>(&read) {
                        utils::set_color_btn_rgba(saved_prop.rgba, layer_rgb);
                        layer_alpha.set_value(saved_prop.rgba[3] as f64);
                        quote.set_value(&saved_prop.quote);
                        subquote.set_value(&saved_prop.subquote);
                        subquote2.set_value(&saved_prop.subquote2);
                        tag.set_value(&saved_prop.tag);
                        tag2.set_value(&saved_prop.tag2);
                        quote_position.set_range(0.0, prop.original_dimension.1);
                        quote_position.set_value(saved_prop.quote_position);
                        subquote_position.set_range(0.0, prop.original_dimension.1);
                        subquote_position.set_value(saved_prop.subquote_position);
                        subquote2_position.set_range(0.0, prop.original_dimension.1);
                        subquote2_position.set_value(saved_prop.subquote2_position);
                        tag_position.set_range(0.0, prop.original_dimension.1);
                        tag_position.set_value(saved_prop.tag_position);
                        tag2_position.set_range(0.0, prop.original_dimension.1);
                        tag2_position.set_value(saved_prop.tag2_position);
                        quote_position_slider.set_range(0.0, prop.original_dimension.1);
                        subquote_position_slider.set_range(0.0, prop.original_dimension.1);
                        subquote2_position_slider.set_range(0.0, prop.original_dimension.1);
                        quote_position_slider.set_value(saved_prop.quote_position);
                        subquote_position_slider.set_value(saved_prop.subquote_position);
                        subquote2_position_slider.set_value(saved_prop.subquote2_position);
                        tag_position_slider.set_range(0.0, prop.original_dimension.1);
                        tag_position_slider.set_value(saved_prop.tag_position);
                        tag2_position_slider.set_range(0.0, prop.original_dimension.1);
                        tag2_position_slider.set_value(saved_prop.tag2_position);
                        dimension.set_label(&format!(
                            "[{}x{}]",
                            prop.original_dimension.0, prop.original_dimension.1
                        ));

                        prop.quote = saved_prop.quote;
                        prop.subquote = saved_prop.subquote;
                        prop.subquote2 = saved_prop.subquote2;
                        prop.tag = saved_prop.tag;
                        prop.tag2 = saved_prop.tag2;
                        prop.quote_position = saved_prop.quote_position;
                        prop.subquote_position = saved_prop.subquote_position;
                        prop.subquote2_position = saved_prop.subquote2_position;
                        prop.tag_position = saved_prop.tag_position;
                        prop.tag2_position = saved_prop.tag2_position;
                        prop.rgba = saved_prop.rgba;
                        prop.is_saved = true;
                        use_defaults = false;
                        drop(prop);

                        match crop {
                            Some((x, y)) => cont.apply_crop_pos(x, y),
                            None => match saved_prop.crop_position {
                                Some((x, y)) => cont.apply_crop_pos(x, y),
                                None => cont.apply_crop(),
                            },
                        }
                    }
                }
                Err(e) => {
                    dialog::alert_default("Failed to open config file!");
                    warn!("Failed to open config file!\n{:?}", e);
                }
            };
        }

        if use_defaults {
            let mut prop = properties.write().unwrap();
            if crop.is_none() {
                quote.set_value("");
                prop.quote = "".to_owned();
                subquote.set_value("");
                prop.subquote = "".to_owned();
                subquote2.set_value("");
                prop.subquote2 = "".to_owned();
            }
            quote_position.set_range(0.0, prop.original_dimension.1);
            quote_position.set_value(prop.quote_position);
            subquote_position.set_range(0.0, prop.original_dimension.1);
            subquote_position.set_value(prop.subquote_position);
            subquote2_position.set_range(0.0, prop.original_dimension.1);
            subquote2_position.set_value(prop.subquote2_position);
            tag_position.set_range(0.0, prop.original_dimension.1);
            tag_position.set_value(prop.tag_position);
            tag2_position.set_range(0.0, prop.original_dimension.1);
            tag2_position.set_value(prop.tag2_position);
            quote_position_slider.set_range(0.0, prop.original_dimension.1);
            quote_position_slider.set_value(prop.quote_position);
            subquote_position_slider.set_range(0.0, prop.original_dimension.1);
            subquote_position_slider.set_value(prop.subquote_position);
            subquote2_position_slider.set_range(0.0, prop.original_dimension.1);
            subquote2_position_slider.set_value(prop.subquote2_position);
            tag_position_slider.set_range(0.0, prop.original_dimension.1);
            tag_position_slider.set_value(prop.tag_position);
            tag2_position_slider.set_range(0.0, prop.original_dimension.1);
            tag2_position_slider.set_value(prop.tag2_position);

            let glob = &globals::CONFIG.read().unwrap();
            utils::set_color_btn_rgba(glob.color_layer, layer_rgb);
            layer_alpha.set_value(glob.color_layer[3] as f64);
            prop.rgba = glob.color_layer;
            drop(glob);

            match crop {
                Some((x, y)) => {
                    prop.is_saved = false;
                    drop(prop);
                    cont.apply_crop_pos(x, y);
                }
                None => {
                    prop.is_saved = true;
                    drop(prop);
                    cont.apply_crop();
                }
            }
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
    match container {
        Some(cont) => {
            app_sender.send(AppMessage::RedrawMainWindowImage(Some(
                cont.buffer.as_rgb8().unwrap().as_raw().to_owned(),
            )));
        }
        None => {
            app_sender.send(AppMessage::RedrawMainWindowImage(None));
        }
    }
}
