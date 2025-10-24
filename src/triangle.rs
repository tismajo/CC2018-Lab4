// triangle.rs
// ------------------------------------------------------------
// Relleno de triángulos con Z-buffer y shader arcoíris
// ------------------------------------------------------------
use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::shader::rainbow_shader;

pub fn draw_filled_triangle(
    framebuffer: &mut Framebuffer,
    v0: Vector3,
    v1: Vector3,
    v2: Vector3,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let scale = 1.0;
    
    let p0 = project(&v0, width, height, scale);
    let p1 = project(&v1, width, height, scale);
    let p2 = project(&v2, width, height, scale);

    // Backface culling: solo dibujamos caras frontales
    let edge1_x = p1.x - p0.x;
    let edge1_y = p1.y - p0.y;
    let edge2_x = p2.x - p0.x;
    let edge2_y = p2.y - p0.y;
    let cross = edge1_x * edge2_y - edge1_y * edge2_x;
    
    // Si el área es negativa o muy pequeña, no dibujar
    if cross <= 0.0 {
        return;
    }

    // Bounding box del triángulo
    let min_x = p0.x.min(p1.x).min(p2.x).max(0.0) as i32;
    let max_x = p0.x.max(p1.x).max(p2.x).min(width - 1.0) as i32;
    let min_y = p0.y.min(p1.y).min(p2.y).max(0.0) as i32;
    let max_y = p0.y.max(p1.y).max(p2.y).min(height - 1.0) as i32;

    let denom = ((p1.y - p2.y) * (p0.x - p2.x) + (p2.x - p1.x) * (p0.y - p2.y));
    if denom.abs() < 1e-6 {
        return;
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;

            let w0 = ((p1.y - p2.y) * (px - p2.x) + (p2.x - p1.x) * (py - p2.y)) / denom;
            let w1 = ((p2.y - p0.y) * (px - p2.x) + (p0.x - p2.x) * (py - p2.y)) / denom;
            let w2 = 1.0 - w0 - w1;

            // Pequeño margen de tolerancia para evitar huecos en los bordes
            const EPSILON: f32 = -0.0001;
            if w0 >= EPSILON && w1 >= EPSILON && w2 >= EPSILON {
                // Interpolamos profundidad
                let depth = w0 * v0.z + w1 * v1.z + w2 * v2.z;
                let idx = (y as u32 * framebuffer.width + x as u32) as usize;
                
                if depth < framebuffer.z_buffer[idx] {
                    framebuffer.z_buffer[idx] = depth;
                    
                    // Interpolamos posición 3D
                    let pos = Vector3::new(
                        w0 * v0.x + w1 * v1.x + w2 * v2.x,
                        w0 * v0.y + w1 * v1.y + w2 * v2.y,
                        w0 * v0.z + w1 * v1.z + w2 * v2.z,
                    );
                    
                    let color = rainbow_shader(&pos);
                    framebuffer.set_pixel_with_color(x, y, color);
                }
            }
        }
    }
}

fn project(v: &Vector3, width: f32, height: f32, scale: f32) -> Vector2 {
    // Proyección simple en perspectiva
    let fov = 1.0 / (v.z + 3.0);
    let x = width / 2.0 + v.x * scale * fov * width / 2.0;
    let y = height / 2.0 - v.y * scale * fov * height / 2.0;
    Vector2::new(x, y)
}
