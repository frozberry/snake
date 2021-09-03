use crate::colors;
use crate::helpers::*;
use rand::Rng;
use sdl2::render::WindowCanvas;

#[derive(Debug, Clone)]
pub struct Fruit {
    pub coord: (i32, i32),
}

impl Fruit {
    pub fn new() -> Fruit {
        let mut rng = rand::thread_rng();
        Fruit {
            coord: (
                rng.gen_range(0..crate::GRID_WIDTH) as i32,
                rng.gen_range(0..crate::GRID_HEIGHT) as i32,
            ),
        }
    }

    pub fn render(&self, mut canvas: &mut WindowCanvas) {
        draw_grid_square(self.coord.0, self.coord.1, colors::green(), &mut canvas);
    }
}
