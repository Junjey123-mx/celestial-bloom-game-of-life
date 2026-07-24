pub mod composition;
pub mod still_lifes;

use crate::framebuffer::Framebuffer;

pub type Pattern = &'static [(i32, i32)];

pub fn place_pattern(
    framebuffer: &mut Framebuffer,
    origin_x: i32,
    origin_y: i32,
    pattern: Pattern,
) {
    place_pattern_flipped(framebuffer, origin_x, origin_y, pattern, false, false);
}

pub fn place_pattern_flipped(
    framebuffer: &mut Framebuffer,
    origin_x: i32,
    origin_y: i32,
    pattern: Pattern,
    flip_x: bool,
    flip_y: bool,
) {
    let max_x = pattern.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let max_y = pattern.iter().map(|(_, y)| *y).max().unwrap_or(0);

    for &(x, y) in pattern {
        let transformed_x = if flip_x { max_x - x } else { x };
        let transformed_y = if flip_y { max_y - y } else { y };

        framebuffer.point(origin_x + transformed_x, origin_y + transformed_y);
    }
}
