use crate::framebuffer::Framebuffer;

use super::{
    oscillators::{BEACON, BLINKER, PENTADECATHLON, PULSAR, TOAD},
    place_pattern,
    place_pattern_flipped,
    spaceships::{GLIDER, LWSS},
    still_lifes::{BEEHIVE, BLOCK, BOAT, LOAF, TUB},
};

fn sprinkle_points(framebuffer: &mut Framebuffer, points: &[(i32, i32)]) {
    for &(x, y) in points {
        framebuffer.point(x, y);
    }
}

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

fn seed_extra_particles(framebuffer: &mut Framebuffer) {
    // Partículas sueltas y pequeñas semillas
    // repartidas por el tablero para que se vea más vivo.
    sprinkle_points(
        framebuffer,
        &[
            // esquina superior izquierda
            (8, 8), (9, 8), (20, 15), (20, 16), (21, 15),

            // banda superior
            (38, 10), (50, 7), (57, 13), (64, 9), (71, 12),
            (90, 8), (98, 14), (108, 10), (116, 7),

            // esquina superior derecha
            (132, 9), (140, 11), (146, 8), (150, 12),

            // lado izquierdo
            (10, 40), (13, 48), (18, 56), (16, 66), (11, 78),

            // zona media superior
            (60, 35), (68, 31), (76, 36), (84, 30), (92, 35),

            // alrededor de la flor pero fuera del anillo inmediato
            (58, 46), (62, 52), (98, 48), (102, 54),
            (56, 86), (64, 91), (98, 88), (104, 93),

            // lado derecho
            (142, 42), (146, 56), (149, 68), (143, 82),

            // esquina inferior izquierda
            (14, 98), (18, 104), (24, 108), (30, 101),

            // banda inferior
            (48, 108), (60, 102), (74, 109), (90, 106), (106, 103),

            // esquina inferior derecha
            (126, 100), (136, 108), (144, 102), (151, 96),
        ],
    );

    // Mini grupos manuales para provocar nacimientos/movimiento.
    // Estos no son still lifes, solo "semillas" visuales.
    sprinkle_points(
        framebuffer,
        &[
            (34, 22), (35, 22), (36, 22), (36, 23),
            (52, 20), (53, 21), (54, 19), (54, 20),
            (101, 20), (102, 20), (103, 20), (102, 21),
            (123, 18), (124, 18), (124, 19), (125, 19),

            (22, 72), (23, 72), (24, 72), (23, 73),
            (36, 90), (37, 91), (38, 89), (38, 90),
            (118, 84), (119, 84), (120, 84), (119, 85),
            (132, 88), (133, 89), (134, 87), (134, 88),
        ],
    );
}

pub fn seed_celestial_bloom(framebuffer: &mut Framebuffer) {
    seed_flower(framebuffer);

    // =========================
    // Organismos principales
    // =========================

    // Zona superior izquierda.
    place_pattern(framebuffer, 12, 12, BLINKER);
    place_pattern(framebuffer, 25, 18, TOAD);
    place_pattern(framebuffer, 34, 8, GLIDER);

    // Zona superior central.
    place_pattern(framebuffer, 54, 12, BLINKER);
    place_pattern(framebuffer, 66, 16, BEACON);
    place_pattern(framebuffer, 92, 14, GLIDER);

    // Zona superior derecha.
    place_pattern(framebuffer, 132, 12, BEACON);
    place_pattern(framebuffer, 118, 24, GLIDER);
    place_pattern(framebuffer, 142, 22, TOAD);

    // Laterales.
    place_pattern(framebuffer, 16, 48, PULSAR);
    place_pattern(framebuffer, 130, 55, PENTADECATHLON);

    // Más vida en laterales medios.
    place_pattern(framebuffer, 8, 70, BLINKER);
    place_pattern(framebuffer, 18, 82, TOAD);
    place_pattern(framebuffer, 140, 72, BLINKER);
    place_pattern(framebuffer, 145, 86, BEACON);

    // Zona inferior izquierda.
    place_pattern(framebuffer, 20, 95, GLIDER);
    place_pattern(framebuffer, 34, 100, BLINKER);
    place_pattern(framebuffer, 44, 92, TOAD);

    // Zona inferior central.
    place_pattern(framebuffer, 92, 98, GLIDER);
    place_pattern(framebuffer, 72, 100, BEACON);

    // Zona inferior derecha.
    place_pattern_flipped(framebuffer, 120, 96, LWSS, true, false);
    place_pattern(framebuffer, 138, 98, GLIDER);

    // =========================
    // Still lifes decorativos
    // =========================
    place_pattern(framebuffer, 6, 52, BOAT);
    place_pattern(framebuffer, 10, 100, TUB);
    place_pattern(framebuffer, 48, 34, BLOCK);
    place_pattern(framebuffer, 108, 34, BOAT);
    place_pattern(framebuffer, 146, 46, TUB);
    place_pattern(framebuffer, 126, 14, BOAT);
    place_pattern(framebuffer, 62, 94, BLOCK);
    place_pattern(framebuffer, 112, 104, TUB);

    // =========================
    // Partículas extra
    // =========================
    seed_extra_particles(framebuffer);
}