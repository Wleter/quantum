/// Enum for mass unit conversion
/// # Examples
/// ```
/// use split_operator::units::MassUnit;
/// let mass_mn = 1.0;
/// let mass_au = MassUnit::Mn.to_au(mass_mn);
pub enum MassUnit {
    Au,
    Mn,
}

impl MassUnit {
    fn to_au_mul(&self) -> f64 {
        match self {
            MassUnit::Au => 1.0,
            MassUnit::Mn => 1822.88839,
        }
    }

    pub fn to_au(&self, mass: f64) -> f64 {
        match self {
            MassUnit::Au => mass,
            _ => mass * self.to_au_mul(),
        }
    }

    pub fn to_mn(&self, mass: f64) -> f64 {
        match self {
            MassUnit::Au => mass / MassUnit::Mn.to_au_mul(),
            _ => MassUnit::Au.to_mn(self.to_au(mass)),
        }
    }
}
