pub mod params;
pub mod particle;
pub mod particle_factory;
pub mod particles;
pub mod problem_selector;
pub mod units;
pub mod utility;

#[cfg(test)]
mod tests {
    use units::{mass_units::{Dalton, Mass}, Au};
    use super::*;

    struct Parameter(u32);

    #[test]
    fn particle() {
        let particle = particle_factory::create_atom("Ne");
        assert!(particle.is_some());
        let mut particle = particle.unwrap();

        let mass = particle.get::<Mass<Au>>().copied();
        assert!(mass.is_some());
        assert_eq!(mass.unwrap().to_au(), Mass(20.1797, Dalton).to_au());

        particle.insert(Parameter(32));
        let parameter = particle.get::<Parameter>();
        assert!(parameter.is_some());
        assert_eq!(parameter.unwrap().0, 32);


        let particle = particle_factory::create_atom("Non existing atom");
        assert!(particle.is_none());
    }
}
