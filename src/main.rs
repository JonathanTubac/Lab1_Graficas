mod framebuffer;

use raylib::prelude::*;
use framebuffer::Framebuffer;

fn main() {
    let width = 800;
    let height = 500;
    let mut framebuffer = Framebuffer::new(width, height);

    let bg = Color::new(30, 30, 30, 255);
    framebuffer.set_background_color(bg);
    framebuffer.clear();

    framebuffer.render_to_png("out.png");

    println!("Hello, world!");
}
