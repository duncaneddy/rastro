use pyo3::prelude::*;
// use pyo3::{wrap_pymodule};

pub mod constants;
pub mod orbits;
pub mod eop;

#[pymodule]
fn rastro(py: Python, module: &PyModule) -> PyResult<()> {
    // Initialize Constants Submodule
    let constants_submodule = PyModule::new(py, "constants")?;
    constants::constants(py, constants_submodule)?;
    module.add_submodule(constants_submodule)?;

    // Initialize Orbits Submodule
    let orbits_submodule = PyModule::new(py, "orbits")?;
    orbits::orbits(py, orbits_submodule)?;
    module.add_submodule(orbits_submodule)?;

    // Initialize Orbits Submodule
    let eop_submodule = PyModule::new(py, "eop")?;
    eop::eop(py, eop_submodule)?;
    module.add_submodule(eop_submodule)?;

    // Fix imports
    // py.import("sys")?
    //     .getattr("modules")?
    //     .set_item("rastro.constants", constants_submodule)?;
    // py.import("sys")?
    //     .getattr("modules")?
    //     .set_item("rastro.orbits", orbits_submodule)?;
    // py.import("sys")?
    //     .getattr("modules")?
    //     .set_item("rastro.eop", eop_submodule)?;

    Ok(())
}