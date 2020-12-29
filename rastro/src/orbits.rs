use std::f64::consts::PI;
use crate::constants::{GM_EARTH};

/// Computes the orbital period of an object around Earth.
///
/// Uses rastro::constants::GM_EARTH as the standard gravitational
/// parameter.
///
/// # Arguments
///
/// * `a` - The semi-major axis of the astronomical object. Units: [m]
///
/// # Returns
///
/// * `T` - The orbital period of the astronomical object. Units: [s]
///
/// # Examples
/// ```
/// use rastro::constants::R_EARTH;
/// use rastro::orbits::orbital_period;
/// let T = orbital_period(R_EARTH + 500e3);
/// ```
pub fn orbital_period(a: f64) -> f64 {
    orbital_period_general(a, GM_EARTH)
}

/// Computes the orbital period of an astronomical object around a general body.
///
///
/// # Arguments
///
/// * `a` - The semi-major axis of the astronomical object. Units: [m]
/// * `gm` - The standard gravitational parameter of primary body. Units: [m^3/s^2]
///
/// # Returns
///
/// * `T` - The orbital period of the astronomical object. Units: [s]
///
/// # Examples
/// ```
/// use rastro::constants::{R_EARTH, GM_EARTH, R_MOON, GM_MOON};
/// use rastro::orbits::orbital_period_general;
/// let T_Earth = orbital_period_general(R_EARTH + 500e3, GM_EARTH);
/// let T_Moon  = orbital_period_general(R_MOON + 500e3, GM_MOON);
/// ```
pub fn orbital_period_general(a: f64, gm: f64) -> f64 {
    2.0 * PI * (a.powi(3) / gm).sqrt()
}

/// Computes the mean motion of an astronomical object around Earth.
///
/// # Arguments
///
/// * `a` - The semi-major axis of the astronomical object. Units: [m]
/// * `as_degrees` - Return output in degrees instead of radians
///
/// # Returns
///
/// * `n` - The mean motion of the astronomical object. Units: [rad] or [deg]
///
/// # Examples
/// ```
/// use rastro::constants::{R_EARTH};
/// use rastro::orbits::mean_motion;
/// let n_rad = mean_motion(R_EARTH + 500e3, false);
/// let n_deg = mean_motion(R_EARTH + 500e3, true);
/// ```
pub fn mean_motion(a: f64, as_degrees: bool) -> f64 {
    mean_motion_general(a,  GM_EARTH, as_degrees)
}

/// Computes the mean motion of an astronomical object around a general body
/// given a semi-major axis.
///
/// # Arguments
///
/// * `a` - The semi-major axis of the astronomical object. Units: [m]
/// * `as_degrees` - Return output in degrees instead of radians
/// * `gm` - The standard gravitational parameter of primary body. Units: [m^3/s^2]
///
/// # Returns
///
/// * `n` - The mean motion of the astronomical object. Units: [rad] or [deg]
///
/// # Examples
/// ```
/// use rastro::constants::{R_EARTH, GM_EARTH, R_MOON, GM_MOON};
/// use rastro::orbits::mean_motion_general;
/// let n_earth = mean_motion_general(R_EARTH + 500e3, GM_EARTH, false);
/// let n_moon  = mean_motion_general(R_MOON + 500e3, GM_MOON, true);
/// ```
pub fn mean_motion_general(a: f64,  gm: f64, as_degrees: bool) -> f64 {
    let n = (gm / a.powi(3)).sqrt();

    if as_degrees == true {
        n * 180.0/PI
    } else { 
        n 
    }
}

/// Computes the semi-major axis of an astronomical object from Earth
/// given the object's mean motion.
///
/// # Arguments
///
/// * `n` - The mean motion of the astronomical object. Units: [rad] or [deg]
/// * `as_degrees` - Interpret mean motion as degrees if `true` or radians if `false`
///
/// # Returns
///
/// * `a` - The semi-major axis of the astronomical object. Units: [m]
///
/// # Examples
/// ```
/// use rastro::orbits::semimajor_axis;
/// let a_earth = semimajor_axis(0.0011067836148773837, false);
/// ```
pub fn semimajor_axis(n: f64, as_degrees: bool) -> f64 {
    semimajor_axis_general(n, GM_EARTH, as_degrees)
}

/// Computes the semi-major axis of an astronomical object from a general body
/// given the object's mean motion.
///
/// # Arguments
///
/// * `n` - The mean motion of the astronomical object. Units: [rad] or [deg]
/// * `as_degrees` - Interpret mean motion as degrees if `true` or radians if `false`
/// * `gm` - The standard gravitational parameter of primary body. Units: [m^3/s^2]
///
/// # Returns
///
/// * `a` - The semi-major axis of the astronomical object. Units: [m]
///
/// # Examples
/// ```
/// use rastro::constants::{GM_MOON};
/// use rastro::orbits::semimajor_axis_general;
/// let a_moon = semimajor_axis_general(0.0011067836148773837, GM_MOON, false);
/// ```
pub fn semimajor_axis_general(n: f64, gm: f64, as_degrees: bool) -> f64 {
    let n = if as_degrees == true { n*PI/180.0 } else { n };

    (gm / n.powi(2)).powf(1.0/3.0)
}


//
// Unit Tests!
//


#[cfg(test)]
mod tests {
    use crate::{constants, orbits::*};
    use crate::constants::{R_EARTH, GM_EARTH};

    use approx::{assert_abs_diff_eq, assert_abs_diff_ne};

    #[test]
    fn test_orbital_period() {
        assert_abs_diff_eq!(orbital_period(R_EARTH + 500e3), 5676.977164028288, epsilon=1e-12);
    }

    #[test]
    fn test_orbital_period_general() {
        assert_abs_diff_eq!(orbital_period_general(R_EARTH + 500e3, GM_EARTH), 5676.977164028288, epsilon=1e-12);
    }

    #[test]
    fn test_mean_motion() {
        let n = mean_motion(R_EARTH + 500e3, false);
        assert_abs_diff_eq!(n, 0.0011067836148773837, epsilon=1e-12);

        let n = mean_motion(R_EARTH + 500e3, true);
        assert_abs_diff_eq!(n, 0.0634140299667068, epsilon=1e-12);
    }

    #[test]
    fn test_mean_motion_general() {
        let n = mean_motion_general(R_EARTH + 500e3, GM_EARTH, false);
        assert_abs_diff_eq!(n, 0.0011067836148773837, epsilon=1e-12);

        let n = mean_motion_general(R_EARTH + 500e3, GM_EARTH, true);
        assert_abs_diff_eq!(n, 0.0634140299667068, epsilon=1e-12);

        let n = mean_motion_general(R_EARTH + 500e3, constants::GM_MOON, false);
        assert_abs_diff_ne!(n, 0.0011067836148773837, epsilon=1e-12);

        let n = mean_motion_general(R_EARTH + 500e3, constants::GM_MOON, true);
        assert_abs_diff_ne!(n, 0.0634140299667068, epsilon=1e-12);

        let n = mean_motion_general(constants::R_MOON + 500e3, constants::GM_MOON, false);
        assert_abs_diff_eq!(n, 0.0006613509296264638, epsilon=1e-12);

        let n = mean_motion_general(constants::R_MOON + 500e3, constants::GM_MOON, true);
        assert_abs_diff_eq!(n, 0.0378926170446499, epsilon=1e-12);
    }

    #[test]
    fn test_semimajor_axis() {
        let n = semimajor_axis(0.0011067836148773837, false);
        assert_abs_diff_eq!(n, R_EARTH + 500e3, epsilon=1e-8);

        let n = semimajor_axis(0.0634140299667068, true);
        assert_abs_diff_eq!(n, R_EARTH + 500e3, epsilon=1e-8);
    }

    #[test]
    fn test_semimajor_axis_general() {
        let n = semimajor_axis_general(0.0011067836148773837, GM_EARTH, false);
        assert_abs_diff_eq!(n, R_EARTH + 500e3, epsilon=1e-8);

        let n = semimajor_axis_general(0.0634140299667068, GM_EARTH, true);
        assert_abs_diff_eq!(n, R_EARTH + 500e3, epsilon=1e-8);

        let n = semimajor_axis_general(0.0006613509296264638, constants::GM_MOON, false);
        assert_abs_diff_ne!(n, constants::R_MOON + 500e3, epsilon=1e-12);

        let n = semimajor_axis_general(0.0378926170446499, constants::GM_MOON, true);
        assert_abs_diff_ne!(n, constants::R_MOON + 500e3, epsilon=1e-12);
    }
}