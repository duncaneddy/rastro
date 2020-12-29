use pyo3::prelude::*;
use rastro::constants;

#[pymodule]
pub fn constants(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add("DEG2RAD", constants::DEG2RAD)?;
    module.add("RAD2DEG", constants::RAD2DEG)?;
    module.add("AS2RAD", constants::AS2RAD)?;
    module.add("RAD2AS", constants::RAD2AS)?;
    module.add("MJD_ZERO", constants::MJD_ZERO)?;
    module.add("MJD2000", constants::MJD2000)?;
    module.add("GPS_TAI", constants::GPS_TAI)?;
    module.add("TAI_GPS", constants::TAI_GPS)?;
    module.add("TT_TAI", constants::TT_TAI)?;
    module.add("TAI_TT", constants::TAI_TT)?;
    module.add("GPS_TT", constants::GPS_TT)?;
    module.add("TT_GPS", constants::TT_GPS)?;
    module.add("GPS_ZERO", constants::GPS_ZERO)?;
    module.add("C_LIGHT", constants::C_LIGHT)?;
    module.add("AU", constants::AU)?;
    module.add("R_EARTH", constants::R_EARTH)?;
    module.add("WGS84_A", constants::WGS84_A)?;
    module.add("WGS84_F", constants::WGS84_F)?;
    module.add("GM_EARTH", constants::GM_EARTH)?;
    module.add("ECC_EARTH", constants::ECC_EARTH)?;
    module.add("J2_EARTH", constants::J2_EARTH)?;
    module.add("OMEGA_EARTH", constants::OMEGA_EARTH)?;
    module.add("GM_SUN", constants::GM_SUN)?;
    module.add("R_SUN", constants::R_SUN)?;
    module.add("P_SUN", constants::P_SUN)?;
    module.add("R_MOON", constants::R_MOON)?;
    module.add("GM_MOON", constants::GM_MOON)?;
    module.add("GM_MERCURY", constants::GM_MERCURY)?;
    module.add("GM_VENUS", constants::GM_VENUS)?;
    module.add("GM_MARS", constants::GM_MARS)?;
    module.add("GM_JUPITER", constants::GM_JUPITER)?;
    module.add("GM_SATURN", constants::GM_SATURN)?;
    module.add("GM_URANUS", constants::GM_URANUS)?;
    module.add("GM_NEPTUNE", constants::GM_NEPTUNE)?;
    module.add("GM_PLUTO", constants::GM_PLUTO)?;

    Ok(())
}
