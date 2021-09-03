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

const GRID_SIZE: u32 = 5;
const GRID_XO: u32 = 300;
const GRID_YO: u32 = 100;
const GRID_HEIGHT: u32 = 50;
const GRID_WIDTH: u32 = 50;

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

    let mut board = [false; (GRID_WIDTH * GRID_HEIGHT) as usize];

    for i in 0..(GRID_WIDTH * GRID_HEIGHT) as usize {
        let mut rng = rand::thread_rng();

        board[i] = if rng.gen_range(0..2) == 0 {
            false
        } else {
            true
        };
    }

    'running: loop {
        canvas.set_draw_color(colors::black());
        canvas.clear();

        for (i, on) in board.iter().enumerate() {
            let x = index_to_xy(i as u32).0;
            let y = index_to_xy(i as u32).1;
            if *on {
                draw_grid_square(x, y, colors::white(), &mut canvas);
            }
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
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    for i in 0..(GRID_WIDTH * GRID_HEIGHT) as usize {
                        let mut rng = rand::thread_rng();

                        board[i] = if rng.gen_range(0..2) == 0 {
                            false
                        } else {
                            true
                        };
                    }
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

        draw_rectangle(&mut canvas, x, y, 40, 40, colors::red());

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
