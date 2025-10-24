// shader.rs
// ------------------------------------------------------------
// Shader arcoíris (rojo → naranja → amarillo → verde)
// ------------------------------------------------------------
use raylib::prelude::*;

pub fn rainbow_shader(pos: &Vector3) -> Color {
    // Normalizamos Y al rango [0,1]
    let t = ((pos.y + 1.0) / 2.0).clamp(0.0, 1.0);

    let (r, g, b) = if t < 0.33 {
        // Rojo → Naranja
        let u = t / 0.33;
        (255, (64.0 + 64.0 * u) as u8, 0)
    } else if t < 0.66 {
        // Naranja → Amarillo
        let u = (t - 0.33) / 0.33;
        (255, (128.0 + 127.0 * u) as u8, 0)
    } else {
        // Amarillo → Verde
        let u = (t - 0.66) / 0.34;
        ((255.0 * (1.0 - u)) as u8, 255, 0)
    };

    Color::new(r, g, b, 255)
}
