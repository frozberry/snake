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

pub fn rect(x: u32, y: u32, width: u32, height: u32) -> Rect {
    Rect::new(x as i32, (crate::HEIGHT - y - height) as i32, width, height)
}

pub fn square(x: u32, y: u32, size: u32) -> Rect {
    Rect::new(x as i32, (crate::HEIGHT - y - size) as i32, size, size)
}
