use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

/// Draw the outline of a closed polygon from a list of vertices.
pub fn polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    if points.len() < 2 {
        return;
    }

    for i in 0..points.len() - 1 {
        line(framebuffer, points[i], points[i + 1]);
    }

    // Close the polygon: connect the last vertex back to the first one.
    line(framebuffer, *points.last().unwrap(), points[0]);
}
