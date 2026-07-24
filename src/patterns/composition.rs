use crate::framebuffer::Framebuffer;

use super::{
    oscillators::{BEACON, BLINKER, PENTADECATHLON, PULSAR, TOAD},
    place_pattern, place_pattern_flipped,
    spaceships::{GLIDER, LWSS},
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

    // Tallo estable.
    for y in [72, 76, 80] {
        place_pattern(framebuffer, 79, y, BLOCK);
    }

    // Hojas.
    place_pattern(framebuffer, 72, 75, TUB);
    place_pattern(framebuffer, 86, 75, TUB);
}

pub fn seed_celestial_bloom(framebuffer: &mut Framebuffer) {
    seed_flower(framebuffer);

    // Zona superior izquierda.
    place_pattern(framebuffer, 12, 12, BLINKER);
    place_pattern(framebuffer, 25, 18, TOAD);

    // Zona superior derecha.
    place_pattern(framebuffer, 132, 12, BEACON);
    place_pattern(framebuffer, 118, 24, GLIDER);

    // Laterales.
    place_pattern(framebuffer, 16, 48, PULSAR);
    place_pattern(framebuffer, 130, 55, PENTADECATHLON);

    // Zona inferior izquierda.
    place_pattern(framebuffer, 20, 95, GLIDER);

    // Zona inferior derecha. Se refleja para viajar hacia el borde.
    place_pattern_flipped(framebuffer, 120, 96, LWSS, true, false);
}
