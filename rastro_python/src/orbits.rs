use pyo3::{exceptions::PyRuntimeError, prelude::*};
use pyo3::wrap_pyfunction;
use rastro::orbits as orbits;


/// Computes the orbital period of an object around Earth.
///
/// Uses rastro.constants.GM_EARTH as the standard gravitational parameter for the calculation.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: [m]
///
/// Returns:
///     period (`float`): The orbital period of the astronomical object. Units: [s]
#[pyfunction]
#[text_signature = "(a)"]
fn orbital_period(a: f64) -> PyResult<f64> {
    Ok(orbits::orbital_period(a))
}

/// Computes the orbital period of an astronomical object around a general body.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: [m]
///     gm (`float`): The standard gravitational parameter of primary body. Units: [m^3/s^2]
///
/// Returns:
///     period (`float`): The orbital period of the astronomical object. Units: [s]
#[pyfunction]
#[text_signature = "(a, gm)"]
fn orbital_period_general(a: f64, gm: f64) -> PyResult<f64> {
    Ok(orbits::orbital_period_general(a, gm))
}

/// Computes the mean motion of an astronomical object around Earth.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: [m]
///     as_degrees (`bool`): Return output in degrees instead of radians
///
/// Returns:
///     n (`float`): The mean motion of the astronomical object. Units: [rad] or [deg]
#[pyfunction(as_degrees="true")]
#[text_signature = "(a, as_degrees)"]
fn mean_motion(a: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::mean_motion(a, as_degrees))
}

/// Computes the mean motion of an astronomical object around a general body
/// given a semi-major axis.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: [m]
///     gm (`float`): The standard gravitational parameter of primary body. Units: [m^3/s^2]
///     as_degrees (`bool`): Return output in degrees instead of radians
///
/// Returns:
///     n (`float`): The mean motion of the astronomical object. Units: [rad] or [deg]
#[pyfunction(as_degrees="true")]
#[text_signature = "(a, gm, as_degrees)"]
fn mean_motion_general(a: f64, gm: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::mean_motion_general(a, gm, as_degrees))
}

/// Computes the semi-major axis of an astronomical object from Earth
/// given the object's mean motion.
///
/// Arguments:
///     n (`float`): The mean motion of the astronomical object. Units: [rad] or [deg]
///     as_degrees (`bool`): Interpret mean motion as degrees if `true` or radians if `false`
///
/// Returns:
///     a (`float`): The semi-major axis of the astronomical object. Units: [m]
#[pyfunction(as_degrees="true")]
#[text_signature = "(a, as_degrees)"]
fn semimajor_axis(n: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::semimajor_axis(n, as_degrees))
}

/// Computes the semi-major axis of an astronomical object from a general body
/// given the object's mean motion.
///
/// Arguments:
///     n (`float`): The mean motion of the astronomical object. Units: [rad] or [deg]
///     gm (`float`): The standard gravitational parameter of primary body. Units: [m^3/s^2]
///     as_degrees (`float`): Interpret mean motion as degrees if `true` or radians if `false`
///
/// Returns:
///     a (`float`): The semi-major axis of the astronomical object. Units: [m]
#[pyfunction(as_degrees="true")]
#[text_signature = "(a, gm, as_degrees)"]
fn semimajor_axis_general(n: f64, gm: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::semimajor_axis_general(n, gm, as_degrees))
}

/// Computes the perigee velocity of an astronomical object around Earth.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: [m]
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///
/// Returns:
///     v (`float`): The magnitude of velocity of the object at perigee. Units: [m/s]
#[pyfunction]
#[text_signature = "(a, e)"]
fn perigee_velocity(a: f64, e: f64) -> PyResult<f64> {
    Ok(orbits::perigee_velocity(a, e))
}

/// Computes the periapsis velocity of an astronomical object around a general body.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: [m]
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     gm (`float`): The standard gravitational parameter of primary body. Units: [m^3/s^2]
///
/// Returns:
///     v (`float`): The magnitude of velocity of the object at periapsis. Units: [m/s]
#[pyfunction]
#[text_signature = "(a, e)"]
fn periapsis_velocity(a: f64, e: f64, gm: f64) -> PyResult<f64> {
    Ok(orbits::periapsis_velocity(a, e, gm))
}

/// Computes the apogee velocity of an astronomical object around Earth.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: [m]
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///
/// Returns:
///     v (`float`): The magnitude of velocity of the object at apogee. Units: [m/s]
#[pyfunction]
#[text_signature = "(a, e)"]
fn apogee_velocity(a: f64, e: f64) -> PyResult<f64> {
    Ok(orbits::apogee_velocity(a, e))
}

/// Computes the apoapsis velocity of an astronomical object around a general body.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: [m]
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     gm (`float`): The standard gravitational parameter of primary body. Units: [m^3/s^2]
///
/// Returns:
///     v (`float`): The magnitude of velocity of the object at apoapsis. Units: [m/s]
#[pyfunction]
#[text_signature = "(a, e)"]
fn apoapsis_velocity(a: f64, e: f64, gm: f64) -> PyResult<f64> {
    Ok(orbits::apoapsis_velocity(a, e, gm))
}

/// Computes the inclination for a Sun-synchronous orbit around Earth based on
/// the J2 gravitational perturbation.
///
/// Arguments:
///     a (`float`) The semi-major axis of the astronomical object. Units: [m]
///     e (`float`) The eccentricity of the astronomical object's orbit. Dimensionless
///     as_degrees (`bool`) Return output in degrees instead of radians
///
/// Returns:
///     inc (`float`) Inclination for a Sun synchronous orbit. Units: [deg] or [rad]
#[pyfunction]
#[text_signature = "(a, e, as_degrees)"]
fn sun_synchronous_inclination(a: f64, e: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::sun_synchronous_inclination(a, e, as_degrees))
}

/// Converts eccentric anomaly into mean anomaly.
///
/// Arguments:
///     anm_ecc (`float`): Eccentric anomaly. Units: [rad] or [deg]
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     as_degrees (`bool`): Interprets input and returns output in [deg] if `true` or [rad] if `false`
///
/// Returns:
///     anm_mean (`float`): Mean anomaly. Units: [rad] or [deg]
#[pyfunction]
#[text_signature = "(anm_ecc, e, as_degrees)"]
fn anomaly_eccentric_to_mean(anm_ecc: f64, e: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::anomaly_eccentric_to_mean(anm_ecc, e, as_degrees))
}

/// Converts mean anomaly into eccentric anomaly
///
/// Arguments:
///     anm_mean (`float`): Mean anomaly. Units: [rad] or [deg]
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     as_degrees (`float`): Interprets input and returns output in [deg] if `true` or [rad] if `false`
///
/// Returns:
///     anm_ecc (`float`): Eccentric anomaly. Units: [rad] or [deg]
#[pyfunction]
#[text_signature = "(anm_mean, e, as_degrees)"]
fn anomaly_mean_to_eccentric(anm_mean: f64, e: f64, as_degrees: bool) -> PyResult<f64> {
    let res = orbits::anomaly_mean_to_eccentric(anm_mean, e, as_degrees);
    if res.is_ok() {
        Ok(res.unwrap())
    } else {
        Err(PyRuntimeError::new_err(res.err().unwrap()))
    }
}

/// Converts true anomaly into eccentric anomaly
///
/// Arguments:
///     anm_true (`float`): true anomaly. Units: [rad] or [deg]
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     as_degrees (`bool`): Interprets input and returns output in [deg] if `true` or [rad] if `false`
///
/// Returns:
///     anm_ecc (`float`): Eccentric anomaly. Units: [rad] or [deg]
#[pyfunction]
#[text_signature = "(anm_true, e, as_degrees))"]
fn anomaly_true_to_eccentric(anm_true: f64, e: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::anomaly_true_to_eccentric(anm_true, e, as_degrees))
}

/// Converts eccentric anomaly into true anomaly
///
/// # Arguments
///     anm_ecc (`float`): Eccentric anomaly. Units: [rad] or [deg]
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     as_degrees (`bool`): Interprets input and returns output in [deg] if `true` or [rad] if `false`
///
/// # Returns
///     anm_true (`float`): true anomaly. Units: [rad] or [deg]
#[pyfunction]
#[text_signature = "(anm_ecc, e, as_degrees))"]
fn anomaly_eccentric_to_true(anm_ecc: f64, e: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::anomaly_eccentric_to_true(anm_ecc, e, as_degrees))
}

#[pymodule]
pub fn orbits(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(orbital_period, module)?)?;
    module.add_function(wrap_pyfunction!(orbital_period_general, module)?)?;
    module.add_function(wrap_pyfunction!(mean_motion, module)?)?;
    module.add_function(wrap_pyfunction!(mean_motion_general, module)?)?;
    module.add_function(wrap_pyfunction!(semimajor_axis, module)?)?;
    module.add_function(wrap_pyfunction!(semimajor_axis_general, module)?)?;
    module.add_function(wrap_pyfunction!(perigee_velocity, module)?)?;
    module.add_function(wrap_pyfunction!(periapsis_velocity, module)?)?;
    module.add_function(wrap_pyfunction!(apogee_velocity, module)?)?;
    module.add_function(wrap_pyfunction!(apoapsis_velocity, module)?)?;
    module.add_function(wrap_pyfunction!(sun_synchronous_inclination, module)?)?;
    module.add_function(wrap_pyfunction!(anomaly_eccentric_to_mean, module)?)?;
    module.add_function(wrap_pyfunction!(anomaly_mean_to_eccentric, module)?)?;
    module.add_function(wrap_pyfunction!(anomaly_true_to_eccentric, module)?)?;
    module.add_function(wrap_pyfunction!(anomaly_eccentric_to_true, module)?)?;

    Ok(())
}