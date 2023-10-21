use crate::internals::{Internals, Scalable};

/// Struct to hold information about a particle. 
/// To create a predefined particle use [`crate::particle_factory`].
#[derive(Default, Clone)]
pub struct Particle {
    name: String,
    pub mass: f64,
    pub internals: Internals<Scalable>
}

impl Particle {
    /// Creates new particle with given name, mass
    pub fn new(name: &str, mass: f64) -> Self {
        Particle {
            name: name.to_string(),
            mass,
            internals: Internals::new()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}