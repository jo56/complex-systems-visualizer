# Complex Systems Visualizer

A fullstack Rust application for exploring complex systems, fractals, and emergent behavior through interactive visualizations. Built with [egui](https://github.com/emilk/egui) for the GUI.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Features

### 2D Simulations

- **Mandelbrot Set** - Explore the famous fractal with infinite detail
  - Adjustable iteration count and zoom
  - Interactive parameter controls
  - Colorization options
  - Quick navigation to interesting locations (Seahorse Valley, Elephant Valley, Spirals)

- **Julia Set** - Related fractals with beautiful variations
  - Adjustable complex parameter (c)
  - Multiple preset configurations (Dendrite, San Marco Dragon, Siegel Disk, Douady's Rabbit)
  - Real-time parameter tweaking

- **Conway's Game of Life** - Cellular automaton showing emergent complexity
  - Pre-loaded with Gosper's Glider Gun pattern
  - Adjustable simulation speed
  - Step-by-step or continuous mode
  - Random initialization

- **Elementary Cellular Automaton** - Wolfram's 1D cellular automata
  - All 256 rules supported
  - Famous patterns: Rule 30 (chaotic), Rule 110 (Turing complete), Rule 90 (Sierpinski triangle)
  - Visual evolution from simple rules to complex patterns

### 3D Simulations

- **Lorenz Attractor** - Chaotic system demonstrating the butterfly effect
  - Real-time 3D trajectory visualization
  - Adjustable parameters (σ, ρ, β)
  - Interactive rotation and zoom
  - Trail rendering to show system evolution

## Architecture

The project uses a Cargo workspace structure for modularity:

```
mandlebrot-visualizer/
├── sim-core/          # Core simulation library
│   ├── src/
│   │   ├── lib.rs              # Simulation traits
│   │   ├── mandelbrot.rs       # Mandelbrot implementation
│   │   ├── julia.rs            # Julia set implementation
│   │   ├── game_of_life.rs     # Game of Life implementation
│   │   ├── cellular_automaton.rs # Elementary CA implementation
│   │   └── lorenz.rs           # Lorenz attractor implementation
│   └── Cargo.toml
│
├── sim-app/           # GUI application
│   ├── src/
│   │   ├── main.rs             # Application entry point
│   │   ├── viewer_2d.rs        # 2D visualization renderer
│   │   └── viewer_3d.rs        # 3D visualization renderer
│   └── Cargo.toml
│
└── Cargo.toml         # Workspace configuration
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later)
- Cargo (comes with Rust)

### Building

```bash
# Clone the repository
git clone <repository-url>
cd mandlebrot-visualizer

# Build the project
cargo build --release

# Run the application
cargo run --release --bin complex-viz
```

### Development Build

```bash
cargo run --bin complex-viz
```

## Usage

1. **Select Simulation Type**: Choose between 2D and 3D simulations using the view mode selector
2. **Choose Simulation**: Use the dropdown menu to select which simulation to run
3. **Adjust Parameters**: Use the control panel sliders and inputs to modify simulation parameters
4. **Explore**:
   - For fractals: Adjust zoom, position, and iteration counts
   - For cellular automata: Change rules, step through generations, or let it run continuously
   - For 3D systems: Rotate the view and adjust system parameters

### Controls

#### 2D Simulations
- Parameters update in real-time
- Use preset buttons for interesting configurations
- Mandelbrot/Julia: Drag values or use sliders for precise control

#### 3D Simulations
- Rotation sliders to change viewing angle
- Zoom slider to adjust scale
- Reset button to return to default view
- System parameters adjust the underlying physics

## Technology Stack

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[egui](https://github.com/emilk/egui)** - Immediate mode GUI library
- **[eframe](https://github.com/emilk/egui/tree/master/crates/eframe)** - Application framework
- **[egui_plot](https://github.com/emilk/egui_plot)** - Plotting library for 3D visualization
- **[rayon](https://github.com/rayon-rs/rayon)** - Data parallelism for performance
- **[num-complex](https://github.com/rust-num/num-complex)** - Complex number arithmetic

## Performance

All 2D simulations use parallel computation via `rayon` for fast rendering. The application is designed to handle:
- High-resolution fractal rendering (800x600+ pixels)
- Real-time parameter updates
- Smooth 60 FPS animations for cellular automata and 3D systems

## Future Enhancements

Potential additions for the project:
- More 3D attractors (Rössler, Aizawa, etc.)
- Langton's Ant and other 2D automata
- Reaction-diffusion systems (Gray-Scott model)
- L-systems for fractal generation
- Export functionality (save images/videos)
- Custom coloring schemes
- Mouse interaction for fractal zooming

## Contributing

Contributions are welcome! Feel free to:
- Add new simulations
- Improve performance
- Enhance UI/UX
- Fix bugs
- Add documentation

## License

This project is licensed under the MIT License.

## Acknowledgments

- Inspired by the beauty of mathematics and emergent complexity
- Built with the excellent Rust ecosystem
- Thanks to the egui community for the amazing GUI framework

## Screenshots

*Run the application to see the visualizations in action!*

---

**Built with ❤️ and Rust**
