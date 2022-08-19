/*
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#![windows_subsystem = "windows"]

#[macro_use]
extern crate log;
extern crate simplelog;

#[macro_use]
mod macros;

mod about_window;
mod config;
mod config_picker;
mod config_window;
mod crop_window;
mod dialog;
mod draw_thread;
mod export_all_window;
mod globals;
mod main_window;
mod result_ext;
mod utils;

use fltk::{
    app::{channel, App},
    prelude::*,
};
use fltk_theme::WidgetTheme;
use main_window::MainWindow;
use result_ext::ResultExt;
use simplelog::*;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub(crate) enum AppMessage {
    /// Copy recived image buffer from draw_thread to Buffer for fltk frame
    RedrawMainWindowImage(Option<Vec<u8>>),
    Message(String),
    Alert(String),
    ProgramPanicMessage(String),

    // Only for Main windows
    DeleteImage,
}

fn main() {
    let app = App::default();
    WidgetTheme::new(globals::THEME.clone().into()).apply();

    if let Err(e) = CombinedLogger::init(vec![
        WriteLogger::new(LevelFilter::Warn, Config::default(), config::log_file()),
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
    ]) {
        dialog::alert_default("Failed to start logger");
        panic!("Failed to start logger\n{:?}", e);
    }

    lazy_static::initialize(&globals::CONFIG);

    // Buffer which will br drawin on fltk frame
    let draw_buff: Arc<RwLock<Option<Vec<u8>>>> = Arc::new(RwLock::new(None));

    let (main_sender, main_receiver) = channel::<AppMessage>();
    *rw_write!(globals::MAIN_SENDER) = Some(main_sender);
    let mut main_window = MainWindow::new(Arc::clone(&draw_buff));

    while app.wait() {
        if let Some(msg) = main_receiver.recv() {
            match msg {
                AppMessage::RedrawMainWindowImage(data) => {
                    let mut buff = rw_write!(draw_buff);
                    *buff = data;
                    main_window.win.redraw();
                }
                AppMessage::Message(msg) => {
                    dialog::message_default(&msg);
                }
                AppMessage::Alert(msg) => dialog::alert_default(&msg),
                AppMessage::ProgramPanicMessage(msg) => {
                    dialog::message_default(&msg);
                    std::process::exit(1);
                }
                // Fltk does not show dialogs from other thread in windows, So this is hack to get things done
                AppMessage::DeleteImage => {
                    let ch = dialog::choice_default("Image is too small", "Delete", "Keep");
                    if ch == 0 {
                        main_window
                            .sender
                            .send(draw_thread::DrawMessage::Delete)
                            .unwrap();
                        main_window
                            .sender
                            .send(draw_thread::DrawMessage::Open)
                            .unwrap();
                        main_window.page.image.redraw();
                        main_window.file_choice.redraw();
                    }
                }
            }
        }
    }
}
