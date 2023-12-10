use crate::{particle::Particle, units::mass_units::{Mass, Dalton}};

pub fn create_atom(name: &str) -> Option<Particle> {
    let mass = match name {
        "Ne" => Mass::new(20.1797, Dalton),
        "Li6" => Mass::new(6.015122, Dalton),
        "Li7" => Mass::new(7.016004, Dalton),
        "Na23" => Mass::new(22.989770, Dalton),
        "K40" => Mass::new(39.963707, Dalton),
        "Rb85" => Mass::new(84.911789, Dalton),
        "Rb87" => Mass::new(86.90918053, Dalton),
        "Cs133" => Mass::new(132.905447, Dalton),
        _ => return None,
    };

    Some(Particle::new(name, mass.to_au()))
}

pub fn create_molecule(name: &str) -> Option<Particle> {
    let mass = match name {
        "OCS" => Mass::new(60.07, Dalton),
        _ => return None,
    };

    let rot_const = match name {
        "OCS" => 9.243165268327e-7,
        _ => return None,
    };

    let mut particle = Particle::new(name, mass.to_au());
    particle.internals.insert_value("rot_const", rot_const);

    Some(particle)
}
