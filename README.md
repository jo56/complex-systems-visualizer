# Complex Systems Visualizer

A fullstack Rust application for exploring complex systems, fractals, and emergent behavior through interactive visualizations. Built with [egui](https://github.com/emilk/egui) for the GUI.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Architecture

The project uses a Cargo workspace structure for modularity:

```
mandlebrot-visualizer/
├── sim-core/          # Core simulation library
│   ├── src/
│   │   ├── lib.rs              # Simulation traits + color system
│   │   ├── mandelbrot.rs       # Mandelbrot (enhanced)
│   │   ├── julia.rs            # Julia set (enhanced)
│   │   ├── burning_ship.rs     # Burning Ship fractal (NEW)
│   │   ├── game_of_life.rs     # Game of Life (6 rules, 7 patterns)
│   │   ├── cellular_automaton.rs # Elementary CA
│   │   ├── lorenz.rs           # Lorenz attractor
│   │   └── rossler.rs          # Rössler attractor (NEW)
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
cargo run --release --bin mandlebrot-viz
```

### Development Build

```bash
cargo run --bin mandlebrot-viz
```

## Usage

1. **Select Simulation Type**: Choose between 2D and 3D simulations using the view mode selector
2. **Choose Simulation**: Use the dropdown menu to select which simulation to run
3. **Adjust Parameters**: Use the control panel sliders and inputs to modify simulation parameters
4. **Explore**

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

## 💻 Technology Stack

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

## 📚 Documentation

- **[QUICKSTART.md](QUICKSTART.md)** - Quick start guide for new users

## License

This project is licensed under the MIT License.

## Acknowledgments

- Built with the excellent Rust ecosystem
- Thanks to the egui community for the amazing GUI framework

