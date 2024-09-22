use std::ops::{Deref, DerefMut};

use crate::{
    params::Params,
    units::{mass_units::Mass, Au, Unit},
};

/// Struct to hold information about a particle.
/// To create a predefined particle use [`crate::particle_factory`].
#[derive(Default)]
pub struct Particle {
    name: String,
    pub params: Params,
}

impl Particle {
    /// Creates new particle with given name and mass
    pub fn new<U: Unit>(name: &str, mass: Mass<U>) -> Self {
        let mut params = Params::default();
        params.insert(mass.to(Au));
        
        Particle {
            name: name.to_string(),
            params,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Deref for Particle {
    type Target = Params;

    fn deref(&self) -> &Self::Target {
        &self.params
    }
}

impl DerefMut for Particle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.params
    }
}