mod framebuffer;
mod game_of_life;
mod patterns;

use framebuffer::Framebuffer;
use game_of_life::GameOfLife;
use patterns::composition::seed_celestial_bloom;
use raylib::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    const WINDOW_WIDTH: i32 = 800;
    const WINDOW_HEIGHT: i32 = 600;

    const FRAMEBUFFER_WIDTH: i32 = 160;
    const FRAMEBUFFER_HEIGHT: i32 = 120;

    const GENERATION_DELAY_MS: u64 = 100;

    let live_color = Color::new(120, 220, 255, 255);
    let dead_color = Color::WHITE;

    let (mut window, raylib_thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Celestial Bloom — Conway's Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);

    framebuffer.set_background_color(dead_color);
    framebuffer.clear();
    framebuffer.set_current_color(live_color);

    seed_celestial_bloom(&mut framebuffer);

    let game = GameOfLife::new(live_color, dead_color);
    let mut last_generation = Instant::now();

    while !window.window_should_close() {
        if last_generation.elapsed() >= Duration::from_millis(GENERATION_DELAY_MS) {
            game.next_generation(&mut framebuffer);
            last_generation = Instant::now();
        }

        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
