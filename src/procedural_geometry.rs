// procedural_geometry.rs
// Generación de geometría procedural: anillos y lunas
use raylib::prelude::*;
use crate::obj_loader::ObjModel;
use std::f32::consts::PI;

/// Genera una luna esférica proceduralmente
pub fn generate_moon(radius: f32, segments: u32) -> ObjModel {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    
    // Generar vértices usando coordenadas esféricas
    for lat in 0..=segments {
        let theta = lat as f32 * PI / segments as f32; // 0 a π
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        
        for lon in 0..=segments {
            let phi = lon as f32 * 2.0 * PI / segments as f32; // 0 a 2π
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();
            
            let x = radius * sin_theta * cos_phi;
            let y = radius * cos_theta;
            let z = radius * sin_theta * sin_phi;
            
            vertices.push(Vector3::new(x, y, z));
        }
    }
    
    // Generar caras (triángulos)
    for lat in 0..segments {
        for lon in 0..segments {
            let current = (lat * (segments + 1) + lon) as usize;
            let next = (current + segments as usize + 1) as usize;
            
            // Primer triángulo
            faces.push(vec![current, next, current + 1]);
            // Segundo triángulo
            faces.push(vec![current + 1, next, next + 1]);
        }
    }
    
    ObjModel { vertices, faces }
}

/// Genera anillos procedurales (disco plano con agujero)
pub fn generate_rings(inner_radius: f32, outer_radius: f32, segments: u32) -> ObjModel {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    
    // Generar vértices en círculos concéntricos
    for ring in 0..=1 {
        let radius = if ring == 0 { inner_radius } else { outer_radius };
        
        for i in 0..=segments {
            let angle = i as f32 * 2.0 * PI / segments as f32;
            let x = radius * angle.cos();
            let z = radius * angle.sin();
            let y = 0.0; // Anillos en el plano XZ
            
            vertices.push(Vector3::new(x, y, z));
        }
    }
    
    // Generar caras conectando los dos círculos
    let segs = (segments + 1) as usize;
    for i in 0..segments as usize {
        let inner_current = i;
        let inner_next = i + 1;
        let outer_current = i + segs;
        let outer_next = i + 1 + segs;
        
        // Dos triángulos por segmento
        faces.push(vec![inner_current, outer_current, inner_next]);
        faces.push(vec![inner_next, outer_current, outer_next]);
    }
    
    ObjModel { vertices, faces }
}

/// Aplica transformaciones a un modelo: traslación, rotación y escala
pub fn transform_model(
    model: &ObjModel,
    translation: Vector3,
    rotation_y: f32,
    rotation_x: f32,
    scale: f32,
) -> Vec<Vector3> {
    model.vertices.iter().map(|v| {
        let mut x = v.x * scale;
        let mut y = v.y * scale;
        let mut z = v.z * scale;
        
        // Rotación en X (para inclinar anillos)
        let ry = y * rotation_x.cos() - z * rotation_x.sin();
        let rz = y * rotation_x.sin() + z * rotation_x.cos();
        y = ry;
        z = rz;
        
        // Rotación en Y
        let rx = x * rotation_y.cos() + z * rotation_y.sin();
        let rz2 = -x * rotation_y.sin() + z * rotation_y.cos();
        x = rx;
        z = rz2;
        
        // Traslación
        Vector3::new(x + translation.x, y + translation.y, z + translation.z)
    }).collect()
}
