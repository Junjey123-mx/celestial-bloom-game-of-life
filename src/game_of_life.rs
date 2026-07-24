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

#[cfg(test)]
mod tests {
    use super::GameOfLife;
    use crate::{
        framebuffer::Framebuffer,
        patterns::{oscillators::BLINKER, place_pattern, still_lifes::BLOCK},
    };
    use raylib::prelude::Color;

    fn live_color() -> Color {
        Color::new(120, 220, 255, 255)
    }

    fn dead_color() -> Color {
        Color::WHITE
    }

    fn empty_board() -> Framebuffer {
        let mut framebuffer = Framebuffer::new(20, 20);

        framebuffer.set_background_color(dead_color());
        framebuffer.clear();
        framebuffer.set_current_color(live_color());

        framebuffer
    }

    fn snapshot(framebuffer: &Framebuffer) -> Vec<bool> {
        let mut cells = Vec::new();

        for y in 0..framebuffer.height() {
            for x in 0..framebuffer.width() {
                cells.push(framebuffer.get_color(x, y) == live_color());
            }
        }

        cells
    }

    #[test]
    fn isolated_cell_dies() {
        let mut framebuffer = empty_board();

        framebuffer.point(5, 5);

        let game = GameOfLife::new(live_color(), dead_color());

        game.next_generation(&mut framebuffer);

        assert_eq!(framebuffer.get_color(5, 5), dead_color());
    }

    #[test]
    fn dead_cell_with_three_neighbors_is_born() {
        let mut framebuffer = empty_board();

        framebuffer.point(4, 5);
        framebuffer.point(5, 4);
        framebuffer.point(6, 5);

        let game = GameOfLife::new(live_color(), dead_color());

        game.next_generation(&mut framebuffer);

        assert_eq!(framebuffer.get_color(5, 5), live_color());
    }

    #[test]
    fn cell_with_four_neighbors_dies() {
        let mut framebuffer = empty_board();

        framebuffer.point(5, 5);
        framebuffer.point(4, 5);
        framebuffer.point(6, 5);
        framebuffer.point(5, 4);
        framebuffer.point(5, 6);

        let game = GameOfLife::new(live_color(), dead_color());

        game.next_generation(&mut framebuffer);

        assert_eq!(framebuffer.get_color(5, 5), dead_color());
    }

    #[test]
    fn block_remains_stable() {
        let mut framebuffer = empty_board();

        place_pattern(&mut framebuffer, 5, 5, BLOCK);

        let before = snapshot(&framebuffer);

        let game = GameOfLife::new(live_color(), dead_color());

        game.next_generation(&mut framebuffer);

        assert_eq!(snapshot(&framebuffer), before);
    }

    #[test]
    fn blinker_has_period_two() {
        let mut framebuffer = empty_board();

        place_pattern(&mut framebuffer, 5, 5, BLINKER);

        let initial_state = snapshot(&framebuffer);

        let game = GameOfLife::new(live_color(), dead_color());

        game.next_generation(&mut framebuffer);

        assert_ne!(snapshot(&framebuffer), initial_state);

        game.next_generation(&mut framebuffer);

        assert_eq!(snapshot(&framebuffer), initial_state);
    }
}
