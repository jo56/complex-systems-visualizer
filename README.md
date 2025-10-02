# Complex Systems Visualizer

A fullstack Rust application for exploring complex systems, fractals, and emergent behavior through interactive visualizations. Built with [egui](https://github.com/emilk/egui) for the GUI.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## ✨ Features

### 🎨 Universal Color System

All fractals support **8 stunning color schemes**:
- Classic, Rainbow, Fire, Ice, Grayscale, Ultra, Sunset, Ocean
- **Smooth coloring** for continuous gradients
- **Color offset** and **cycling** for dynamic animations
- **Invert colors** option

### 🌀 Fractal Simulations (2D)

#### **Mandelbrot Set** - Classic fractal explorer
- **Power parameter** (z^2 to z^8) - Explore Multibrot sets
- **8 color schemes** with smooth coloring
- **Color cycling animation**
- **Extended zoom** up to 10,000×
- **5 preset locations** (Seahorse Valley, Elephant Valley, Spirals, Mini Mandelbrot)
- Adjustable escape radius and iterations (up to 1000)

#### **Julia Set** - Morphing fractals
- **Animation mode** - Watch c parameter orbit in real-time
- **Power parameter** - Multijulia sets
- **8 color schemes** with all effects
- **5 classic presets** (Dendrite, San Marco Dragon, Siegel Disk, Douady's Rabbit, Galaxy)
- Smooth coloring with adjustable parameters

#### **Burning Ship** - Unique fractal (NEW!)
- Uses absolute values for unique ship-like structure
- Full color scheme support
- 3 detailed zoom locations
- Smooth iteration coloring

#### **Conway's Game of Life** - Emergent complexity
- **6 rule variations**:
  - Conway (B3/S23) - Classic
  - HighLife (B36/S23) - Creates replicators
  - Seeds (B2/S) - Two-neighbor birth
  - Life Without Death - Immortal cells
  - Day & Night (B3678/S34678) - Symmetric
  - Maze (B3/S12345) - Maze generator
- **7 pattern library**: Glider Gun, Glider, Pulsar, Pentadecathlon, LWSS, Acorn, Random Soup
- **Color by age** visualization
- Generation counter and live cell statistics
- Pause/play controls

#### **Elementary Cellular Automaton** - Wolfram's 1D CA
- All 256 rules (including 30, 110, 90, 184)
- Color-coded by rule
- Multiple initial conditions
- Auto-stepping with speed control

### 🎲 3D Chaotic Systems

#### **Lorenz Attractor** - The butterfly effect
- Classic chaotic system (σ, ρ, β parameters)
- Interactive 3D rotation
- Trail length control (up to 10,000 points)
- RK4 numerical integration
- 2 preset configurations

#### **Rössler Attractor** - Continuous chaos (NEW!)
- Adjustable system parameters (a, b, c)
- 3 interesting configurations
- Smooth trajectory rendering
- Speed and trail controls

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

## 🎯 Key Highlights

- **7 Total Simulations** (5 × 2D, 2 × 3D)
- **8 Color Schemes** for all fractals
- **6 Game of Life Rules** with 7 pattern presets
- **Smooth Coloring** - Continuous iteration escape algorithms
- **Power Parameters** - Generalized Mandelbrot/Julia sets (z^n)
- **Animation Support** - Julia set morphing, color cycling
- **Organized UI** - Collapsible sections with emoji icons
- **Resizable Panel** - 320-400px adjustable control sidebar

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

- **[FEATURES.md](FEATURES.md)** - Comprehensive feature guide with all settings explained
- **[QUICKSTART.md](QUICKSTART.md)** - Quick start guide for new users

## 🎨 What's New in v0.2.0

### Color & Rendering
- ✨ 8 beautiful color schemes (Classic, Rainbow, Fire, Ice, Grayscale, Ultra, Sunset, Ocean)
- ✨ Smooth (continuous) coloring for all fractals
- ✨ Color offset and cycling animations
- ✨ Color inversion option

### Fractals Enhanced
- 🌀 Mandelbrot: Power parameter (z^2 to z^8), color cycling, 2 new locations
- 🌊 Julia: Animation mode, power parameter, new Galaxy preset
- 🔥 Burning Ship: NEW fractal simulation added

### Game of Life Expanded
- 🧬 6 rule variations (Conway, HighLife, Seeds, Life Without Death, Day & Night, Maze)
- 📚 7 pattern library (Glider Gun, Pulsar, Pentadecathlon, LWSS, Acorn, etc.)
- 🎨 Color by age visualization
- 📊 Generation counter and population stats

### 3D Systems
- 🎲 Rössler Attractor: NEW chaotic system added
- 🦋 Lorenz: Enhanced with organized collapsible UI

### UI/UX
- 🎭 Collapsible setting sections with emoji icons
- 📏 Resizable control panel (320-400px)
- 📜 Scrollable settings for long parameter lists

## 🚀 Future Enhancements

Potential additions:
- Aizawa and Chen attractors
- Reaction-diffusion systems (Gray-Scott model)
- Langton's Ant
- L-systems
- Mouse interaction for fractal zooming
- Image/video export

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
