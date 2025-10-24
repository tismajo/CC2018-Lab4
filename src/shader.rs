use raylib::prelude::*;

// ============= PLANETA ROCOSO (Tierra) =============
pub fn rocky_planet_shader(pos: &Vector3, normal: &Vector3, time: f32) -> Color {
    // Capa 1: Gradiente vertical base (océano/tierra)
    let height = pos.y;
    let water_threshold = -0.2 + (pos.x * 5.0).sin() * 0.1;
    
    // Capa 2: Ruido procedimental para continentes
    let noise = fbm_noise(pos.x * 3.0, pos.z * 3.0, 4);
    
    // Capa 3: Iluminación direccional
    let light_dir = Vector3::new(0.5, 0.8, 0.3).normalized();
    let brightness = normal.dot(light_dir).max(0.0);
    
    // Determinar si es océano o tierra
    let base_color = if height < water_threshold {
        // Océano profundo (azul)
        let deep_blue = Color::new(10, 50, 120, 255);
        let shallow_blue = Color::new(40, 100, 180, 255);
        lerp_color(deep_blue, shallow_blue, (height + 1.0) / 2.0)
    } else {
        // Tierra (verde/café según ruido)
        if noise > 0.5 {
            Color::new(34, 139, 34, 255) // Verde
        } else {
            Color::new(139, 90, 43, 255) // Café
        }
    };
    
    // Capa 4: Nubes procedurales
    let cloud_noise = fbm_noise(pos.x * 8.0 + time * 0.1, pos.z * 8.0, 3);
    let cloud_alpha = if cloud_noise > 0.65 { 0.3 } else { 0.0 };
    
    // Aplicar iluminación
    let lit = apply_brightness(base_color, brightness * 0.7 + 0.3);
    
    // Mezclar con nubes
    blend_colors(lit, Color::WHITE, cloud_alpha)
}

// ============= GIGANTE GASEOSO (Júpiter) =============
pub fn gas_giant_shader(pos: &Vector3, normal: &Vector3, time: f32) -> Color {
    // Capa 1: Gradiente radial del centro hacia afuera
    let distance_from_center = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
    let radial_gradient = 1.0 - distance_from_center.min(1.0);
    
    // Capa 2: Bandas horizontales con movimiento
    let band_freq = 10.0;
    let band_offset = time * 0.05;
    let bands = ((pos.y + band_offset) * band_freq).sin() * 0.5 + 0.5;
    
    // Capa 3: Turbulencia atmosférica
    let turbulence = fbm_noise(
        pos.x * 5.0 + time * 0.02,
        pos.y * 8.0,
        5
    );
    
    // Capa 4: Iluminación
    let light_dir = Vector3::new(0.5, 0.3, 0.8).normalized();
    let brightness = normal.dot(light_dir).max(0.1);
    
    // Colores base (azul oscuro a azul claro)
    let dark_blue = Color::new(20, 40, 100, 255);
    let light_blue = Color::new(100, 150, 220, 255);
    let band_color = Color::new(150, 180, 230, 255);
    
    // Mezclar gradiente radial
    let mut color = lerp_color(dark_blue, light_blue, radial_gradient);
    
    // Aplicar bandas
    color = lerp_color(color, band_color, bands * 0.4);
    
    // Añadir turbulencia
    let turb_intensity = turbulence * 0.3;
    color = apply_brightness(color, 1.0 + turb_intensity);
    
    // Aplicar iluminación final
    apply_brightness(color, brightness * 0.8 + 0.2)
}

// ============= PLANETA DE CRISTAL (Morado con reflejos) =============
pub fn crystal_planet_shader(pos: &Vector3, normal: &Vector3, time: f32) -> Color {
    // Capa 1: Base cristalina morada
    let base_purple = Color::new(138, 43, 226, 255);
    let deep_purple = Color::new(75, 0, 130, 255);
    
    // Capa 2: Patrones de cristal (Voronoi simplificado)
    let crystal_pattern = voronoi_pattern(pos.x * 6.0, pos.y * 6.0, pos.z * 6.0);
    
    // Capa 3: Reflejos especulares intensos
    let view_dir = Vector3::new(0.0, 0.0, 1.0);
    let light_dir = Vector3::new(
        (time * 0.3).cos(),
        0.5,
        (time * 0.3).sin()
    ).normalized();
    
    let reflect_dir = reflect(&light_dir.scale_by(-1.0), normal);
    let specular = view_dir.dot(reflect_dir).max(0.0).powf(32.0);
    
    // Capa 4: Iluminación difusa
    let diffuse = normal.dot(light_dir).max(0.0);
    
    // Mezclar colores base con patrón cristalino
    let mut color = lerp_color(deep_purple, base_purple, crystal_pattern);
    
    // Aplicar iluminación difusa
    color = apply_brightness(color, diffuse * 0.6 + 0.4);
    
    // Añadir reflejos blancos brillantes
    blend_colors(color, Color::WHITE, specular * 0.8)
}

// ============= PLANETA EXTRA 1: Lava =============
pub fn lava_planet_shader(pos: &Vector3, normal: &Vector3, time: f32) -> Color {
    // Capa 1: Flujo de lava animado
    let flow = fbm_noise(
        pos.x * 4.0 + time * 0.2,
        pos.z * 4.0 + time * 0.15,
        4
    );
    
    // Capa 2: Grietas brillantes
    let cracks = ((pos.x * 20.0).sin() * (pos.z * 20.0).cos()).abs();
    let is_crack = cracks < 0.1;
    
    // Capa 3: Pulsación de calor
    let pulse = ((time * 2.0).sin() * 0.5 + 0.5) * 0.3;
    
    // Colores
    let dark_rock = Color::new(40, 20, 10, 255);
    let hot_lava = Color::new(255, 100, 0, 255);
    let bright_lava = Color::new(255, 200, 50, 255);
    
    let mut color = if is_crack {
        bright_lava
    } else if flow > 0.4 {
        lerp_color(hot_lava, bright_lava, (flow - 0.4) / 0.6)
    } else {
        lerp_color(dark_rock, hot_lava, flow / 0.4)
    };
    
    // Aplicar pulsación
    color = apply_brightness(color, 1.0 + pulse);
    
    // Iluminación suave
    let light_dir = Vector3::new(0.5, 0.8, 0.3).normalized();
    let brightness = normal.dot(light_dir).max(0.3);
    apply_brightness(color, brightness)
}

// ============= PLANETA EXTRA 2: Hielo =============
pub fn ice_planet_shader(pos: &Vector3, normal: &Vector3, time: f32) -> Color {
    // Capa 1: Base helada
    let ice_color = Color::new(200, 230, 255, 255);
    let deep_ice = Color::new(150, 180, 220, 255);
    
    // Capa 2: Grietas de hielo
    let crack_pattern = fbm_noise(pos.x * 12.0, pos.z * 12.0, 3);
    let has_crack = crack_pattern > 0.7;
    
    // Capa 3: Reflejos helados
    let view_dir = Vector3::new(0.0, 0.0, 1.0);
    let light_dir = Vector3::new(0.5, 0.8, 0.3).normalized();
    let reflect_dir = reflect(&light_dir.scale_by(-1.0), normal);
    let specular = view_dir.dot(reflect_dir).max(0.0).powf(16.0);
    
    // Capa 4: Variación de profundidad
    let depth = (pos.y + 1.0) / 2.0;
    
    let mut color = lerp_color(deep_ice, ice_color, depth);
    
    if has_crack {
        color = apply_brightness(color, 0.7);
    }
    
    // Añadir brillo especular
    color = blend_colors(color, Color::WHITE, specular * 0.6);
    
    // Iluminación
    let brightness = normal.dot(light_dir).max(0.2);
    apply_brightness(color, brightness)
}

// ============= FUNCIONES AUXILIARES =============

// Fractional Brownian Motion para ruido multi-octava
fn fbm_noise(x: f32, y: f32, octaves: u32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_value = 0.0;
    
    for _ in 0..octaves {
        value += noise2d(x * frequency, y * frequency) * amplitude;
        max_value += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    
    (value / max_value + 1.0) / 2.0 // Normalizar a [0,1]
}

// Ruido 2D simple (hash-based)
fn noise2d(x: f32, y: f32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    
    let hash = ((xi.wrapping_mul(374761393))
        .wrapping_add(yi.wrapping_mul(668265263))) as u32;
    
    let random = (hash as f32 / 4294967295.0) * 2.0 - 1.0;
    random
}

// Patrón Voronoi simplificado
fn voronoi_pattern(x: f32, y: f32, z: f32) -> f32 {
    let xi = x.floor();
    let yi = y.floor();
    let zi = z.floor();
    
    let mut min_dist = f32::INFINITY;
    
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                let nx = xi + dx as f32;
                let ny = yi + dy as f32;
                let nz = zi + dz as f32;
                
                let px = nx + hash_to_float(nx, ny, nz);
                let py = ny + hash_to_float(ny, nz, nx);
                let pz = nz + hash_to_float(nz, nx, ny);
                
                let dist = ((x - px).powi(2) + (y - py).powi(2) + (z - pz).powi(2)).sqrt();
                min_dist = min_dist.min(dist);
            }
        }
    }
    
    min_dist.min(1.0)
}

fn hash_to_float(x: f32, y: f32, z: f32) -> f32 {
    let xi = x as i32;
    let yi = y as i32;
    let zi = z as i32;
    
    let hash = ((xi.wrapping_mul(374761393))
        .wrapping_add(yi.wrapping_mul(668265263))
        .wrapping_add(zi.wrapping_mul(1274126177))) as u32;
    
    (hash as f32 / 4294967295.0)
}

// Reflexión de vector
fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    let dot = incident.dot(*normal);
    Vector3::new(
        incident.x - 2.0 * dot * normal.x,
        incident.y - 2.0 * dot * normal.y,
        incident.z - 2.0 * dot * normal.z,
    )
}

// Interpolación lineal de colores
fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::new(
        (a.r as f32 * (1.0 - t) + b.r as f32 * t) as u8,
        (a.g as f32 * (1.0 - t) + b.g as f32 * t) as u8,
        (a.b as f32 * (1.0 - t) + b.b as f32 * t) as u8,
        255,
    )
}

// Aplicar brillo multiplicativo
fn apply_brightness(color: Color, brightness: f32) -> Color {
    Color::new(
        ((color.r as f32 * brightness).min(255.0)) as u8,
        ((color.g as f32 * brightness).min(255.0)) as u8,
        ((color.b as f32 * brightness).min(255.0)) as u8,
        color.a,
    )
}

// Mezcla de colores con alpha
fn blend_colors(base: Color, top: Color, alpha: f32) -> Color {
    let alpha = alpha.clamp(0.0, 1.0);
    Color::new(
        (base.r as f32 * (1.0 - alpha) + top.r as f32 * alpha) as u8,
        (base.g as f32 * (1.0 - alpha) + top.g as f32 * alpha) as u8,
        (base.b as f32 * (1.0 - alpha) + top.b as f32 * alpha) as u8,
        255,
    )
}
