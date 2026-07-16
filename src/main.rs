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

    // Polygon 1
    let p1: &[Vector2] = &[
        Vector2::new(165.0, 380.0),
        Vector2::new(185.0, 360.0),
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];
    framebuffer.set_current_color(Color::new(255, 220, 0, 255));
    fill_polygon(&mut framebuffer, p1);
    framebuffer.set_current_color(Color::WHITE);
    polygon(&mut framebuffer, p1);

    framebuffer.render_to_png("out.png");

    println!("Puntos dibujados en out.png");
}
