extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const SPEED: u32 = 10;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Col {
    r: u8,
    g: u8,
    b: u8,
}

pub fn main() {
    let white = Col {
        r: 255,
        g: 255,
        b: 255,
    };
    let grey = Col {
        r: 150,
        g: 150,
        b: 150,
    };
    let black = Col { r: 0, g: 0, b: 0 };

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

    let mut speed = 0;
    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::Up;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

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
                } => {
                    direction = Direction::Left;
                    speed = SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    direction = Direction::Right;
                    speed = SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    direction = Direction::Up;
                    speed = SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    direction = Direction::Down;
                    speed = SPEED;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    speed = 0;
                }

                _ => {}
            }
        }

        match direction {
            Direction::Up => y += speed,
            Direction::Down => y -= speed,
            Direction::Left => x -= speed,
            Direction::Right => x += speed,
        }

        draw_rectangle(&mut canvas, x, y, 40, 40, white);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_rectangle(canvas: &mut WindowCanvas, x: u32, y: u32, width: u32, height: u32, color: Col) {
    canvas.set_draw_color(Color::RGB(color.r, color.g, color.b));
    canvas.fill_rect(rect(x, y, width, height)).unwrap();
}

fn rect(x: u32, y: u32, width: u32, height: u32) -> Rect {
    Rect::new(x as i32, (HEIGHT - y - height) as i32, width, height)
}

fn square(x: u32, y: u32, size: u32) -> Rect {
    Rect::new(x as i32, (HEIGHT - y - size) as i32, size, size)
}
