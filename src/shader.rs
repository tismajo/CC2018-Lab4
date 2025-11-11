use raylib::prelude::*;

/// Shader procedural de ESTRELLA (CPU-side)
/// - Usa fbm_noise para producir turbulencias en la superficie
/// - star_vertex_displacement: desplaza vértices para crear flare/distorsión
/// - star_shader: dado pos y normal y time devuelve Color con emisión variable y gradiente según "temperatura"

/// Publica la función de desplazamiento para usarla desde main (simula vertex shader)
pub fn star_vertex_displacement(mut pos: Vector3, time: f32) -> Vector3 {
    // Coordenadas esféricas aproximadas (para escala de ruido estable)
    let r = (pos.x*pos.x + pos.y*pos.y + pos.z*pos.z).sqrt().max(0.0001);
    let nx = pos.x / r;
    let ny = pos.y / r;
    let nz = pos.z / r;

    // Escala base de la superficie
    let base_radius = r;

    // Múltiples octavas de ruido para detalle fino + grandes estructuras
    let detail = fbm_noise(nx * 3.0 + time * 0.2, ny * 3.0 + time * 0.15, 5);
    let turbulence = fbm_noise(nx * 12.0 + time * 1.2, nz * 12.0 - time * 0.9, 3);

    // Pulso global (cíclico) que simula latidos / picos solares
    let pulse = (time * 1.25).sin() * 0.12 + 0.12; // rango ~ [0,0.24]

    // Displacement combinado: pulso + detalle * turbulencia
    let displacement = (0.15 * detail * (1.0 + turbulence)) + pulse * (0.6 + 0.6 * turbulence);

    // Aplicar desplazamiento a lo largo de la normal (aprox el vector desde centro)
    pos.x = nx * (base_radius + displacement);
    pos.y = ny * (base_radius + displacement);
    pos.z = nz * (base_radius + displacement);

    pos
}

/// Shader de fragment (simulado en CPU): color + emisión
pub fn star_shader(pos: &Vector3, normal: &Vector3, time: f32) -> Color {
    // Normal aproximada: normal ya provisto; si fuera 0 fallback a pos normalizado
    let n = if normal.length() == 0.0 {
        let len = (pos.x*pos.x + pos.y*pos.y + pos.z*pos.z).sqrt().max(1e-6);
        Vector3::new(pos.x / len, pos.y / len, pos.z / len)
    } else {
        *normal
    };

    // Coordenadas en esfera para mapas de ruido (estable)
    let r = (pos.x*pos.x + pos.y*pos.y + pos.z*pos.z).sqrt();
    let nx = pos.x / r;
    let ny = pos.y / r;
    let nz = pos.z / r;

    // Ruido multi-octava para manchas activas
    let spots = fbm_noise(nx * 4.0 + time * 0.5, ny * 4.0 - time * 0.3, 5);
    let micro = fbm_noise(nx * 20.0 + time * 2.0, nz * 20.0 - time * 1.5, 3);

    // Pulso global (cíclico) — sincronizado con vertex displacement para coherencia visual
    let global_pulse = ((time * 1.25).sin() * 0.5 + 0.5).powf(1.2); // [0,1]

    // Emisión base (de 0.5 a 1.6) modulada por ruido y pulso
    let emission = 0.8 + spots * 0.9 + micro * 0.25 + global_pulse * 0.8;

    // Simular picos de energía (flare corto y brillante)
    let flare_noise = fbm_noise(nx * 40.0 + time * 6.0, ny * 40.0 - time * 4.0, 2);
    let peak = if flare_noise > 0.92 { (flare_noise - 0.92) / 0.08 } else { 0.0 };
    let peak_emission = peak.powf(2.0) * 5.0;

    let total_emission = (emission + peak_emission).min(6.0);

    // Temperatura simulada: zonas más calientes -> más blancas/amarillas; frías -> rojizas
    // Usamos spots para variar la "temperatura local"
    let temp = 3500.0 + 4000.0 * spots + 1500.0 * micro + 2000.0 * global_pulse; // Kelvin-like proxy

    // Convertir temperatura a color aproximado (simple gradient)
    let base_color = color_from_temperature(temp);

    // Aplicar brillo/emisión multiplicativa
    let bright = apply_brightness(base_color, total_emission as f32);

    // Aplicar ligerísimo sombreado con dirección de luz (simula relieve)
    let light_dir = Vector3::new(0.3, 0.7, 0.2).normalized();
    let diffuse = n.dot(light_dir).max(0.0);
    let shaded = apply_brightness(bright, 0.6 + 0.6 * diffuse);

    // Añadir "glow" local según micro-ruido (mezcla por adición)
    let glow = apply_brightness(Color::new(255, 240, 200, 255), (micro * 0.6 + spots * 0.3 + global_pulse * 0.2) as f32);
    blend_additive(shaded, glow, 0.25)
}

/// Map temperature (approx Kelvin) to an RGB color (simple)
fn color_from_temperature(k: f32) -> Color {
    // Simple, non-physical mapping:
    //  ~3000K -> orange-red ; ~6500K -> white ; ~9000K -> blue-white
    let t = (k - 2000.0) / 8000.0; // normalize roughly 2000..10000
    let t = t.clamp(0.0, 1.0);

    // Interpolación entre rojo->amarillo->blanco->blueish
    if t < 0.33 {
        // rojizo -> naranja
        let u = t / 0.33;
        lerp_color(Color::new(180, 60, 30, 255), Color::new(255, 140, 40, 255), u)
    } else if t < 0.66 {
        let u = (t - 0.33) / 0.33;
        lerp_color(Color::new(255, 140, 40, 255), Color::new(255, 230, 200, 255), u)
    } else {
        let u = (t - 0.66) / 0.34;
        lerp_color(Color::new(255, 230, 200, 255), Color::new(200, 230, 255, 255), u)
    }
}

// ---------------- Ruido / FBM (hash-based, determinístico) ----------------

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

    // normalizar a [0,1]
    (value / max_value + 1.0) * 0.5
}

// Ruido 2D simple (hash-based) — mejora a gradiente usando suavizado
fn noise2d(x: f32, y: f32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let xf = x - x.floor();
    let yf = y - y.floor();

    // valores de esquina pseudoaleatorios
    let v00 = hash_to_signed_float(xi, yi);
    let v10 = hash_to_signed_float(xi + 1, yi);
    let v01 = hash_to_signed_float(xi, yi + 1);
    let v11 = hash_to_signed_float(xi + 1, yi + 1);

    // interpolación suave (fade)
    let u = fade(xf);
    let v = fade(yf);

    let ix0 = lerp_f32(v00, v10, u);
    let ix1 = lerp_f32(v01, v11, u);
    lerp_f32(ix0, ix1, v)
}

fn fade(t: f32) -> f32 {
    // 6t^5 - 15t^4 + 10t^3 (Perlin fade)
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn hash_to_signed_float(x: i32, y: i32) -> f32 {
    let mut h = x.wrapping_mul(374761393).wrapping_add(y.wrapping_mul(668265263)) as u32;
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    // map to [-1,1]
    (h as f32 / 4294967295.0) * 2.0 - 1.0
}

// ---------------- Utilidades de color ----------------

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::new(
        (a.r as f32 * (1.0 - t) + b.r as f32 * t) as u8,
        (a.g as f32 * (1.0 - t) + b.g as f32 * t) as u8,
        (a.b as f32 * (1.0 - t) + b.b as f32 * t) as u8,
        255,
    )
}

fn apply_brightness(color: Color, brightness: f32) -> Color {
    Color::new(
        ((color.r as f32 * brightness).min(255.0)) as u8,
        ((color.g as f32 * brightness).min(255.0)) as u8,
        ((color.b as f32 * brightness).min(255.0)) as u8,
        color.a,
    )
}

/// Mezcla aditiva parcial (simula glow)
fn blend_additive(base: Color, add: Color, factor: f32) -> Color {
    let r = (base.r as f32 + add.r as f32 * factor).min(255.0) as u8;
    let g = (base.g as f32 + add.g as f32 * factor).min(255.0) as u8;
    let b = (base.b as f32 + add.b as f32 * factor).min(255.0) as u8;
    Color::new(r, g, b, 255)
}
