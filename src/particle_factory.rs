use crate::{particle::Particle, units::mass_units::MassUnit};

pub fn create_atom(name: &str) -> Option<Particle> {
    let mass = match name {
        "Ne" => MassUnit::Mn.to_au(20.1797),
        "Li6" => MassUnit::Mn.to_au(6.015122),
        "Li7" => MassUnit::Mn.to_au(7.016004),
        "Na23" => MassUnit::Mn.to_au(22.989770),
        "K40" => MassUnit::Mn.to_au(39.963707),
        "Rb85" => MassUnit::Mn.to_au(84.911789),
        "Rb87" => MassUnit::Mn.to_au(86.90918053),
        "Cs133" => MassUnit::Mn.to_au(132.905447),
        _ => return None,
    };

    Some(Particle::new(name, mass))
}

pub fn create_molecule(name: &str) -> Option<Particle> {
    let mass = match name {
        "OCS" => MassUnit::Mn.to_au(60.07),
        _ => return None,
    };

    let rot_const = match name {
        "OCS" => 9.243165268327e-7,
        _ => return None,
    };

    let mut particle = Particle::new(name, mass);
    particle.internals.insert_value("rot_const", rot_const);

    Some(particle)
}
