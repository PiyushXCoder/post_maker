use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use fltk::dialog;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use serde::{Deserialize, Serialize};

use crate::globals;

pub(crate) struct Coord(pub(crate) f64, pub(crate) f64);

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Coord(x as f64, y as f64)
    }
}

impl From<(u32, u32)> for Coord {
    fn from((x, y): (u32, u32)) -> Self {
        Coord(x as f64, y as f64)
    }
}

impl From<(f32, f32)> for Coord {
    fn from((x, y): (f32, f32)) -> Self {
        Coord(x as f64, y as f64)
    }
}

impl Into<(f64, f64)> for Coord {
    fn into(self) -> (f64, f64) {
        (self.0, self.1)
    }
}

impl Into<(u32, u32)> for Coord {
    fn into(self) -> (u32, u32) {
        (self.0 as u32, self.1 as u32)
    }
}

impl Into<(i32, i32)> for Coord {
    fn into(self) -> (i32, i32) {
        (self.0 as i32, self.1 as i32)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ImageContainer {
    pub(crate) image: DynamicImage,  //plain
    pub(crate) buffer: DynamicImage, //buffer to show
    pub(crate) properties: Arc<RwLock<ImageProperties>>,
}

impl ImageContainer {
    pub(crate) fn new(path: &PathBuf, properties: Arc<RwLock<ImageProperties>>) -> Self {
        let img = match image::open(path) {
            Ok(i) => i,
            Err(_) => {
                dialog::message_default("Failed to open image");
                panic!("Failed to open image");
            }
        };

        let img = DynamicImage::ImageRgb8(img.into_rgb8());
        let (width, height): (f64, f64) = Coord::from(img.dimensions()).into();
        let (width, height) = (width, height);

        let mut prop = properties.write().unwrap();
        prop.path = Some(path.to_owned());
        prop.original_dimension = (width, height);
        prop.quote_position = (height * 2.0) / 3.0;
        prop.tag_position = height / 2.0;

        Self {
            image: img.clone(),
            buffer: img,
            properties: Arc::clone(&properties),
        }
    }

    pub(crate) fn apply_scale(&mut self) {
        let mut prop = self.properties.write().unwrap();
        let (width, height) = prop.dimension;
        let (s_width, s_height) = ((width * 500.0) / height, 500.0);

        self.image = self.image.thumbnail_exact(s_width as u32, s_height as u32);

        self.buffer = self.image.clone();
        prop.dimension = (s_width, s_height);
    }

    pub(crate) fn apply_crop(&mut self) {
        let mut prop = self.properties.write().unwrap();
        let (original_width, original_height) = prop.original_dimension;
        let (origina_crop_width, origina_crop_height) = get_4_5(original_width, original_height);
        prop.crop_position = Some((
            original_width / 2.0 - origina_crop_width / 2.0,
            original_height / 2.0 - origina_crop_height / 2.0,
        ));

        let (s_width, s_height): (f64, f64) = Coord::from(self.image.dimensions()).into();
        let (c_width, c_height) = get_4_5(s_width, s_height);
        let (cx, cy) = ((s_width - c_width) / 2.0, (s_height - c_height) / 2.0);

        prop.dimension = (c_width, c_height);

        self.image = self
            .image
            .crop(cx as u32, cy as u32, c_width as u32, c_height as u32);
        self.buffer = self.image.clone();
    }

    pub(crate) fn apply_crop_pos(&mut self, original_x: f64, original_y: f64) {
        let mut prop = self.properties.write().unwrap();
        let (original_width, original_height) = prop.original_dimension;
        prop.crop_position = Some((original_x, original_y));

        let (s_width, s_height): (f64, f64) = Coord::from(self.image.dimensions()).into();
        let (c_width, c_height) = get_4_5(s_width, s_height);
        let (cx, cy) = (
            (original_x * s_width) / original_width,
            (original_y * s_height) / original_height,
        );

        prop.dimension = (c_width, c_height);

        self.image = self
            .image
            .crop(cx as u32, cy as u32, c_width as u32, c_height as u32);
        self.buffer = self.image.clone();
    }

    pub(crate) fn recalc(&mut self) {
        let prop = self.properties.read().unwrap();
        let mut tmp = self.image.clone();

        draw_layer_and_text(
            &mut tmp,
            &prop.rgba,
            &prop.quote,
            prop.quote_position,
            &prop.tag,
            prop.tag_position,
            prop.original_dimension.1,
        );

        self.buffer = tmp;
    }

    pub(crate) fn save(&self) {
        let prop = self.properties.read().unwrap();

        let path_original = match &prop.path {
            Some(p) => Path::new(p),
            None => return,
        };
        let path_conf = path_original.with_extension("conf");
        let export = path_original.parent().unwrap().join("export").join(
            path_original
                .with_extension("png")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        );

        if let Err(_) = fs::write(&path_conf, serde_json::to_string(&*prop).unwrap()) {
            dialog::message_default("Failed to save conf!");
        }

        let mut img = image::open(&path_original).unwrap();
        let (width, height): (f64, f64) = Coord::from(img.dimensions()).into();
        let (crop_x, crop_y) = prop.crop_position.unwrap();
        let (crop_width, crop_height) = get_4_5(width, height);
        let mut img = img.crop(
            crop_x as u32,
            crop_y as u32,
            crop_width as u32,
            crop_height as u32,
        );

        draw_layer_and_text(
            &mut img,
            &prop.rgba,
            &prop.quote,
            prop.quote_position,
            &prop.tag,
            prop.tag_position,
            prop.original_dimension.1,
        );

        if let Err(_) = img.save_with_format(&export, image::ImageFormat::Png) {
            dialog::message_default("Failed to export png!");
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ImageProperties {
    pub(crate) path: Option<PathBuf>,
    pub(crate) dimension: (f64, f64),
    pub(crate) original_dimension: (f64, f64),
    pub(crate) crop_position: Option<(f64, f64)>,
    pub(crate) quote: String,
    pub(crate) tag: String,
    pub(crate) quote_position: f64, // as per original
    pub(crate) tag_position: f64,   // as per original
    pub(crate) rgba: [u8; 4],
    pub(crate) is_saved: bool,
}

impl ImageProperties {
    pub(crate) fn new() -> Self {
        Self {
            path: None,
            dimension: (0.0, 0.0),
            original_dimension: (0.0, 0.0),
            crop_position: None,
            quote: "".to_owned(),
            tag: "".to_owned(),
            quote_position: 0.0,
            tag_position: 0.0,
            rgba: [0; 4],
            is_saved: true,
        }
    }
}

fn draw_layer_and_text(
    tmp: &mut DynamicImage,
    rgba: &[u8; 4],
    quote: &str,
    quote_position: f64,
    tag: &str,
    tag_position: f64,
    original_height: f64,
) {
    let (width, height): (f64, f64) = Coord::from(tmp.dimensions()).into();
    let layer =
        DynamicImage::ImageRgba8(ImageBuffer::from_fn(width as u32, height as u32, |_, _| {
            image::Rgba(rgba.to_owned())
        }));
    image::imageops::overlay(tmp, &layer, 0, 0);

    let size = quote_from_height(height);
    for (index, line) in quote.lines().enumerate() {
        let (text_width, text_height) = measure_line(
            &globals::FONT_QUOTE,
            line,
            rusttype::Scale::uniform(size as f32),
        );

        imageproc::drawing::draw_text_mut(
            tmp,
            image::Rgba([255, 255, 255, 255]),
            ((width - text_width) / 2.0) as u32,
            ((quote_position * height) / original_height
                + (text_height / 2.0)
                + index as f64 * (text_height * 1.2)) as u32,
            rusttype::Scale::uniform(size as f32),
            &globals::FONT_QUOTE,
            line,
        );
    }

    let size = tag_from_height(height);
    for (index, line) in tag.lines().enumerate() {
        let (text_width, text_height) = measure_line(
            &globals::FONT_TAG,
            line,
            rusttype::Scale::uniform(size as f32),
        );

        imageproc::drawing::draw_text_mut(
            tmp,
            image::Rgba([255, 255, 255, 255]),
            (width * 0.99 - text_width) as u32,
            ((tag_position * height) / original_height
                + (text_height / 2.0)
                + index as f64 * (text_height * 1.2)) as u32,
            rusttype::Scale::uniform(size as f32),
            &globals::FONT_TAG,
            line,
        );
    }
}

pub(crate) fn get_4_5(width: f64, height: f64) -> (f64, f64) {
    if width > width_from_height(height) {
        (width_from_height(height), height)
    } else {
        (width, height_from_width(width))
    }
}

pub(crate) fn width_from_height(height: f64) -> f64 {
    (4.0 * height) / 5.0
}

pub(crate) fn height_from_width(width: f64) -> f64 {
    (5.0 * width) / 4.0
}

pub(crate) fn quote_from_height(height: f64) -> f64 {
    (height * globals::CONFIG.read().unwrap().quote_font_ratio) / 5000.0
}

pub(crate) fn tag_from_height(height: f64) -> f64 {
    (height * globals::CONFIG.read().unwrap().tag_font_ratio) / 5000.0
}

pub(crate) fn measure_line(
    font: &rusttype::Font,
    text: &str,
    scale: rusttype::Scale,
) -> (f64, f64) {
    let width = font
        .layout(text, scale, rusttype::point(0.0, 0.0))
        .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .last()
        .unwrap_or(0.0);

    let v_metrics = font.v_metrics(scale);
    let height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

    Coord::from((width, height)).into()
}
