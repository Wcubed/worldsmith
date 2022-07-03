use std::fmt::{write, Display, Formatter};

/// Call [calculate_parameters] to get the values of a semi realistic star.
/// TODO (Wybe 2022-07-03): Change the f32 into new unit types, so that unit conversions are always correct.
pub struct MainSequenceStar {
    pub class: MainSequenceStellarClass,
    pub solar_mass: f32,
    // age_gigayears: f32,
    pub max_age_gigayears: f32,
    pub solar_radius: f32,
    pub solar_luminosity: f32,
    pub solar_density: f32,
    pub temperature: f32,
}

impl MainSequenceStar {
    /// TODO (Wybe 2022-07-03): Somehow incorporate brown dwarfs, and stars that are no longer in the main sequence.
    /// A Gyr (gigayear) is 1_000_000_000 years
    pub fn calculate_parameters(solar_mass: f32, age_gigayears: f32) -> MainSequenceStar {
        let solar_radius = Self::calculate_solar_radius(solar_mass);
        let solar_luminosity = Self::calculate_solar_luminosity(solar_mass);
        let solar_density = Self::calculate_solar_density(solar_mass, solar_radius);
        let maximum_age = Self::calculate_maximum_age_gigayears(solar_mass, solar_luminosity);
        let temperature = Self::calculate_temperature_kelvin(solar_radius, solar_luminosity);

        let class = MainSequenceStellarClass::caluclate(temperature);

        MainSequenceStar {
            class,
            solar_mass,
            solar_radius,
            solar_luminosity,
            solar_density,
            max_age_gigayears: maximum_age,
            temperature,
        }
    }

    /// In actuality, the radius of larger main sequence stars is also said to be dependent
    /// on age and composition, but this is close enough for now.
    ///
    /// TODO (Wybe 2022-07-03): Make dependent on age as well?
    /// TODO (Wybe 2022-07-03): Make the magic numbers no longer magic.
    fn calculate_solar_radius(solar_mass: f32) -> f32 {
        if solar_mass < 1. {
            solar_mass.powf(0.8)
        } else {
            solar_mass.powf(0.57)
        }
    }

    /// TODO (Wybe 2022-07-03): Make dependent on age as well?
    /// TODO (Wybe 2022-07-03): Make the magic numbers no longer magic.
    fn calculate_solar_luminosity(solar_mass: f32) -> f32 {
        if solar_mass < 0.43 {
            solar_mass.powf(2.3) * 0.23
        } else if solar_mass < 2. {
            solar_mass.powi(4)
        } else {
            solar_mass.powf(3.5) * 1.4
        }
    }

    /// TODO (Wybe 2022-07-03): Make the magic numbers no longer magic.
    fn calculate_solar_density(solar_mass: f32, solar_radius: f32) -> f32 {
        solar_mass / solar_radius.powi(3)
    }

    /// TODO (Wybe 2022-07-03): Make the magic numbers no longer magic.
    fn calculate_maximum_age_gigayears(solar_mass: f32, solar_luminosity: f32) -> f32 {
        (solar_mass / solar_luminosity) * 10.
    }

    /// TODO (Wybe 2022-07-03): Make the magic numbers no longer magic.
    fn calculate_temperature_kelvin(solar_radius: f32, solar_luminosity: f32) -> f32 {
        (solar_luminosity / solar_radius.powi(2)).powf(0.25) * 5776.
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
    pub fn caluclate(temperature: f32) -> Self {
        let spectral_class = SpectralClass::calculate(temperature);

        // TODO (Wybe 2022-07-03): Remove these magic numbers. this is actually just range mapping onto the temperature range of that class.
        let subdivision = 10.
            * match spectral_class {
                SpectralClass::M => 1. - ((temperature - 2000.) / 1700.),
                SpectralClass::K => 1. - ((temperature - 3700.) / 1500.),
                SpectralClass::G => 1. - ((temperature - 5200.) / 800.),
                SpectralClass::F => 1. - ((temperature - 6000.) / 1500.),
                SpectralClass::A => 1. - ((temperature - 7500.) / 2500.),
                SpectralClass::B => 1. - ((temperature - 10_000.) / 20_000.),
                SpectralClass::O => 1. - ((temperature - 30_000.) / 62_000.),
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

#[derive(Debug)]
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
    pub fn calculate(temperature: f32) -> Self {
        match temperature as u32 {
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

#[cfg(test)]
mod tests {
    use crate::MainSequenceStar;

    // TODO (Wybe 2022-07-03): Use macros or something to generate test cases for each set of values
    //      we want to test.
    #[test]
    fn test_calculate_solar_radius() {
        assert_eq!(1., MainSequenceStar::calculate_solar_radius(1.));
        assert_eq!(1.4845235, MainSequenceStar::calculate_solar_radius(2.));
        assert_eq!(0.57434916, MainSequenceStar::calculate_solar_radius(0.5));
    }

    #[test]
    fn test_calculate_solar_luminosity() {
        assert_eq!(1., MainSequenceStar::calculate_solar_luminosity(1.));
        assert_eq!(15.839191, MainSequenceStar::calculate_solar_luminosity(2.));
        assert_eq!(0.0625, MainSequenceStar::calculate_solar_luminosity(0.5));
    }
}
