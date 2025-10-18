use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::shader::fragment_shader;
use std::cmp::{max, min};

fn edge_function(a: Vector2, b: Vector2, c: Vector2) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}

pub fn draw_filled_triangle(
    fb: &mut Framebuffer,
    p0: Vector2,
    p1: Vector2,
    p2: Vector2,
    w0: Vector3,
    w1: Vector3,
    w2: Vector3,
) {
    // bounding box
    let min_x = max(0, (p0.x.min(p1.x.min(p2.x))).floor() as i32);
    let max_x = min(fb.width as i32 - 1, (p0.x.max(p1.x.max(p2.x))).ceil() as i32);
    let min_y = max(0, (p0.y.min(p1.y.min(p2.y))).floor() as i32);
    let max_y = min(fb.height as i32 - 1, (p0.y.max(p1.y.max(p2.y))).ceil() as i32);

    // tri fuera de pantalla
    if max_x < 0 || min_x >= fb.width as i32 || max_y < 0 || min_y >= fb.height as i32 {
        return;
    }

    let area = edge_function(p0, p1, p2);
    if area.abs() < 1e-6 { return; }

    let z0 = w0.z;
    let z1 = w1.z;
    let z2 = w2.z;

    let inv_area = 1.0 / area;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vector2::new(x as f32 + 0.5, y as f32 + 0.5);

            let w0b = edge_function(p1, p2, p);
            if w0b < 0.0 { continue; }
            let w1b = edge_function(p2, p0, p);
            if w1b < 0.0 { continue; }
            let w2b = edge_function(p0, p1, p);
            if w2b < 0.0 { continue; }

            let alpha = w0b * inv_area;
            let beta = w1b * inv_area;
            let gamma = w2b * inv_area;

            let depth = alpha * z0 + beta * z1 + gamma * z2;

            let px = alpha * w0.x + beta * w1.x + gamma * w2.x;
            let py = alpha * w0.y + beta * w1.y + gamma * w2.y;
            let pz = depth;
            let color = fragment_shader(&Vector3::new(px, py, pz), alpha, beta, gamma);

            fb.set_pixel_depth_with_color(x, y, depth, color);
        }
    }
}
