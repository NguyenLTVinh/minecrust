# MineCRust - A Voxel Engine in Rust

A Minecraft-inspired voxel engine built from scratch in Rust using OpenGL, featuring procedural world generation, multiple biomes, dynamic day-night cycles, and realistic lighting.

## Overview

MineCRust is a 3D voxel-based game engine that generates infinite procedurally generated worlds with multiple biome types, vegetation, trees, and more. The project demonstrates graphics programming, procedural generation algorithms, and Rust's capabilities for systems programming.

## How To Run

### Prerequisites

- Rust 1.70 or later
- OpenGL 3.3 capable GPU
- GLFW3 development libraries (for compilation)

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

### Project Dependencies

- **glfw** (0.42): Window and input management
- **gl** (0.14): OpenGL bindings
- **cgmath** (0.18): 3D math (vectors, matrices)
- **noise** (0.8): Perlin noise generation
- **rand** (0.9.2): Random number generation
- **image** (0.25.8): Texture loading (PNG support)
- **tokio** (1.42): Multi-threaded and async runtime
- **crossbeam-channel** (0.5): Multi-threaded communication
- **lazy_static** (1.4): Global state management

## Features

### 🌍 World Generation

- **Procedural Terrain**: Perlin noise-based terrain generation
- **Multiple Biomes**: Grasslands, deserts, taigas—you name it. New biomes can be added easily thanks to the modular design
- **Water Bodies**: Dynamic water generation at configurable water levels
- **River Channels**: Procedurally generated river systems
- **Mountains**: Layered mountain generation that can be tweaked for extreme heights

### 🌳 Vegetation

- **Tree Generation**: Multiple tree types with realistic structures
- **Flora & Fauna**: Tall and short grass, various flowers, mushrooms, ferns, dead bushes, and more

### 🎮 Gameplay Mechanics

- **Free Camera Movement**: WASD for movement, Space/Shift for vertical movement
- **Mouse Look**: Free-look camera control
- **Chunk System**: Multi-threaded, asynchronous chunk-based rendering with configurable render distance
- **Mesh Optimization**: Only visible faces are rendered using greedy meshing
- **Level of Detail**: Dynamic mesh generation and unloading based on player proximity

### 🎨 Graphics

- **Texture Atlas**: All block textures combined into a single texture atlas
- **Biome-Specific Textures**: Different texture variants for different biomes
- **Dynamic Tinting**: Grass and foliage colors adapt based on biome
- **Vertex Lighting**: Per-vertex lighting calculations for realistic shading

## Credits

MineCRust was created as a demonstration of procedural generation and graphics programming in Rust. Inspired by Minecraft and built with modern Rust practices.