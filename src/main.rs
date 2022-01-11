mod config;
// mod crop_window;
mod main_window;
mod properties;
mod utils;

// use crop_window::CropWindow;
use fltk::{app::App, enums::Font};
use fltk_theme::WidgetTheme;
use main_window::MainWindow;
use std::{cell::RefCell, rc::Rc};
use utils::ImageContainer;

fn main() {
    let app = App::default();

    WidgetTheme::new(
        config::config()
            .theme
            .unwrap_or(config::Themes::System)
            .into(),
    )
    .apply();

    let f1 = Font::load_font("ReenieBeanie-Regular.ttf").unwrap();
    let f2 = Font::load_font("Kalam-Regular.ttf").unwrap();
    Font::set_font(Font::Times, &f1);
    Font::set_font(Font::TimesItalic, &f2);

    let container: Rc<RefCell<Option<ImageContainer>>> = Rc::new(RefCell::new(None));

    let main_win = MainWindow::new(Rc::clone(&container));
    app.run().unwrap();
}
