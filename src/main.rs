mod framebuffer;
mod game_of_life;
mod patterns;

use framebuffer::Framebuffer;
use game_of_life::GameOfLife;
use patterns::composition::seed_celestial_bloom;
use raylib::prelude::*;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

const FRAMEBUFFER_WIDTH: i32 = 160;
const FRAMEBUFFER_HEIGHT: i32 = 120;

const INITIAL_GENERATION_DELAY_MS: u64 = 100;
const MIN_GENERATION_DELAY_MS: u64 = 40;
const MAX_GENERATION_DELAY_MS: u64 = 500;
const SPEED_STEP_MS: u64 = 20;

fn initialize_board(framebuffer: &mut Framebuffer, live_color: Color, dead_color: Color) {
    framebuffer.set_background_color(dead_color);
    framebuffer.clear();
    framebuffer.set_current_color(live_color);

    seed_celestial_bloom(framebuffer);
}

fn main() {
    let live_color = Color::new(120, 220, 255, 255);
    let dead_color = Color::WHITE;

    let (mut window, raylib_thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Celestial Bloom — Conway's Game of Life")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);

    initialize_board(&mut framebuffer, live_color, dead_color);

    let game = GameOfLife::new(live_color, dead_color);

    let mut paused = false;
    let mut generation: u64 = 0;
    let mut generation_delay_ms = INITIAL_GENERATION_DELAY_MS;
    let mut last_generation = Instant::now();

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
            last_generation = Instant::now();
        }

        if window.is_key_pressed(KeyboardKey::KEY_R) {
            initialize_board(&mut framebuffer, live_color, dead_color);

            generation = 0;
            last_generation = Instant::now();
        }

        if window.is_key_pressed(KeyboardKey::KEY_UP) {
            generation_delay_ms = generation_delay_ms
                .saturating_sub(SPEED_STEP_MS)
                .max(MIN_GENERATION_DELAY_MS);
        }

        if window.is_key_pressed(KeyboardKey::KEY_DOWN) {
            generation_delay_ms =
                (generation_delay_ms + SPEED_STEP_MS).min(MAX_GENERATION_DELAY_MS);
        }

        if window.is_key_pressed(KeyboardKey::KEY_S) {
            framebuffer.render_to_file("snapshot.png");
            println!("Snapshot saved as snapshot.png");
        }

        let manual_step = paused && window.is_key_pressed(KeyboardKey::KEY_N);

        let automatic_step =
            !paused && last_generation.elapsed() >= Duration::from_millis(generation_delay_ms);

        if manual_step || automatic_step {
            game.next_generation(&mut framebuffer);
            generation += 1;
            last_generation = Instant::now();
        }

        let status = if paused { "PAUSED" } else { "RUNNING" };

        let title = format!(
            "Celestial Bloom | {status} | Generation: {generation} | Delay: {generation_delay_ms} ms"
        );

        window.set_window_title(&raylib_thread, &title);

        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
