extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

enum Col {
    Black,
    White,
    Grey,
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let speed = 40;
    let mut x = 0;
    let mut y = 0;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        draw_rectangle(&mut canvas, x, y, 40, 40, Col::White);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => x -= speed,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => x += speed,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => y += speed,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => y -= speed,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_rectangle(canvas: &mut WindowCanvas, x: u32, y: u32, width: u32, height: u32, color: Col) {
    canvas.set_draw_color(Color::RGB(rgb(&color).0, rgb(&color).1, rgb(&color).2));
    canvas.fill_rect(rect(x, y, width, height)).unwrap();
}

fn rect(x: u32, y: u32, width: u32, height: u32) -> Rect {
    Rect::new(x as i32, (HEIGHT - y - height) as i32, width, height)
}

fn square(x: u32, y: u32, size: u32) -> Rect {
    Rect::new(x as i32, (HEIGHT - y - size) as i32, size, size)
}

fn rgb(color: &Col) -> (u8, u8, u8) {
    match color {
        Col::White => (255, 255, 255),
        Col::Black => (0, 0, 0),
        Col::Grey => (150, 150, 150),
    }
}
