use super::Unit;

pub struct Mass<U: Unit> {
    pub value: f64,
    pub unit: U,
}

impl<U: Unit> Mass<U> {
    pub fn new(value: f64, unit: U) -> Self {
        Self { value, unit }
    }

    pub fn to_au(&self) -> f64 {
        self.unit.to_au(self.value)
    }

    pub fn to<V: Unit>(&self, unit: V) -> Mass<V> {
        Mass {
            value: self.unit.to_au(self.value) / unit.to_au(1.0),
            unit,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Dalton;
#[allow(dead_code)]
impl Unit for Dalton {
    const TO_AU_MUL: f64 = 1822.88839;
}

/// Enum for mass unit conversion
/// # Examples
/// ```
/// use quantum::units::mass_units::MassUnit;
/// let mass_mn = 1.0;
/// let mass_au = MassUnit::Mn.to_au(mass_mn);
#[deprecated(note = "Use Mass struct instead")]
pub enum MassUnit {
    Au,
    Mn,
}


#[deprecated(note = "Use Mass struct instead")]
#[allow(warnings)]
impl MassUnit {
    #[deprecated(note = "Use Mass struct instead")]
    fn to_au_mul(&self) -> f64 {
        match self {
            MassUnit::Au => 1.0,
            MassUnit::Mn => 1822.88839,
        }
    }

    #[deprecated(note = "Use Mass struct instead")]
    pub fn to_au(&self, mass: f64) -> f64 {
        match self {
            MassUnit::Au => mass,
            _ => mass * self.to_au_mul(),
        }
    }

    #[deprecated(note = "Use Mass struct instead")]
    pub fn to_mn(&self, mass: f64) -> f64 {
        match self {
            MassUnit::Au => mass / MassUnit::Mn.to_au_mul(),
            _ => MassUnit::Au.to_mn(self.to_au(mass)),
        }
    }
}
