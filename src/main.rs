extern crate sdl2;

mod colors;
mod helpers;

use helpers::*;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

const GRID_SIZE: i32 = 10;
const GRID_HEIGHT: i32 = 40;
const GRID_WIDTH: i32 = 40;
const GRID_XO: i32 = (WIDTH as i32 / 2) - (GRID_WIDTH * GRID_SIZE / 2);
const GRID_YO: i32 = (HEIGHT as i32 / 2) - (GRID_HEIGHT * GRID_SIZE / 2);

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
        .window("rust-sdl2 demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(colors::black());
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut frame = 0;
    let mut delay = 4;
    let mut direction = Direction::Up;

    let mut snake: (i32, i32) = (10, 10);
    let mut tail: Vec<(i32, i32)> = vec![(11, 10), (12, 10), (13, 10), (14, 10)];

    let mut rng = rand::thread_rng();
    let mut fruit = (
        rng.gen_range(0..GRID_WIDTH) as i32,
        rng.gen_range(0..GRID_HEIGHT) as i32,
    );

    'running: loop {
        frame += 1;
        canvas.set_draw_color(colors::black());
        canvas.clear();
        draw_grid_outline(&mut canvas);

        if frame % delay == 0 {
            let old_snake = snake.clone();
            let old_tail = tail.clone();

            match direction {
                Direction::Up => snake.1 += 1,
                Direction::Down => snake.1 -= 1,
                Direction::Left => snake.0 -= 1,
                Direction::Right => snake.0 += 1,
            }
            if snake == fruit {
                tail.insert(0, (snake.0, snake.1));
                fruit = (
                    rng.gen_range(0..GRID_WIDTH) as i32,
                    rng.gen_range(0..GRID_HEIGHT) as i32,
                );
            }

            tail[0] = old_snake;
            for i in 1..tail.len() {
                tail[i] = old_tail[i - 1];
            }

            if tail.contains(&snake) || !valid_coord(snake.0, snake.1) {
                snake = (10, 10);
                tail = vec![(11, 10), (12, 10), (13, 10), (14, 10)];
                direction = Direction::Up;
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
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    direction = Direction::Left;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    direction = Direction::Right;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    direction = Direction::Up;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    direction = Direction::Down;
                }

                _ => {}
            }
        }

        draw_grid_square(snake.0, snake.1, colors::blue(), &mut canvas);
        for i in &tail {
            draw_grid_square(i.0, i.1, colors::white(), &mut canvas);
        }
        draw_grid_square(fruit.0, fruit.1, colors::green(), &mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
