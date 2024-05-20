#![windows_subsystem="windows"]

mod image_parsing_client;

use fltk::{app, button, group, window};
use fltk::app::{keyboard_screen_scaling, screen_scale, screen_size, set_screen_scale};
use fltk::draw::draw_image;
use fltk::enums::{Color, ColorDepth, Event, FrameType};
use fltk::frame::Frame;
use fltk::prelude::{GroupExt, ImageExt, WidgetBase, WidgetExt, WindowExt};
use crate::image_parsing_client::get_regions;

fn main() {
    let app = app::App::default();
    set_screen_scale(0, 1.0);
    let screen_size = screen_size();
    let screen_scale = screen_scale(0);
    let (window_width, window_height) = (screen_size.0 * 0.5 * screen_scale as f64, screen_size.1 * 0.5 * screen_scale as f64);
    let mut win = window::Window::new(0, 0, window_width as i32, window_height as i32, "Image to Buttons");
    keyboard_screen_scaling(true);
    win.make_resizable(true);
    win.set_color(Color::Black);

    // Put all elements in a group to force aspect ratio 16:9
    // The width/height of the group should equal the PNG image of the UI
    const UI_IMAGE_WIDTH: i32 = 1920;
    const UI_IMAGE_HEIGHT: i32 = 1080;
    let mut aspect_group = group::Group::new(0, 0, UI_IMAGE_WIDTH, UI_IMAGE_HEIGHT, "");
    aspect_group.end();

    // Use a frame as the new background
    let mut content_frame = Frame::new(
        aspect_group.x(),
        aspect_group.y(),
        UI_IMAGE_WIDTH,
        UI_IMAGE_HEIGHT,
        "",
    );
    content_frame.set_frame(FrameType::FlatBox);
    aspect_group.add(&content_frame);

    // Create a buffer of pixels
    let depth = 3;
    let buf = get_buffer(UI_IMAGE_WIDTH, UI_IMAGE_HEIGHT, depth);
    // Create an RgbImage from the buffer
    // let mut img = RgbImage::new(&buf, UI_IMAGE_WIDTH, UI_IMAGE_HEIGHT, ColorDepth::Rgb8).unwrap();
    content_frame.draw(move |f| {
        draw_image(&buf, f.x(), f.y(), UI_IMAGE_WIDTH, UI_IMAGE_HEIGHT, ColorDepth::Rgb8).expect("TODO: panic message");
    });

    // Load the image
    let path = "path_to_your_image.png";
    let regions = unsafe { get_regions(path) };

    // Create buttons based on the detected regions
    for region in regions {
        let min_x = region.bounds.0;
        let min_y = region.bounds.1;
        let max_x = region.bounds.2;
        let max_y = region.bounds.3;
        let width = (max_x - min_x + 1) as i32;
        let height = (max_y - min_y + 1) as i32;
        let uvx = width as f64 / UI_IMAGE_WIDTH as f64;
        let uvy = height as f64 / UI_IMAGE_HEIGHT as f64;
        let btn_width = uvx * aspect_group.width() as f64;
        let btn_height = uvy * aspect_group.height() as f64;
        let min_uvx = min_x as f64 / UI_IMAGE_WIDTH as f64;
        let min_uvy = min_y as f64 / UI_IMAGE_HEIGHT as f64;
        let btn_x = min_uvx * aspect_group.width() as f64;
        let btn_y = min_uvy * aspect_group.height() as f64;
        let mut btn = button::Button::new(btn_x as i32, btn_y as i32, btn_width as i32, btn_height as i32, "Press me");
        btn.set_color(Color::from_rgb(region.color[0], region.color[1], region.color[2]));
        btn.set_frame(FrameType::RoundedBox);
        aspect_group.add(&btn);
    }

    win.handle(move |w, ev| match ev {
        Event::Resize => {
            let (wx, wy) = (w.width(), w.height());
            let (new_width, new_height) = if wx as f32 / wy as f32 > 16.0 / 9.0 {
                // Window is too wide
                ((16.0 / 9.0 * wy as f32) as i32, wy)
            } else {
                // Window is too tall
                (wx, (9.0 / 16.0 * wx as f32) as i32)
            };

            let new_x = (wx - new_width) / 2;
            let new_y = (wy - new_height) / 2;

            aspect_group.resize(new_x, new_y, new_width, new_height);
            true
        },
        _ => false,
    });

    // win.fullscreen(true);
    win.end();
    win.show();
    app.run().unwrap();
}

fn get_buffer(width: i32, height: i32, depth: i32) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![0; (width * height * depth) as usize];
    // Fill the buffer with some colors
    for i in 0..height {
        for j in 0..width {
            let offset: usize = ((i * width + j) * depth) as usize;
            buf[offset] = (j % 255) as u8; // Red
            buf[offset + 1] = (i % 255) as u8; // Green
            buf[offset + 2] = ((i + j) % 255) as u8; // Blue
        }
    }
    buf
}

