use crate::globals;
use fltk::{
    app,
    browser::{Browser, BrowserType},
    button::Button,
    frame::Frame,
    group::Flex,
    image::SvgImage,
    prelude::*,
    window::Window,
};

pub(crate) struct ConfigPicker {
    pub(crate) win: Window,
    pub(crate) browse: Browser,
    pub(crate) apply_btn: Button,
    pub(crate) configs: Vec<String>,
}

impl ConfigPicker {
    pub(crate) fn new(configs: Vec<String>) -> Self {
        let mut win = Window::new(0, 0, 500, 400, "Configurations").center_screen();
        win.set_icon(Some(
            SvgImage::from_data(globals::ICON.to_str().unwrap()).unwrap(),
        ));

        let mut main_flex = Flex::default().size_of_parent().column();

        // Work area
        let mut browse = Browser::default().with_type(BrowserType::Hold);
        for name in &configs {
            browse.add(&name);
        }
        browse.select(1);

        // Panel
        let top_padding_btn = Frame::default();
        let mut panel_flex = Flex::default().row();
        Frame::default();
        let apply_btn = Button::default().with_label("apply");
        Frame::default();
        panel_flex.set_size(&apply_btn, 100);
        panel_flex.end();
        let bottom_padding_btn = Frame::default();

        main_flex.set_size(&top_padding_btn, 5);
        main_flex.set_size(&panel_flex, 30);
        main_flex.set_size(&bottom_padding_btn, 5);
        main_flex.end();

        win.end();
        win.make_resizable(true);

        let mut config_picker = Self {
            win,
            browse,
            apply_btn,
            configs,
        };
        config_picker.event();

        config_picker.win.show();
        while config_picker.win.shown() {
            app::wait();
        }
        config_picker
    }

    fn event(&mut self) {
        let mut win = self.win.clone();
        self.apply_btn.set_callback(move |_| {
            win.hide();
        });
    }

    pub(crate) fn selected(&self) -> Option<String> {
        let idx = self.browse.value();
        if idx == 0 {
            None
        } else {
            self.configs.get(idx as usize - 1).map(|a| a.to_owned())
        }
    }
}
