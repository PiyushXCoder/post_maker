mod config;
mod crop_window;
mod draw_thread;
mod globals;
mod main_window;
mod utils;

// use crop_window::CropWindow;
use fltk::{
    app::{channel, App},
    prelude::*,
};
use fltk_theme::WidgetTheme;

use main_window::MainWindow;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub(crate) enum AppMessage {
    RedrawMainWindowImage(Vec<u8>),
}

fn main() {
    let app = App::default();

    WidgetTheme::new(
        config::config()
            .theme
            .unwrap_or(config::Themes::System)
            .into(),
    )
    .apply();

    let draw_buff: Arc<RwLock<Vec<u8>>> = Arc::new(RwLock::new(vec![]));
    let (main_sender, main_receiver) = channel::<AppMessage>();
    let mut main_window = MainWindow::new(main_sender, Arc::clone(&draw_buff));

    while app.wait() {
        if let Some(msg) = main_receiver.recv() {
            match msg {
                AppMessage::RedrawMainWindowImage(data) => {
                    let mut buff = draw_buff.write().unwrap();
                    *buff = data;
                    main_window.win.redraw();
                }
            }
        }
    }
}
