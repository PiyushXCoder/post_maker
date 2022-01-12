use lazy_static::lazy_static;
use rusttype::Font;
use std::io::Read;

lazy_static! {
    pub static ref FONT_QUOTE: Font<'static> = {
        let mut buffer = Vec::new();
        std::fs::File::open("ReenieBeanie-Regular.ttf")
            .unwrap()
            .read_to_end(&mut buffer)
            .unwrap();
        rusttype::Font::try_from_vec(buffer).unwrap()
    };
    pub static ref FONT_TAG: Font<'static> = {
        let mut buffer = Vec::new();
        std::fs::File::open("Kalam-Regular.ttf")
            .unwrap()
            .read_to_end(&mut buffer)
            .unwrap();
        rusttype::Font::try_from_vec(buffer).unwrap()
    };
}
