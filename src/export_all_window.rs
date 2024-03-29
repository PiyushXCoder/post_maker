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

//! Picker to pick config if multiple configs are present or defalut config is not present
use crate::{
    config, dialog, globals,
    result_ext::ResultExt,
    utils::{self, ImageContainer, ImageInfo, ImageProperties, ImagePropertiesFile},
};
use bichannel::Channel;
use fltk::{
    app::{self},
    button::Button,
    enums,
    frame::Frame,
    group::Flex,
    image::SvgImage,
    misc::Progress,
    prelude::*,
    window::Window,
};
use std::{
    fs::File,
    sync::{Arc, RwLock},
    thread,
};

pub(crate) struct ExportAllWindow {
    pub(crate) win: Window,
    pub(crate) progress: Progress,
    pub(crate) image_name: Frame,
    pub(crate) close_btn: Button,
    pub(crate) images_list: Arc<RwLock<Vec<ImageInfo>>>,
    pub(crate) channel: Arc<RwLock<Option<Channel<ThreadMessage, ThreadMessage>>>>,
    pub(crate) finished: Arc<RwLock<bool>>,
}

impl ExportAllWindow {
    pub(crate) fn new(images_list: Arc<RwLock<Vec<ImageInfo>>>) -> Self {
        let mut win = Window::new(0, 0, 500, 130, "Export All").center_screen();
        win.set_icon(Some(
            SvgImage::from_data(globals::ICON.to_str().unwrap()).unwrap(),
        ));

        let progress_color = if *globals::THEME == config::Themes::Dark
            || *globals::THEME == config::Themes::HighContrast
        {
            enums::Color::rgb_color(0, 116, 80)
        } else {
            enums::Color::rgb_color(34, 203, 135)
        };

        let mut main_flex = Flex::default().size_of_parent().column();

        //label
        let mut panel_flex = Flex::default().row();
        panel_flex.set_size(&Frame::default(), 1);
        Frame::default()
            .with_label("Exporting all with quotes")
            .with_align(enums::Align::Left | enums::Align::Inside);
        panel_flex.end();
        main_flex.set_size(&panel_flex, 25);

        //image name
        let mut panel_flex = Flex::default().row();
        panel_flex.set_size(&Frame::default(), 1);
        let image_name = Frame::default()
            .with_label("")
            .with_align(enums::Align::Left | enums::Align::Inside);
        panel_flex.end();
        main_flex.set_size(&panel_flex, 25);

        // progress bar
        let mut panel_flex = Flex::default().row();
        Frame::default();
        let mut progress = Progress::default().with_label("Exporting...");
        progress.set_maximum(1.0);
        progress.set_value(0.0);
        progress.set_frame(enums::FrameType::ThinDownBox);
        progress.set_selection_color(progress_color);

        Frame::default();
        panel_flex.set_size(&progress, 490);
        panel_flex.end();
        main_flex.set_size(&panel_flex, 30);

        //close button
        let mut panel_flex = Flex::default().row();
        Frame::default();
        let close_btn = Button::default().with_label("Cancel");
        panel_flex.set_size(&Frame::default(), 1);
        panel_flex.set_size(&close_btn, 100);
        panel_flex.end();
        main_flex.set_size(&panel_flex, 30);

        main_flex.end();

        win.end();
        win.make_modal(true);

        let mut config_picker = Self {
            win,
            progress,
            image_name,
            close_btn,
            images_list,
            channel: Arc::new(RwLock::new(None)),
            finished: Arc::new(RwLock::new(false)),
        };
        config_picker.event();

        config_picker
    }

    pub(crate) fn export(&mut self) {
        self.image_name.set_label("");
        self.progress.set_label("Exporting...");
        self.close_btn.set_label("Cancel");
        self.progress.set_maximum(1.0);
        self.progress.set_value(0.0);
        *rw_write!(self.finished) = false;
        self.win.show();
        let (left, right) = bichannel::channel();
        *rw_write!(self.channel) = Some(left);
        spawn_export_thread(self, right);
        while self.win.shown() {
            if let Some(channel) = &*rw_read!(self.channel) {
                if let Ok(msg) = channel.try_recv() {
                    match msg {
                        ThreadMessage::HideWindow => {
                            self.win.hide();
                        }
                        _ => (),
                    }
                }
            }
            app::wait();
        }
    }

    // Set callbacks of elements
    fn event(&mut self) {
        // Close Button
        let channel = Arc::clone(&self.channel);
        let finished = Arc::clone(&self.finished);
        let mut win = self.win.clone();
        self.close_btn.set_callback(move |_| {
            if *rw_read!(finished) == true {
                win.hide();
            } else if dialog::choice_default("Are you sure?", "Yes", "No") == 0 {
                if let Some(c) = &*rw_read!(channel) {
                    c.send(ThreadMessage::Stop).error_log("Failed to stop task");
                }
            }
        });

        // Window Close
        let channel = Arc::clone(&self.channel);
        let finished = Arc::clone(&self.finished);
        self.win.set_callback(move |f| {
            if *rw_read!(finished) == true {
                f.hide();
            } else if dialog::choice_default("Are you sure?", "Yes", "No") == 0 {
                if let Some(c) = &*rw_read!(channel) {
                    c.send(ThreadMessage::Stop).error_log("Failed to stop task");
                }
            }
        });
    }
}

pub(crate) enum ThreadMessage {
    Stop,
    HideWindow,
}

fn spawn_export_thread(
    export_all: &mut ExportAllWindow,
    channel: Channel<ThreadMessage, ThreadMessage>,
) {
    let mut win = export_all.win.clone();
    let mut progress = export_all.progress.clone();
    let mut image_name = export_all.image_name.clone();
    let mut close_btn = export_all.close_btn.clone();
    let finished = Arc::clone(&export_all.finished);
    let images_list = Arc::clone(&export_all.images_list);

    thread::spawn(move || {
        let total = rw_read!(images_list).len();
        progress.set_maximum(total as f64);
        progress.set_value(0.0);

        let mut update_progress = |idx: usize| {
            progress.set_value(idx as f64 + 1.0);
            progress.set_label(&format!("[{}/{}]", idx + 1, total));
        };

        for (idx, image) in (*rw_read!(images_list)).iter().enumerate() {
            image_name.set_label(
                image
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            );
            let properties = Arc::new(RwLock::new(ImageProperties::default()));
            let container = ImageContainer::new(image, properties);
            let properties_file = utils::get_properties_path(image);
            let read = match File::open(&properties_file) {
                Ok(r) => r,
                Err(_) => {
                    update_progress(idx);
                    continue;
                }
            };
            let read = match serde_json::from_reader::<File, ImagePropertiesFile>(read) {
                Ok(r) => r,
                Err(_) => {
                    update_progress(idx);
                    continue;
                }
            };

            rw_write!(container.properties).merge(read, "", "");

            if rw_read!(container.properties).quote.trim().len() == 0 {
                update_progress(idx);
                continue;
            }

            container.save();

            update_progress(idx);
            win.redraw();
            app::awake();

            if let Ok(msg) = channel.try_recv() {
                match msg {
                    ThreadMessage::Stop => {
                        channel
                            .send(ThreadMessage::HideWindow)
                            .error_log("Failed to close window");
                        return;
                    }
                    _ => (),
                }
            }
        }
        image_name.set_label("Finished");
        progress.set_value(total as f64);
        close_btn.set_label("Close");
        *rw_write!(finished) = true;
        win.redraw();
        app::awake();
    });
}
