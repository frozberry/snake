extern crate sdl2;

mod colors;
mod helpers;

use helpers::*;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const SPEED: u32 = 10;

const GRID_SIZE: u32 = 10;
const GRID_HEIGHT: u32 = 40;
const GRID_WIDTH: u32 = 40;
const GRID_XO: u32 = (WIDTH / 2) - (GRID_WIDTH * GRID_SIZE / 2);
const GRID_YO: u32 = (HEIGHT / 2) - (GRID_HEIGHT * GRID_SIZE / 2);

enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    canvas.set_draw_color(colors::black());
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut speed = 0;
    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::Up;

    let mut snake: (u32, u32) = (10, 10);
    let mut tail: Vec<(u32, u32)> = vec![(11, 10), (12, 10), (13, 10), (14, 10)];

    'running: loop {
        canvas.set_draw_color(colors::black());
        canvas.clear();
        draw_grid_outline(&mut canvas);

        match direction {
            Direction::Up => y += speed,
            Direction::Down => y -= speed,
            Direction::Left => x -= speed,
            Direction::Right => x += speed,
        }

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

        draw_grid_square(snake.0, snake.1, colors::white(), &mut canvas);
        for i in &tail {
            draw_grid_square(i.0, i.1, colors::white(), &mut canvas);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
