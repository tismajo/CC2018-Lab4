use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub z_buffer: Vec<f32>,
    background_color: Color,
    current_color: Color,
    pub texture: Option<Texture2D>, // Ahora es público
}

impl Framebuffer {
    /// Crea un nuevo framebuffer con un Z-buffer inicializado
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        let z_buffer = vec![f32::INFINITY; (width * height) as usize];
        
        Self {
            width,
            height,
            color_buffer,
            z_buffer,
            background_color,
            current_color: Color::WHITE,
            texture: None,
        }
    }

    /// Limpia el framebuffer y el Z-buffer
    pub fn clear(&mut self) {
        self.color_buffer.clear_background(self.background_color);
        self.z_buffer.fill(f32::INFINITY);
    }

    /// Cambia el color actual de dibujo
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    /// Cambia el color de fondo
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /// Dibuja un píxel en la posición (x, y)
    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }

    /// Dibuja un píxel con color explícito
    pub fn set_pixel_with_color(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.color_buffer.draw_pixel(x, y, color);
        }
    }

    /// Dibuja un píxel con control de profundidad (Z-buffer)
    pub fn set_pixel_depth(&mut self, x: i32, y: i32, depth: f32) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            let idx = (y as u32 * self.width + x as u32) as usize;
            if depth < self.z_buffer[idx] {
                self.z_buffer[idx] = depth;
                self.color_buffer.draw_pixel(x, y, self.current_color);
            }
        }
    }

    /// Inicializa la textura para la GPU (una sola vez)
    pub fn init_texture(&mut self, window: &mut RaylibHandle, thread: &RaylibThread) {
        if self.texture.is_none() {
            if let Ok(tex) = window.load_texture_from_image(thread, &self.color_buffer) {
                self.texture = Some(tex);
            }
        }
    }

    /// Guarda el framebuffer en una imagen
    pub fn render_to_file(&self, path: &str) {
        self.color_buffer.export_image(path);
    }
}
