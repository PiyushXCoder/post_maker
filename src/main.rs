#![windows_subsystem = "windows"]

mod config;
mod config_picker;
mod config_window;
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
    RedrawMainWindowImage(Option<Vec<u8>>),
}

fn main() {
    let app = App::default();
    WidgetTheme::new(globals::THEME.clone().into()).apply();
    lazy_static::initialize(&globals::CONFIG);

    let draw_buff: Arc<RwLock<Option<Vec<u8>>>> = Arc::new(RwLock::new(None));
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
