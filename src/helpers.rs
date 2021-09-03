#![allow(dead_code)]
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub fn draw_rectangle(
    canvas: &mut WindowCanvas,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: Color,
) {
    canvas.set_draw_color(color);
    canvas.fill_rect(rect(x, y, width, height)).unwrap();
}

const GRID_SIZE: u32 = 16;
const GRID_XO: u32 = 300;
const GRID_YO: u32 = 300;

pub fn draw_grid_square(x: u32, y: u32, color: Color, canvas: &mut WindowCanvas) {
    let square_xo = GRID_XO + x * GRID_SIZE;
    let square_yo = GRID_YO + y * GRID_SIZE;

    let square = square(square_xo, square_yo, GRID_SIZE);

    canvas.set_draw_color(color);
    canvas.fill_rect(square).unwrap();
}

pub fn index_to_xy(index: usize) -> (u32, u32) {
    let x = index % 4;
    let y = index / 4;
    (x as u32, y as u32)
}

fn rect(x: u32, y: u32, width: u32, height: u32) -> Rect {
    Rect::new(x as i32, (crate::HEIGHT - y - height) as i32, width, height)
}

fn square(x: u32, y: u32, size: u32) -> Rect {
    Rect::new(x as i32, (crate::HEIGHT - y - size) as i32, size, size)
}
