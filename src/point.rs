use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

/// Draw a dot at the position given using the actual color from the Framebuffer.
pub fn point(framebuffer: &mut Framebuffer, position: Vector2) {
    framebuffer.set_pixel(position.x as i32, position.y as i32);
}
