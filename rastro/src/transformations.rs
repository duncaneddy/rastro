use is_close::is_close;
use nalgebra as na;
use nalgebra::{Vector3, Vector6};
use std::f64::consts::PI;

use crate::constants;
use crate::constants::GM_EARTH;
use crate::frames;
use crate::orbits;
use crate::time::Epoch;
use crate::utils::*;

/////////////////////////////////////
// Orbital Element Transformations //
/////////////////////////////////////

/// Convert an osculating orbital element state vector into the equivalent
/// Cartesian (position and velocity) inertial state.
///
/// The osculating elements are (in order):
/// 1. _a_, Semi-major axis Units: (*m*)
/// 2. _e_, Eccentricity. Units: (*dimensionless*)
/// 3. _i_, Inclination. Units: (*rad* or *deg*)
/// 4. _Ω_, Right Ascension of the Ascending Node (RAAN). Units: (*rad*)
/// 5. _ω_, Argument of Perigee. Units: (*rad* or *deg*)
/// 6. _M_, Mean anomaly. Units: (*rad* or *deg*)
///
/// # Arguments
/// - `x_oe`: Osculating orbital elements
/// - `use_degrees`: Interprets input as (deg) if `true` or (rad) if `false`
///
/// # Returns
/// - `x_cart`: Cartesian inertial state. Units: (_m_; _m/s_)
///
/// # Examples
/// ```rust
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector6_from_array;
/// use rastro::transformations::*;
///
/// let osc = vector6_from_array([R_EARTH + 500e3, 0.0, 0.0, 0.0, 0.0, 0.0]);
/// let cart = state_osculating_to_cartesian(osc, false);
/// // Returns state [R_EARTH + 500e3, 0, 0, perigee_velocity(R_EARTH + 500e3, 0.0), 0]
/// ```
///
/// # Reference
/// 1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and Applications*, pp. 24, eq. 2.43 & 2.44, 2012.
#[allow(non_snake_case)]
pub fn state_osculating_to_cartesian(x_oe: na::Vector6<f64>, as_degrees: bool) -> na::Vector6<f64> {
    // Unpack input
    let a = x_oe[0];
    let e = x_oe[1];
    let i = from_degrees(x_oe[2], as_degrees);
    let RAAN = from_degrees(x_oe[3], as_degrees);
    let omega = from_degrees(x_oe[4], as_degrees);
    let M = from_degrees(x_oe[5], as_degrees);

    let E = orbits::anomaly_mean_to_eccentric(M, e, false).unwrap();

    let P: Vector3<f64> = Vector3::new(
        omega.cos() * RAAN.cos() - omega.sin() * i.cos() * RAAN.sin(),
        omega.cos() * RAAN.sin() + omega.sin() * i.cos() * RAAN.cos(),
        omega.sin() * i.sin(),
    );

    let Q: Vector3<f64> = Vector3::new(
        -omega.sin() * RAAN.cos() - omega.cos() * i.cos() * RAAN.sin(),
        -omega.sin() * RAAN.sin() + omega.cos() * i.cos() * RAAN.cos(),
        omega.cos() * i.sin(),
    );

    let p = a * (E.cos() - e) * P + a * (1.0 - e * e).sqrt() * E.sin() * Q;
    let v = (constants::GM_EARTH * a).sqrt() / p.norm()
        * (-E.sin() * P + (1.0 - e * e).sqrt() * E.cos() * Q);
    Vector6::new(p[0], p[1], p[2], v[0], v[1], v[2])
}

/// Convert a Cartesian (position and velocity) inertial state into the equivalent
/// osculating orbital element state vector.
///
/// The osculating elements are (in order):
/// 1. _a_, Semi-major axis Units: (*m*)
/// 2. _e_, Eccentricity. Units: (*dimensionless*)
/// 3. _i_, Inclination. Units: (*rad* or *deg*)
/// 4. _Ω_, Right Ascension of the Ascending Node (RAAN). Units: (*rad*)
/// 5. _ω_, Argument of Perigee. Units: (*rad* or *deg*)
/// 6. _M_, Mean anomaly. Units: (*rad* or *deg*)
///
/// # Arguments
/// - `x_cart`: Cartesian inertial state. Units: (_m_; _m/s_)
/// - `use_degrees`: Returns output as (deg) if `true` or (rad) if `false`
///
/// # Returns
/// - `x_oe`: Osculating orbital elements
///
/// # Examples
/// ```rust
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector6_from_array;
/// use rastro::orbits::perigee_velocity;
/// use rastro::transformations::*;
///
/// let cart = vector6_from_array([R_EARTH + 500e3, 0.0, 0.0, 0.0, perigee_velocity(R_EARTH + 500e3, 0.0), 0.0, ]);
/// let osc = state_cartesian_to_osculating(cart, true);
/// // Returns state [R_EARTH + 500e3, 0, 0, 0, 0, 0]
/// ```
///
/// # Reference
/// 1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and Applications*, pp. 28-29, eq. 2.56-2.68, 2012.
#[allow(non_snake_case)]
pub fn state_cartesian_to_osculating(
    x_cart: na::Vector6<f64>,
    as_degrees: bool,
) -> na::Vector6<f64> {
    // # Initialize Cartesian Polistion and Velocity
    let r: Vector3<f64> = Vector3::from(x_cart.fixed_rows::<3>(0));
    let v: Vector3<f64> = Vector3::from(x_cart.fixed_rows::<3>(3));

    let h: Vector3<f64> = Vector3::from(r.cross(&v)); // Angular momentum vector
    let W: Vector3<f64> = h / h.norm();

    let i = ((W[0] * W[0] + W[1] * W[1]).sqrt()).atan2(W[2]); // Compute inclination
    let RAAN = (W[0]).atan2(-W[1]); // Right ascension of ascending node
    let p = h.norm() * h.norm() / GM_EARTH; // Semi-latus rectum
    let a = 1.0 / (2.0 / r.norm() - v.norm() * v.norm() / GM_EARTH); // Semi-major axis
    let n = GM_EARTH / a.powi(3); // Mean motion

    // Numerical stability hack for circular and near-circular orbits
    // to ensures that (1-p/a) is always positive
    let p = if is_close!(a, p, abs_tol = 1e-9, rel_tol = 1e-8) {
        a
    } else {
        p
    };

    let e = (1.0 - p / a).sqrt(); // Eccentricity
    let E = (r.dot(&v) / (n * a * a)).atan2(1.0 - r.norm() / a); // Eccentric Anomaly
    let M = orbits::anomaly_eccentric_to_mean(E, e, false); // Mean Anomaly
    let u = (r[2]).atan2(-r[0] * W[1] + r[1] * W[0]); // Mean longiude
    let nu = ((1.0 - e * e).sqrt() * E.sin()).atan2(E.cos() - e); // True Anomaly
    let omega = u - nu; // Argument of perigee

    // # Correct angles to run from 0 to 2PI
    let RAAN = RAAN + 2.0 * PI;
    let omega = omega + 2.0 * PI;
    let M = M + 2.0 * PI;

    let RAAN = RAAN % (2.0 * PI);
    let omega = omega % (2.0 * PI);
    let M = M % (2.0 * PI);

    Vector6::new(
        a,
        e,
        to_degrees(i, as_degrees),
        to_degrees(RAAN, as_degrees),
        to_degrees(omega, as_degrees),
        to_degrees(M, as_degrees),
    )
}

///////////////////////////////
// Cartesian Transformations //
///////////////////////////////

/// Transforms a Cartesian Earth-inertial position into the
/// equivalent Cartesian Earth-fixed position.
///
/// The transformation is accomplished using the IAU 2006/2000A, CIO-based
/// theory using classical angles. The method as described in section 5.5 of
/// the SOFA C transformation cookbook.
///
/// # Arguments
/// - `epc`: Epoch instant for computation of the transformation
/// - `x_eci`: Cartesian Earth-inertial position. Units: (*m*)
///
/// # Returns
/// - `x_ecef`: Cartesian Earth-fixed position. Units: (*m*)
///
/// # Example
/// ```rust
/// use rastro::eop::{set_global_eop_from_default_standard, EOPExtrapolation, EOPType};
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector3_from_array;
/// use rastro::orbits::perigee_velocity;
/// use rastro::time::{Epoch, TimeSystem};
/// use rastro::transformations::*;
///
/// // Quick EOP initialization
/// set_global_eop_from_default_standard(EOPExtrapolation::Hold, true, EOPType::StandardBulletinA).unwrap();
///
/// let epc = Epoch::from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, TimeSystem::UTC);
///
/// // Create Cartesian state
/// let x_cart = vector3_from_array([R_EARTH, 0.0, 0.0]);
///
/// // Convert to ECEF state
/// let x_ecef = position_eci_to_ecef(epc, x_cart);
/// ```
pub fn position_eci_to_ecef(epc: Epoch, x: Vector3<f64>) -> Vector3<f64> {
    frames::rotation_eci_to_ecef(epc) * x
}

/// Transforms a Cartesian Earth-fixed position into the
/// equivalent Cartesian Earth-inertial position.
///
/// The transformation is accomplished using the IAU 2006/2000A, CIO-based
/// theory using classical angles. The method as described in section 5.5 of
/// the SOFA C transformation cookbook.
///
/// # Arguments
/// - `epc`: Epoch instant for computation of the transformation
/// - `x_ecef`: Cartesian Earth-fixed position. Units: (*m*)
///
/// # Returns
/// - `x_eci`: Cartesian Earth-inertial position. Units: (*m*)
///
/// # Example
/// ```rust
/// use rastro::eop::{set_global_eop_from_default_standard, EOPExtrapolation, EOPType};
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector3_from_array;
/// use rastro::orbits::perigee_velocity;
/// use rastro::time::{Epoch, TimeSystem};
/// use rastro::transformations::*;
///
/// // Quick EOP initialization
/// set_global_eop_from_default_standard(EOPExtrapolation::Hold, true, EOPType::StandardBulletinA).unwrap();
///
/// let epc = Epoch::from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, TimeSystem::UTC);
///
/// // Create Cartesian state
/// let x_ecef = vector3_from_array([R_EARTH, 0.0, 0.0]);
///
/// // Convert to ECEF state
/// let x_eci = position_ecef_to_eci(epc, x_ecef);
/// ```
pub fn position_ecef_to_eci(epc: Epoch, x: Vector3<f64>) -> Vector3<f64> {
    frames::rotation_ecef_to_eci(epc) * x
}

/// Transforms a Cartesian Earth inertial state (position and velocity) into the
/// equivalent Cartesian Earth-fixed state.
///
/// The transformation is accomplished using the IAU 2006/2000A, CIO-based
/// theory using classical angles. The method as described in section 5.5 of
/// the SOFA C transformation cookbook.
///
/// # Arguments
/// - `epc`: Epoch instant for computation of the transformation
/// - `x_eci`: Cartesian Earth inertial state (position, velocity). Units: (*m*; *m/s*)
///
/// # Returns
/// - `x_ecef`: Cartesian Earth-fixed state (position, velocity). Units: (*m*; *m/s*)
///
/// # Example
/// ```rust
/// use rastro::eop::{set_global_eop_from_default_standard, EOPExtrapolation, EOPType};
/// use rastro::utils::vector6_from_array;
/// use rastro::constants::R_EARTH;
/// use rastro::orbits::perigee_velocity;
/// use rastro::time::{Epoch, TimeSystem};
/// use rastro::transformations::*;
///
/// // Quick EOP initialization
/// set_global_eop_from_default_standard(EOPExtrapolation::Hold, true, EOPType::StandardBulletinA).unwrap();
///
/// let epc = Epoch::from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, TimeSystem::UTC);
///
/// // Create Cartesian state
/// let x_cart = vector6_from_array([R_EARTH + 500e3, 0.0, 0.0, 0.0, perigee_velocity(R_EARTH + 500e3, 0.0), 0.0]);
///
/// // Convert to ECEF state
/// let x_ecef = state_eci_to_ecef(epc, x_cart);
/// ```
pub fn state_eci_to_ecef(epc: Epoch, x_eci: na::Vector6<f64>) -> na::Vector6<f64> {
    // Compute Sequential Transformation Matrices
    let bpn = frames::bias_precession_nutation(epc);
    let r = frames::earth_rotation(epc);
    let pm = frames::polar_motion(epc);

    // Create Earth's Angular Rotation Vector
    let omega_vec = Vector3::new(0.0, 0.0, constants::OMEGA_EARTH);

    let r_eci = x_eci.fixed_rows::<3>(0);
    let v_eci = x_eci.fixed_rows::<3>(3);

    let p: Vector3<f64> = Vector3::from(pm * r * bpn * r_eci);
    let v: Vector3<f64> = pm * (r * bpn * v_eci - omega_vec.cross(&(r * bpn * r_eci)));

    Vector6::new(p[0], p[1], p[2], v[0], v[1], v[2])
}

/// Transforms a Cartesian Earth-fixed state (position and velocity) into the
/// equivalent Cartesian Earth-inertial state.
///
/// The transformation is accomplished using the IAU 2006/2000A, CIO-based
/// theory using classical angles. The method as described in section 5.5 of
/// the SOFA C transformation cookbook.
///
/// # Arguments
/// - `epc`: Epoch instant for computation of the transformation
/// - `x_ecef`: Cartesian Earth-fixed state (position, velocity). Units: (*m*; *m/s*)
///
/// # Returns
/// - `x_eci`: Cartesian Earth inertial state (position, velocity). Units: (*m*; *m/s*)
///
/// # Example
/// ```rust
/// use rastro::eop::{set_global_eop_from_default_standard, EOPExtrapolation, EOPType};
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector6_from_array;
/// use rastro::orbits::perigee_velocity;
/// use rastro::time::{Epoch, TimeSystem};
/// use rastro::transformations::*;
///
/// // Quick EOP initialization
/// set_global_eop_from_default_standard(EOPExtrapolation::Hold, true, EOPType::StandardBulletinA).unwrap();
///
/// let epc = Epoch::from_datetime(2007, 4, 5, 12, 0, 0.0, 0.0, TimeSystem::UTC);
///
/// // Create Cartesian inertial state
/// let x_cart = vector6_from_array([R_EARTH + 500e3, 0.0, 0.0, 0.0, perigee_velocity(R_EARTH + 500e3, 0.0), 0.0]);
///
/// // Convert to ECEF state
/// let x_ecef = state_eci_to_ecef(epc, x_cart);
///
/// // Convert ECEF state back to inertial state
/// let x_eci = state_ecef_to_eci(epc, x_ecef);
/// ```
pub fn state_ecef_to_eci(epc: Epoch, x_ecef: na::Vector6<f64>) -> na::Vector6<f64> {
    // Compute Sequential Transformation Matrices
    let bpn = frames::bias_precession_nutation(epc);
    let r = frames::earth_rotation(epc);
    let pm = frames::polar_motion(epc);

    // Create Earth's Angular Rotation Vector
    let omega_vec = Vector3::new(0.0, 0.0, constants::OMEGA_EARTH);

    let r_ecef = x_ecef.fixed_rows::<3>(0);
    let v_ecef = x_ecef.fixed_rows::<3>(3);

    let p: Vector3<f64> = Vector3::from((pm * r * bpn).transpose() * r_ecef);
    let v: Vector3<f64> = (r * bpn).transpose()
        * (pm.transpose() * v_ecef + omega_vec.cross(&(pm.transpose() * r_ecef)));

    Vector6::new(p[0], p[1], p[2], v[0], v[1], v[2])
}

/////////////////////////////////
// Earth-Fixed Transformations //
/////////////////////////////////

const ECC2: f64 = constants::WGS84_F * (2.0 - constants::WGS84_F);

/// Convert geocentric position to equivalent Earth-fixed position.
///
/// # Arguments:
/// - `x_geoc`: Geocentric coordinates (lon, lat, altitude). Units: (*rad* or *deg* and *m*)
/// - `use_degrees`: Interprets input as (deg) if `true` or (rad) if `false`
///
/// # Returns
/// - `x_ecef`: Earth-fixed coordinates. Units (*m*)
///
/// # Examples
/// ```rust
/// ```
pub fn position_geocentric_to_ecef(x_geoc: Vector3<f64>, as_degrees: bool) -> Vector3<f64> {
    let lon = from_degrees(x_geoc[0], as_degrees);
    let lat = from_degrees(x_geoc[1], as_degrees);
    let alt = x_geoc[2];

    // Check validity of inputs
    if lat < -PI / 2.0 || lat > PI / 2.0 {
        panic!("Input latitude out of range. Input must be between -90 and 90 degrees.");
    }

    // Compute Earth-fixed position
    let r = constants::WGS84_A + alt;
    let x = r * lat.cos() * lon.cos();
    let y = r * lat.cos() * lon.sin();
    let z = r * lat.sin();

    Vector3::new(x, y, z)
}

/// Convert Earth-fixed position into equivalent of geocentric position.
///
/// # Arguments:
/// - `x_ecef`: Earth-fixed coordinates. Units (*m*)
/// - `use_degrees`: Produces output in (deg) if `true` or (rad) if `false`
///
/// # Returns
/// - `x_geoc`: Geocentric coordinates (lon, lat, altitude). Units: (*rad* or *deg* and *m*)
///
/// # Examples
/// ```rust
/// ```
pub fn position_ecef_to_geocentric(x_ecef: Vector3<f64>, as_degrees: bool) -> Vector3<f64> {
    let x = x_ecef[0];
    let y = x_ecef[1];
    let z = x_ecef[2];

    // Compute geocentric coordinates
    let lon = y.atan2(x);
    let lat = z.atan2((x * x + y * y).sqrt());
    let alt = (x * x + y * y + z * z).sqrt() - constants::WGS84_A;

    Vector3::new(
        to_degrees(lon, as_degrees),
        to_degrees(lat, as_degrees),
        alt,
    )
}

/// Convert geodetic position to equivalent Earth-fixed position.
///
/// # Arguments:
/// - `x_geod`: Geodetic coordinates (lon, lat, altitude). Units: (*rad* or *deg* and *m*)
/// - `use_degrees`: Interprets input as (deg) if `true` or (rad) if `false`
///
/// # Returns
/// - `x_ecef`: Earth-fixed coordinates. Units (*m*)
///
/// # Examples
/// ```rust
/// ```
pub fn position_geodetic_to_ecef(x_geod: Vector3<f64>, as_degrees: bool) -> Vector3<f64> {
    let lon = from_degrees(x_geod[0], as_degrees);
    let lat = from_degrees(x_geod[1], as_degrees);
    let alt = x_geod[2];

    // Check validity of inputs
    if lat < -PI / 2.0 || lat > PI / 2.0 {
        panic!("Input latitude out of range. Input must be between -90 and 90 degrees.");
    }

    // Compute Earth-fixed position
    let N = constants::WGS84_A / (1.0 - ECC2 * (lat.sin()).powi(2));
    let x = (N + alt) * lat.cos() * lon.cos();
    let y = (N + alt) * lat.cos() * lon.sin();
    let z = ((1.0 - ECC2) * N + alt) * lat.sin();

    Vector3::new(x, y, z)
}

/// Convert Earth-fixed position into equivalent of geodetic position.
///
/// # Arguments:
/// - `x_ecef`: Earth-fixed coordinates. Units (*m*)
/// - `use_degrees`: Produces output in (deg) if `true` or (rad) if `false`
///
/// # Returns
/// - `x_geod`: Geodetic coordinates (lon, lat, altitude). Units: (*rad* or *deg* and *m*)
///
/// # Examples
/// ```rust
/// ```
pub fn position_ecef_to_geodetic(x_ecef: Vector3<f64>, as_degrees: bool) -> Vector3<f64> {
    let x = x_ecef[0];
    let y = x_ecef[1];
    let z = x_ecef[2];

    // Compute intermediate quantities
    let eps = f64::EPSILON * 1.0e3;
    let rho2 = x * x + y * x;
    let mut dz = ECC2 * z;
    let mut N = 0.0;

    // Iterative refine coordinate estimate
    loop {
        let zdz = z + dz;
        let Nh = (rho2 + zdz * zdz).sqrt();
        let sinphi = zdz / Nh;
        N = constants::WGS84_A / (1.0 - ECC2 * sinphi * sinphi).sqrt();
        let dz_new = N * ECC2 * sinphi;

        // Check convergence requirement
        if (dz - dz_new).abs() < eps {
            break;
        }
    }

    // Extract geodetic coordiantes
    let zdz = z + dz;
    let lon = y.atan2(x);
    let lat = zdz.atan2(rho2.sqrt());
    let alt = (rho2 + zdz).sqrt() - N;

    Vector3::new(
        to_degrees(lon, as_degrees),
        to_degrees(lat, as_degrees),
        alt,
    )
}

pub fn position_enz_to_ecef(x_enz: Vector3<f64>, as_degrees: bool) -> Vector3<f64> {
    Vector3::zeros()
}
pub fn position_ecef_to_enz(x_ecef: Vector3<f64>, as_degrees: bool) -> Vector3<f64> {
    Vector3::zeros()
}

pub fn state_enz_to_ecef(x_enz: Vector6<f64>, as_degrees: bool) -> Vector6<f64> {
    Vector6::zeros()
}
pub fn state_ecef_to_enz(x_ecef: Vector6<f64>, as_degrees: bool) -> Vector6<f64> {
    Vector6::zeros()
}

pub fn position_sez_to_ecef(x_sez: Vector3<f64>, as_degrees: bool) -> Vector3<f64> {
    Vector3::zeros()
}
pub fn position_ecef_to_sez(x_ecef: Vector3<f64>, as_degrees: bool) -> Vector3<f64> {
    Vector3::zeros()
}

pub fn state_sez_to_ecef(x_sez: Vector6<f64>, as_degrees: bool) -> Vector6<f64> {
    Vector6::zeros()
}
pub fn state_ecef_to_sez(x_ecef: Vector6<f64>, as_degrees: bool) -> Vector6<f64> {
    Vector6::zeros()
}

pub fn position_enz_to_azel(x_enz: Vector3<f64>, as_degrees: bool) -> Vector3<f64> {
    Vector3::zeros()
}

pub fn state_enz_to_azel(x_enz: Vector6<f64>, as_degrees: bool) -> Vector6<f64> {
    Vector6::zeros()
}

pub fn position_sez_to_azel(x_sez: Vector3<f64>, as_degrees: bool) -> Vector3<f64> {
    Vector3::zeros()
}

pub fn state_sez_to_azel(x_sez: Vector6<f64>, as_degrees: bool) -> Vector6<f64> {
    Vector6::zeros()
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use crate::constants::R_EARTH;
    use crate::eop::*;
    use crate::orbits::*;
    use crate::time::TimeSystem;
    use crate::transformations::*;
    use approx::assert_abs_diff_eq;
    use std::env;
    use std::path::Path;

    fn set_global_test_eop() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir)
            .join("test_assets")
            .join("iau2000A_c04_14.txt");

        let eop_extrapolation = EOPExtrapolation::Hold;
        let eop_interpolation = true;

        set_global_eop_from_c04_file(
            filepath.to_str().unwrap(),
            eop_extrapolation,
            eop_interpolation,
        )
        .unwrap();
    }

    #[test]
    fn test_state_osculating_to_cartesian() {
        set_global_test_eop();

        let osc = vector6_from_array([R_EARTH + 500e3, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let cart = state_osculating_to_cartesian(osc, false);

        assert_eq!(cart[0], R_EARTH + 500e3);
        assert_eq!(cart[1], 0.0);
        assert_eq!(cart[2], 0.0);
        assert_eq!(cart[3], 0.0);
        assert_eq!(cart[4], perigee_velocity(R_EARTH + 500e3, 0.0));
        assert_eq!(cart[5], 0.0);

        let osc = vector6_from_array([R_EARTH + 500e3, 0.0, 90.0, 0.0, 0.0, 0.0]);
        let cart = state_osculating_to_cartesian(osc, true);

        assert_eq!(cart[0], R_EARTH + 500e3);
        assert_eq!(cart[1], 0.0);
        assert_eq!(cart[2], 0.0);
        assert_eq!(cart[3], 0.0);
        assert_abs_diff_eq!(cart[4], 0.0, epsilon = 1.0e-12);
        assert_eq!(cart[5], perigee_velocity(R_EARTH + 500e3, 0.0));
    }

    #[test]
    fn test_state_cartesian_to_osculating() {
        set_global_test_eop();

        let cart = vector6_from_array([
            R_EARTH + 500e3,
            0.0,
            0.0,
            0.0,
            perigee_velocity(R_EARTH + 500e3, 0.0),
            0.0,
        ]);
        let osc = state_cartesian_to_osculating(cart, true);

        assert_abs_diff_eq!(osc[0], R_EARTH + 500e3, epsilon = 1e-9);
        assert_eq!(osc[1], 0.0);
        assert_eq!(osc[2], 0.0);
        assert_eq!(osc[3], 180.0);
        assert_eq!(osc[4], 0.0);
        assert_eq!(osc[5], 0.0);

        let cart = vector6_from_array([
            R_EARTH + 500e3,
            0.0,
            0.0,
            0.0,
            0.0,
            perigee_velocity(R_EARTH + 500e3, 0.0),
        ]);
        let osc = state_cartesian_to_osculating(cart, true);

        assert_abs_diff_eq!(osc[0], R_EARTH + 500e3, epsilon = 1.0e-9);
        assert_eq!(osc[1], 0.0);
        assert_eq!(osc[2], 90.0);
        assert_eq!(osc[3], 0.0);
        assert_eq!(osc[4], 0.0);
        assert_eq!(osc[5], 0.0);
    }

    #[test]
    fn test_position_eci_to_ecef() {
        set_global_test_eop();
        let epc = Epoch::from_datetime(2022, 4, 5, 0, 0, 0.0, 0.0, TimeSystem::UTC);

        let p_eci = Vector3::new(R_EARTH + 500e3, 0.0, 0.0);

        let p_ecef = position_eci_to_ecef(epc, p_eci);

        assert_ne!(p_eci[0], p_ecef[0]);
        assert_ne!(p_eci[1], p_ecef[1]);
        assert_ne!(p_eci[2], p_ecef[2]);
    }

    #[test]
    fn test_position_ecef_to_eci() {
        set_global_test_eop();
        let epc = Epoch::from_datetime(2022, 4, 5, 0, 0, 0.0, 0.0, TimeSystem::UTC);

        let p_ecef = Vector3::new(R_EARTH + 500e3, 0.0, 0.0);

        let p_eci = position_ecef_to_eci(epc, p_ecef);

        assert_ne!(p_eci[0], p_ecef[0]);
        assert_ne!(p_eci[1], p_ecef[1]);
        assert_ne!(p_eci[2], p_ecef[2]);
    }

    #[test]
    fn test_state_eci_to_ecef_circular() {
        set_global_test_eop();
        let epc = Epoch::from_datetime(2022, 4, 5, 0, 0, 0.0, 0.0, TimeSystem::UTC);

        let oe = vector6_from_array([R_EARTH + 500e3, 1e-3, 97.8, 75.0, 25.0, 45.0]);
        let eci = state_osculating_to_cartesian(oe, true);

        // Perform circular transformations
        let ecef = state_eci_to_ecef(epc, eci);
        let eci2 = state_ecef_to_eci(epc, ecef);
        let ecef2 = state_eci_to_ecef(epc, eci2);

        let tol = 1e-6;
        // Check equivalence of ECI transforms
        assert_abs_diff_eq!(eci2[0], eci[0], epsilon = tol);
        assert_abs_diff_eq!(eci2[1], eci[1], epsilon = tol);
        assert_abs_diff_eq!(eci2[2], eci[2], epsilon = tol);
        assert_abs_diff_eq!(eci2[3], eci[3], epsilon = tol);
        assert_abs_diff_eq!(eci2[4], eci[4], epsilon = tol);
        assert_abs_diff_eq!(eci2[5], eci[5], epsilon = tol);
        // Check equivalence of ECEF transforms
        assert_abs_diff_eq!(ecef2[0], ecef[0], epsilon = tol);
        assert_abs_diff_eq!(ecef2[1], ecef[1], epsilon = tol);
        assert_abs_diff_eq!(ecef2[2], ecef[2], epsilon = tol);
        assert_abs_diff_eq!(ecef2[3], ecef[3], epsilon = tol);
        assert_abs_diff_eq!(ecef2[4], ecef[4], epsilon = tol);
        assert_abs_diff_eq!(ecef2[5], ecef[5], epsilon = tol);
    }

    #[test]
    fn test_position_geocentric_to_ecef() {}

    #[test]
    fn test_position_ecef_to_geocentric() {}

    #[test]
    fn test_position_geodetic_to_ecef() {}

    #[test]
    fn test_position_ecef_to_geodetic() {}
}
