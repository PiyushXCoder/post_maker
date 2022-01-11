use crate::utils::{self, ImageContainer};
use fltk::{
    app, button::Button, draw, enums::Event, frame::Frame, group::Flex, prelude::*, window::Window,
};
use image::GenericImageView;
use std::{cell::RefCell, rc::Rc};

pub(crate) struct CropWindow {
    pub win: Window,
    apply_btn: Button,
    img_view: Frame,
    img: Rc<RefCell<ImageContainer>>,
    bound: Rc<RefCell<(u32, u32, u32, u32)>>,
}

impl CropWindow {
    pub(crate) fn new(container: Rc<RefCell<ImageContainer>>) -> Self {
        // let image = &container.borrow().image;
        let (image_width, image_height) = container.borrow().image.dimensions();
        let mut win = Window::default()
            .with_size(image_width as i32, 600)
            .with_label("Crop");

        let mut main_flex = Flex::default().size_of_parent().column();

        // Work area
        let mut center_row_flex = Flex::default().row();
        Frame::default();

        let mut center_col_flex = Flex::default().column();
        Frame::default();
        let img_view = Frame::default();
        Frame::default();
        center_col_flex.set_size(&img_view, image_height as i32);
        center_col_flex.end();

        Frame::default();
        center_row_flex.set_size(&center_col_flex, image_width as i32);
        center_row_flex.end();

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

        let (bound_width, bound_height) = utils::get_4_5(image_width, image_height);
        let bound_x = image_width / 2 - bound_width / 2;
        let bound_y = image_height / 2 - bound_height / 2;

        let mut crop_win = Self {
            win,
            apply_btn,
            img_view,
            img: Rc::clone(&container),
            bound: Rc::new(RefCell::new((bound_x, bound_y, bound_width, bound_height))),
        };

        crop_win.draw();
        crop_win.event();
        crop_win
    }

    fn draw(&mut self) {
        let cont = Rc::clone(&self.img);
        let bound = Rc::clone(&self.bound);
        self.img_view.draw(move |f| {
            let image = &cont.borrow().image;
            let (bound_x, bound_y, bound_width, bound_height) = *bound.borrow();
            draw::draw_image(
                image.as_rgb8().unwrap().as_raw(),
                f.x(),
                f.y(),
                image.width() as i32,
                image.height() as i32,
                fltk::enums::ColorDepth::Rgb8,
            )
            .unwrap();
            draw::set_color_rgb(255, 0, 0);
            draw::draw_rect(
                f.x() + bound_x as i32,
                f.y() + bound_y as i32,
                bound_width as i32,
                bound_height as i32,
            );
        });
    }

    fn event(&mut self) {
        let mut last: Option<(i32, i32)> = None;
        let cont = Rc::clone(&self.img);
        let bound = Rc::clone(&self.bound);
        self.img_view.handle(move |f, ev| {
            let image = &cont.borrow().image;
            let (bound_x, bound_y, bound_width, bound_height) = *bound.borrow();
            if ev == Event::Push {
                last = Some((app::event_x(), app::event_y()));
            } else if ev == Event::Drag {
                if let Some((lx, ly)) = last {
                    let dx = app::event_x() - lx;
                    if (dx > 0 && bound_x + bound_width < image.width()) || (dx < 0 && bound_x > 0)
                    {
                        let mut new_x = bound_x as i32 + dx;
                        if new_x + bound_width as i32 > image.width() as i32 {
                            new_x = (image.width() - bound_width) as i32
                        } else if new_x < 0 {
                            new_x = 0
                        }

                        bound.borrow_mut().0 = new_x as u32;
                    }

                    let dy = app::event_y() - ly;
                    if (dy > 0 && bound_y + bound_height < image.height())
                        || (dy < 0 && bound_y > 0)
                    {
                        let mut new_y = bound_y as i32 + dy;
                        if new_y + bound_height as i32 > image.height() as i32 {
                            new_y = (image.height() - bound_height) as i32
                        } else if new_y < 0 {
                            new_y = 0
                        }

                        bound.borrow_mut().1 = new_y as u32;
                    }

                    f.redraw();
                    last = Some((app::event_x(), app::event_y()));
                }
            } else if ev == Event::Released {
                last = None;
            }
            true
        });

        let mut wind = self.win.clone();
        let cont = Rc::clone(&self.img);
        let bound = Rc::clone(&self.bound);
        self.apply_btn.set_callback(move |_| {
            let (bound_x, bound_y, bound_width, bound_height) = *bound.borrow();

            let image = cont
                .borrow_mut()
                .image
                .crop(bound_x, bound_y, bound_width, bound_height);

            cont.borrow_mut().image = image;

            let (width, height) = cont.borrow().original_dimension;
            cont.borrow_mut().crop_position = Some((
                (bound_x * width) / bound_width,
                (bound_y * height) / bound_height,
            ));

            wind.do_callback();
        });
    }
}
