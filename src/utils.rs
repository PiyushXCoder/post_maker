use fltk::prelude::ImageExt;
use image::{DynamicImage, GenericImageView, ImageBuffer};

#[derive(Debug)]
pub(crate) struct ImageContainer {
    pub(crate) image: DynamicImage,
    pub(crate) original_dimension: (u32, u32),
    pub(crate) crop_position: Option<(u32, u32)>,
    pub(crate) quote: String,
    pub(crate) tag: String,
    pub(crate) quote_position: u32,
    pub(crate) tag_position: u32,
    pub(crate) rgba: [u8; 4],
    pub(crate) is_saved: bool,
}

impl ImageContainer {
    pub(crate) fn new(path: &str) -> Self {
        let img = image::open(path).unwrap();
        let (width, height) = img.dimensions();
        let (s_width, s_height) = ((width * 500) / height, 500);
        let mut img = img.resize(s_width, s_height, image::imageops::FilterType::Triangle);

        Self {
            image: img,
            original_dimension: (width, height),
            crop_position: None,
            quote: String::new(),
            tag: String::new(),
            quote_position: (width * 2) / 3,
            tag_position: width / 2,
            rgba: [0; 4],
            is_saved: true,
        }
    }

    pub(crate) fn apply_crop(&mut self) {
        let (original_width, original_height) = self.original_dimension;
        let (origina_crop_width, origina_crop_height) = get_4_5(original_width, original_height);
        self.crop_position = Some((
            original_width / 2 - origina_crop_width / 2,
            original_height / 2 - origina_crop_height / 2,
        ));

        let (s_width, s_height) = self.image.dimensions();
        let (c_width, c_height) = get_4_5(s_width, s_height);
        let (cx, cy) = (s_width / 2 - c_width / 2, s_height / 2 - c_height / 2);

        self.image = self.image.crop(cx, cy, c_width, c_height);
    }

    pub(crate) fn apply_crop_pos(&mut self, original_x: u32, original_y: u32) {
        let (original_width, original_height) = self.original_dimension;
        self.crop_position = Some((original_x, original_y));

        let (s_width, s_height) = self.image.dimensions();
        let (c_width, c_height) = get_4_5(s_width, s_height);
        let (cx, cy) = (
            (original_x * s_width) / original_width,
            (original_y * s_height) / original_height,
        );

        self.image = self.image.crop(cx, cy, c_width, c_height);
    }

    pub(crate) fn apply_layer(&mut self) {
        let (width, height) = self.image.dimensions();
        let layer = DynamicImage::ImageRgba8(ImageBuffer::from_fn(width, height, |_, _| {
            image::Rgba(self.rgba)
        }));
        image::imageops::overlay(&mut self.image, &layer, 0, 0);
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
    (height * 65) / 1556
}

pub(crate) fn id_from_height(height: u32) -> u32 {
    (height * 30) / 1556
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
