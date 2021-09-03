extern crate sdl2;

mod colors;
mod helpers;

use helpers::*;
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

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Snake {
    head: (i32, i32),
    tail: Vec<(i32, i32)>,
    direction: Direction,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            head: (10, 10),
            tail: vec![(11, 10), (12, 10), (31, 10), (14, 10)],
            direction: Direction::Up,
        }
    }

    fn tick(&mut self, fruit: bool) {
        let old_snake = self.head.clone();
        let old_tail = self.tail.clone();

        match self.direction {
            Direction::Up => self.head.1 += 1,
            Direction::Down => self.head.1 -= 1,
            Direction::Left => self.head.0 -= 1,
            Direction::Right => self.head.0 += 1,
        }
        if fruit {
            self.tail.insert(0, (self.head.0, self.head.1));
        }

        self.tail[0] = old_snake;
        for i in 1..self.tail.len() {
            self.tail[i] = old_tail[i - 1];
        }

        if self.tail.contains(&self.head) || !valid_coord(self.head.0, self.head.1) {
            self.head = (10, 10);
            self.tail = vec![(11, 10), (12, 10), (13, 10), (14, 10)];
            self.direction = Direction::Up;
        }
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
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

        draw_grid_square(snake.head.0, snake.head.1, colors::blue(), &mut canvas);
        for i in &snake.tail {
            draw_grid_square(i.0, i.1, colors::white(), &mut canvas);
        }
        draw_grid_square(fruit.0, fruit.1, colors::green(), &mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
