use raylib::prelude::*;

pub struct Framebuffer {
    image: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let background_color = Color::BLACK;
        let current_color = Color::WHITE;

        let image = Image::gen_image_color(width, height, background_color);

        Self {
            image,
            background_color,
            current_color,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        self.image.clear_background(self.background_color);
    }

    pub fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.image.width() && y >= 0 && y < self.image.height() {
            self.image.draw_pixel(x, y, self.current_color);
        }
    }

    pub fn render_to_file(&self, file_name: &str) {
        self.image.export_image(file_name);
    }
}
