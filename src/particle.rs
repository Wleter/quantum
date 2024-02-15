use crate::{internals::{Internals, Scalable}, units::{mass_units::Mass, Unit}};

/// Struct to hold information about a particle.
/// To create a predefined particle use [`crate::particle_factory`].
#[derive(Default, Clone)]
pub struct Particle {
    name: String,
    pub mass: f64,
    pub internals: Internals<Scalable>,
}

impl Particle {
    /// Creates new particle with given name and mass
    pub fn new<U: Unit>(name: &str, mass: Mass<U>) -> Self {
        Particle {
            name: name.to_string(),
            mass: mass.to_au(),
            internals: Internals::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
