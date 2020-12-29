use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rastro::orbits as orbits;


#[pyfunction]
#[text_signature = "(a)"]
fn orbital_period(a: f64) -> PyResult<f64> {
    Ok(orbits::orbital_period(a))
}

#[pyfunction]
#[text_signature = "(a, gm)"]
fn orbital_period_general(a: f64, gm: f64) -> PyResult<f64> {
    Ok(orbits::orbital_period_general(a, gm))
}

#[pyfunction(as_degrees="true")]
#[text_signature = "(a, as_degrees)"]
fn mean_motion(a: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::mean_motion(a, as_degrees))
}

#[pyfunction(as_degrees="true")]
#[text_signature = "(a, gm, as_degrees)"]
fn mean_motion_general(a: f64, gm: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::mean_motion_general(a, gm, as_degrees))
}

#[pyfunction(as_degrees="true")]
#[text_signature = "(a, as_degrees)"]
fn semimajor_axis(n: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::semimajor_axis(n, as_degrees))
}

#[pyfunction(as_degrees="true")]
#[text_signature = "(a, gm, as_degrees)"]
fn semimajor_axis_general(n: f64, gm: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::semimajor_axis_general(n, gm, as_degrees))
}

#[pymodule]
pub fn orbits(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(orbital_period, module)?)?;
    module.add_function(wrap_pyfunction!(orbital_period_general, module)?)?;
    module.add_function(wrap_pyfunction!(mean_motion, module)?)?;
    module.add_function(wrap_pyfunction!(mean_motion_general, module)?)?;
    module.add_function(wrap_pyfunction!(semimajor_axis, module)?)?;
    module.add_function(wrap_pyfunction!(semimajor_axis_general, module)?)?;


    Ok(())
}