use std::sync::{Arc, RwLock};

use image::{DynamicImage, GenericImageView, ImageBuffer};
use serde::{Deserialize, Serialize};

use crate::properties;

#[derive(Debug)]
pub(crate) struct ImageContainer {
    pub(crate) image: DynamicImage,  //plain
    pub(crate) buffer: DynamicImage, //buffer to show
    pub(crate) properties: Arc<RwLock<ImageProperties>>,
}

impl ImageContainer {
    pub(crate) fn new(path: &str, properties: Arc<RwLock<ImageProperties>>) -> Self {
        let img = image::open(path).unwrap();
        let (width, height) = img.dimensions();
        let (s_width, s_height) = ((width * 500) / height, 500);
        let img = img.resize(s_width, s_height, image::imageops::FilterType::Triangle);

        let mut prop = properties.write().unwrap();
        prop.path = path.to_owned();
        prop.dimension = (s_width, s_height);
        prop.original_dimension = (width, height);
        prop.quote_position = height / 2;
        prop.tag_position = (height * 2) / 3;

        Self {
            image: img.clone(),
            buffer: img,
            properties: Arc::clone(&properties),
        }
    }

    pub(crate) fn apply_crop(&mut self) {
        let mut prop = self.properties.write().unwrap();
        let (original_width, original_height) = prop.original_dimension;
        let (origina_crop_width, origina_crop_height) = get_4_5(original_width, original_height);
        prop.crop_position = Some((
            original_width / 2 - origina_crop_width / 2,
            original_height / 2 - origina_crop_height / 2,
        ));

        let (s_width, s_height) = self.image.dimensions();
        let (c_width, c_height) = get_4_5(s_width, s_height);
        let (cx, cy) = (s_width / 2 - c_width / 2, s_height / 2 - c_height / 2);

        prop.dimension = (c_width, c_height);

        self.image = self.image.crop(cx, cy, c_width, c_height);
        self.buffer = self.image.clone();
    }

    pub(crate) fn apply_crop_pos(&mut self, original_x: u32, original_y: u32) {
        let mut prop = self.properties.write().unwrap();
        let (original_width, original_height) = prop.original_dimension;
        prop.crop_position = Some((original_x, original_y));

        let (s_width, s_height) = self.image.dimensions();
        let (c_width, c_height) = get_4_5(s_width, s_height);
        let (cx, cy) = (
            (original_x * s_width) / original_width,
            (original_y * s_height) / original_height,
        );

        prop.dimension = (c_width, c_height);

        self.image = self.image.crop(cx, cy, c_width, c_height);
        self.buffer = self.image.clone();
    }

    pub(crate) fn recalc(&mut self) {
        let prop = self.properties.read().unwrap();
        let mut tmp = self.image.clone();
        let (width, height) = tmp.dimensions();

        let layer = DynamicImage::ImageRgba8(ImageBuffer::from_fn(width, height, |_, _| {
            image::Rgba(prop.rgba)
        }));
        image::imageops::overlay(&mut tmp, &layer, 0, 0);

        let size = quote_from_height(height);
        for (index, line) in prop.quote.lines().enumerate() {
            let (text_width, text_height) = measure_line(
                &properties::FONT_QUOTE,
                line,
                rusttype::Scale::uniform(size as f32),
            );

            imageproc::drawing::draw_text_mut(
                &mut tmp,
                image::Rgba([255, 255, 255, 255]),
                ((width as f32 - text_width) / 2.0) as u32,
                (prop.quote_position * height) / prop.original_dimension.1
                    + (text_height / 2.0) as u32
                    + index as u32 * (text_height * 1.2) as u32,
                rusttype::Scale::uniform(size as f32),
                &properties::FONT_QUOTE,
                line,
            );
        }

        let size = tag_from_height(height);
        for (index, line) in prop.tag.lines().enumerate() {
            let (text_width, text_height) = measure_line(
                &properties::FONT_TAG,
                line,
                rusttype::Scale::uniform(size as f32),
            );

            imageproc::drawing::draw_text_mut(
                &mut tmp,
                image::Rgba([255, 255, 255, 255]),
                (width as f32 * 0.99 - text_width) as u32,
                (prop.tag_position * height) / prop.original_dimension.1
                    + (text_height / 2.0) as u32
                    + index as u32 * (text_height * 1.2) as u32,
                rusttype::Scale::uniform(size as f32),
                &properties::FONT_TAG,
                line,
            );
        }

        self.buffer = tmp;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ImageProperties {
    pub(crate) path: String,
    pub(crate) dimension: (u32, u32),
    pub(crate) original_dimension: (u32, u32),
    pub(crate) crop_position: Option<(u32, u32)>,
    pub(crate) quote: String,
    pub(crate) tag: String,
    pub(crate) quote_position: u32,
    pub(crate) tag_position: u32,
    pub(crate) rgba: [u8; 4],
    pub(crate) is_saved: bool,
}

impl ImageProperties {
    pub(crate) fn new() -> Self {
        Self {
            path: "".to_owned(),
            dimension: (0, 0),
            original_dimension: (0, 0),
            crop_position: None,
            quote: "".to_owned(),
            tag: "".to_owned(),
            quote_position: 0,
            tag_position: 0,
            rgba: [0; 4],
            is_saved: true,
        }
    }
}

pub(crate) fn get_4_5(width: u32, height: u32) -> (u32, u32) {
    if width > width_from_height(height) {
        (width_from_height(height), height)
    } else {
        (width, height_from_width(width))
    }
}

pub(crate) fn width_from_height(height: u32) -> u32 {
    (4 * height) / 5
}

pub(crate) fn height_from_width(width: u32) -> u32 {
    (5 * width) / 4
}

pub(crate) fn quote_from_height(height: u32) -> u32 {
    (height * 70) / 1350
}

pub(crate) fn tag_from_height(height: u32) -> u32 {
    (height * 50) / 1350
}

pub(crate) fn measure_line(
    font: &rusttype::Font,
    text: &str,
    scale: rusttype::Scale,
) -> (f32, f32) {
    let width = font
        .layout(text, scale, rusttype::point(0.0, 0.0))
        .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .last()
        .unwrap_or(0.0);

    let v_metrics = font.v_metrics(scale);
    let height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

    (width, height)
}
