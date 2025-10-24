#![allow(unused_imports)]
mod framebuffer;
mod line;
mod obj_loader;
mod shader;
mod triangle;
mod procedural_geometry;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use obj_loader::ObjModel;
use triangle::ShaderType;
use procedural_geometry::{generate_moon, generate_rings, transform_model};
use std::f32::consts::PI;

fn main() {
    let (mut window, thread) = raylib::init()
        .size(800, 600)
        .title("Planetary Shader Lab")
        .build();

    let mut fb = Framebuffer::new(800, 600, Color::new(5, 5, 15, 255));

    // Cargar modelos base
    println!("Cargando modelos...");
    let model_sphere = ObjModel::load("sphere-1.obj")
        .expect("No se pudo cargar sphere-1.obj");
    
    let model_crystal = ObjModel::load("crystal_planet.obj")
        .unwrap_or_else(|_| {
            println!("⚠️ No se encontró crystal_planet.obj, usando sphere-1.obj");
            model_sphere.clone()
        });
    
    // Generar geometría procedural
    let moon_model = generate_moon(0.3, 16); // Radio 0.3, 16 segmentos
    let rings_model = generate_rings(1.3, 2.0, 64); // Radio interno 1.3, externo 2.0
    
    println!("✓ Modelos cargados correctamente");
    println!("✓ Luna generada con {} vértices", moon_model.vertices.len());
    println!("✓ Anillos generados con {} vértices", rings_model.vertices.len());

    let mut angle_y = 0.0f32;
    let mut scale = 1.5;
    let mut current_planet = 0;
    let mut auto_rotate = true;
    let mut time = 0.0f32;
    let mut orbital_angle = 0.0f32; // Para traslación de luna

    window.set_target_fps(60);

    let planet_names = vec![
        "Steinbruch (Planeta Rocoso + Luna)",
        "Ätherblase (Gigante Gaseoso + Anillos)",
        "Kristallschloss (Planeta de Cristal)",
        "Feuerglut (Planeta de Lava)",
        "Eispalast (Planeta de Hielo)"
    ];

    let planet_models = vec![
        "sphere-1.obj + Luna Procedural",
        "sphere-1.obj + Anillos Procedurales",
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
        fb.clear();
        
        time += 0.016;
        orbital_angle += 0.02; // Traslación lunar

        // Controles
        if window.is_key_pressed(KeyboardKey::KEY_ONE) { 
            current_planet = 0;
            println!("Cambiado a: {}", planet_names[current_planet]);
        }
        if window.is_key_pressed(KeyboardKey::KEY_TWO) { 
            current_planet = 1;
            println!("Cambiado a: {}", planet_names[current_planet]);
        }
        if window.is_key_pressed(KeyboardKey::KEY_THREE) { 
            current_planet = 2;
            println!("Cambiado a: {}", planet_names[current_planet]);
        }
        if window.is_key_pressed(KeyboardKey::KEY_FOUR) { 
            current_planet = 3;
            println!("Cambiado a: {}", planet_names[current_planet]);
        }
        if window.is_key_pressed(KeyboardKey::KEY_FIVE) { 
            current_planet = 4;
            println!("Cambiado a: {}", planet_names[current_planet]);
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

        // Seleccionar modelo principal
        let current_model = if current_planet == 2 {
            &model_crystal
        } else {
            &model_sphere
        };

        // Transformar planeta principal
        let rotated = transform_model(
            current_model,
            Vector3::new(0.0, 0.0, 0.0),
            angle_y,
            0.0,
            scale
        );

        // Determinar shader
        let shader_type = match current_planet {
            0 => ShaderType::Rocky,
            1 => ShaderType::Gas,
            2 => ShaderType::Crystal,
            3 => ShaderType::Lava,
            _ => ShaderType::Ice,
        };

        // Renderizar planeta principal
        for face in &current_model.faces {
            if face.len() < 3 { continue; }
            
            for i in 1..(face.len() - 1) {
                let v0 = rotated[face[0]];
                let v1 = rotated[face[i]];
                let v2 = rotated[face[i + 1]];
                triangle::draw_filled_triangle(&mut fb, v0, v1, v2, shader_type, time);
            }
        }

        // Renderizar LUNA para planeta rocoso (0)
        if current_planet == 0 {
            let moon_distance = 2.5;
            let moon_x = orbital_angle.cos() * moon_distance;
            let moon_z = orbital_angle.sin() * moon_distance;
            
            let moon_transformed = transform_model(
                &moon_model,
                Vector3::new(moon_x * scale, 0.5 * scale, moon_z * scale),
                angle_y * 0.5, // Rotación propia más lenta
                0.0,
                scale * 0.8
            );

            for face in &moon_model.faces {
                if face.len() < 3 { continue; }
                
                for i in 1..(face.len() - 1) {
                    let v0 = moon_transformed[face[0]];
                    let v1 = moon_transformed[face[i]];
                    let v2 = moon_transformed[face[i + 1]];
                    // Luna usa shader de hielo para aspecto rocoso grisáceo
                    triangle::draw_filled_triangle(&mut fb, v0, v1, v2, ShaderType::Ice, time);
                }
            }
        }

        // Renderizar ANILLOS para gigante gaseoso (1)
        if current_planet == 1 {
            let rings_transformed = transform_model(
                &rings_model,
                Vector3::new(0.0, 0.0, 0.0),
                angle_y * 0.3, // Rotación más lenta
                0.4, // Inclinación de 0.4 radianes (~23°)
                scale
            );

            for face in &rings_model.faces {
                if face.len() < 3 { continue; }
                
                for i in 1..(face.len() - 1) {
                    let v0 = rings_transformed[face[0]];
                    let v1 = rings_transformed[face[i]];
                    let v2 = rings_transformed[face[i + 1]];
                    // Anillos usan shader cristalino semitransparente
                    triangle::draw_filled_triangle(&mut fb, v0, v1, v2, ShaderType::Crystal, time);
                }
            }
        }

        // Actualizar textura y renderizar
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
                
                // UI
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
            }
        }
    }

    println!("\n¡Renderizado completado!");
}
