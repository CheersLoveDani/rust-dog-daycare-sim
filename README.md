# Pixel Grid Fluid Simulation

A real-time 2D fluid simulation implemented in Rust using Navier-Stokes equations on a 500x500 pixel grid.

## Features

- **Real-time fluid dynamics simulation** using Navier-Stokes solver
- **Interactive controls** - Click and drag to add fluid and velocity
- **500x500 grid resolution** for detailed fluid behavior
- **Colorful visualization** with blue-to-white density gradient
- **Physics-based simulation** including:
  - Velocity advection
  - Density diffusion
  - Divergence-free projection
  - Viscosity effects

## Building and Running

### Prerequisites

- Rust toolchain (1.70+)
- Graphics drivers supporting OpenGL or Vulkan

### Build

```bash
# Debug build
cargo build

# Release build (recommended for better performance)
cargo build --release
```

### Run

```bash
# Run in debug mode
cargo run

# Run in release mode (recommended)
cargo run --release
```

## Usage

- **Left Click + Drag**: Add fluid density and create velocity in the direction of mouse movement
- **Close Window**: Exit the application

The fluid will naturally dissipate over time, creating beautiful swirling patterns as it flows across the grid.

## Implementation Details

### Navier-Stokes Solver

The simulation implements a stable fluids algorithm with the following steps each frame:

1. **Velocity Diffusion**: Spreads velocity through viscosity
2. **Velocity Projection**: Makes the velocity field divergence-free (incompressible)
3. **Velocity Advection**: Moves velocity through itself
4. **Density Diffusion**: Spreads density values
5. **Density Advection**: Moves density through the velocity field

### Parameters

- Grid size: 500x500
- Diffusion rate: 0.2
- Viscosity: 0.0000001
- Time step: 0.1
- Solver iterations: 16

## Technical Stack

- **pixels**: Hardware-accelerated pixel buffer for rendering
- **winit**: Cross-platform window creation and event handling
- **Rust**: Safe, fast systems programming

## License

This project is open source and available under the MIT License.
