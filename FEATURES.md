# Complete Features Guide

## Overview

**Complex Systems Visualizer** includes **39 unique simulations** organized into 2D and 3D categories, each with extensive customization options and interactive controls.

## Core Features

### Interactive Controls
- **Real-time Parameter Adjustment**: All sliders update visualization immediately
- **Mousewheel Zoom**: Smooth zooming for fractals (Mandelbrot, Julia, Burning Ship)
- **Drag-to-Pan**: Navigate fractal landscapes by clicking and dragging
- **Preset Configurations**: Quick access to interesting parameter combinations
- **Collapsible UI Sections**: Organized controls with emoji category icons
- **Resizable Side Panel**: Adjust control panel width (400-550px)

### Visual Enhancements
- **26 Color Schemes**: Beautiful palettes for all fractals and visualizations
- **Smooth Coloring**: Continuous gradients for fractals
- **Color Cycling**: Animated color rotation
- **Auto-Rotate**: Automatic 3D view rotation
- **Trail Visualization**: Motion trails for particles and attractors
- **Age-Based Coloring**: Visualize temporal data

## Color Scheme System

All fractal simulations support **26 professional color schemes**:

### Scientific Palettes
- **Plasma** - Purple-pink-orange gradient
- **Viridis** - Perceptually uniform blue-green-yellow
- **Inferno** - Black-purple-orange-yellow
- **Magma** - Black-purple-pink-yellow
- **Cividis** - Blue-yellow (colorblind-friendly)
- **Turbo** - Rainbow alternative

### Classic Palettes
- **Classic** - Traditional rainbow with value variation
- **Rainbow** - Full spectrum
- **Grayscale** - Simple black to white
- **Ultra** - High-speed color cycling

### Nature-Inspired
- **Fire** - Black → Red → Orange → Yellow
- **Ice** - Black → Blue → Cyan → White
- **Sunset** - Purple → Orange → Gold
- **Ocean** - Deep blue → Turquoise → Foam
- **Galaxy** - Deep space purples and blues
- **Earth** - Browns and greens

### Artistic
- **Neon** - Vibrant electric colors
- **Pastel** - Soft, muted tones
- **Copper** - Metallic warm tones
- **CoolWarm** - Blue-white-red diverging
- **Spectral** - Full spectral rainbow
- **Purple, Green, Blues** - Single-hue gradients
- **YellowOrangeBrown** - Warm earth tones
- **PinkYellow** - Vibrant dual-tone

### Color Features
- **Smooth Coloring**: Toggle continuous vs. banded colors
- **Color Offset**: Shift palette for different effects
- **Invert Colors**: Flip black/white and all colors
- **Color Cycling**: Animated palette rotation

## 🔬 2D Simulations (22 Total)

### Fractals

#### Mandelbrot Set
**Description**: The iconic fractal showing infinite self-similar complexity

**Features**:
- **Zoom Range**: 0.1x to 10,000x
- **Power Parameter**: Generalized Mandelbrot (z^2 to z^8)
- **Escape Radius**: Adjustable bailout (2.0-10.0)
- **Max Iterations**: 10-1000 for detail control
- **Smooth Coloring**: Continuous iteration escape
- **Color Cycling**: Animated rainbow rotation
- **All 26 Color Schemes**

**Preset Locations**:
- Default Overview
- Seahorse Valley
- Elephant Valley
- Spiral
- Triple Spiral
- Mini Mandelbrot

**Controls**:
- Mousewheel zoom
- Drag to pan
- Zoom slider (0.1-10,000x)
- Center X/Y position
- Max iterations slider
- Power slider (2.0-8.0)

#### Julia Set
**Description**: Beautiful fractal variations based on complex parameter c

**Features**:
- **Animation Mode**: Auto-animate the c parameter
- **Animation Radius**: Control path size (0.1-1.0)
- **Power Parameter**: Generalized Julia sets (z^2 to z^8)
- **All Color Schemes**: 26 palettes
- **Smooth Coloring**: Continuous gradients
- **Interactive Parameter Control**: Real-time morphing

**Preset Parameters**:
- Dendrite
- San Marco Dragon
- Siegel Disk
- Douady's Rabbit
- Galaxy

**Controls**:
- C Real/Imaginary sliders
- Animate checkbox
- Animation radius
- Power parameter
- All fractal controls

#### Burning Ship
**Description**: Unique fractal using absolute values

**Features**:
- Similar to Mandelbrot but with abs() function
- Creates ship-like structures
- All color schemes
- Smooth coloring
- Zoom up to 10,000x

**Preset Locations**:
- Main Ship
- Antenna Detail
- Mast Detail

### Cellular Automata

#### Game of Life
**Description**: Conway's classic cellular automaton

**Features**:
- **6 Rule Sets**: Conway, HighLife, Seeds, Life Without Death, Day & Night, Maze
- **7 Pattern Presets**: Glider Gun, Glider, Pulsar, Pentadecathlon, LWSS, Acorn, Random
- **Color by Age**: Gradient based on cell longevity
- **Generation Counter**: Track simulation progress
- **Live Cell Count**: Population monitoring
- **Pause/Play/Step**: Precise control
- **Speed Control**: 1-60 steps per second

**Rule Sets**:
- **Conway (B3/S23)**: Classic Game of Life
- **HighLife (B36/S23)**: Creates replicators
- **Seeds (B2/S)**: Two-neighbor birth only
- **Life Without Death (B3/S012345678)**: Cells never die
- **Day & Night (B3678/S34678)**: Symmetric patterns
- **Maze (B3/S12345)**: Maze-like structures

#### Elementary Cellular Automaton
**Description**: 1D cellular automata (256 rules)

**Features**:
- Rule number selector (0-255)
- Random or single-cell start
- Sierpinski triangles (Rule 90)
- Chaos (Rule 30)
- Speed control (1-60 FPS)

#### Langton's Ant
**Description**: Emergent complexity from simple movement rules

**Features**:
- Grid size control (50x50 to 400x300)
- Speed adjustment (1-1000 steps/frame)
- Randomize position
- Highway formation visualization

#### Cyclic Cellular Automaton
**Description**: Rock-paper-scissors style CA

**Features**:
- State count (3-24 colors)
- Threshold control (1-8)
- Grid size adjustment
- Speed control (0.1-60 FPS)
- Spiral pattern formation

### Growth & Self-Organization

#### DLA (Diffusion-Limited Aggregation)
**Description**: Particle aggregation creating coral-like structures

**Features**:
- Particle count (100-10,000)
- Stickiness factor (0.1-1.0)
- Real-time growth visualization
- Fractal dimension ~1.71

#### Sandpile
**Description**: Abelian sandpile model

**Features**:
- Drop rate control (0.1-100 grains/second)
- Critical mass threshold (3-8)
- Grid size (50x50 to 250x250)
- Avalanche visualization
- Self-organized criticality

### Physical Simulations

#### Double Pendulum
**Description**: Chaotic motion of coupled pendulums

**Features**:
- Length adjustment (both arms)
- Mass adjustment (both bobs)
- Gravity control (0.1-3.0)
- Damping factor (0.99-1.0)
- Trace length (10-2000)
- Scale adjustment
- Reset to random state

#### Reaction-Diffusion
**Description**: Gray-Scott model creating organic patterns

**Features**:
- Feed rate (0.01-0.1)
- Kill rate (0.03-0.08)
- Diffusion rates for A and B
- Resolution control (32x32 to 256x256)
- Pattern types: spots, stripes, coral, worms

#### Lissajous Curves
**Description**: Parametric curves from harmonic motion

**Features**:
- Frequency X/Y (1.0-10.0)
- Phase shift (0-2π)
- Amplitude X/Y (0.1-0.5)
- Point count (100-5000)
- Line width (1.0-5.0)
- Animation speed
- Multiple color schemes

#### Wave Interference
**Description**: Multiple wave sources creating interference patterns

**Features**:
- Wave count (1-6 sources)
- Wavelength control (10-150)
- Amplitude (10-200)
- Speed adjustment (0.1-10.0)
- Damping factor (0-2.0)
- Source position control
- Real-time wave simulation

### Generative Patterns

#### Koch Snowflake
**Description**: Classic fractal snowflake

**Features**:
- Iteration depth (0-7)
- Scale and rotation
- All color schemes
- Line width control
- Animation speed
- Center position

#### Phyllotaxis
**Description**: Fibonacci spirals found in nature

**Features**:
- Dot count (10-2000)
- Golden angle (120-150°)
- C value (1.0-10.0)
- Scale (0.1-2.0)
- Size variation
- Color schemes
- Animation

#### Perlin Flow
**Description**: Particles following Perlin noise flow field

**Features**:
- Particle count (10-5000)
- Flow strength
- Noise scale and octaves
- Velocity damping
- Trail length (2-50)
- Background fade
- Animation speed
- Multiple noise parameters

#### Boids (2D)
**Description**: Flocking behavior simulation

**Features**:
- Boid count (10-500)
- Max speed and force
- Separation/Alignment/Cohesion radii and strengths
- Size variation
- Trail fade
- Predator count (0-10)
- Mouse avoidance
- Color schemes

#### De Jong Attractor
**Description**: Strange attractor creating organic patterns

**Features**:
- Four parameters (a, b, c, d)
- Point count (100-100,000)
- Zoom and center control
- Start position
- Color by iteration/position/distance
- Multiple presets
- Animation mode

#### Clifford Attractor
**Description**: Another strange attractor variant

**Features**:
- Similar to De Jong
- Different equation
- Unique organic patterns
- All customization options

### Complex Emergent Simulations

#### Slime Mold
**Description**: Physarum polycephalum behavior simulation

**Features**:
- Agent count (100-10,000)
- Move speed (0.1-5.0)
- Sensor distance/angle
- Turn angle
- Deposit amount
- Decay rate
- Trail brightness
- Network formation

#### Falling Sand
**Description**: Particle-based sand simulation

**Features**:
- Brush size (1-10)
- Gravity control (0.1-2.0)
- Click to add sand
- Pile formation
- Real-time physics

## 🌐 3D Simulations (17 Total)

### Visual Animations

#### DNA Helix
**Description**: Double helix structure

**Features**:
- Radius (2.0-10.0)
- Helix height (20-80)
- Twist rate (1.0-10.0)
- Base pair count (5-50)
- Animation speed
- Points per turn
- Color schemes

#### Torus Knot
**Description**: Mathematical knot on torus surface

**Features**:
- P and Q parameters (1-10 each)
- Major/minor radius
- Tube radius and segments
- Point count (100-1000)
- Animation speed
- Generates trefoil, cinquefoil, etc.

#### Galaxy Spiral
**Description**: Spiral galaxy visualization

**Features**:
- Number of arms (2-8)
- Stars per arm (50-500)
- Arm spread (0.1-1.0)
- Core radius (2-15)
- Max radius (20-80)
- Disk thickness (2-20)
- Rotation speed
- Differential rotation

### Particle Systems

#### 3D Particle Attractor
**Description**: Particles following Lorenz attractor

**Features**:
- Particle count (10-200)
- Spawn rate
- Lifetime (1-30s)
- Trail length (10-200)
- Lorenz parameters (σ, ρ, β)
- Speed control

#### 3D Boids
**Description**: Flocking in 3D space

**Features**:
- Boid count (10-150)
- Max speed/force
- Bound radius
- Separation/Alignment/Cohesion parameters
- 3D flocking behavior
- Speed control

#### N-Body Gravity
**Description**: Gravitational n-body simulation

**Features**:
- Body count (10-200)
- Central mass (10-500)
- Spawn radius
- Initial velocity
- Gravitational constant
- Softening parameter
- Trail length
- Orbital mechanics

#### Fluid SPH
**Description**: Smoothed Particle Hydrodynamics

**Features**:
- Particle count (50-1000)
- Smoothing radius
- Particle mass
- Gravity
- Gas constant (pressure)
- Viscosity
- Damping
- Boundary size

#### Magnetic Field
**Description**: Charged particles in magnetic field

**Features**:
- Magnet count (1-4)
- Magnet strength (10-500)
- Particle count (50-500)
- Particle speed
- Field strength
- Spawn radius
- Trail length
- Damping

### Chaotic Attractors

All attractors share common features:
- Trail length control (100-10,000 points)
- Speed adjustment (0.1-5.0)
- Scale parameter
- Preset configurations
- RK4 numerical integration
- Auto-rotation

#### Lorenz Attractor
**Parameters**: σ (sigma), ρ (rho), β (beta)

**Presets**:
- Classic Butterfly
- Chaotic

#### Rössler Attractor
**Parameters**: a, b, c

**Presets**:
- Classic Rössler
- Chaotic Spiral
- Banded Chaos

#### Aizawa Attractor
**Parameters**: a, b, c, d, e, f (6 parameters)
**Features**: Most complex parameter space

#### Halvorsen Attractor
**Parameters**: a
**Features**: Symmetric structure

#### Dadras Attractor
**Parameters**: a, b, c, d, e (5 parameters)
**Features**: Unique curved structure

#### Thomas Attractor
**Parameters**: b
**Features**: Cyclically symmetric

#### Chen Attractor
**Parameters**: a, b, c
**Features**: Similar to Lorenz but distinct

### Advanced Effects

#### Vortex Turbulence
**Description**: Turbulent vortex flow

**Features**:
- Vortex count (1-4)
- Vortex strength (5-50)
- Particle count (100-1000)
- Flow speed
- Turbulence factor
- Particle lifetime
- Spawn rate
- Trail length

#### Lightning Bolt
**Description**: Procedural lightning generation

**Features**:
- Strike frequency (0.5-5.0)
- Segment length (1-8)
- Chaos factor (0.5-5.0)
- Downward bias (0.3-1.0)
- Branch probability (0-0.5)
- Max branches (5-50)
- Energy decay (0.8-0.99)
- Realistic branching

#### 3D Fractal Tree
**Description**: Recursive 3D tree structure

**Features**:
- Max depth (4-12)
- Branch factor (2-5)
- Length decay (0.5-0.9)
- Branch angle (10-50°)
- Twist angle (0-180°)
- Randomness (0-0.5)
- Growth animation
- Speed control

## Advanced Features

### Fractal Zoom & Navigation
- **Mousewheel Zoom**: Smooth, exponential zooming
- **Drag-to-Pan**: Intuitive navigation
- **Deep Zoom**: Up to 10,000x magnification
- **Precision Controls**: Fine-tune center position
- **Reset Buttons**: Quick return to defaults

### 3D View Controls
- **Rotation Sliders**: Precise angle control (0-2π)
- **Auto-Rotate**: Continuous rotation at adjustable speed
- **Zoom**: 0.5x to 5.0x perspective
- **Mousewheel**: Quick zoom when hovering
- **Point Size**: Adjustable particle/point rendering
- **Reset View**: Return to default angles

### Performance Optimizations
- **Parallel Rendering**: Rayon for all 2D simulations
- **Adaptive Quality**: Auto-scaling based on window size
- **Frame Rate Control**: Target 60 FPS
- **Efficient Algorithms**: Optimized numerical methods
- **Bounds Checking**: Prevent crashes from invalid parameters

### UI/UX Enhancements
- **Collapsible Headers**: Organized parameter groups
- **Emoji Icons**: Visual category identification
- **Scrollable Panel**: Handle long parameter lists
- **Resizable Sidebar**: Adjustable width (400-550px)
- **Tooltips**: Hover information (where applicable)
- **Value Input**: Click sliders to type exact values

## Performance Notes

### 2D Simulations
- All fractals use parallel computation (Rayon)
- Typical render time: <16ms (60 FPS) at 800x600
- High iteration counts may slow rendering
- Reduce resolution for real-time parameter exploration

### 3D Simulations
- Software rendering with depth sorting
- Point limits prevent performance issues
- Auto-rotation runs at 60 FPS
- Trail lengths affect memory usage

### Optimization Tips
1. Always use `cargo run --release` (10x faster)
2. Reduce particle/agent counts for smoother interaction
3. Lower resolution for faster fractal exploration
4. Shorter trails for faster 3D rendering
5. Disable auto-rotate when adjusting parameters

## Technical Details

### Fractal Algorithms
- **Escape-time algorithm** for Mandelbrot/Julia/Burning Ship
- **Smooth coloring** using logarithmic interpolation
- **Parallel iteration** across all pixels
- **Arbitrary precision** could be added for extreme zooms

### Numerical Integration
- **RK4 (Runge-Kutta 4th order)** for all attractors
- **Adaptive timesteps** for stability
- **Fixed-point integration** for cellular automata

### Rendering Techniques
- **Software 3D rendering** with matrix rotations
- **Perspective projection** with depth sorting
- **Alpha blending** for trails and overlays
- **Efficient pixel manipulation** for 2D

### Cellular Automata
- **Moore neighborhood** for Game of Life
- **Life-like rules** (B/S notation)
- **Bitwise operations** for efficiency
- **Wraparound boundaries** (toroidal topology)

## Usage Tips

### Fractals
1. Start at low zoom, increase iterations as you zoom
2. Try all color schemes - each reveals different details
3. Enable smooth coloring for professional look
4. Use color cycling for dynamic wallpapers
5. Screenshot interesting discoveries!

### Cellular Automata
1. Try all rule sets with pattern library
2. Color by age shows temporal structure
3. Pause/step for detailed analysis
4. Different rules create wildly different behaviors

### Attractors
1. Long trails show overall structure
2. Short trails show current motion
3. Adjust speed for exploration vs. visualization
4. Try all presets before custom parameters

### Particle Systems
1. More particles = denser but slower
2. Longer trails = more memory
3. Balance visual appeal vs. performance
4. Enable auto-rotate for full 3D appreciation

## Enjoy Exploring!

With 39 unique simulations and thousands of parameter combinations, there's always something new to discover. The beauty of complex systems lies in how simple rules create infinite variety.
