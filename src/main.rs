// main.rs
#![allow(unused_imports)]
mod framebuffer;
mod line;
mod obj_loader;
mod shader;
mod triangle;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use line::line;
use obj_loader::ObjModel;
use std::f32::consts::PI;

fn project_vertex(v: &Vector3, width: f32, height: f32, scale: f32) -> Vector2 {
    // Proyección simple en perspectiva
    let fov = 1.0 / (v.z + 3.0); // +3 evita división por 0
    let x = width / 2.0 + v.x * scale * fov * width / 2.0;
    let y = height / 2.0 - v.y * scale * fov * height / 2.0;
    Vector2::new(x, y)
}

fn main() {
    let (mut window, thread) = raylib::init()
        .size(800, 600)
        .title("Wireframe Renderer - OBJ Viewer")
        .build();

    let mut fb = Framebuffer::new(800, 600, Color::new(10, 10, 40, 255));

    // 🔹 Carga del modelo .obj exportado desde Blender
    let model = ObjModel::load("nave.obj").expect("No se pudo cargar el modelo");

    println!("Modelo cargado: {} vértices, {} caras", model.vertices.len(), model.faces.len());

    let mut angle_x = 0.0f32;
    let mut angle_y = 0.0f32;
    let mut scale = 1.0;

    window.set_target_fps(60);

    while !window.window_should_close() {
        fb.clear();

        // Controles
        if window.is_key_down(KeyboardKey::KEY_RIGHT) { angle_y += 1.0 * 0.02; }
        if window.is_key_down(KeyboardKey::KEY_LEFT) { angle_y -= 1.0 * 0.02; }
        if window.is_key_down(KeyboardKey::KEY_UP) { angle_x += 1.0 * 0.02; }
        if window.is_key_down(KeyboardKey::KEY_DOWN) { angle_x -= 1.0 * 0.02; }
        if window.is_key_down(KeyboardKey::KEY_Q) { scale *= 1.02; }
        if window.is_key_down(KeyboardKey::KEY_E) { scale /= 1.02; }

        // Rotación de vértices
        let rotated: Vec<Vector3> = model.vertices.iter().map(|v| {
            let mut vx = v.x;
            let mut vy = v.y;
            let mut vz = v.z;

            // Rotación en X
            let ry = vy * angle_x.cos() - vz * angle_x.sin();
            let rz = vy * angle_x.sin() + vz * angle_x.cos();
            vy = ry; vz = rz;

            // Rotación en Y
            let rx = vx * angle_y.cos() + vz * angle_y.sin();
            let rz = -vx * angle_y.sin() + vz * angle_y.cos();
            vx = rx; vz = rz;

            Vector3::new(vx, vy, vz)
        }).collect();

        fb.set_current_color(Color::WHITE);

        // Dibujar modelo con shader
        for face in &model.faces {
            if face.len() < 3 { continue; }

            let v0 = rotated[face[0]];
            let v1 = rotated[face[1]];
            let v2 = rotated[face[2]];

            triangle::draw_filled_triangle(&mut fb, v0, v1, v2);
        }

        fb.swap_buffers(&mut window, &thread);
    }
}
