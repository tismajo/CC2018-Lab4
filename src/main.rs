#![allow(unused_imports)]
mod framebuffer;
mod line;
mod obj_loader;
mod shader;
mod triangle;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use obj_loader::ObjModel;
use triangle::ShaderType;
use std::f32::consts::PI;

fn main() {
    let (mut window, thread) = raylib::init()
        .size(800, 600)
        .title("Planetary Shader Lab")
        .build();

    let mut fb = Framebuffer::new(800, 600, Color::new(5, 5, 15, 255));

    // Cargar todos los modelos
    println!("Cargando modelos...");
    let model_sphere = ObjModel::load("sphere-1.obj")
        .expect("No se pudo cargar sphere-1.obj");
    
    let model_crystal = ObjModel::load("crystal_planet.obj")
        .unwrap_or_else(|_| {
            println!("⚠️ No se encontró crystal_planet.obj, usando sphere-1.obj");
            model_sphere.clone()
        });
    
    println!("✓ Modelos cargados correctamente");

    let mut angle_y = 0.0f32;
    let mut scale = 1.5;
    let mut current_planet = 0;
    let mut auto_rotate = true;
    let mut time = 0.0f32;

    window.set_target_fps(60);

    let planet_names = vec![
        "Planeta Rocoso (Tierra)",
        "Gigante Gaseoso (Júpiter)",
        "Planeta de Cristal",
        "Planeta de Lava",
        "Planeta de Hielo"
    ];

    let planet_models = vec![
        "sphere-1.obj",
        "sphere-1.obj",
        "crystal_planet.obj",
        "sphere-1.obj",
        "sphere-1.obj"
    ];

    println!("\n=== CONTROLES ===");
    println!("1-5: Cambiar planeta");
    println!("SPACE: Activar/desactivar rotación automática");
    println!("LEFT/RIGHT: Rotar manualmente");
    println!("Q/E: Zoom in/out");
    println!("ESC: Salir\n");

    while !window.window_should_close() {
        // Limpiar framebuffer al inicio de cada frame
        fb.clear();
        
        time += 0.016; // ~60 FPS

        // Controles
        if window.is_key_pressed(KeyboardKey::KEY_ONE) { 
            current_planet = 0;
            println!("Cambiado a: {} ({})", planet_names[current_planet], planet_models[current_planet]);
        }
        if window.is_key_pressed(KeyboardKey::KEY_TWO) { 
            current_planet = 1;
            println!("Cambiado a: {} ({})", planet_names[current_planet], planet_models[current_planet]);
        }
        if window.is_key_pressed(KeyboardKey::KEY_THREE) { 
            current_planet = 2;
            println!("Cambiado a: {} ({})", planet_names[current_planet], planet_models[current_planet]);
        }
        if window.is_key_pressed(KeyboardKey::KEY_FOUR) { 
            current_planet = 3;
            println!("Cambiado a: {} ({})", planet_names[current_planet], planet_models[current_planet]);
        }
        if window.is_key_pressed(KeyboardKey::KEY_FIVE) { 
            current_planet = 4;
            println!("Cambiado a: {} ({})", planet_names[current_planet], planet_models[current_planet]);
        }

        if window.is_key_pressed(KeyboardKey::KEY_SPACE) { 
            auto_rotate = !auto_rotate;
            println!("Auto-rotación: {}", if auto_rotate { "ON" } else { "OFF" });
        }

        if auto_rotate {
            angle_y += 0.01;
        } else {
            if window.is_key_down(KeyboardKey::KEY_RIGHT) { angle_y += 0.02; }
            if window.is_key_down(KeyboardKey::KEY_LEFT) { angle_y -= 0.02; }
        }

        if window.is_key_down(KeyboardKey::KEY_Q) { scale *= 1.02; }
        if window.is_key_down(KeyboardKey::KEY_E) { scale /= 1.02; }

        // Seleccionar modelo según el planeta
        let current_model = if current_planet == 2 {
            &model_crystal
        } else {
            &model_sphere
        };

        // Rotación del modelo
        let rotated: Vec<Vector3> = current_model.vertices.iter().map(|v| {
            let mut vx = v.x * scale;
            let mut vy = v.y * scale;
            let mut vz = v.z * scale;

            // Rotación en Y
            let rx = vx * angle_y.cos() + vz * angle_y.sin();
            let rz = -vx * angle_y.sin() + vz * angle_y.cos();
            vx = rx;
            vz = rz;

            Vector3::new(vx, vy, vz)
        }).collect();

        // Determinar shader actual
        let shader_type = match current_planet {
            0 => ShaderType::Rocky,
            1 => ShaderType::Gas,
            2 => ShaderType::Crystal,
            3 => ShaderType::Lava,
            _ => ShaderType::Ice,
        };

        // Renderizar todas las caras
        for face in &current_model.faces {
            if face.len() < 3 { continue; }
            
            for i in 1..(face.len() - 1) {
                let v0 = rotated[face[0]];
                let v1 = rotated[face[i]];
                let v2 = rotated[face[i + 1]];
                triangle::draw_filled_triangle(&mut fb, v0, v1, v2, shader_type, time);
            }
        }

        // IMPORTANTE: Combinar swap_buffers y UI en un solo bloque de dibujo
        {
            // Actualizar textura del framebuffer
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

                // Dibujar TODO en un solo begin_drawing
                let mut d = window.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_texture(tex, 0, 0, Color::WHITE);
                
                // UI encima del framebuffer
                d.draw_text(&planet_names[current_planet], 10, 10, 20, Color::WHITE);
                d.draw_text(
                    &format!("Modelo: {} | Presiona 1-5 para cambiar", planet_models[current_planet]), 
                    10, 40, 14, Color::YELLOW
                );
                d.draw_text(
                    &format!("SPACE: Auto-rotar {} | Q/E: Zoom | ←→: Rotar manual", 
                            if auto_rotate { "ON" } else { "OFF" }), 
                    10, 570, 14, Color::LIGHTGRAY
                );
                // begin_drawing se cierra automáticamente al salir del scope
            }
        }
    }

    println!("\n¡Renderizado completado!");
}
