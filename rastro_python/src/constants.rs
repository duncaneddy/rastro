use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use rastro::constants;

#[pymodule]
pub fn constants(py: Python, module: &PyModule) -> PyResult<()> {
    module.add("DEG2RAD", rastro::constants::DEG2RAD)?;
    module.add("RAD2DEG", rastro::constants::RAD2DEG)?;
    module.add("AS2RAD", rastro::constants::AS2RAD)?;
    module.add("RAD2AS", rastro::constants::RAD2AS)?;
    module.add("MJD_ZERO", rastro::constants::MJD_ZERO)?;
    module.add("MJD2000", rastro::constants::MJD2000)?;
    module.add("GPS_TAI", rastro::constants::GPS_TAI)?;
    module.add("TAI_GPS", rastro::constants::TAI_GPS)?;
    module.add("TT_TAI", rastro::constants::TT_TAI)?;
    module.add("TAI_TT", rastro::constants::TAI_TT)?;
    module.add("GPS_TT", rastro::constants::GPS_TT)?;
    module.add("TT_GPS", rastro::constants::TT_GPS)?;
    module.add("GPS_ZERO", rastro::constants::GPS_ZERO)?;
    module.add("C_LIGHT", rastro::constants::C_LIGHT)?;
    module.add("AU", rastro::constants::AU)?;
    module.add("R_EARTH", rastro::constants::R_EARTH)?;
    module.add("WGS84_A", rastro::constants::WGS84_A)?;
    module.add("WGS84_F", rastro::constants::WGS84_F)?;
    module.add("GM_EARTH", rastro::constants::GM_EARTH)?;
    module.add("ECC_EARTH", rastro::constants::ECC_EARTH)?;
    module.add("J2_EARTH", rastro::constants::J2_EARTH)?;
    module.add("OMEGA_EARTH", rastro::constants::OMEGA_EARTH)?;
    module.add("GM_SUN", rastro::constants::GM_SUN)?;
    module.add("R_SUN", rastro::constants::R_SUN)?;
    module.add("P_SUN", rastro::constants::P_SUN)?;
    module.add("GM_MOON", rastro::constants::GM_MOON)?;
    module.add("GM_MERCURY", rastro::constants::GM_MERCURY)?;
    module.add("GM_VENUS", rastro::constants::GM_VENUS)?;
    module.add("GM_MARS", rastro::constants::GM_MARS)?;
    module.add("GM_JUPITER", rastro::constants::GM_JUPITER)?;
    module.add("GM_SATURN", rastro::constants::GM_SATURN)?;
    module.add("GM_URANUS", rastro::constants::GM_URANUS)?;
    module.add("GM_NEPTUNE", rastro::constants::GM_NEPTUNE)?;
    module.add("GM_PLUTO", rastro::constants::GM_PLUTO)?;

    Ok(())
}
