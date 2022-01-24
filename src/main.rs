#![windows_subsystem = "windows"]

#[macro_use]
extern crate log;
extern crate simplelog;

mod config;
mod config_picker;
mod config_window;
mod crop_window;
mod draw_thread;
mod globals;
mod main_window;
mod utils;

use fltk::{
    app::{channel, App},
    dialog,
    prelude::*,
};
use fltk_theme::WidgetTheme;
use simplelog::*;

use main_window::MainWindow;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub(crate) enum AppMessage {
    RedrawMainWindowImage(Option<Vec<u8>>),
}

fn main() {
    let app = App::default();
    WidgetTheme::new(globals::THEME.clone().into()).apply();

    if let Err(e) = CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        config::log_file(),
    )]) {
        dialog::alert_default("Failed to start logger");
        panic!("Failed to start logger\n{:?}", e);
    }

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
