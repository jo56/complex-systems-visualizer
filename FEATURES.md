# Enhanced Features Guide

## 🎨 Color Scheme System

All fractal simulations now support **8 beautiful color schemes**:

- **Classic** - Original rainbow gradient
- **Rainbow** - Vibrant full spectrum
- **Fire** - Black → Red → Orange → Yellow
- **Ice** - Black → Blue → Cyan
- **Grayscale** - Simple black to white
- **Ultra** - High-speed color cycling
- **Sunset** - Purple → Orange → Gold
- **Ocean** - Deep blue → Turquoise

Each scheme supports:
- **Smooth Coloring** - Continuous gradient vs banded colors
- **Color Offset** - Shift the color palette
- **Invert Colors** - Flip black/white and all colors

## 🌀 Mandelbrot Set Enhancements

### New Features:
- **Power Parameter** (z^n) - Explore generalized Mandelbrot sets from z^2 to z^8
- **Escape Radius** - Adjustable bailout value (2-10)
- **Color Cycling** - Animated color rotation
- **Smooth Coloring** - Continuous iteration escape coloring
- **Extended Zoom** - Up to 10,000x zoom
- **5 New Locations**:
  - Seahorse Valley
  - Elephant Valley
  - Spiral
  - Triple Spiral
  - Mini Mandelbrot

### Settings Organization:
- ⚙ **Calculation Settings** - Iterations, power, escape radius
- 🎨 **Color Settings** - Schemes, smoothing, inversion, offset, cycling
- 🔍 **Navigation** - Zoom, center position controls
- 📍 **Interesting Locations** - Quick navigation presets

## 🔥 Burning Ship Fractal (NEW)

A unique fractal using absolute values in iteration:
- All color scheme support
- Smooth coloring
- 3 Interesting locations:
  - Main Ship view
  - Antenna Detail
  - Mast Detail

## 🌊 Julia Set Enhancements

### New Features:
- **Animation Mode** - Automatically animate the c parameter in a circle
- **Animation Radius** - Control size of animation path
- **Power Parameter** - Generalized Julia sets (z^n)
- **All Color Schemes** - Full color palette support
- **Smooth Coloring** - Continuous gradients
- **5 Classic Parameters**:
  - Dendrite
  - San Marco Dragon
  - Siegel Disk
  - Douady's Rabbit
  - Galaxy (new!)

### Settings:
- ⚙ **Calculation Settings** - Iterations, power, escape radius
- 🌀 **Julia Parameter (c)** - Real/imaginary controls + animation
- 🎨 **Color Settings** - Full scheme selection
- 📍 **Interesting Parameters** - Quick presets

## 🧬 Game of Life Massive Upgrade

### New Rule Sets (6 total):
- **Conway (B3/S23)** - Classic Game of Life
- **HighLife (B36/S23)** - Creates replicators
- **Seeds (B2/S)** - Two-neighbor birth only
- **Life Without Death** - Cells never die
- **Day & Night (B3678/S34678)** - Symmetric rule
- **Maze (B3/S12345)** - Creates maze-like patterns

### Pattern Library:
- **Glider Gun** - Creates infinite gliders
- **Glider** - Simple moving pattern
- **Pulsar** - Period-3 oscillator
- **Pentadecathlon** - Period-15 oscillator
- **LWSS** - Lightweight spaceship
- **Acorn** - Creates 633 generations of growth
- **Random Soup** - Random starting configuration

### New Features:
- **Color by Age** - Visualize cell longevity with color gradient
- **Pause/Play** - Independent simulation control
- **Generation Counter** - Track simulation progress
- **Live Cell Count** - Monitor population
- **Rule Selection** - Switch between 6 different CA rules

### UI Organization:
- 🎮 **Controls** - Play/pause, step, clear, speed
- 📐 **Rules** - Select from 6 rule variations
- 🎨 **Visualization** - Age-based coloring
- 🧬 **Pattern Library** - 7 classic patterns

## 🎲 Rössler Attractor (NEW 3D)

New chaotic 3D system with:
- **System Parameters** - a, b, c controls
- **3 Presets**:
  - Classic Rössler
  - Chaotic Spiral
  - Banded Chaos
- Trail length control
- Speed adjustment
- RK4 numerical integration

### Settings:
- ⚙ **System Parameters** - a, b, c sliders
- 🎬 **Visualization** - Speed, trail length
- 📍 **Interesting Configurations** - Quick presets

## 🦋 Lorenz Attractor (Enhanced)

Now with organized collapsible settings:
- ⚙ **System Parameters** - σ, ρ, β controls
- 🎬 **Visualization** - Speed, trail length, reset
- 📍 **Configurations** - Classic Butterfly, Chaotic

## 🎭 UI Improvements

### Visual Enhancements:
- **Collapsible Headers** - All settings organized in expandable sections
- **Emoji Icons** - Visual category identification
- **Scrollable Panel** - Handle long control lists
- **Resizable Sidebar** - 320-400px adjustable width
- **Centered Title** - Professional header layout

### Navigation:
- 📊 **View Mode Toggle** - Quick 2D/3D switching with icons
- 🎨/🌀 **Simulation Selector** - Dropdown with all options
- Organized subsections for each simulation type

## 📊 Simulation Count

**Total Simulations: 7**
- 5 × 2D Simulations (Mandelbrot, Julia, Burning Ship, Game of Life, Cellular Automaton)
- 2 × 3D Simulations (Lorenz, Rössler)

## 🚀 Performance

- All fractals use **parallel computation** via rayon
- Smooth 60 FPS for cellular automata
- RK4 integration for accurate 3D dynamics
- Optimized release build for maximum speed

## 🎯 Quick Tips

1. **Fractals**: Try different color schemes, enable smooth coloring, and use color cycling for dynamic visuals
2. **Mandelbrot**: Increase max iterations when zooming deep (500-1000)
3. **Julia**: Enable animation to watch morphing fractals
4. **Game of Life**: Try different rules with pattern library - Seeds and Maze create unique patterns
5. **3D Attractors**: Adjust trail length for clearer or denser visualizations

## 🔧 Technical Details

### Color System:
- 8 predefined schemes with smooth interpolation
- HSV-based color generation
- Support for continuous (smooth) coloring
- Color offset and cycling animations

### Fractal Features:
- Smooth iteration coloring using escape-time algorithm
- Generalized power parameters (Multibrot/Multijulia sets)
- Adjustable escape radius
- Logarithmic zoom controls

### Cellular Automata:
- 6 different Life-like rule sets
- Cell age tracking with color visualization
- 7 classic pattern presets
- Generation and population statistics

Enjoy exploring the infinite complexity of these systems! 🌌
