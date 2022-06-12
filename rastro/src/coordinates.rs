use is_close::is_close;
use nalgebra as na;
use nalgebra::{Matrix3, Vector3, Vector6};
use std::f64::consts::PI;

use crate::constants;
use crate::constants::GM_EARTH;
use crate::orbits;
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
/// use rastro::coordinates::*;
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
/// - `use_degrees`: Returns output as (*deg*) if `true` or (*rad*) if `false`
///
/// # Returns
/// - `x_oe`: Osculating orbital elements
///
/// # Examples
/// ```rust
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector6_from_array;
/// use rastro::orbits::perigee_velocity;
/// use rastro::coordinates::*;
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

/////////////////////////////////
// Earth-Fixed Transformations //
/////////////////////////////////

const ECC2: f64 = constants::WGS84_F * (2.0 - constants::WGS84_F);

/// Convert geocentric position to equivalent Earth-fixed position.
///
/// The valid input range for each component is:
/// - lon: [-inf, +inf]. Larger values will be wrapped appropriately
/// - lat: [-90, +90], Out-of-bounds values will result in an `Error`
/// - alt: [-inf, +inf]. All values are valid, but may give unintended results
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
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let geoc = vector3_from_array([0.0, 0.0, 0.0]);
/// let ecef = position_geocentric_to_ecef(geoc, true).unwrap();
/// // Returns state [R_EARTH, 0.0, 0.0]
/// ```
pub fn position_geocentric_to_ecef(
    x_geoc: Vector3<f64>,
    as_degrees: bool,
) -> Result<Vector3<f64>, String> {
    let lon = from_degrees(x_geoc[0], as_degrees);
    let lat = from_degrees(x_geoc[1], as_degrees);
    let alt = x_geoc[2];

    // Check validity of inputs
    if lat < -PI / 2.0 || lat > PI / 2.0 {
        return Err(format!(
            "Input latitude out of range. Input must be between -90 and 90 degrees. Input: {}",
            lat
        ));
    }

    // Compute Earth-fixed position
    let r = constants::WGS84_A + alt;
    let x = r * lat.cos() * lon.cos();
    let y = r * lat.cos() * lon.sin();
    let z = r * lat.sin();

    Ok(Vector3::new(x, y, z))
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
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let ecef = vector3_from_array([R_EARTH, 0.0, 0.0]);
/// let geoc = position_ecef_to_geocentric(ecef, true);
/// // Returns state [0.0, 0.0, 0.0]
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
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let geod = vector3_from_array([0.0, 0.0, 0.0]);
/// let ecef = position_geodetic_to_ecef(geod, true).unwrap();
/// // Returns state [R_EARTH, 0.0, 0.0]
/// ```
#[allow(non_snake_case)]
pub fn position_geodetic_to_ecef(
    x_geod: Vector3<f64>,
    as_degrees: bool,
) -> Result<Vector3<f64>, String> {
    let lon = from_degrees(x_geod[0], as_degrees);
    let lat = from_degrees(x_geod[1], as_degrees);
    let alt = x_geod[2];

    // Check validity of inputs
    if lat < -PI / 2.0 || lat > PI / 2.0 {
        return Err(format!(
            "Input latitude out of range. Input must be between -90 and 90 degrees. Input: {}",
            lat
        ));
    }

    // Compute Earth-fixed position
    let N = constants::WGS84_A / (1.0 - ECC2 * lat.sin().powi(2)).sqrt();
    let x = (N + alt) * lat.cos() * lon.cos();
    let y = (N + alt) * lat.cos() * lon.sin();
    let z = ((1.0 - ECC2) * N + alt) * lat.sin();

    Ok(Vector3::new(x, y, z))
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
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let ecef = vector3_from_array([R_EARTH, 0.0, 0.0]);
/// let geoc = position_ecef_to_geodetic(ecef, true);
/// // Returns state [0.0, 0.0, 0.0]
/// ```
#[allow(non_snake_case)]
pub fn position_ecef_to_geodetic(x_ecef: Vector3<f64>, as_degrees: bool) -> Vector3<f64> {
    let x = x_ecef[0];
    let y = x_ecef[1];
    let z = x_ecef[2];

    // Compute intermediate quantities
    let eps = f64::EPSILON * 1.0e3;
    let rho2 = x * x + y * y;
    let mut dz = ECC2 * z;
    let mut N = 0.0;

    // Iterative refine coordinate estimate
    let mut iter = 0;
    while iter < 10 {
        let zdz = z + dz;
        let Nh = (rho2 + zdz * zdz).sqrt();
        let sinphi = zdz / Nh;
        N = constants::WGS84_A / (1.0 - ECC2 * sinphi * sinphi).sqrt();
        let dz_new = N * ECC2 * sinphi;

        // Check convergence requirement
        if (dz - dz_new).abs() < eps {
            break;
        }

        dz = dz_new;
        iter += 1;
    }

    if iter == 10 {
        panic!("Reached maximum number of iterations.");
    }

    // Extract geodetic coordiantes
    let zdz = z + dz;
    let lon = y.atan2(x);
    let lat = zdz.atan2(rho2.sqrt());
    let alt = (rho2 + zdz * zdz).sqrt() - N;

    Vector3::new(
        to_degrees(lon, as_degrees),
        to_degrees(lat, as_degrees),
        alt,
    )
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum EllipsoidalConversionType {
    Geocentric,
    Geodetic,
}

/// Compute the rotation matrix from body-fixed to East-North-Zenith (ENZ)
/// Cartesian coordinates for a given set of coordinates on an ellipsoidal body.
/// The ellipsoidal coordinates can either be geodetic or geocentric.
///
/// # Args:
/// - `x_ellipsoid`: Ellipsoidal coordinates.  Expected format (lon, lat, alt)
/// - `use_degrees`: Interprets input as (deg) if `true` or (rad) if `false`
///
/// # Returns:
/// - `E`: Earth-fixed to Topocentric rotation matrix
///
/// # Examples:
/// ```rust
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let x_geo = vector3_from_array([30.0, 60.0, 0.0]);
/// let rot = rotation_ellipsoid_to_enz(x_geo, true);
/// ```
pub fn rotation_ellipsoid_to_enz(x_ellipsoid: Vector3<f64>, as_degrees: bool) -> Matrix3<f64> {
    let lon = from_degrees(x_ellipsoid[0], as_degrees);
    let lat = from_degrees(x_ellipsoid[1], as_degrees);

    // Construct Rotation matrix
    Matrix3::new(
        -lon.sin(),
        lon.cos(),
        0.0, // E-base vector
        -lat.sin() * lon.cos(),
        -lat.sin() * lon.sin(),
        lat.cos(), // N-base vector
        lat.cos() * lon.cos(),
        lat.cos() * lon.sin(),
        lat.sin(), // Z-base vector
    )
}

/// Compute the rotation matrix from East-North-Zenith (ENZ) to body-fixed
/// Cartesian coordinates for a given set of coordinates on an ellipsoidal body.
/// The ellipsoidal coordinates can either be geodetic or geocentric.
///
/// # Args:
/// - `x_ellipsoid`: Ellipsoidal coordinates.  Expected format (lon, lat, alt)
/// - `use_degrees`: Interprets input as (deg) if `true` or (rad) if `false`
///
/// # Returns:
/// - `E`: Topocentric to Earth-fixed rotation matrix
///
/// # Examples:
/// ```rust
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let x_geo = vector3_from_array([30.0, 60.0, 0.0]);
/// let rot = rotation_enz_to_ellipsoid(x_geo, true);
/// ```
pub fn rotation_enz_to_ellipsoid(x_ellipsoid: Vector3<f64>, as_degrees: bool) -> Matrix3<f64> {
    rotation_ellipsoid_to_enz(x_ellipsoid, as_degrees).transpose()
}

/// Computes the relative state in East-North-Zenith (ENZ) coordinates for a target
/// object in the ECEF frame with respect to a fixed location (station) also in
/// the ECEF frame.
///
/// # Args:
/// - `location_ecef`: Cartesian position of the observing station in the ECEF frame.
/// - `x_ecef`: Cartesian position of the observed object in the ECEF frame
/// - `conversion_type`: Type of conversion to apply for computing the topocentric frame based on station coordinates.
///
/// # Returns:
/// - `r_rel`: Relative position of object in ENZ coordinates based on the station location.
///
/// # Examples:
/// ```rust
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let x_station = vector3_from_array([R_EARTH, 0.0, 0.0]);
/// let x_sat = vector3_from_array([R_EARTH + 500.0e3, 0.0, 0.0]);
///
/// let r_enz = relative_position_ecef_to_enz(
///     x_station, x_sat, EllipsoidalConversionType::Geocentric
/// );
/// ```
#[allow(non_snake_case)]
pub fn relative_position_ecef_to_enz(
    location_ecef: Vector3<f64>,
    r_ecef: Vector3<f64>,
    conversion_type: EllipsoidalConversionType,
) -> Vector3<f64> {
    // Create ENZ rotation matrix
    let E = match conversion_type {
        EllipsoidalConversionType::Geocentric => {
            rotation_ellipsoid_to_enz(position_ecef_to_geocentric(location_ecef, false), false)
        }
        EllipsoidalConversionType::Geodetic => {
            rotation_ellipsoid_to_enz(position_ecef_to_geodetic(location_ecef, false), false)
        }
    };

    // Compute range transformation
    let r = r_ecef - location_ecef;
    E * r
}

/// Computes the absolute Earth-fixed coordinates for an object given its relative
/// position in East-North-Zenith (ENZ) coordinates and the Cartesian body-fixed
/// coordinates of the observing location/station.
///
/// # Args:
/// - `location_ecef`: Cartesian position of the observing station in the ECEF frame.
/// - `r_rel`: Relative position of object in ENZ coordinates based on the station location.
/// - `conversion_type`: Type of conversion to apply for computing the topocentric frame based on station coordinates.
///
/// # Returns:
/// - `r_ecef`: Cartesian position of the observed object in the ECEF frame
///
/// # Examples:
/// ```rust
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let x_station = vector3_from_array([R_EARTH, 0.0, 0.0]);
/// let r_enz = vector3_from_array([0.0, 0.0, 500.0e3]);
///
/// let r_ecef = relative_position_enz_to_ecef(
///     x_station, r_enz, EllipsoidalConversionType::Geocentric
/// );
/// ```
#[allow(non_snake_case)]
pub fn relative_position_enz_to_ecef(
    location_ecef: Vector3<f64>,
    r_enz: Vector3<f64>,
    conversion_type: EllipsoidalConversionType,
) -> Vector3<f64> {
    // Create ENZ rotation matrix
    let Et = match conversion_type {
        EllipsoidalConversionType::Geocentric => {
            rotation_enz_to_ellipsoid(position_ecef_to_geocentric(location_ecef, false), false)
        }
        EllipsoidalConversionType::Geodetic => {
            rotation_enz_to_ellipsoid(position_ecef_to_geodetic(location_ecef, false), false)
        }
    };

    // Compute range transformation
    let r = r_enz;
    location_ecef + Et * r
}

/// Compute the rotation matrix from body-fixed to South-East-Zenith (SEZ)
/// Cartesian coordinates for a given set of coordinates on an ellipsoidal body.
/// The ellipsoidal coordinates can either be geodetic or geocentric.
///
/// # Args:
/// - `x_ellipsoid`: Ellipsoidal coordinates.  Expected format (lon, lat, alt)
/// - `use_degrees`: Interprets input as (deg) if `true` or (rad) if `false`
///
/// # Returns:
/// - `E`: Earth-fixed to Topocentric rotation matrix
///
/// # Examples:
/// ```rust
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let x_geo = vector3_from_array([30.0, 60.0, 0.0]);
/// let rot = rotation_sez_to_ellipsoid(x_geo, true);
/// ```
pub fn rotation_ellipsoid_to_sez(x_ellipsoid: Vector3<f64>, as_degrees: bool) -> Matrix3<f64> {
    let lon = from_degrees(x_ellipsoid[0], as_degrees);
    let lat = from_degrees(x_ellipsoid[1], as_degrees);

    // Construct Rotation matrix
    Matrix3::new(
        lat.sin() * lon.cos(),
        lat.sin() * lon.sin(),
        -lat.cos(), // S-base vector
        -lon.sin(),
        lon.cos(),
        0.0, // E-base vector
        lat.cos() * lon.cos(),
        lat.cos() * lon.sin(),
        lat.sin(), // Z-base vector
    )
}

/// Compute the rotation matrix from South-East-Zenith (SEZ) to body-fixed
/// Cartesian coordinates for a given set of coordinates on an ellipsoidal body.
/// The ellipsoidal coordinates can either be geodetic or geocentric.
///
/// # Args:
/// - `x_ellipsoid`: Ellipsoidal coordinates. Expected format (lon, lat, alt)
/// - `use_degrees`: Interprets input as (deg) if `true` or (rad) if `false`
///
/// # Returns:
/// - `E`: Topocentric to Earth-fixed rotation matrix
///
/// # Examples:
/// ```rust
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let x_geo = vector3_from_array([30.0, 60.0, 0.0]);
/// let rot = rotation_sez_to_ellipsoid(x_geo, true);
/// ```
pub fn rotation_sez_to_ellipsoid(x_ellipsoid: Vector3<f64>, as_degrees: bool) -> Matrix3<f64> {
    rotation_ellipsoid_to_sez(x_ellipsoid, as_degrees).transpose()
}

/// Computes the relative state in South-East-Zenith (SEZ) coordinates for a target
/// object in the ECEF frame with respect to a fixed location (station) also in
/// the ECEF frame.
///
/// # Args:
/// - `location_ecef`: Cartesian position of the observing station in the ECEF frame.
/// - `r_ecef`: Cartesian position of the observed object in the ECEF frame
/// - `conversion_type`: Type of conversion to apply for computing the topocentric frame based on station coordinates.
///
/// # Returns:
/// - `r_rel`: Relative position of object in ENZ coordinates based on the station location.
///
/// # Examples:
/// ```rust
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let x_station = vector3_from_array([R_EARTH, 0.0, 0.0]);
/// let x_sat = vector3_from_array([R_EARTH + 500.0e3, 0.0, 0.0]);
///
/// let r_enz = relative_position_ecef_to_enz(
///     x_station, x_sat, EllipsoidalConversionType::Geocentric
/// );
/// ```
#[allow(non_snake_case)]
pub fn relative_position_ecef_to_sez(
    location_ecef: Vector3<f64>,
    r_ecef: Vector3<f64>,
    conversion_type: EllipsoidalConversionType,
) -> Vector3<f64> {
    // Create ENZ rotation matrix
    let E = match conversion_type {
        EllipsoidalConversionType::Geocentric => {
            rotation_ellipsoid_to_sez(position_ecef_to_geocentric(location_ecef, false), false)
        }
        EllipsoidalConversionType::Geodetic => {
            rotation_ellipsoid_to_sez(position_ecef_to_geodetic(location_ecef, false), false)
        }
    };

    // Compute range transformation
    let r = r_ecef - location_ecef;
    E * r
}

/// Computes the absolute Earth-fixed coordinates for an object given its relative
/// position in East-North-Zenith (ENZ) coordinates and the Cartesian body-fixed
/// coordinates of the observing location/station.
///
/// # Args:
/// - `location_ecef`: Cartesian position of the observing station in the ECEF frame.
/// - `r_rel`: Relative position of object in ENZ coordinates based on the station location.
/// - `conversion_type`: Type of conversion to apply for computing the topocentric frame based on station coordinates.
///
/// # Returns:
/// - `r_ecef`: Cartesian position of the observed object in the ECEF frame
///
/// # Examples:
/// ```rust
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let x_station = vector3_from_array([R_EARTH, 0.0, 0.0]);
/// let r_sez = vector3_from_array([0.0, 0.0, 500.0e3]);
///
/// let r_ecef = relative_position_sez_to_ecef(
///     x_station, r_sez, EllipsoidalConversionType::Geocentric
/// );
/// ```
#[allow(non_snake_case)]
pub fn relative_position_sez_to_ecef(
    location_ecef: Vector3<f64>,
    x_sez: Vector3<f64>,
    conversion_type: EllipsoidalConversionType,
) -> Vector3<f64> {
    // Create SEZ rotation matrix
    let Et = match conversion_type {
        EllipsoidalConversionType::Geocentric => {
            rotation_sez_to_ellipsoid(position_ecef_to_geocentric(location_ecef, false), false)
        }
        EllipsoidalConversionType::Geodetic => {
            rotation_sez_to_ellipsoid(position_ecef_to_geodetic(location_ecef, false), false)
        }
    };

    // Compute range transformation
    let r = x_sez;
    location_ecef + Et * r
}

/// Converts East-North-Zenith topocentric coordinates of an location
/// into azimuth, elevation, and range from that same location. Azimuth is measured
/// clockwise from North.
///
/// # Args:
/// - `x_enz`: Relative Cartesian position of object to location East-North-Up coordinates. Units: (*m*)
/// - `use_degrees`: Returns output as (*deg*) if `true` or (*rad*) if `false`
///
/// # Returns:
/// - `x_azel`: Azimuth, elevation and range. Units: (*angle*, *angle*, *m*)
///
/// # Examples:
/// ```rust
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let x_enz = vector3_from_array([100.0, 0.0, 0.0]);
///
/// let x_azel = position_enz_to_azel(x_enz, true);
/// // x_azel = [90.0, 0.0, 100.0]
/// ```
pub fn position_enz_to_azel(x_enz: Vector3<f64>, as_degrees: bool) -> Vector3<f64> {
    // Range
    let rho = x_enz.norm();

    // Elevation
    let el = ((x_enz[0].powi(2) + x_enz[1].powi(2)).sqrt()).atan2(x_enz[2]);

    // Azimuth
    let az = if el != PI / 2.0 {
        let azt = x_enz[1].atan2(x_enz[0]);

        if azt >= 0.0 {
            azt
        } else {
            azt + 2.0 * PI
        }
    } else {
        // If at peak elevation azimuth is ambiguous so define as 0.0
        0.0
    };

    Vector3::new(to_degrees(az, as_degrees), to_degrees(el, as_degrees), rho)
}

/// Converts South-East-Zenith topocentric coordinates of an location
/// into azimuth, elevation, and range from that same location. Azimuth is measured
/// clockwise from North.
///
/// # Args:
/// - `x_sez`: Relative Cartesian position of object to location South-East-Zenith coordinates. Units: (*m*)
/// - `use_degrees`: Returns output as (*deg*) if `true` or (*rad*) if `false`
///
/// # Returns:
/// - `x_azel`: Azimuth, elevation and range. Units: (*angle*, *angle*, *m*)
///
/// # Examples:
/// ```rust
/// use rastro::constants::R_EARTH;
/// use rastro::utils::vector3_from_array;
/// use rastro::coordinates::*;
///
/// let x_enz = vector3_from_array([0.0, 100.0, 0.0]);
///
/// let x_azel = position_sez_to_azel(x_enz, true);
/// // x_azel = [90.0, 0.0, 100.0]
/// ```
pub fn position_sez_to_azel(x_sez: Vector3<f64>, as_degrees: bool) -> Vector3<f64> {
    // Range
    let rho = x_sez.norm();

    // Elevation
    let el = ((x_sez[0].powi(2) + x_sez[1].powi(2)).sqrt()).atan2(x_sez[2]);

    // Azimuth
    let az = if el != PI / 2.0 {
        let azt = (-x_sez[0]).atan2(x_sez[1]);

        if azt >= 0.0 {
            azt
        } else {
            azt + 2.0 * PI
        }
    } else {
        // If at peak elevation azimuth is ambiguous so define as 0.0
        0.0
    };

    Vector3::new(to_degrees(az, as_degrees), to_degrees(el, as_degrees), rho)
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use crate::constants::{R_EARTH, WGS84_A, WGS84_F};
    use crate::coordinates::*;
    use crate::eop::*;
    use crate::orbits::*;
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
    fn test_position_geocentric() {
        let tol = 1.0e-7;

        // Test known position conversions
        let geoc1 = Vector3::new(0.0, 0.0, 0.0);
        let ecef1 = position_geocentric_to_ecef(geoc1, false).unwrap();

        assert_abs_diff_eq!(ecef1[0], WGS84_A, epsilon = tol);
        assert_abs_diff_eq!(ecef1[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef1[2], 0.0, epsilon = tol);

        let geoc2 = Vector3::new(90.0, 0.0, 0.0);
        let ecef2 = position_geocentric_to_ecef(geoc2, true).unwrap();

        assert_abs_diff_eq!(ecef2[0], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef2[1], WGS84_A, epsilon = tol);
        assert_abs_diff_eq!(ecef2[2], 0.0, epsilon = tol);

        let geoc3 = Vector3::new(0.0, 90.0, 0.0);
        let ecef3 = position_geocentric_to_ecef(geoc3, true).unwrap();

        assert_abs_diff_eq!(ecef3[0], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef3[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef3[2], WGS84_A, epsilon = tol);

        // Test two-input format
        let geoc = Vector3::new(0.0, 0.0, 0.0);
        let ecef = position_geocentric_to_ecef(geoc, false).unwrap();

        assert_abs_diff_eq!(ecef[0], WGS84_A, epsilon = tol);
        assert_abs_diff_eq!(ecef[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef[2], 0.0, epsilon = tol);

        let geoc = Vector3::new(90.0, 0.0, 0.0);
        let ecef = position_geocentric_to_ecef(geoc, true).unwrap();

        assert_abs_diff_eq!(ecef[0], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef[1], WGS84_A, epsilon = tol);
        assert_abs_diff_eq!(ecef[2], 0.0, epsilon = tol);

        let geoc = Vector3::new(0.0, 90.0, 0.0);
        let ecef = position_geocentric_to_ecef(geoc, true).unwrap();

        assert_abs_diff_eq!(ecef[0], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef[2], WGS84_A, epsilon = tol);

        // Test circularity
        let geoc4 = position_ecef_to_geocentric(ecef1, true);
        let geoc5 = position_ecef_to_geocentric(ecef2, true);
        let geoc6 = position_ecef_to_geocentric(ecef3, true);

        assert_abs_diff_eq!(geoc4[0], geoc1[0], epsilon = tol);
        assert_abs_diff_eq!(geoc4[1], geoc1[1], epsilon = tol);
        assert_abs_diff_eq!(geoc4[2], geoc1[2], epsilon = tol);

        assert_abs_diff_eq!(geoc5[0], geoc2[0], epsilon = tol);
        assert_abs_diff_eq!(geoc5[1], geoc2[1], epsilon = tol);
        assert_abs_diff_eq!(geoc5[2], geoc2[2], epsilon = tol);

        assert_abs_diff_eq!(geoc6[0], geoc3[0], epsilon = tol);
        assert_abs_diff_eq!(geoc6[1], geoc3[1], epsilon = tol);
        assert_abs_diff_eq!(geoc6[2], geoc3[2], epsilon = tol);

        // Random point circularity
        let geoc = Vector3::new(77.875000, 20.975200, 0.000000);
        let ecef = position_geocentric_to_ecef(geoc, true).unwrap();
        let geocc = position_ecef_to_geocentric(ecef, true);
        assert_abs_diff_eq!(geoc[0], geocc[0], epsilon = tol);
        assert_abs_diff_eq!(geoc[1], geocc[1], epsilon = tol);
        assert_abs_diff_eq!(geoc[2], geocc[2], epsilon = tol);

        assert!(position_geocentric_to_ecef(Vector3::new(0.0, 90.1, 0.0), true).is_err());

        assert!(position_geocentric_to_ecef(Vector3::new(0.0, -90.1, 0.0), true).is_err());
    }

    #[test]
    fn test_position_geodetic() {
        let tol = 1.0e-7;

        // Test known position conversions
        let geod1 = Vector3::new(0.0, 0.0, 0.0);
        let ecef1 = position_geodetic_to_ecef(geod1, false).unwrap();

        assert_abs_diff_eq!(ecef1[0], WGS84_A, epsilon = tol);
        assert_abs_diff_eq!(ecef1[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef1[2], 0.0, epsilon = tol);

        let geod2 = Vector3::new(90.0, 0.0, 0.0);
        let ecef2 = position_geodetic_to_ecef(geod2, true).unwrap();

        assert_abs_diff_eq!(ecef2[0], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef2[1], WGS84_A, epsilon = tol);
        assert_abs_diff_eq!(ecef2[2], 0.0, epsilon = tol);

        let geod3 = Vector3::new(0.0, 90.0, 0.0);
        let ecef3 = position_geodetic_to_ecef(geod3, true).unwrap();

        assert_abs_diff_eq!(ecef3[0], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef3[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef3[2], WGS84_A * (1.0 - WGS84_F), epsilon = tol);

        // Test two-input format
        let geod = Vector3::new(0.0, 0.0, 0.0);
        let ecef = position_geodetic_to_ecef(geod, false).unwrap();

        assert_abs_diff_eq!(ecef[0], WGS84_A, epsilon = tol);
        assert_abs_diff_eq!(ecef[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef[2], 0.0, epsilon = tol);

        let geod = Vector3::new(90.0, 0.0, 0.0);
        let ecef = position_geodetic_to_ecef(geod, true).unwrap();

        assert_abs_diff_eq!(ecef[0], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef[1], WGS84_A, epsilon = tol);
        assert_abs_diff_eq!(ecef[2], 0.0, epsilon = tol);

        let geod = Vector3::new(0.0, 90.0, 0.0);
        let ecef = position_geodetic_to_ecef(geod, true).unwrap();

        assert_abs_diff_eq!(ecef[0], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(ecef[2], WGS84_A * (1.0 - WGS84_F), epsilon = tol);

        // Test circularity
        let geod4 = position_ecef_to_geodetic(ecef1, true);
        let geod5 = position_ecef_to_geodetic(ecef2, true);
        let geod6 = position_ecef_to_geodetic(ecef3, true);

        assert_abs_diff_eq!(geod4[0], geod1[0], epsilon = tol);
        assert_abs_diff_eq!(geod4[1], geod1[1], epsilon = tol);
        assert_abs_diff_eq!(geod4[2], geod1[2], epsilon = tol);

        assert_abs_diff_eq!(geod5[0], geod2[0], epsilon = tol);
        assert_abs_diff_eq!(geod5[1], geod2[1], epsilon = tol);
        assert_abs_diff_eq!(geod5[2], geod2[2], epsilon = tol);

        assert_abs_diff_eq!(geod6[0], geod3[0], epsilon = tol);
        assert_abs_diff_eq!(geod6[1], geod3[1], epsilon = tol);
        assert_abs_diff_eq!(geod6[2], geod3[2], epsilon = tol);

        // Random point circularity
        let geod = Vector3::new(77.875000, 20.975200, 0.000000);
        let ecef = position_geodetic_to_ecef(geod, true).unwrap();
        let geodd = position_ecef_to_geodetic(ecef, true);
        assert_abs_diff_eq!(geod[0], geodd[0], epsilon = tol);
        assert_abs_diff_eq!(geod[1], geodd[1], epsilon = tol);
        assert_abs_diff_eq!(geod[2], geodd[2], epsilon = tol);

        assert!(position_geodetic_to_ecef(Vector3::new(0.0, 90.1, 0.0), true).is_err());

        assert!(position_geodetic_to_ecef(Vector3::new(0.0, -90.1, 0.0), true).is_err());
    }

    #[test]
    fn test_rotation_ellipsoid_to_enz() {
        // Epsilon Tolerance
        let tol = f64::EPSILON;

        // Test aligned coordinates
        let x_sta = Vector3::new(0.0, 0.0, 0.0);
        let rot1 = rotation_ellipsoid_to_enz(x_sta, true);

        // ECEF input X - [1, 0, 0] - Expected output is ENZ Z-dir
        assert_abs_diff_eq!(rot1[(0, 0)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 0)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 0)], 1.0, epsilon = tol);

        // ECEF input Y - [0, 1, 0] - Expected output is ENZ E-dir
        assert_abs_diff_eq!(rot1[(0, 1)], 1.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 1)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 1)], 0.0, epsilon = tol);

        // ECEF input Z - [0, 0, 1] - Expected output is ENZ N-dir
        assert_abs_diff_eq!(rot1[(0, 2)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 2)], 1.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 2)], 0.0, epsilon = tol);

        assert_abs_diff_eq!(rot1.determinant(), 1.0, epsilon = tol);

        // Test 90 degree longitude
        let x_sta = Vector3::new(90.0, 0.0, 0.0);
        let rot1 = rotation_ellipsoid_to_enz(x_sta, true);

        // ECEF input X - [1, 0, 0] - Expected output is ENZ -E-dir
        assert_abs_diff_eq!(rot1[(0, 0)], -1.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 0)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 0)], 0.0, epsilon = tol);

        // ECEF input Y - [0, 1, 0] - Expected output is ENZ Z-dir
        assert_abs_diff_eq!(rot1[(0, 1)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 1)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 1)], 1.0, epsilon = tol);

        // ECEF input Z - [0, 0, 1] - Expected output is ENZ N-dir
        assert_abs_diff_eq!(rot1[(0, 2)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 2)], 1.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 2)], 0.0, epsilon = tol);

        assert_abs_diff_eq!(rot1.determinant(), 1.0, epsilon = tol);

        // Test 90 degree latitude
        let x_sta = Vector3::new(00.0, 90.0, 0.0);
        let rot1 = rotation_ellipsoid_to_enz(x_sta, true);

        // ECEF input X - [1, 0, 0] - Expected output is ENZ -N-dir
        assert_abs_diff_eq!(rot1[(0, 0)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 0)], -1.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 0)], 0.0, epsilon = tol);

        // ECEF input Y - [0, 1, 0] - Expected output is ENZ E-dir
        assert_abs_diff_eq!(rot1[(0, 1)], 1.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 1)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 1)], 0.0, epsilon = tol);

        // ECEF input Z - [0, 0, 1] - Expected output is ENZ Z-dir
        assert_abs_diff_eq!(rot1[(0, 2)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 2)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 2)], 1.0, epsilon = tol);

        assert_abs_diff_eq!(rot1.determinant(), 1.0, epsilon = tol);
    }

    #[test]
    fn test_rotation_enz_to_ellipsoid() {
        let tol = f64::EPSILON;

        let x_sta = Vector3::new(42.1, 53.9, 100.0);
        let rot = rotation_ellipsoid_to_enz(x_sta, true);
        let rot_t = rotation_enz_to_ellipsoid(x_sta, true);

        let r = rot * rot_t;

        // Confirm identity
        assert_abs_diff_eq!(r[(0, 0)], 1.0, epsilon = tol);
        assert_abs_diff_eq!(r[(0, 1)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r[(0, 2)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r[(1, 0)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r[(1, 1)], 1.0, epsilon = tol);
        assert_abs_diff_eq!(r[(1, 2)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r[(2, 0)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r[(2, 1)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r[(2, 2)], 1.0, epsilon = tol);
    }

    #[test]
    fn test_relative_position_ecef_to_enz() {
        let tol = f64::EPSILON;

        // 100m Overhead
        let x_sta = Vector3::new(R_EARTH, 0.0, 0.0);
        let r_ecef = Vector3::new(R_EARTH + 100.0, 0.0, 0.0);

        let r_enz =
            relative_position_ecef_to_enz(x_sta, r_ecef, EllipsoidalConversionType::Geocentric);

        assert_abs_diff_eq!(r_enz[0], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r_enz[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r_enz[2], 100.0, epsilon = tol);

        // 100m North
        let x_sta = Vector3::new(R_EARTH, 0.0, 0.0);
        let r_ecef = Vector3::new(R_EARTH, 0.0, 100.0);

        let r_enz =
            relative_position_ecef_to_enz(x_sta, r_ecef, EllipsoidalConversionType::Geocentric);

        assert_abs_diff_eq!(r_enz[0], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r_enz[1], 100.0, epsilon = tol);
        assert_abs_diff_eq!(r_enz[2], 0.0, epsilon = tol);

        // 100m East
        let x_sta = Vector3::new(R_EARTH, 0.0, 0.0);
        let r_ecef = Vector3::new(R_EARTH, 100.0, 0.0);

        let r_enz =
            relative_position_ecef_to_enz(x_sta, r_ecef, EllipsoidalConversionType::Geocentric);

        assert_abs_diff_eq!(r_enz[0], 100.0, epsilon = tol);
        assert_abs_diff_eq!(r_enz[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r_enz[2], 0.0, epsilon = tol);

        // Confirm higher latitude and longitude is (+E, +N, -Z)
        let x_sta = Vector3::new(R_EARTH, 0.0, 0.0);
        let x_geoc = Vector3::new(0.5, 0.5, 0.0);
        let r_ecef = position_geocentric_to_ecef(x_geoc, true).unwrap();

        let r_enz_geoc =
            relative_position_ecef_to_enz(x_sta, r_ecef, EllipsoidalConversionType::Geocentric);

        assert!(r_enz_geoc[0] > 0.0);
        assert!(r_enz_geoc[1] > 0.0);
        assert!(r_enz_geoc[2] < 0.0);

        // Confirm difference in geocentric and geodetic conversions
        let x_sta = Vector3::new(R_EARTH, 0.0, 0.0);
        let x_geod = Vector3::new(0.5, 0.5, 0.0);
        let r_ecef = position_geodetic_to_ecef(x_geod, true).unwrap();

        let r_enz_geod =
            relative_position_ecef_to_enz(x_sta, r_ecef, EllipsoidalConversionType::Geodetic);

        assert!(r_enz_geod[0] > 0.0);
        assert!(r_enz_geod[1] > 0.0);
        assert!(r_enz_geod[2] < 0.0);

        for i in 0..3 {
            assert_ne!(r_enz_geoc[i], r_enz_geod[i]);
        }
    }

    #[test]
    fn test_relative_position_enz_to_ecef() {
        let tol = f64::EPSILON;

        let x_sta = Vector3::new(R_EARTH, 0.0, 0.0);
        let r_enz = Vector3::new(0.0, 0.0, 100.0);

        let r_ecef =
            relative_position_enz_to_ecef(x_sta, r_enz, EllipsoidalConversionType::Geodetic);

        assert_abs_diff_eq!(r_ecef[0], R_EARTH + 100.0, epsilon = tol);
        assert_abs_diff_eq!(r_ecef[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r_ecef[2], 0.0, epsilon = tol);
    }

    #[test]
    fn test_rotation_ellipsoid_to_sez() {
        // Epsilon Tolerance
        let tol = f64::EPSILON;

        // Test aligned coordinates
        let x_sta = Vector3::new(0.0, 0.0, 0.0);
        let rot1 = rotation_ellipsoid_to_sez(x_sta, true);

        // ECEF input X - [1, 0, 0] - Expected output is ENZ Z-dir
        assert_abs_diff_eq!(rot1[(0, 0)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 0)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 0)], 1.0, epsilon = tol);

        // ECEF input Y - [0, 1, 0] - Expected output is ENZ E-dir
        assert_abs_diff_eq!(rot1[(0, 1)], 1.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 1)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 1)], 0.0, epsilon = tol);

        // ECEF input Z - [0, 0, 1] - Expected output is ENZ N-dir
        assert_abs_diff_eq!(rot1[(0, 2)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 2)], 1.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 2)], 0.0, epsilon = tol);

        assert_abs_diff_eq!(rot1.determinant(), 1.0, epsilon = tol);

        // Test 90 degree longitude
        let x_sta = Vector3::new(90.0, 0.0, 0.0);
        let rot1 = rotation_ellipsoid_to_sez(x_sta, true);

        // ECEF input X - [1, 0, 0] - Expected output is ENZ -E-dir
        assert_abs_diff_eq!(rot1[(0, 0)], -1.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 0)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 0)], 0.0, epsilon = tol);

        // ECEF input Y - [0, 1, 0] - Expected output is ENZ Z-dir
        assert_abs_diff_eq!(rot1[(0, 1)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 1)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 1)], 1.0, epsilon = tol);

        // ECEF input Z - [0, 0, 1] - Expected output is ENZ N-dir
        assert_abs_diff_eq!(rot1[(0, 2)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 2)], 1.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 2)], 0.0, epsilon = tol);

        assert_abs_diff_eq!(rot1.determinant(), 1.0, epsilon = tol);

        // Test 90 degree latitude
        let x_sta = Vector3::new(00.0, 90.0, 0.0);
        let rot1 = rotation_ellipsoid_to_sez(x_sta, true);

        // ECEF input X - [1, 0, 0] - Expected output is ENZ -N-dir
        assert_abs_diff_eq!(rot1[(0, 0)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 0)], -1.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 0)], 0.0, epsilon = tol);

        // ECEF input Y - [0, 1, 0] - Expected output is ENZ E-dir
        assert_abs_diff_eq!(rot1[(0, 1)], 1.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 1)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 1)], 0.0, epsilon = tol);

        // ECEF input Z - [0, 0, 1] - Expected output is ENZ Z-dir
        assert_abs_diff_eq!(rot1[(0, 2)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(1, 2)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(rot1[(2, 2)], 1.0, epsilon = tol);

        assert_abs_diff_eq!(rot1.determinant(), 1.0, epsilon = tol);
    }

    #[test]
    fn test_rotation_sez_to_ellipsoid() {
        let tol = f64::EPSILON;

        let x_sta = Vector3::new(42.1, 53.9, 100.0);
        let rot = rotation_ellipsoid_to_sez(x_sta, true);
        let rot_t = rotation_sez_to_ellipsoid(x_sta, true);

        let r = rot * rot_t;

        // Confirm identity
        assert_abs_diff_eq!(r[(0, 0)], 1.0, epsilon = tol);
        assert_abs_diff_eq!(r[(0, 1)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r[(0, 2)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r[(1, 0)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r[(1, 1)], 1.0, epsilon = tol);
        assert_abs_diff_eq!(r[(1, 2)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r[(2, 0)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r[(2, 1)], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r[(2, 2)], 1.0, epsilon = tol);
    }

    #[test]
    fn test_relative_position_ecef_to_sez() {
        let tol = f64::EPSILON;

        // 100m Overhead
        let x_sta = Vector3::new(R_EARTH, 0.0, 0.0);
        let r_ecef = Vector3::new(R_EARTH + 100.0, 0.0, 0.0);

        let r_sez =
            relative_position_ecef_to_sez(x_sta, r_ecef, EllipsoidalConversionType::Geocentric);

        assert_abs_diff_eq!(r_sez[0], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r_sez[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r_sez[2], 100.0, epsilon = tol);

        // 100m North
        let x_sta = Vector3::new(R_EARTH, 0.0, 0.0);
        let r_ecef = Vector3::new(R_EARTH, 0.0, 100.0);

        let r_sez =
            relative_position_ecef_to_sez(x_sta, r_ecef, EllipsoidalConversionType::Geocentric);

        assert_abs_diff_eq!(r_sez[0], -100.0, epsilon = tol);
        assert_abs_diff_eq!(r_sez[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r_sez[2], 0.0, epsilon = tol);

        // 100m East
        let x_sta = Vector3::new(R_EARTH, 0.0, 0.0);
        let r_ecef = Vector3::new(R_EARTH, 100.0, 0.0);

        let r_sez =
            relative_position_ecef_to_sez(x_sta, r_ecef, EllipsoidalConversionType::Geocentric);

        assert_abs_diff_eq!(r_sez[0], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r_sez[1], 100.0, epsilon = tol);
        assert_abs_diff_eq!(r_sez[2], 0.0, epsilon = tol);

        // Confirm higher latitude and longitude is (+E, +N, -Z)
        let x_sta = Vector3::new(R_EARTH, 0.0, 0.0);
        let x_geoc = Vector3::new(0.5, 0.5, 0.0);
        let r_ecef = position_geocentric_to_ecef(x_geoc, true).unwrap();

        let r_sez_geoc =
            relative_position_ecef_to_sez(x_sta, r_ecef, EllipsoidalConversionType::Geocentric);

        assert!(r_sez_geoc[0] < 0.0);
        assert!(r_sez_geoc[1] > 0.0);
        assert!(r_sez_geoc[2] < 0.0);

        // Confirm difference in geocentric and geodetic conversions
        let x_sta = Vector3::new(R_EARTH, 0.0, 0.0);
        let x_geod = Vector3::new(0.5, 0.5, 0.0);
        let r_ecef = position_geodetic_to_ecef(x_geod, true).unwrap();

        let r_sez_geod =
            relative_position_ecef_to_sez(x_sta, r_ecef, EllipsoidalConversionType::Geodetic);

        assert!(r_sez_geod[0] < 0.0);
        assert!(r_sez_geod[1] > 0.0);
        assert!(r_sez_geod[2] < 0.0);

        for i in 0..3 {
            assert_ne!(r_sez_geoc[i], r_sez_geod[i]);
        }
    }

    #[test]
    fn test_relative_position_sez_to_ecef() {
        let tol = f64::EPSILON;

        let x_sta = Vector3::new(R_EARTH, 0.0, 0.0);
        let r_sez = Vector3::new(0.0, 0.0, 100.0);

        let r_ecef =
            relative_position_sez_to_ecef(x_sta, r_sez, EllipsoidalConversionType::Geodetic);

        assert_abs_diff_eq!(r_ecef[0], R_EARTH + 100.0, epsilon = tol);
        assert_abs_diff_eq!(r_ecef[1], 0.0, epsilon = tol);
        assert_abs_diff_eq!(r_ecef[2], 0.0, epsilon = tol);
    }
}
