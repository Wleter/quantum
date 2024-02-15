use crate::{
    internals::{Internals, Scalable},
    particle::Particle,
    units::{energy_units::Energy, Unit},
};

/// Struct to hold information about a particle composition.
#[derive(Default, Clone)]
pub struct Particles {
    particles: Vec<Particle>,

    reduced_mass: Scalable,
    pub internals: Internals<Scalable>,
}

impl Particles {
    /// Creates two particle composition with given collision energy inserted inside `internals` as "energy".
    pub fn new_pair<U: Unit>(
        first_particle: Particle,
        second_particle: Particle,
        energy: Energy<U>,
    ) -> Self {
        let inverse_reduced_mass: f64 = 1.0 / first_particle.mass + 1.0 / second_particle.mass;

        let mut internals = Internals::new();
        internals.insert_value("energy", energy.to_au());

        Self {
            particles: vec![first_particle, second_particle],
            reduced_mass: (1.0 / inverse_reduced_mass, 1.0),
            internals,
        }
    }

    /// Mutably borrows the first particle with given name.
    pub fn particle_mut(&mut self, name: &str) -> Option<&mut Particle> {
        self.particles.iter_mut().find(|p| p.name() == name)
    }

    /// Creates a particle composition given a vector of particles.
    pub fn new_custom(particles: Vec<Particle>) -> Self {
        let inverse_reduced_mass = particles
            .iter()
            .fold(0.0, |acc, particle| acc + 1.0 / particle.mass);

        Self {
            particles,
            reduced_mass: (1.0 / inverse_reduced_mass, 1.0),
            internals: Internals::new(),
        }
    }

    /// Sets scale of the reduced mass.
    pub fn scale_red_mass(&mut self, scaling: f64) {
        self.reduced_mass.1 = scaling;
    }

    /// Gets the reduced mass.
    pub fn red_mass(&self) -> f64 {
        self.reduced_mass.0 * self.reduced_mass.1
    }
}
