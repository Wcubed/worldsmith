pub mod units;

use crate::units::*;
use std::fmt::{Display, Formatter};

/// Call [calculate_parameters] to get the values of a semi realistic star.
/// TODO (Wybe 2022-07-03): Change the f32 into new unit types, so that unit conversions are always correct.
pub struct MainSequenceStar {
    pub class: MainSequenceStellarClass,
    pub mass: SolarMass,
    // age_gigayears: f32,
    pub max_age_gigayears: f32,
    pub radius: SolarRadius,
    pub luminosity: SolarLuminosity,
    pub density: SolarDensity,
    pub temperature: Kelvin,
}

impl MainSequenceStar {
    /// TODO (Wybe 2022-07-03): Somehow incorporate brown dwarfs, and stars that are no longer in the main sequence.
    /// A Gyr (gigayear) is 1_000_000_000 years
    pub fn calculate_parameters(mass: SolarMass, age_gigayears: f32) -> MainSequenceStar {
        let radius = SolarRadius::calculate(mass);
        let luminosity = SolarLuminosity::calculate(mass);
        let density = SolarDensity::calculate(mass, radius);
        let maximum_age = calculate_maximum_age_gigayears(mass, luminosity);
        let temperature = calculate_stellar_temperature(radius, luminosity);

        let class = MainSequenceStellarClass::calculate(temperature);

        MainSequenceStar {
            class,
            mass,
            radius,
            luminosity,
            density,
            max_age_gigayears: maximum_age,
            temperature,
        }
    }
}

pub struct MainSequenceStellarClass {
    pub spectral_class: SpectralClass,
    /// Extra subdivision of spectral class.
    /// Ranges from 0 to 9 inclusive, where 0 is hottest, and 0 is the coolest.
    pub subdivision: f32,
}

impl MainSequenceStellarClass {
    /// Temperature in kelvin.
    pub fn calculate(temperature: Kelvin) -> Self {
        let spectral_class = SpectralClass::calculate(temperature);
        let kelvin: f32 = temperature.into();

        // TODO (Wybe 2022-07-03): Remove these magic numbers. this is actually just range mapping onto the temperature range of that class.
        let subdivision = 10.
            * match spectral_class {
                SpectralClass::M => 1. - ((kelvin - 2000.) / 1700.),
                SpectralClass::K => 1. - ((kelvin - 3700.) / 1500.),
                SpectralClass::G => 1. - ((kelvin - 5200.) / 800.),
                SpectralClass::F => 1. - ((kelvin - 6000.) / 1500.),
                SpectralClass::A => 1. - ((kelvin - 7500.) / 2500.),
                SpectralClass::B => 1. - ((kelvin - 10_000.) / 20_000.),
                SpectralClass::O => 1. - ((kelvin - 30_000.) / 62_000.),
            };

        MainSequenceStellarClass {
            spectral_class,
            // TODO (Wybe 2022-07-03): Calculate subdivision
            subdivision,
        }
    }
}

impl Display for MainSequenceStellarClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:.1}", self.spectral_class, self.subdivision)
    }
}
