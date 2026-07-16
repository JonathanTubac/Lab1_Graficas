mod framebuffer;
mod point;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use polygon::{polygon, fill_polygon};

fn main() {
    let width = 800;
    let height = 500;
    let mut framebuffer = Framebuffer::new(width, height);

    let bg = Color::new(30, 30, 30, 255);
    framebuffer.set_background_color(bg);
    framebuffer.clear();

    // Polygon 3
    let p3: &[Vector2] = &[
        Vector2::new(377.0, 249.0),
        Vector2::new(411.0, 197.0),
        Vector2::new(436.0, 249.0),
    ];
    framebuffer.set_current_color(Color::new(255, 60, 60, 255));
    fill_polygon(&mut framebuffer, p3);
    framebuffer.set_current_color(Color::WHITE);
    polygon(&mut framebuffer, p3);

    framebuffer.render_to_png("out.png");

    println!("Puntos dibujados en out.png");
}
