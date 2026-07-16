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

    // Polygon 2
    let p2: &[Vector2] = &[
        Vector2::new(321.0, 335.0),
        Vector2::new(288.0, 286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0),
    ];
    framebuffer.set_current_color(Color::new(0, 120, 255, 255));
    fill_polygon(&mut framebuffer, p2);
    framebuffer.set_current_color(Color::WHITE);
    polygon(&mut framebuffer, p2);

    framebuffer.render_to_png("out.png");

    println!("Puntos dibujados en out.png");
}
