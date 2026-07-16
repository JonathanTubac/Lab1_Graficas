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

    // Polygon 4
    let p4: &[Vector2] = &[
        Vector2::new(413.0, 177.0),
        Vector2::new(448.0, 159.0),
        Vector2::new(502.0, 88.0),
        Vector2::new(553.0, 53.0),
        Vector2::new(535.0, 36.0),
        Vector2::new(676.0, 37.0),
        Vector2::new(660.0, 52.0),
        Vector2::new(750.0, 145.0),
        Vector2::new(761.0, 179.0),
        Vector2::new(672.0, 192.0),
        Vector2::new(659.0, 214.0),
        Vector2::new(615.0, 214.0),
        Vector2::new(632.0, 230.0),
        Vector2::new(580.0, 230.0),
        Vector2::new(597.0, 215.0),
        Vector2::new(552.0, 214.0),
        Vector2::new(517.0, 144.0),
        Vector2::new(466.0, 180.0),
    ];
    framebuffer.set_current_color(Color::new(0, 200, 120, 255));
    fill_polygon(&mut framebuffer, p4);

    // Polygon 5 - a hole inside polygon 4, so it is filled with the background color
    let p5: &[Vector2] = &[
        Vector2::new(682.0, 175.0),
        Vector2::new(708.0, 120.0),
        Vector2::new(735.0, 148.0),
        Vector2::new(739.0, 170.0),
    ];
    framebuffer.set_current_color(bg);
    fill_polygon(&mut framebuffer, p5);

    // Outlines go last so the hole's fill does not erase the border of polygon 4.
    framebuffer.set_current_color(Color::WHITE);
    polygon(&mut framebuffer, p4);
    polygon(&mut framebuffer, p5);

    framebuffer.render_to_png("out.png");

    println!("Puntos dibujados en out.png");
}
