# 🌀 Ray Tracer in Rust

A minimal ray tracer written in Rust that renders spheres with basic shading and outputs the result as a PNG image.

## 🛠 Features

- Renders a 3D scene with spheres
- Gradient background sky
- Outputs to `output.png`
- Written entirely in safe, idiomatic Rust

## 🚀 Getting Started

### Prerequisites
- Rust (install from [rustup.rs](https://rustup.rs))

### Build & Run

```bash
git clone https://github.com/your-username/ray-tracer.git
cd ray-tracer
cargo run
```

## 📁 Project Structure

```bash
src/
├── main.rs          # Entry point
├── ray.rs           # Ray struct and functions
├── vec3.rs          # 3D vector math
├── sphere.rs        # Sphere object and intersection logic
├── hit.rs           # Hit record and hittable trait
├── camera.rs        # Basic camera for ray generation
└── render.rs        # Scene rendering logic
```

### 🧠 TODO
Add lighting and shadows

Support for multiple objects and materials

Anti-aliasing and reflections
