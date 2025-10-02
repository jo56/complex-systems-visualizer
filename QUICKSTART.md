# Quick Start Guide

## Running the Application

```bash
# Build and run in release mode (recommended for best performance)
cargo run --release --bin mandlebrot-viz

# Or for development (faster compile, slower runtime)
cargo run --bin mandlebrot-viz
```

## What You'll See

The application opens with a side panel on the left for controls and a main visualization area.

### Getting Started

1. **Choose View Mode**: Click "2D" or "3D" at the top of the control panel
2. **Select a Simulation**: Use the dropdown to choose which simulation to run
3. **Adjust Parameters**: Use sliders and inputs to modify the simulation in real-time
4. **Explore Presets**: Click preset buttons to jump to interesting configurations

## Simulations Overview

### Mandelbrot Set (2D)
- **What it is**: The classic fractal showing infinite complexity
- **Try this**:
  - Start with default view
  - Click "Seahorse Valley" to see intricate details
  - Adjust "Zoom" slider to explore deeper
  - Increase "Max Iterations" for more detail at high zoom

### Julia Set (2D)
- **What it is**: Related fractals with beautiful variations
- **Try this**:
  - Drag "C Real" and "C Imaginary" to see how the fractal changes
  - Click "Douady's Rabbit" for a famous shape
  - Adjust zoom to see fine details

### Conway's Game of Life (2D)
- **What it is**: Cellular automaton where simple rules create complex patterns
- **Try this**:
  - Watch the Glider Gun create moving patterns
  - Click "Randomize" for chaos
  - Adjust "Steps per second" to speed up or slow down
  - Click "Step" to advance one generation at a time

### Elementary Cellular Automaton (2D)
- **What it is**: 1D cellular automata showing how simple rules generate complexity
- **Try this**:
  - Start with "Rule 30" (chaotic)
  - Try "Rule 90" to see Sierpinski triangle emerge
  - Change the rule number (0-255) to explore all possibilities
  - Click "Random Start" for varied patterns

### Lorenz Attractor (3D)
- **What it is**: Chaotic system demonstrating sensitive dependence (butterfly effect)
- **Try this**:
  - Watch the butterfly pattern form
  - Drag "Rotation X/Y" sliders to view from different angles
  - Adjust "Rho" to change the system behavior
  - Click "Chaotic" preset for wild trajectories

## Performance Tips

- **2D Simulations**: Automatically use all CPU cores for fast rendering
- **High Zoom**: Increase "Max Iterations" for fractals at high zoom levels
- **Slow Performance**: Use `cargo run --release` instead of debug mode
- **Window Size**: Smaller windows render faster

## Keyboard Shortcuts

- The application uses egui's default shortcuts
- Use mouse wheel or trackpad to adjust sliders
- Drag values for precise control

## Troubleshooting

**Build Errors**: Make sure you have Rust 1.70 or later installed
```bash
rustc --version
```

**Performance Issues**: Always use `--release` flag for optimal performance

**Window Doesn't Open**: Check that your graphics drivers support OpenGL/DirectX

## Next Steps

- Experiment with all the presets
- Try combining different parameter values
- Explore the code in `sim-core/src/` to understand the algorithms
- Add your own simulations by implementing the `Simulation2D` or `Simulation3D` traits

## Enjoy Exploring Complex Systems!

The beauty of these simulations lies in discovering how simple mathematical rules can generate infinite complexity. Take your time to explore and experiment!
