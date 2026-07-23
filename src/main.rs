mod framebuffer;

use framebuffer::Framebuffer;
use raylib::prelude::*;

fn main() {
    let mut framebuffer = Framebuffer::new(160, 120);

    framebuffer.set_background_color(Color::WHITE);
    framebuffer.clear();

    framebuffer.set_current_color(Color::new(120, 220, 255, 255));
    framebuffer.point(80, 60);

    framebuffer.render_to_file("baseline.png");

    println!("Reusable framebuffer validated successfully.");
}
