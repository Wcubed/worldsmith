use derive_more::{Display, From, Into, Sub};
use std::fmt::{Display, Formatter};

pub trait Unit {
    const SYMBOL: &'static str;
    const NAME: &'static str;
}

/// M☉ = 1.98847 * 10^30 kg
#[derive(Copy, Clone, Display, From, Into)]
pub struct SolarMass(f32);

impl SolarMass {
    pub fn new(mass: f32) -> Self {
        SolarMass(mass)
    }
}

impl Unit for SolarMass {
    const SYMBOL: &'static str = "M☉";
    const NAME: &'static str = "solar mass";
}

#[derive(Copy, Clone, Display, From, Into)]
pub struct SolarRadius(f32);

impl SolarRadius {
    /// In actuality, the radius of larger main sequence stars is also said to be dependent
    /// on age and composition, but this is close enough for now.
    ///
    /// TODO (Wybe 2022-07-03): Make dependent on age as well?
    /// TODO (Wybe 2022-07-03): Make the magic numbers no longer magic.
    pub fn calculate(mass: SolarMass) -> Self {
        let mass = mass.0;
        if mass < 1. {
            Self(mass.powf(0.8))
        } else {
            Self(mass.powf(0.57))
        }
    }
}

impl Unit for SolarRadius {
    const SYMBOL: &'static str = "R☉";
    const NAME: &'static str = "solar radius";
}

#[derive(Copy, Clone, Display, PartialEq, PartialOrd)]
pub struct SolarLuminosity(f32);

impl SolarLuminosity {
    pub fn new(luminosity: f32) -> Self {
        SolarLuminosity(luminosity)
    }

    /// TODO (Wybe 2022-07-03): Make dependent on age as well?
    /// TODO (Wybe 2022-07-03): Make the magic numbers no longer magic.
    pub fn calculate(mass: SolarMass) -> Self {
        let mass = mass.0;

        let luminosity = if mass < 0.43 {
            mass.powf(2.3) * 0.23
        } else if mass < 2. {
            mass.powi(4)
        } else {
            mass.powf(3.5) * 1.4
        };

        SolarLuminosity(luminosity)
    }
}

impl Unit for SolarLuminosity {
    const SYMBOL: &'static str = "L☉";
    const NAME: &'static str = "solar luminosity";
}

#[derive(Copy, Clone, Display)]
pub struct SolarDensity(f32);

impl SolarDensity {
    /// TODO (Wybe 2022-07-03): Make the magic numbers no longer magic.
    pub fn calculate(mass: SolarMass, radius: SolarRadius) -> Self {
        SolarDensity(mass.0 / radius.0.powi(3))
    }
}

impl Unit for SolarDensity {
    const SYMBOL: &'static str = "D☉";
    const NAME: &'static str = "solar density";
}

#[derive(Clone, Debug)]
pub enum SpectralClass {
    O,
    A,
    B,
    F,
    G,
    K,
    M,
}

impl SpectralClass {
    /// Temperature in kelvin
    /// Class boundaries taken from
    /// [The Stellar Classification Wikipedia article](https://en.wikipedia.org/wiki/Stellar_classification)
    pub fn calculate(temperature: Kelvin) -> Self {
        match temperature.0 as u32 {
            // TODO (Wybe 2022-07-03): What to to with objects lower than 2400K? These are not actually M class stars.
            0..=2400 => SpectralClass::M,
            2400..=3700 => SpectralClass::M,
            3701..=5200 => SpectralClass::K,
            5201..=6000 => SpectralClass::G,
            6001..=7500 => SpectralClass::F,
            7501..=10_000 => SpectralClass::A,
            10_001..=30_000 => SpectralClass::B,
            _ => SpectralClass::O,
        }
    }
}

impl Display for SpectralClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, Display, From, Into, Sub)]
pub struct Kelvin(f32);

impl Unit for Kelvin {
    const SYMBOL: &'static str = "K";
    const NAME: &'static str = "kelvin";
}

#[derive(Clone)]
pub struct ColorRgb(u8, u8, u8);

impl ColorRgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        ColorRgb(r, g, b)
    }

    pub fn r(&self) -> u8 {
        self.0
    }

    pub fn g(&self) -> u8 {
        self.1
    }

    pub fn b(&self) -> u8 {
        self.2
    }
}

/// TODO (Wybe 2022-07-03): Make the magic numbers no longer magic.
pub fn calculate_maximum_age_gigayears(mass: SolarMass, luminosity: SolarLuminosity) -> f32 {
    (mass.0 / luminosity.0) * 10.
}

/// TODO (Wybe 2022-07-03): Make the magic numbers no longer magic.
pub fn calculate_stellar_temperature(radius: SolarRadius, luminosity: SolarLuminosity) -> Kelvin {
    Kelvin((luminosity.0 / radius.0.powi(2)).powf(0.25) * 5776.)
}

#[cfg(test)]
mod tests {
    use crate::units::*;

    // TODO (Wybe 2022-07-03): Use macros or something to generate test cases for each set of values
    //      we want to test.
    #[test]
    fn test_calculate_solar_radius() {
        assert_eq!(1., SolarRadius::calculate(SolarMass(1.)).0);
        assert_eq!(1.4845235, SolarRadius::calculate(SolarMass(2.)).0);
        assert_eq!(0.57434916, SolarRadius::calculate(SolarMass(0.5)).0);
    }

    #[test]
    fn test_calculate_solar_luminosity() {
        assert_eq!(1., SolarLuminosity::calculate(SolarMass(1.)).0);
        assert_eq!(15.839191, SolarLuminosity::calculate(SolarMass(2.)).0);
        assert_eq!(0.0625, SolarLuminosity::calculate(SolarMass(0.5)).0);
    }
}
