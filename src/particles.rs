use std::collections::HashMap;

use crate::{particle::Particle, internals::{Internals, Scalable}};

/// Struct to hold information about a particle composition.
#[derive(Default, Clone)]
pub struct Particles {
    particles: HashMap<String, Particle>,
    reduced_mass: Scalable,

    pub internals: Internals<Scalable>
}

impl Particles {
    /// Creates two particle composition with given collision energy.
    pub fn new_pair(
        first_particle: Particle,
        second_particle: Particle,
        energy: f64,
    ) -> Self {
        let inverse_reduced_mass: f64 = 1.0 / first_particle.mass + 1.0 / second_particle.mass;

        let mut particles_map = HashMap::<String, Particle>::new();
        particles_map.insert(first_particle.name().to_string(), first_particle);
        particles_map.insert(second_particle.name().to_string(), second_particle);

        let mut internals = Internals::new();
        internals.insert_value("collision_energy", energy);

        Self {
            particles: particles_map,
            reduced_mass: (1.0 / inverse_reduced_mass, 1.0),
            internals,
        }
    }

    /// Creates a particle composition given a vector of particles.
    pub fn new_custom(particles: Vec<Particle>) -> Self {

        let inverse_reduced_mass = particles.iter().fold(0.0, |acc, particle| acc + 1.0 / particle.mass);

        let mut particles_map = HashMap::<String, Particle>::new();
        for particle in particles {
            particles_map.insert(particle.name().to_string(), particle);
        }

        Self {
            particles: particles_map,
            reduced_mass: (1.0 / inverse_reduced_mass, 1.0),
            internals: Internals::new()
        }
    }

    /// Gets the mutable reference to particle with given name.
    pub fn particle_mut(&mut self, name: &str) -> Option<&mut Particle> {
        self.particles.get_mut(name)
    }

    /// Sets scale of the reduced mass by given scale.
    pub fn set_scale_red_mass(&mut self, scale: f64) {
        self.reduced_mass.1 = scale;
    }


    /// Scale the reduced mass by given scaling factor. Can be used multiple times.
    pub fn scale_red_mass(&mut self, scaling: f64) {
        self.reduced_mass.1 *= scaling;
    }

    /// Gets the reduced mass.
    pub fn red_mass(&self) -> f64 {
        self.reduced_mass.0 * self.reduced_mass.1
    }
}