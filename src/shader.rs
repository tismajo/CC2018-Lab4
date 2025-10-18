// shader.rs
// ------------------------------------------------------------
// "Shader" por software: patrón checker (blanco y negro)
// ------------------------------------------------------------
use raylib::prelude::*;

pub fn checker_shader(pos: &Vector3) -> Color {
    // Escala del patrón (cuán grande o pequeño es el cuadriculado)
    let scale = 5.0;

    // Convertimos coordenadas a un patrón 3D
    let v = (
        (pos.x * scale).floor() as i32,
        (pos.y * scale).floor() as i32,
        (pos.z * scale).floor() as i32,
    );

    // Checker 3D: alterna color según paridad de la suma
    let parity = (v.0 + v.1 + v.2) & 1;

    if parity == 0 {
        Color::WHITE
    } else {
        Color::BLACK
    }
}
