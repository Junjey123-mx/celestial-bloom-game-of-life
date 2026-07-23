mod framebuffer;

use framebuffer::Framebuffer;
use raylib::prelude::*;

fn main() {
    const WINDOW_WIDTH: i32 = 800;
    const WINDOW_HEIGHT: i32 = 600;

    const FRAMEBUFFER_WIDTH: i32 = 160;
    const FRAMEBUFFER_HEIGHT: i32 = 120;

    let (mut window, raylib_thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Celestial Bloom — Conway's Game of Life")
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);

    framebuffer.set_background_color(Color::WHITE);
    framebuffer.clear();

    framebuffer.set_current_color(Color::new(120, 220, 255, 255));
    framebuffer.point(80, 60);

    while !window.window_should_close() {
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
