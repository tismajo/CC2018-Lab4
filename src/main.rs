#![allow(unused_imports)]
mod framebuffer;
mod line;
mod obj_loader;
mod shader;
mod triangle;
// mod procedural_geometry; // ya no es necesario para la estrella

use raylib::prelude::*;
use framebuffer::Framebuffer;
use obj_loader::ObjModel;
use triangle::ShaderType;
use std::f32::consts::PI;

fn main() {
    let (mut window, thread) = raylib::init()
        .size(800, 600)
        .title("Star Shader Lab - Estrella Procedural")
        .build();

    let mut fb = Framebuffer::new(800, 600, Color::new(0, 0, 0, 255));

    // Cargar modelo único: sphere-1.obj (la esfera base)
    println!("Cargando sphere-1.obj ...");
    let model_sphere = ObjModel::load("sphere-1.obj")
        .expect("No se pudo cargar sphere-1.obj (asegúrate que esté en la carpeta del ejecutable)");

    println!("Vértices: {}, caras: {}", model_sphere.vertices.len(), model_sphere.faces.len());

    let mut angle_y = 0.0f32;
    let mut scale = 1.6f32;
    let mut time = 0.0f32;

    window.set_target_fps(60);

    println!("\n=== CONTROLES ===");
    println!("Q/E: Zoom in/out | ←→: Rotar manual | ESC: Salir\n");

    while !window.window_should_close() {
        fb.clear();

        // Avanzar tiempo en segundos (aprox 1/60)
        time += 1.0 / 60.0;

        // Controles simples
        if window.is_key_down(KeyboardKey::KEY_RIGHT) { angle_y += 0.02; }
        if window.is_key_down(KeyboardKey::KEY_LEFT)  { angle_y -= 0.02; }
        if window.is_key_down(KeyboardKey::KEY_Q) { scale *= 1.01; }
        if window.is_key_down(KeyboardKey::KEY_E) { scale /= 1.01; }

        // Transformar y desplazar vértices (simulando vertex shader)
        // Aplicamos rotación Y + escala + desplazamiento procedimental
        let mut transformed_vertices: Vec<Vector3> = Vec::with_capacity(model_sphere.vertices.len());
        for v in &model_sphere.vertices {
            // Escala
            let mut x = v.x * scale;
            let mut y = v.y * scale;
            let mut z = v.z * scale;

            // Rotación Y
            let rx = x * angle_y.cos() + z * angle_y.sin();
            let rz = -x * angle_y.sin() + z * angle_y.cos();
            x = rx;
            z = rz;

            let mut pos = Vector3::new(x, y, z);

            // Desplazamiento tipo "vertex shader" usando ruido + time
            pos = shader::star_vertex_displacement(pos, time);

            transformed_vertices.push(pos);
        }

        // Renderizar triángulos con shader de estrella
        for face in &model_sphere.faces {
            if face.len() < 3 { continue; }
            for i in 1..(face.len() - 1) {
                let v0 = transformed_vertices[face[0]];
                let v1 = transformed_vertices[face[i]];
                let v2 = transformed_vertices[face[i + 1]];
                triangle::draw_filled_triangle(&mut fb, v0, v1, v2, ShaderType::Star, time);
            }
        }

        // Actualizar textura y renderizar en la ventana
        {
            if fb.texture.is_none() {
                fb.init_texture(&mut window, &thread);
            }

            if let Some(tex) = &mut fb.texture {
                let pixels: Vec<Color> = fb.color_buffer.get_image_data().to_vec();
                let mut raw: Vec<u8> = Vec::with_capacity(pixels.len() * 4);
                for c in pixels {
                    raw.push(c.r);
                    raw.push(c.g);
                    raw.push(c.b);
                    raw.push(c.a);
                }

                tex.update_texture_rec(
                    Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: tex.width() as f32,
                        height: tex.height() as f32,
                    },
                    &raw,
                );

                let mut d = window.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_texture(tex, 0, 0, Color::WHITE);

                d.draw_text("Estrella Procedural", 10, 10, 20, Color::WHITE);
                d.draw_text(&format!("sphere-1.obj | scale: {:.2}", scale), 10, 40, 14, Color::YELLOW);
                d.draw_text("Q/E: Zoom | ←→: Rotar", 10, 570, 14, Color::LIGHTGRAY);
            }
        }
    }

    println!("\n¡Renderizado completado!");
}
