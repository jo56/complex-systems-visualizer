# Quick Start Guide

## Running the Application

```bash
# Build and run in release mode (recommended for best performance)
cargo run --release

# Or for development (faster compile, slower runtime)
cargo run
```

The application will open with a control panel on the left and visualization area on the right.

## First Steps

1. **Choose View Mode**: Click **2D** or **3D** at the top of the control panel
2. **Select a Simulation**: Use the dropdown menu to choose which simulation to explore
3. **Adjust Parameters**: Use sliders and controls to modify the simulation in real-time
4. **Explore Presets**: Click preset buttons to jump to interesting configurations

## Recommended Starting Points

### Mandelbrot Set (2D Fractal)
**What it is**: The iconic fractal showing infinite self-similar complexity

**Try this**:
1. Start with the default view
2. Click **"Seahorse Valley"** preset to see intricate spirals
3. Use **mousewheel** to zoom in smoothly
4. **Drag** with your mouse to pan around
5. Try different **color schemes** (Rainbow, Fire, Ice, Galaxy)
6. Increase **Max Iterations** (500-1000) for more detail at high zoom levels

**Tips**:
- Zoom range: 0.1x to 10,000x
- Higher zoom needs more iterations for crisp details
- Try **Color Cycling** for animated rainbow effects

### Julia Set (2D Fractal)
**What it is**: Beautiful fractal variations, each with unique character

**Try this**:
1. Click **"Douady's Rabbit"** to see the famous rabbit shape
2. **Drag the C Real and C Imaginary sliders** to morph the fractal in real-time
3. Enable **Animate** to watch the fractal continuously transform
4. Use **mousewheel** to zoom into fine details
5. Try **"San Marco Dragon"** or **"Siegel Disk"** presets

**Tips**:
- Each (C Real, C Imag) pair creates a completely different fractal
- Animation creates a smooth journey through parameter space

### Burning Ship (2D Fractal)
**What it is**: A fractal using absolute values, creating ship-like structures

**Try this**:
1. Start at **"Main Ship"** to see the overall structure
2. Click **"Antenna Detail"** for intricate masts and rigging
3. Zoom with **mousewheel** and pan by **dragging**
4. Try **Ocean** or **Ice** color schemes

### Conway's Game of Life (2D Cellular Automaton)
**What it is**: Emergent patterns from simple rules - cells live, die, and reproduce

**Try this**:
1. Click **"Glider Gun"** to watch it create infinite moving gliders
2. Use **Pause/Play** button to control time
3. Click **"Step"** to advance one generation at a time
4. Try different **Rules** (HighLife, Maze, Seeds)
5. Enable **"Color by Age"** to see how long cells have lived
6. Click **"Randomize"** for chaos, then watch patterns emerge

**Pattern Guide**:
- **Glider Gun**: Creates moving gliders indefinitely
- **Pulsar**: Oscillates with period 3
- **Pentadecathlon**: Period 15 oscillator
- **Acorn**: Small pattern that evolves for 5,206 generations
- **LWSS**: Lightweight spaceship that moves diagonally

### Lorenz Attractor (3D Chaotic System)
**What it is**: The famous "butterfly effect" system showing chaos

**Try this**:
1. Watch the butterfly-shaped pattern form
2. Drag **Rotation X/Y** sliders to view from different angles
3. Use **mousewheel** while hovering to zoom in/out
4. Adjust **Rho** parameter (0-50) to see bifurcations
5. Try **"Chaotic"** preset for wild behavior
6. Enable **Auto-Rotate** for continuous viewing

**Tips**:
- σ (sigma) = 10 is standard
- ρ (rho) around 28 creates the butterfly
- Increase **Trail Length** for denser visualization

### Reaction-Diffusion (2D Chemical Simulation)
**What it is**: Pattern formation like animal spots and coral growth

**Try this**:
1. Watch patterns emerge from noise
2. Adjust **Feed Rate** and **Kill Rate** for different patterns
3. Lower values create spots, higher values create stripes
4. Wait 10-20 seconds for patterns to stabilize

**Pattern Types**:
- Feed ~0.04, Kill ~0.06: Spots
- Feed ~0.03, Kill ~0.06: Worms
- Feed ~0.06, Kill ~0.06: Coral

### Slime Mold (2D Emergent)
**What it is**: Simulation of Physarum polycephalum behavior

**Try this**:
1. Watch agents create networks optimizing for food sources
2. Increase **Agent Count** for denser trails
3. Adjust **Sensor Angle** and **Turn Angle** for different patterns
4. Higher **Decay Rate** creates sharper trails

### Boids (2D Flocking)
**What it is**: Emergent flocking behavior from simple rules

**Try this**:
1. Watch birds flock naturally
2. Increase **Boid Count** up to 500
3. Adjust **Separation**, **Alignment**, **Cohesion** strengths
4. Enable **Trail Fade** for motion trails
5. Add **Predators** to see avoidance behavior

### N-Body Gravity (3D Physics)
**What it is**: Gravitational orbital mechanics

**Try this**:
1. Watch planets orbit the central mass
2. Increase **Body Count** for more chaos
3. Adjust **Gravitational Constant** for faster/slower orbits
4. Higher **Initial Velocity** creates elliptical orbits
5. Enable **Auto-Rotate** to see 3D structure

### DNA Helix (3D Visual)
**What it is**: Double helix structure visualization

**Try this**:
1. Adjust **Twist Rate** to wind/unwind the helix
2. Change **Number of Base Pairs** (5-50)
3. Increase **Animation Speed** for rotation
4. Drag rotation sliders for different viewing angles

### Galaxy Spiral (3D Astrophysics)
**What it is**: Spiral galaxy with differential rotation

**Try this**:
1. Increase **Number of Arms** (2-8)
2. Adjust **Stars per Arm** for density
3. Change **Rotation Speed** to see galaxy spin
4. Higher **Disk Thickness** for more 3D structure

## 🎨 Color Schemes

All fractals support 26 color schemes:
- **Classic, Rainbow, Fire, Ice** - Traditional palettes
- **Plasma, Viridis, Inferno, Magma** - Scientific visualization
- **Sunset, Ocean, Galaxy** - Nature-inspired
- **Neon, Pastel** - Artistic styles
- **And many more!**

**Tip**: Enable **Smooth Coloring** for continuous gradients!

## Controls Summary

### 2D Fractals
- **Mousewheel**: Zoom in/out
- **Left-click + Drag**: Pan around
- **Sliders**: Adjust all parameters
- **Presets**: Quick navigation

### 3D Simulations
- **Mousewheel** (when hovering): Quick zoom
- **Rotation Sliders**: Change viewing angle
- **Auto-Rotate**: Automatic rotation
- **Parameter Sliders**: Adjust system behavior

### General
- **View Mode Toggle**: Switch 2D ↔ 3D
- **Simulation Dropdown**: Select which simulation
- **Collapsible Sections**: Organize parameters
- **Reset Buttons**: Return to defaults

## Performance Tips

1. **Always use `cargo run --release`** for 10x better performance
2. **High zoom fractals**: Increase Max Iterations (500-1000)
3. **Slow simulations**: Reduce particle/agent counts
4. **Smooth 60 FPS**: Release mode + reasonable parameters
5. **Parallel rendering**: Automatic on all 2D simulations

## Troubleshooting

**Build errors**: Ensure Rust 1.70+ is installed
```bash
rustc --version
```

**Poor performance**: Use `cargo run --release`, not debug mode

**Window doesn't open**: Check graphics drivers support OpenGL/DirectX

**Sliders unresponsive**: Try clicking directly on values to type numbers

## Learning Path

### Beginner (Start Here)
1. Mandelbrot Set - Explore with mousewheel zoom
2. Julia Set - Try different presets
3. Game of Life - Watch glider guns
4. Lorenz Attractor - See chaos in 3D

### Intermediate
5. Reaction-Diffusion - Create spot/stripe patterns
6. Boids - Understand emergent flocking
7. Burning Ship - Unusual fractal forms
8. Galaxy Spiral - Astrophysical simulation

### Advanced
9. Slime Mold - Complex emergent networks
10. N-Body Gravity - Multi-body chaos
11. Fluid SPH - Particle fluid dynamics
12. Strange Attractors - Mathematical art

## Pro Tips

1. **Fractals**: Zoom slowly for smooth transitions, increase iterations at high zoom
2. **Game of Life**: Try different rules with same patterns for variety
3. **Attractors**: Long trails (5000+) show structure, short trails (100) show motion
4. **Color cycling**: Animate stationary fractals for dynamic effects
5. **Combination**: Switch between 2D and 3D frequently for variety

## Next Steps

- Experiment with **all 39 simulations**
- Try every preset and color scheme
- Adjust parameters to extremes and see what happens
- Read **[FEATURES.md](FEATURES.md)** for detailed documentation
- Explore the source code in `sim-core/src/` to understand algorithms
- Add your own simulations using the trait system!

## 🌌 Enjoy Exploring!

These simulations demonstrate how simple mathematical rules create infinite complexity. Take your time to explore, experiment, and discover the beauty hidden in mathematics and physics!

**Pro tip**: Screenshot your favorite discoveries - you may never find that exact view again! 📸
