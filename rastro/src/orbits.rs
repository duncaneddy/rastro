use std::{f64::consts::PI};
use nalgebra::{Vector3, Vector6};
use crate::constants::{GM_EARTH, R_EARTH, J2_EARTH};

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
/// * `period` - The orbital period of the astronomical object. Units: [s]
///
/// # Examples
/// ```
/// use rastro::constants::R_EARTH;
/// use rastro::orbits::orbital_period;
/// let period = orbital_period(R_EARTH + 500e3);
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
/// let period_earth = orbital_period_general(R_EARTH + 500e3, GM_EARTH);
/// let period_moon  = orbital_period_general(R_MOON + 500e3, GM_MOON);
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


/// Computes the perigee velocity of an astronomical object around Earth.
///
/// # Arguments
///
/// * `a` - The semi-major axis of the astronomical object. Units: [m]
/// * `e` - The eccentricity of the astronomical object's orbit. Dimensionless
///
/// # Returns
///
/// * `v` - The magnitude of velocity of the object at perigee. Units: [m/s]
///
/// # Examples
/// ```
/// use rastro::constants::{R_EARTH};
/// use rastro::orbits::perigee_velocity;
/// let vp = perigee_velocity(R_EARTH + 500e3, 0.001);
/// ```
pub fn perigee_velocity(a: f64, e: f64) -> f64 {
    perigee_velocity_general(a, e, GM_EARTH)
}

/// Computes the perigee velocity of an astronomical object around a general body.
///
/// # Arguments
///
/// * `a` - The semi-major axis of the astronomical object. Units: [m]
/// * `e` - The eccentricity of the astronomical object's orbit. Dimensionless
/// * `gm` - The standard gravitational parameter of primary body. Units: [m^3/s^2]
///
/// # Returns
///
/// * `v` - The magnitude of velocity of the object at perigee. Units: [m/s]
///
/// # Examples
/// ```
/// use rastro::constants::{R_EARTH, GM_EARTH};
/// use rastro::orbits::perigee_velocity_general;
/// let vp = perigee_velocity_general(R_EARTH + 500e3, 0.001, GM_EARTH);
/// ```
pub fn perigee_velocity_general(a: f64, e: f64, gm: f64) -> f64 {
    // math.sqrt(gm/a)*math.sqrt((1+e)/(1-e))
    (gm / a).sqrt() * ((1.0 + e) / (1.0 - e)).sqrt()
}

/// Computes the apogee velocity of an astronomical object around Earth.
///
/// # Arguments
///
/// * `a` - The semi-major axis of the astronomical object. Units: [m]
/// * `e` - The eccentricity of the astronomical object's orbit. Dimensionless
///
/// # Returns
///
/// * `v` - The magnitude of velocity of the object at apogee. Units: [m/s]
///
/// # Examples
/// ```
/// use rastro::constants::{R_EARTH};
/// use rastro::orbits::apogee_velocity;
/// let va = apogee_velocity(R_EARTH + 500e3, 0.001);
/// ```
pub fn apogee_velocity(a: f64, e: f64) -> f64 {
    apogee_velocity_general(a, e, GM_EARTH)
}

/// Computes the apogee velocity of an astronomical object around a general body.
///
/// # Arguments
///
/// * `a` - The semi-major axis of the astronomical object. Units: [m]
/// * `e` - The eccentricity of the astronomical object's orbit. Dimensionless
/// * `gm` - The standard gravitational parameter of primary body. Units: [m^3/s^2]
///
/// # Returns
///
/// * `v` - The magnitude of velocity of the object at apogee. Units: [m/s]
///
/// # Examples
/// ```
/// use rastro::constants::{R_EARTH, GM_EARTH};
/// use rastro::orbits::apogee_velocity_general;
/// let va = apogee_velocity_general(R_EARTH + 500e3, 0.001, GM_EARTH);
/// ```
pub fn apogee_velocity_general(a: f64, e: f64, gm: f64) -> f64 {
    (gm / a).sqrt() * ((1.0 - e) / (1.0 + e)).sqrt()
}


/// Computes the inclination for a Sun-synchronous orbit around Earth based on
/// the J2 gravitational perturbation.
///
/// # Arguments
///
/// * `a` - The semi-major axis of the astronomical object. Units: [m]
/// * `e` - The eccentricity of the astronomical object's orbit. Dimensionless
/// * `as_degrees` - Return output in degrees instead of radians
///
/// # Returns
///
/// * `inc` - Inclination for a Sun synchronous orbit. Units: [deg] or [rad]
///
/// # Examples
/// ```
/// use rastro::constants::{R_EARTH, GM_EARTH};
/// use rastro::orbits::sun_synchronous_inclination;
/// let inc = sun_synchronous_inclination(R_EARTH + 500e3, 0.001, true); // approx 97.5 deg
/// ```
pub fn sun_synchronous_inclination(a: f64, e: f64, as_degrees: bool) -> f64 {
    // The required RAAN precession for a sun-synchronous orbit
    let omega_dot_ss = 2.0 * PI / 365.2421897 / 86400.0;

    // Compute inclination required for the desired RAAN precession
    let i = (-2.0 * a.powf(3.5) * omega_dot_ss * (1.0-e.powi(2)).powi(2) / (3.0*(R_EARTH.powi(2)) * J2_EARTH * GM_EARTH.sqrt())).acos();

    if as_degrees == true {
        i * 180.0 / PI
    } else {
        i
    }
}

/// Converts an eccentric anomaly into an mean anomaly.
///
/// # Arguments
///
/// * `E` - Eccentric anomaly. Units: [m]
/// * `e` - The eccentricity of the astronomical object's orbit. Dimensionless
/// * `as_degrees` - Interprets input and returns output in degrees if `true` or radians if `false`
///
/// # Returns
///
/// * `M` - Mean anomaly. Units: [deg] or [rad]
///
/// # Examples
/// ```
/// use rastro::orbits::anomaly_mean_to_eccentric;
/// let e = anomaly_mean_to_eccentric(90.0, 0.001, true);
/// ```
#[allow(non_snake_case)]
pub fn anomaly_eccentric_to_mean(E: f64, e: f64, as_degrees: bool) -> f64 {
    // Ensure E is in radians regardless of input
    let E = if as_degrees == true { E * PI / 180.0 } else { E };

    // Convert to mean anomaly
    let M = E - e * E.sin();

    // Convert output to desired angular format
    if as_degrees == true {
        M * 180.0 / PI
    } else {
        M
    }
}

/// Converts a mean anomaly into an eccentric anomaly
///
/// # Arguments
///
/// * `M` - Mean anomaly. Units: [m]
/// * `e` - The eccentricity of the astronomical object's orbit. Dimensionless
/// * `as_degrees` - Interprets input and returns output in degrees if `true` or radians if `false`
///
/// # Returns
///
/// * `E` - Eccentric anomaly. Units: [deg] or [rad]
///
/// # Examples
/// ```
/// use rastro::orbits::anomaly_mean_to_eccentric;
/// let e = anomaly_mean_to_eccentric(90.0, 0.001, true);
/// ```
#[allow(non_snake_case)]
pub fn anomaly_mean_to_eccentric(M: f64, e: f64, as_degrees: bool) -> Result<f64, String> {
    // Ensure M is in radians regardless of input
    let M = if as_degrees == true { M * PI / 180.0 } else { M };

    // Set constants of iteration
    let MAX_ITER = 5;
    let EPS = 100.0 * f64::EPSILON;

    // Initialize starting iteration values
    let M = M % (2.0 * PI);
    let mut E = if e < 0.8 { M } else { PI };

    let mut f = E - e * E.sin() - M;
    let mut i = 0;

    // Iterate until convergence
    while f.abs() > EPS {
        f = E - e * E.sin() - M;
        E = E - f / (1.0 - e * E.cos());

        i += 1;
        if i > MAX_ITER {
            return Err(format!("Reached maximum number of iterations ({}) before convergence.", MAX_ITER));
        }
    }

    // Convert output to desired angular format
    if as_degrees == true {
        Ok(E * 180.0 / PI)
    } else {
        Ok(E)
    }
}

///
///
///
pub fn state_osculating_to_cartesian(oe: Vector6<f64>, as_degrees: bool) -> Vector6<f64> {
    state_osculating_to_cartesian_general(oe, GM_EARTH, as_degrees)
}

pub fn state_osculating_to_cartesian_general(oe: Vector6<f64>, gm: f64, as_degrees: bool) -> Vector6<f64> {
    Vector6::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
}

//
// Unit Tests!
//


#[cfg(test)]
mod tests {
    use crate::{constants, orbits::*};
    use crate::constants::{R_EARTH, GM_EARTH, R_MOON};

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

    #[test]
    fn test_perigee_velocity() {
        let vp = perigee_velocity(R_EARTH + 500e3, 0.001);
        assert_abs_diff_eq!(vp, 7620.224976404526, epsilon=1e-12);
    }

    #[test]
    fn test_perigee_velocity_general() {
        let vp = perigee_velocity_general(R_MOON + 500e3, 0.001, constants::GM_MOON);
        assert_abs_diff_eq!(vp, 1481.5842246768275, epsilon=1e-12);
    }

    #[test]
    fn test_apogee_velocity() {
        let va = apogee_velocity(R_EARTH + 500e3, 0.001);
        assert_abs_diff_eq!(va, 7604.999751676446, epsilon=1e-12);
    }

    #[test]
    fn test_apogee_velocity_general() {
        let va = apogee_velocity_general(R_MOON + 500e3, 0.001, constants::GM_MOON);
        assert_abs_diff_eq!(va, 1478.624016435715, epsilon=1e-12);
    }

    #[test]
    fn test_sun_synchronous_inclination() {
        let inc = sun_synchronous_inclination(R_EARTH + 500e3, 0.001, true);
        assert_abs_diff_eq!(inc, 97.40172901366881, epsilon=1e-12);
    }

    #[test]
    fn test_anm_ecc_to_mean() {
        // 0 degrees
        let m = anomaly_eccentric_to_mean(0.0, 0.0, false);
        assert_eq!(m, 0.0);

        let m = anomaly_eccentric_to_mean(0.0, 0.0, true);
        assert_eq!(m, 0.0);

        // 180 degrees
        let m = anomaly_eccentric_to_mean(PI, 0.0, false);
        assert_eq!(m, PI);

        let m = anomaly_eccentric_to_mean(180.0, 0.0, true);
        assert_eq!(m, 180.0);

        // 90 degrees 
        let m = anomaly_eccentric_to_mean(PI / 2.0, 0.1, false);
        assert_abs_diff_eq!(m, 1.4707963267948965, epsilon=1e-12);

        let m = anomaly_eccentric_to_mean(90.0, 0.1, true);
        assert_abs_diff_eq!(m, 84.27042204869177, epsilon=1e-12);
    }

    #[test]
    fn test_anm_mean_to_ecc() {
        // 0 degrees
        let e = anomaly_mean_to_eccentric(0.0, 0.0, false).unwrap();
        assert_eq!(e, 0.0);

        let e = anomaly_mean_to_eccentric(0.0, 0.0, true).unwrap();
        assert_eq!(e, 0.0);

        // 180 degrees
        let e = anomaly_mean_to_eccentric(PI, 0.0, false).unwrap();
        assert_eq!(e, PI);

        let e = anomaly_mean_to_eccentric(180.0, 0.0, true).unwrap();
        assert_eq!(e, 180.0);

        // 90 degrees 
        let e = anomaly_mean_to_eccentric(1.4707963267948965, 0.1, false).unwrap();
        assert_abs_diff_eq!(e, PI/2.0, epsilon=1e-12);

        let e = anomaly_mean_to_eccentric(84.27042204869177, 0.1, true).unwrap();
        assert_abs_diff_eq!(e, 90.0, epsilon=1e-12);
    }
}