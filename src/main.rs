extern crate sdl2;

mod colors;
mod helpers;
mod snake;

use helpers::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use snake::*;
use std::time::Duration;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

const GRID_SIZE: i32 = 10;
const GRID_HEIGHT: i32 = 40;
const GRID_WIDTH: i32 = 40;
const GRID_XO: i32 = (WIDTH as i32 / 2) - (GRID_WIDTH * GRID_SIZE / 2);
const GRID_YO: i32 = (HEIGHT as i32 / 2) - (GRID_HEIGHT * GRID_SIZE / 2);

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
    let delay = 4;

    let mut snake = Snake::new();

    let mut fruit = new_fruit();

    'running: loop {
        frame += 1;
        canvas.set_draw_color(colors::black());
        canvas.clear();
        draw_grid_outline(&mut canvas);

        if frame % delay == 0 {
            if snake.head == fruit {
                snake.tick(true);
                fruit = new_fruit();
            } else {
                snake.tick(false);
            }
        }

        snake.draw(&mut canvas);
        draw_grid_square(fruit.0, fruit.1, colors::green(), &mut canvas);

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
                    snake.set_direction(Direction::Left);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    snake.set_direction(Direction::Right);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    snake.set_direction(Direction::Up);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    snake.set_direction(Direction::Down);
                }

                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
