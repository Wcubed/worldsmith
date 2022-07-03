use std::f32::consts::PI;

/// Call [calculate_parameters] to get the values of a semi realistic star.
pub struct MainSequenceStar {
    //class: MainSequenceStellarClass,
    pub solar_mass: f32,
    //age: f32,
    //max_age: f32,
    pub solar_radius: f32,
    pub solar_luminosity: f32,
    //solar_density: f32,
    //temperature: f32,
}

impl MainSequenceStar {
    /// TODO (Wybe 2022-07-03): Somehow incorporate brown dwarfs, and stars that are no longer in the main sequence.
    pub fn calculate_parameters(solar_mass: f32, age: f32) -> MainSequenceStar {
        let solar_radius = Self::calculate_solar_radius(solar_mass);
        let solar_luminosity = Self::calculate_solar_luminosity(solar_mass);

        MainSequenceStar {
            solar_mass,
            solar_radius,
            solar_luminosity,
        }
    }

    /// In actuality, the radius of larger main sequence stars is also said to be dependent
    /// on age and composition, but this is close enough for now.
    ///
    /// TODO (Wybe 2022-07-03): Make dependent on age as well?
    fn calculate_solar_radius(solar_mass: f32) -> f32 {
        if solar_mass < 1. {
            solar_mass.powf(0.8)
        } else {
            solar_mass.powf(0.57)
        }
    }

    /// TODO (Wybe 2022-07-03): Make dependent on age as well?
    fn calculate_solar_luminosity(solar_mass: f32) -> f32 {
        if solar_mass < 0.43 {
            solar_mass.powf(2.3) * 0.23
        } else if solar_mass < 2. {
            solar_mass.powi(4)
        } else {
            solar_mass.powf(3.5) * 1.4
        }
    }
}

pub struct MainSequenceStellarClass {
    spectral_type: SpectralClass,
    /// Extra subdivision of spectral class.
    /// Ranges from 0 to 9 inclusive, where 0 is hottest, and 0 is the coolest.
    subdivision: f32,
}

enum SpectralClass {
    O,
    A,
    B,
    F,
    G,
    K,
    M,
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
