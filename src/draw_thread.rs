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

//! Thread to manage drawing in background

use crate::{
    globals,
    main_window::{MainWindow, Page},
    result_ext::ResultExt,
    utils::{self, ImageContainer, ImageInfo, ImageProperties, ImagePropertiesFile},
    AppMessage,
};
use fltk::{
    app,
    button::Button,
    enums,
    frame::Frame,
    input::{Input, MultilineInput},
    menu,
    misc::Spinner,
    prelude::*,
    valuator::Slider,
};
use std::{
    fs,
    sync::{mpsc, Arc, RwLock},
};

#[derive(Debug, Clone)]
pub(crate) enum DrawMessage {
    /// Open file or cropped file
    Open,
    /// Load file with specific cropped size
    ChangeCrop((f64, f64)),
    /// Recalculate and draw on buffer image in Container
    RedrawToBuffer,
    /// Flush buffer to u8 vector present in main, to draw on screen
    Flush,
    /// Save to file
    Save,
    /// Clone file
    Clone,
    /// Delete file
    Delete,
    /// Show details about images linke count of quotes
    ShowImagesDetails,
    /// Check If image is proper
    CheckImage,
}

/// Spawn thread to manage all actions related to image, like: edit, save, delete
pub(crate) fn spawn_image_thread(
    reciver: mpsc::Receiver<DrawMessage>,
    app_sender: app::Sender<crate::AppMessage>,
    properties: Arc<RwLock<ImageProperties>>,
    main_win: &MainWindow,
) {
    let mut win = main_win.win.clone();
    let mut file_choice = main_win.file_choice.clone();
    let mut name_prefix = main_win.name_prefix.clone();
    let mut quote = main_win.quote.clone();
    let mut subquote = main_win.subquote.clone();
    let mut subquote2 = main_win.subquote2.clone();
    let mut tag = main_win.tag.clone();
    let mut tag2 = main_win.tag2.clone();
    let mut layer_rgb = main_win.translucent_layer_rgb.clone();
    let mut layer_alpha = main_win.translucent_layer_alpha.clone();
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
    let images_list = Arc::clone(&main_win.images_list);

    let mut _container: Option<ImageContainer> = None;
    std::thread::spawn(move || loop {
        if let Ok(val) = reciver.recv() {
            match val {
                DrawMessage::Open => {
                    status.set_label("Loading...");
                    load_image(
                        &mut file_choice,
                        Arc::clone(&images_list),
                        None,
                        &mut name_prefix,
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
                        Arc::clone(&properties),
                        &mut _container,
                    );
                    status.set_label("");
                    status.set_label("");
                }
                DrawMessage::ChangeCrop((x, y)) => {
                    status.set_label("Loading...");
                    load_image(
                        &mut file_choice,
                        Arc::clone(&images_list),
                        Some((x, y)),
                        &mut name_prefix,
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
                        Arc::clone(&properties),
                        &mut _container,
                    );
                    status.set_label("");
                }
                DrawMessage::RedrawToBuffer => {
                    if let Some(cont) = &mut _container {
                        cont.redraw_to_buffer();
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
                        status.set_label("");
                        win.activate();
                        win.redraw();
                        app::awake();
                    }
                }
                DrawMessage::Clone => {
                    if let Some(cont) = &mut _container {
                        status.set_label("Cloning...");
                        win.deactivate();
                        if let Some(image_info) = cont.clone_img() {
                            let idx = file_choice.value();
                            let mut imgs = rw_write!(images_list);
                            imgs.insert(idx as usize, image_info.clone());
                            file_choice.insert(
                                idx,
                                image_info.path.file_name().unwrap().to_str().unwrap(),
                                enums::Shortcut::None,
                                menu::MenuFlag::Normal,
                                |a| a.do_callback(),
                            );
                            file_choice.set_value(idx);
                        }
                        status.set_label("");
                        win.activate();
                        win.redraw();
                        app::awake();
                    }
                }
                DrawMessage::Delete => {
                    if let Some(cont) = &mut _container {
                        status.set_label("Deleting...");
                        win.deactivate();
                        cont.delete();
                        let mut imgs = rw_write!(images_list);
                        imgs.remove(file_choice.value() as usize);
                        file_choice.remove(file_choice.value());
                        if file_choice.value() != imgs.len() as i32 {
                            file_choice.set_value(file_choice.value());
                        } else {
                            file_choice.set_value(file_choice.value() - 1);
                        }
                        status.set_label("");
                        win.activate();
                        win.redraw();
                        app::awake();
                    }
                }
                DrawMessage::ShowImagesDetails => show_images_details(Arc::clone(&images_list)),
                DrawMessage::CheckImage => {
                    let (width, height) = rw_read!(properties).original_dimension;
                    if utils::is_too_small(width, height) {
                        if let Some(a) = &*rw_read!(globals::MAIN_SENDER) {
                            a.send(crate::AppMessage::DeleteImage);
                        }
                    }
                }
            }
        }
    });
}

/// Loads the selected image in file_choice to ImageContainer to edit
fn load_image(
    file_choice: &mut menu::Choice,
    images_list: Arc<RwLock<Vec<ImageInfo>>>,
    crop: Option<(f64, f64)>,
    name_prefix: &mut Input,
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
    properties: Arc<RwLock<ImageProperties>>,
    container: &mut Option<ImageContainer>,
) {
    let imgs = rw_read!(images_list);
    if imgs.len() == 0 {
        *container = None;
        flush_buffer(app_sender, container);
        return;
    }
    count.set_label(&format!("[{}/{}]", file_choice.value() + 1, imgs.len()));
    let image_info = imgs.get(file_choice.value() as usize).unwrap();

    *container = Some(ImageContainer::new(&image_info, Arc::clone(&properties)));

    if let Some(cont) = container {
        let properties_file = utils::get_properties_path(&image_info);

        let read = fs::read_to_string(&properties_file).unwrap_or("{}".to_owned());
        let read = match serde_json::from_str::<ImagePropertiesFile>(&read) {
            Ok(r) => r,
            Err(e) => {
                Result::<(), _>::Err(e).warn_log("Config is corrupt");
                fs::remove_file(&properties_file)
                    .warn_log("Failed to delete image properties file!");
                ImagePropertiesFile::default()
            }
        };

        let mut properties = rw_write!(cont.properties);
        properties.merge(read, &tag.value(), &tag2.value());
        properties.is_saved = true;

        name_prefix.set_value(&properties.name_prefix);
        quote.set_value(&properties.quote);
        subquote.set_value(&properties.subquote);
        subquote2.set_value(&properties.subquote2);
        tag.set_value(&properties.tag);
        tag2.set_value(&properties.tag2);

        quote_position.set_range(0.0, properties.original_dimension.1);
        quote_position.set_value(properties.quote_position);
        quote_position_slider.set_range(0.0, properties.original_dimension.1);
        quote_position_slider.set_value(properties.quote_position);

        subquote_position.set_range(0.0, properties.original_dimension.1);
        subquote_position.set_value(properties.subquote_position);
        subquote_position_slider.set_range(0.0, properties.original_dimension.1);
        subquote_position_slider.set_value(properties.subquote_position);

        subquote2_position.set_range(0.0, properties.original_dimension.1);
        subquote2_position.set_value(properties.subquote2_position);
        subquote2_position_slider.set_range(0.0, properties.original_dimension.1);
        subquote2_position_slider.set_value(properties.subquote2_position);

        tag_position.set_range(0.0, properties.original_dimension.1);
        tag_position.set_value(properties.tag_position);
        tag_position_slider.set_range(0.0, properties.original_dimension.1);
        tag_position_slider.set_value(properties.tag_position);

        tag2_position.set_range(0.0, properties.original_dimension.1);
        tag2_position.set_value(properties.tag2_position);
        tag2_position_slider.set_range(0.0, properties.original_dimension.1);
        tag2_position_slider.set_value(properties.tag2_position);

        utils::set_color_btn_rgba(properties.translucent_layer_color, layer_rgb);
        layer_alpha.set_value(properties.translucent_layer_color[3] as f64);

        dimension.set_label(&format!(
            "[{}x{}]",
            properties.original_dimension.0, properties.original_dimension.1
        ));

        let crop_position = properties.crop_position;
        drop(properties);
        match crop {
            Some((x, y)) => cont.apply_crop_position(x, y),
            None => match crop_position {
                Some((x, y)) => cont.apply_crop_position(x, y),
                None => cont.apply_crop(),
            },
        }

        cont.apply_resize();
        let (width, height) = rw_read!(cont.properties).dimension;
        page.col_flex.set_size(&page.image, height as i32);
        page.row_flex.set_size(&page.col_flex, width as i32);
        page.col_flex.recalc();
        page.row_flex.recalc();
        cont.redraw_to_buffer();
    }
    flush_buffer(&app_sender, &container);
}

fn show_images_details(images_list: Arc<RwLock<Vec<ImageInfo>>>) {
    let mut image_with_quote: usize = 0;
    let mut image_without_quote: usize = 0;

    let list = rw_read!(images_list);
    for image_info in list.iter() {
        let properties_file = utils::get_properties_path(&image_info);

        let read = fs::read_to_string(&properties_file).unwrap_or("{}".to_owned());
        let read = match serde_json::from_str::<ImagePropertiesFile>(&read) {
            Ok(r) => r,
            Err(_) => {
                image_without_quote += 1;
                continue;
            }
        };

        if let Some(t) = read.quote {
            if t.trim().len() == 0 {
                image_without_quote += 1;
            } else {
                image_with_quote += 1;
            }
        } else {
            image_without_quote += 1;
        }
    }

    utils::show_message(&format!(
        "With Quote: {}\nWithout Quote: {}",
        image_with_quote, image_without_quote
    ));
}

/// Flush the Buffer from image container to drawing buffer for fltk
// for drawing buffer for fltk (check in main.rs)
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
