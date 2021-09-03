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

pub fn draw_grid_square(x: u32, y: u32, color: Color, canvas: &mut WindowCanvas) {
    let square_xo = crate::GRID_XO + x * crate::GRID_SIZE;
    let square_yo = crate::GRID_YO + y * crate::GRID_SIZE;

    let square = square(square_xo, square_yo, crate::GRID_SIZE - 2);

    canvas.set_draw_color(color);
    canvas.fill_rect(square).unwrap();
}

pub fn index_to_xy(index: u32) -> (u32, u32) {
    let x = index % crate::GRID_HEIGHT;
    let y = index / crate::GRID_WIDTH;
    (x, y)
}

fn rect(x: u32, y: u32, width: u32, height: u32) -> Rect {
    Rect::new(x as i32, (crate::HEIGHT - y - height) as i32, width, height)
}

fn square(x: u32, y: u32, size: u32) -> Rect {
    Rect::new(x as i32, (crate::HEIGHT - y - size) as i32, size, size)
}
