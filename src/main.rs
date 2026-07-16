mod framebuffer;
mod point;
mod line;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use point::point;
use line::line;

fn main() {
    let width = 800;
    let height = 500;
    let mut framebuffer = Framebuffer::new(width, height);

    let bg = Color::new(30, 30, 30, 255);
    framebuffer.set_background_color(bg);
    framebuffer.clear();

    // draw a dot
    framebuffer.set_current_color(Color::WHITE);
    point(&mut framebuffer, Vector2::new(400.0, 250.0));

    // draw a line
    framebuffer.set_current_color(Color::new(255, 220, 0, 255));
    line(
        &mut framebuffer,
        Vector2::new(100.0, 100.0),
        Vector2::new(700.0, 400.0),
    );

    framebuffer.render_to_png("out.png");

    println!("Puntos dibujados en out.png");
}
