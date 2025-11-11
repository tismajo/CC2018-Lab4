# ☀️ Star Shader Lab – Estrella Procedural en Rust

Proyecto del **Laboratorio de Shaders**, cuyo objetivo es diseñar y animar una **estrella (sol)** utilizando únicamente funciones de ruido y una malla esférica base.

---

## 🌟 Objetivo

Simular una estrella animada que muestre:
- **Turbulencias** y actividad solar sobre la superficie.
- **Pulsaciones cíclicas** de brillo y emisión.
- **Gradiente dinámico de color** según temperatura e intensidad.
- **Desplazamientos superficiales (flare)** en el vertex shader.

Todo generado **procedimentalmente** mediante funciones de ruido y una variable de tiempo.

---

## ⚙️ Especificaciones Técnicas

| Requisito | Implementación |
|------------|----------------|
| **Geometría base** | `sphere-1.obj` (único modelo utilizado). |
| **Texturas / materiales** | No se usan. Todo el color y emisión son procedimentales. |
| **Uniformes** | `time` – controla animación y pulsaciones. |
| **Shader principal** | `star_shader()` – combina ruido + tiempo para color y emisión. |
| **Vertex displacement** | `star_vertex_displacement()` – desplaza vértices radialmente con FBM. |
| **Ruido** | `fbm_noise()` (Fractional Brownian Motion) con `noise2d()` tipo Perlin. |
| **Animación** | Cíclica y continua (`sin(time)`, `fbm` animado con desplazamiento de coordenadas). |

---

## 🧠 Descripción Técnica

### 🔸 Funciones de ruido
- **`noise2d(x, y)`**  
  Ruido pseudo-Perlin implementado con hashing e interpolación `fade()` (6t⁵ - 15t⁴ + 10t³).  
  Devuelve valores suaves en `[-1, 1]`.

- **`fbm_noise(x, y, octaves)`**  
  Combina múltiples capas de `noise2d` con distintas frecuencias y amplitudes (0.5 por octava), generando turbulencias a diferentes escalas.

### 🔸 Vertex Shader (CPU-simulado)
`star_vertex_displacement()` aplica:
- Desplazamiento radial dependiente de `fbm_noise`.
- Pulso global `sin(time)` que expande/contrae la superficie.
- Simula actividad solar y flujos de energía en la corona.

### 🔸 Fragment Shader (CPU-simulado)
`star_shader()` calcula:
- **Emisión variable:** mezcla de ruido y pulsaciones periódicas (`emission`, `peak_emission`).
- **Gradiente de temperatura:** colores de rojo → amarillo → blanco → azul según intensidad.
- **Flare adicional:** mezcla aditiva de brillo en zonas de mayor ruido.
- **Sombreado direccional** para sugerir relieve.

---

## 🕹️ Controles

| Tecla | Acción |
|-------|--------|
| ← / → | Rotar la estrella |
| Q / E | Zoom in/out |
| ESC | Salir |

---

## 🎬 Resultado esperado

La animación muestra una **estrella viva**, con:
- Pulsaciones regulares en su superficie.
- Zonas más calientes y brillantes.
- Pequeños picos de energía aleatorios (flares).
- Distorsiones suaves del contorno esférico.

---

## 📸 Demostración

<video controls src="Screen Recording 2025-11-11 134146.mp4" title="Title"></video>
