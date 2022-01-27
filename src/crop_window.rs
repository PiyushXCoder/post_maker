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

//! Window to change Crop properties of image
use crate::{
    globals,
    utils::{self, Coord, ImageContainer, ImageProperties},
};
use fltk::{
    app, button::Button, draw, enums::Event, frame::Frame, group::Flex, image::SvgImage,
    prelude::*, window::Window,
};
use image::GenericImageView;
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, RwLock},
};

/// Window to crop the existing image
pub(crate) struct CropWindow {
    pub(crate) win: Window,
    apply_btn: Button,
    cancel_btn: Button,
    container: Rc<RefCell<Option<ImageContainer>>>,
    page: Page,
}

#[derive(Clone)]
pub(crate) struct Page {
    pub(crate) image_view: Frame,
    pub(crate) row_flex: Flex,
    pub(crate) col_flex: Flex,
}

impl CropWindow {
    pub(crate) fn new() -> Self {
        let mut win = Window::new(0, 0, 500, 600, "Crop").center_screen();
        win.set_icon(Some(
            SvgImage::from_data(globals::ICON.to_str().unwrap()).unwrap(),
        ));

        let mut main_flex = Flex::default().size_of_parent().column();

        // Work area
        let center_row_flex = Flex::default().row();
        Frame::default();

        let center_col_flex = Flex::default().column();
        Frame::default();
        let img_view = Frame::default();
        Frame::default();
        center_col_flex.end();

        Frame::default();
        center_row_flex.end();

        // Panel
        let top_padding_btn = Frame::default();
        let mut panel_flex = Flex::default().row();
        Frame::default();
        let apply_btn = Button::default().with_label("Apply");
        panel_flex.set_size(&apply_btn, 100);
        let cancel_btn = Button::default().with_label("Cancel");
        panel_flex.set_size(&cancel_btn, 100);
        panel_flex.set_size(&Frame::default(), 10);
        panel_flex.end();
        let bottom_padding_btn = Frame::default();

        main_flex.set_size(&top_padding_btn, 5);
        main_flex.set_size(&panel_flex, 30);
        main_flex.set_size(&bottom_padding_btn, 5);
        main_flex.end();

        win.end();
        win.make_modal(true);
        win.make_resizable(true);

        let mut crop_win = Self {
            win,
            apply_btn,
            cancel_btn,
            container: Rc::new(RefCell::new(None)),
            page: Page {
                image_view: img_view,
                row_flex: center_row_flex,
                col_flex: center_col_flex,
            },
        };

        crop_win.draw();
        crop_win.event();
        crop_win
    }

    /// Call it to show window to crop image
    pub(crate) fn load_to_crop(
        &mut self,
        path: &PathBuf,
        crop_pos: Option<(f64, f64)>,
    ) -> Option<(f64, f64)> {
        let mut container =
            ImageContainer::new(path, Arc::new(RwLock::new(ImageProperties::default())));
        {
            let prop = &mut container.properties.write().unwrap();
            prop.dimension = prop.original_dimension;
            prop.crop_position = match crop_pos {
                Some(a) => Some(a),
                None => Some((0.0, 0.0)),
            };
        }

        container.apply_resize();
        let (image_width, image_height): (f64, f64) =
            Coord::from(container.image.dimensions()).into();
        self.win.set_size(image_width as i32, 600);

        self.page
            .row_flex
            .set_size(&self.page.col_flex, image_width as i32);
        self.page.row_flex.recalc();

        self.page
            .col_flex
            .set_size(&self.page.image_view, image_height as i32);
        self.page.col_flex.recalc();

        *self.container.borrow_mut() = Some(container);

        self.page.image_view.redraw();
        self.win.show();
        while self.win.shown() {
            app::wait();
        }

        if let Some(cont) = &*self.container.borrow() {
            cont.properties.read().unwrap().crop_position
        } else {
            None
        }
    }

    /// Set drawing in window
    fn draw(&mut self) {
        let container = Rc::clone(&self.container);
        self.page.image_view.draw(move |f| {
            if let Some(cont) = &*container.borrow() {
                let image = &cont.buffer;

                draw::draw_image(
                    image.as_rgb8().unwrap().as_raw(),
                    f.x(),
                    f.y(),
                    image.width() as i32,
                    image.height() as i32,
                    fltk::enums::ColorDepth::Rgb8,
                )
                .unwrap();

                let prop = cont.properties.read().unwrap();
                let (original_width, original_height) = prop.original_dimension;
                let (original_x, original_y) = prop.crop_position.unwrap();
                let (resized_width, resized_height) = (image.width() as f64, image.height() as f64);
                let (bound_width, bound_height) =
                    utils::croped_ratio(resized_width, resized_height);

                let (bound_x, bound_y) = (
                    (original_x * resized_width as f64) / original_width,
                    (original_y * resized_height as f64) / original_height,
                );

                draw::set_color_rgb(255, 0, 0);
                draw::draw_rect(
                    f.x() + bound_x as i32,
                    f.y() + bound_y as i32,
                    bound_width as i32,
                    bound_height as i32,
                );
            }
        });
    }

    /// Set callbacks of elements
    fn event(&mut self) {
        // Handle mosue events for crop area in image view
        let mut last: Option<(f64, f64)> = None;
        let container = Rc::clone(&self.container);
        self.page.image_view.handle(move |f, ev| {
            if let Some(cont) = &*container.borrow_mut() {
                let image = &cont.buffer;

                let mut prop = cont.properties.write().unwrap();

                let (original_x, original_y) = match prop.crop_position {
                    Some(v) => v,
                    None => return true,
                };

                let (original_width, original_heigth) = prop.original_dimension;
                let (original_bound_width, original_bound_height) =
                    utils::croped_ratio(original_width, original_heigth);
                let point = original_width / image.width() as f64;
                let (event_x, event_y) = (
                    (app::event_x() - f.x()) as f64 * point,
                    (app::event_y() - f.y()) as f64 * point,
                );
                if ev == Event::Push {
                    last = Some((event_x, event_y));
                } else if ev == Event::Drag {
                    if let Some((lx, ly)) = last {
                        let dx = event_x - lx;
                        if (dx > 0.0 && original_x + original_bound_width < original_width)
                            || (dx < 0.0 && original_x > 0.0)
                        {
                            let mut new_x = original_x + dx;
                            if new_x + original_bound_width > original_width {
                                new_x = original_width - original_bound_width;
                            } else if new_x < 0.0 {
                                new_x = 0.0;
                            }

                            prop.crop_position = prop.crop_position.map(|(_, y)| (new_x, y));
                        }

                        let dy = event_y - ly;
                        if (dy > 0.0 && original_y + original_bound_height < original_heigth)
                            || (dy < 0.0 && original_y > 0.0)
                        {
                            let mut new_y = original_y + dy;
                            if new_y + original_bound_height > original_heigth {
                                new_y = original_heigth - original_bound_height;
                            } else if new_y < 0.0 {
                                new_y = 0.0;
                            }

                            prop.crop_position = prop.crop_position.map(|(x, _)| (x, new_y));
                        }

                        f.redraw();
                        last = Some((event_x, event_y));
                    }
                } else if ev == Event::Released {
                    last = None;
                }
            }
            true
        });

        // Window close
        let mut win = self.win.clone();
        self.apply_btn.set_callback(move |_| {
            win.hide();
        });

        let mut win = self.win.clone();
        self.cancel_btn.set_callback(move |_| {
            win.do_callback();
        });

        let container = Rc::clone(&self.container);
        self.win.set_callback(move |f| {
            if let Some(cont) = &*container.borrow_mut() {
                cont.properties.write().unwrap().crop_position = None;
            }
            f.hide();
        });
    }
}
