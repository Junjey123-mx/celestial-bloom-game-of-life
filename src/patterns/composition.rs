use crate::framebuffer::Framebuffer;

use super::{
    place_pattern, place_pattern_flipped,
    still_lifes::{BEEHIVE, BLOCK, BOAT, LOAF, TUB},
};

pub fn seed_flower(framebuffer: &mut Framebuffer) {
    // Núcleo.
    place_pattern(framebuffer, 79, 59, BLOCK);

    // Pétalos superior e inferior.
    place_pattern(framebuffer, 78, 51, BEEHIVE);
    place_pattern_flipped(framebuffer, 78, 66, BEEHIVE, false, true);

    // Pétalos laterales.
    place_pattern(framebuffer, 68, 57, LOAF);
    place_pattern_flipped(framebuffer, 89, 57, LOAF, true, false);

    // Pétalos diagonales.
    place_pattern(framebuffer, 72, 51, BOAT);
    place_pattern_flipped(framebuffer, 86, 51, BOAT, true, false);

    // Tallo estable construido con bloques separados.
    for y in [72, 76, 80] {
        place_pattern(framebuffer, 79, y, BLOCK);
    }

    // Hojas estables.
    place_pattern(framebuffer, 72, 75, TUB);
    place_pattern(framebuffer, 86, 75, TUB);
}
