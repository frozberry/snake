#![allow(dead_code)]
use crate::colors;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub fn new_fruit() -> (i32, i32) {
    let mut rng = rand::thread_rng();
    let fruit = (
        rng.gen_range(0..crate::GRID_WIDTH) as i32,
        rng.gen_range(0..crate::GRID_HEIGHT) as i32,
    );
    fruit
}

pub fn draw_grid_square(x: i32, y: i32, color: Color, canvas: &mut WindowCanvas) {
    let square_xo = crate::GRID_XO + x * crate::GRID_SIZE;
    let square_yo = crate::GRID_YO + y * crate::GRID_SIZE;

    let square = square(square_xo, square_yo, crate::GRID_SIZE - 2);

    canvas.set_draw_color(color);
    canvas.fill_rect(square).unwrap();
}

pub fn draw_grid_outline(canvas: &mut WindowCanvas) {
    let grid = rect(
        crate::GRID_XO,
        crate::GRID_YO,
        crate::GRID_WIDTH * crate::GRID_SIZE,
        crate::GRID_HEIGHT * crate::GRID_SIZE,
    );
    canvas.set_draw_color(colors::white());
    canvas.draw_rect(grid).unwrap();
}

fn index_to_xy(index: i32) -> (i32, i32) {
    let x = index % crate::GRID_HEIGHT;
    let y = index / crate::GRID_WIDTH;
    (x, y)
}

fn draw_rectangle(
    canvas: &mut WindowCanvas,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color,
) {
    canvas.set_draw_color(color);
    canvas.fill_rect(rect(x, y, width, height)).unwrap();
}

fn rect(x: i32, y: i32, width: i32, height: i32) -> Rect {
    Rect::new(
        x as i32,
        (crate::HEIGHT - y - height) as i32,
        width as u32,
        height as u32,
    )
}

fn square(x: i32, y: i32, size: i32) -> Rect {
    Rect::new(
        x as i32,
        (crate::HEIGHT - y - size) as i32,
        size as u32,
        size as u32,
    )
}
