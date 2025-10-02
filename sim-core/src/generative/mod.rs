// Generative pattern simulations
pub mod koch_snowflake;
pub mod phyllotaxis;
pub mod perlin_flow;
pub mod boids;
pub mod strange_attractors;

pub use koch_snowflake::KochSnowflake;
pub use phyllotaxis::Phyllotaxis;
pub use perlin_flow::PerlinFlow;
pub use boids::Boids;
pub use strange_attractors::{DeJongAttractor, CliffordAttractor};
