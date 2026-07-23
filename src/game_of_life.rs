use crate::framebuffer::Framebuffer;
use raylib::prelude::Color;

pub struct GameOfLife {
    live_color: Color,
    dead_color: Color,
}

impl GameOfLife {
    pub fn new(live_color: Color, dead_color: Color) -> Self {
        Self {
            live_color,
            dead_color,
        }
    }

    fn is_alive(&self, framebuffer: &Framebuffer, x: i32, y: i32) -> bool {
        framebuffer.get_color(x, y) == self.live_color
    }

    fn count_live_neighbors(&self, framebuffer: &Framebuffer, x: i32, y: i32) -> u8 {
        let mut neighbors = 0;

        for offset_y in -1..=1 {
            for offset_x in -1..=1 {
                if offset_x == 0 && offset_y == 0 {
                    continue;
                }

                if self.is_alive(framebuffer, x + offset_x, y + offset_y) {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    pub fn next_generation(&self, framebuffer: &mut Framebuffer) {
        let width = framebuffer.width();
        let height = framebuffer.height();

        let mut next_generation = vec![false; (width * height) as usize];

        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize;
                let alive = self.is_alive(framebuffer, x, y);
                let neighbors = self.count_live_neighbors(framebuffer, x, y);

                next_generation[index] =
                    matches!((alive, neighbors), (true, 2) | (true, 3) | (false, 3));
            }
        }

        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize;

                let color = if next_generation[index] {
                    self.live_color
                } else {
                    self.dead_color
                };

                framebuffer.set_current_color(color);
                framebuffer.point(x, y);
            }
        }
    }
}
