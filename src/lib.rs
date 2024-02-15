pub mod internals;
pub mod particle;
pub mod particle_factory;
pub mod particles;
pub mod problem_selector;
pub mod units;
pub mod saving;
pub mod utility;

#[cfg(test)]
mod tests {
    use crate::units::energy_units::{Kelvin, Energy};

    use super::*;
    #[test]
    fn particle_creation() {
        let particle = particle_factory::create_atom("Ne");
        assert!(particle.is_some());

        let mut particle = particle.unwrap();
        let value = 234111.234;

        particle.internals.insert_value("test value", value);
        assert_eq!(particle.internals.get_value("test value"), value);
        assert_eq!(*particle.internals.get_param("test value"), (value, 1.0));

        particle.internals.set_scaling("test value", 2.0);
        assert_eq!(particle.internals.get_value("test value"), value * 2.0);

        let particle = particle_factory::create_atom("Non existing atom");
        assert!(particle.is_none());
    }

    #[test]
    fn particle_composition() {
        let particle1 = particle_factory::create_atom("Ne").unwrap();
        let particle2 = particle_factory::create_atom("Li6").unwrap();
        let energy = Energy(100.0, Kelvin);

        let mut composition = particles::Particles::new_pair(particle1, particle2, energy);
        assert_eq!(composition.particle_mut("Ne").name(), "Ne");

        let value = 234111.234;
        composition
            .particle_mut("Ne")
            .internals
            .insert_value("test value", value);

        assert_eq!(
            composition
                .particle_mut("Ne")
                .internals
                .get_value("test value"),
            value
        );

        assert_eq!(composition.internals.get_value("energy"), energy.to_au());
        assert_eq!(*composition.internals.get_param("energy"), (energy.to_au(), 1.0));
        composition.internals.set_scaling("energy", 2.0);
        assert_eq!(composition.internals.get_value("energy"), energy.to_au() * 2.0);

        composition.internals.insert_value("test value", value);
        assert_eq!(composition.internals.get_value("test value"), value);
    }
}
