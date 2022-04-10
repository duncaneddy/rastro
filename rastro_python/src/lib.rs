/// This is the wrapper for the rastro script.
///
/// It is currently all in one file because of the PyO3 issues discussed in
/// https://github.com/PyO3/pyo3/issues/1444 which prevents sharing rust-defined
/// PyO3 classes between different modules. This leads to a need to keep the
/// entire wrapper in a single file until this is addressed.
///
/// While unfortunate, that's where we are at.

use pyo3::{wrap_pyfunction, exceptions::PyRuntimeError};
use pyo3::prelude::*;
use rastro::{constants, eop, time, orbits};

// TODO: Remove test module when unneeded
pub mod test;

////////////////
//  Consants  //
////////////////

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


/////////////////////////
//  Earth Orientation  //
/////////////////////////

/// Stores Rust instance of EarthOrientationData
///
/// The structure assumes the input data uses the IAU 2010/2000A conventions. That is the
/// precession/nutation parameter values are in terms of `dX` and `dY`, not `dPsi` and `dEps`.
///
/// Loaded Earth orientation data is stored as a dictionary with structure key and values as
/// follows. The data cannot be accessed directly but only through specific retrieval methods.
///
/// Key:
/// - `mjd`: Modified Julian date of the parameter values
///
/// Values:
/// - `pm_x`: x-component of polar motion correction. Units: (radians)
/// - `pm_y`: y-component of polar motion correction. Units: (radians)
/// - `ut1_utc`: Offset of UT1 time scale from UTC time scale. Units: (seconds)
/// - `dX`: "X" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
/// - `dY`: "Y" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
/// - `lod`: Difference between astronomically determined length of day and 86400 second TAI.Units: (seconds)
///   day. Units: (seconds)
#[pyclass]
#[derive(Clone,Debug)]
pub struct EarthOrientationData {
    /// Stored object for underlying EOP
    pub obj: eop::EarthOrientationData,
}

#[pymethods]
impl EarthOrientationData {

    fn __repr__(&self) -> String {
        format!("EarthOrientationData<type: {}, {} entries, mjd_min: {}, mjd_max: {},  \
        mjd_last_lod: \
        {}, mjd_last_dxdy: {}, extrapolate: {}, \
        interpolate: {}>", self.obj.eop_type, self.obj.data.len(), self.obj.mjd_min, self.obj
            .mjd_max,
                self.obj.mjd_last_lod, self.obj.mjd_last_dxdy, self.obj.extrapolate, self.obj.interpolate)
    }

    fn __str__(&self) -> String {
        format!("EarthOrientationData<type: {}, {} entries, mjd_min: {}, mjd_max: {},  \
        mjd_last_lod: \
        {}, mjd_last_dxdy: {}, extrapolate: {}, \
        interpolate: {}>", self.obj.eop_type, self.obj.data.len(), self.obj.mjd_min, self.obj.mjd_max,
                self.obj.mjd_last_lod, self.obj.mjd_last_dxdy, self.obj.extrapolate, self.obj.interpolate)
    }

    // Define attribute access methods
    /// `str`: Type of Earth orientation data loaded. Can be "C04", "StandardBulletinA", or "StandardBulletinB"
    #[getter]
    fn eop_type(&self) -> String {
        match self.obj.eop_type {
            eop::EOPType::C04 => String::from("C04"),
            eop::EOPType::StandardBulletinA => String::from("StandardBulletinA"),
            eop::EOPType::StandardBulletinB => String::from("StandardBulletinB")
        }
    }

    #[getter]
    /// `str`: Extrapolation setting. Can be "Zero", "Hold", or "Error"
    fn extrapolate(&self) -> String {
        match self.obj.extrapolate {
            eop::EOPExtrapolation::Zero => String::from("Zero"),
            eop::EOPExtrapolation::Hold => String::from("Hold"),
            eop::EOPExtrapolation::Error => String::from("Error")
        }
    }

    /// `bool`: Whether to interpolate for time between loaded data set
    #[getter]
    fn interpolate(&self) -> bool {
        self.obj.interpolate
    }

    /// mjd_min (`float`): Minimum date of stored data. This is the value of the smallest
    /// accessible date
    #[getter]
    fn mjd_min(&self) -> u32 {
        self.obj.mjd_min
    }

    /// mjd_min (`float`): Minimum date of stored data. This is the value of the largest
    /// accessible date
    #[getter]
    fn mjd_max(&self) -> u32 {
        self.obj.mjd_max
    }

    /// mjd_last_lod (`float`): Minimum date of stored data. This is the value of the last date
    /// which will return a LOD value not given by the `extrapolate` value
    #[getter]
    fn mjd_last_lod(&self) -> u32 {
        self.obj.mjd_last_lod
    }

    /// mjd_last_lod (`float`): Minimum date of stored data. This is the value of the last date
    /// which will return dX and dY values not given by the `extrapolate` value
    #[getter]
    fn mjd_last_dxdy(&self) -> u32 {
        self.obj.mjd_last_dxdy
    }

    /// Return length of stored data array
    ///
    /// Returns:
    ///     len (`int`): Number of data points in loaded Earth orientation data file.
    fn len(&self) -> usize {
        self.obj.data.len()
    }

    /// Load C04 Earth orientation data from file.
    ///
    /// Takes a path to a given file which will be read on the assumption that it is an Earth
    /// orientation parameter data file formatted according to [IERS C04 formatting standards](https://www.iers.org/IERS/EN/DataProducts/EarthOrientationData/eop.html)
    ///
    /// Args:
    ///     filepath (`str`): Path of input data file
    ///     extrapolate (`str`): Set EOP Extrapolation behavior for resulting EarthOrientationData
    /// object. Can be `"Zero"`, `"Hold"`, or `"Error"`.
    ///     interpolate (`bool`): Set EOP interpolation behavior for resulting EarthOrientationData
    /// object.
    ///
    /// Returns:
    ///     `EarthOrientationData`: On successful parse returns `EarthOrientationData` object
    #[staticmethod]
    #[pyo3(text_signature = "(filepath, extrapolate, interpolate)")]
    fn from_c04_file(filepath: &str, extrapolate: &str, interpolate: bool) ->
    PyResult<EarthOrientationData> {
        let eop_extrapolate = match extrapolate.as_ref() {
            "Zero" => eop::EOPExtrapolation::Zero,
            "Hold" => eop::EOPExtrapolation::Hold,
            "Error" => eop::EOPExtrapolation::Error,
            _ => return Err(PyRuntimeError::new_err(format!("Unknown extrapolation type '{}'. Must \
            be 'Zero', 'Hold', or 'Error'", extrapolate)))
        };

        match eop::EarthOrientationData::from_c04_file(filepath.as_ref(), eop_extrapolate,
                                                       interpolate) {
            Ok(eop_obj) => Ok(EarthOrientationData{obj:eop_obj}),
            _ => Err(PyRuntimeError::new_err(format!("Error loading file as C04 EOP data: {}",
                                                     filepath.as_ref() as &str)))
        }
    }

    /// Load package-default C04 Earth orientation data.
    ///
    /// Parses the Earth orientation data packaged with the RAstro library return a valid
    /// `EarthOrientationData`.
    ///
    /// Args
    ///     extrapolate (`str`): Set EOP Extrapolation behavior for resulting EarthOrientationData
    /// object. Can be `"Zero"`, `"Hold"`, or `"Error"`.
    ///     interpolate (`bool`): Set EOP interpolation behavior for resulting EarthOrientationData
    /// object.
    ///
    /// Returns
    /// `EarthOrientationData`: Returns Earth Orientation data object
    #[staticmethod]
    #[pyo3(text_signature = "(extrapolate, interpolate)")]
    fn from_default_c04(extrapolate: &str, interpolate: bool) -> PyResult<EarthOrientationData> {
        let eop_extrapolate = match extrapolate.as_ref() {
            "Zero" => eop::EOPExtrapolation::Zero,
            "Hold" => eop::EOPExtrapolation::Hold,
            "Error" => eop::EOPExtrapolation::Error,
            _ => return Err(PyRuntimeError::new_err(format!("Unknown extrapolation type '{}'. Must \
            be 'Zero', 'Hold', or 'Error'", extrapolate)))
        };

        let eop_obj = eop::EarthOrientationData::from_default_c04(eop_extrapolate, interpolate);

        Ok(EarthOrientationData{obj:eop_obj})
    }

    /// Load standard Earth orientation data from file.
    ///
    /// Takes a path to a given file which will be read on the assumption that it is an Earth
    /// orientation parameter data file formatted according to [IERS Standard EOP Data format](https://www.iers.org/IERS/EN/DataProducts/EarthOrientationData/eop.html)
    ///
    /// Args:
    ///     filepath (`str`): Path of input data file
    ///     extrapolate (`str`): Set EOP Extrapolation behavior for resulting EarthOrientationData
    /// object. Can be `"Zero"`, `"Hold"`, or `"Error"`.
    ///     interpolate (`bool`): Set EOP interpolation behavior for resulting EarthOrientationData
    /// object.
    ///     eop_type (`str`): Type to parse data file as. Can be `"StandardBulletinA"` or `"EOPType::StandardBulletinB"`
    ///
    /// Returns:
    ///     `EarthOrientationData`: On successful parse returns `EarthOrientationData` object
    #[staticmethod]
    #[pyo3(text_signature = "(filepath, extrapolate, interpolate, eop_type)")]
    fn from_standard_file(filepath: &str, extrapolate: &str, interpolate: bool, eop_type: &str) ->
    PyResult<EarthOrientationData> {
        let eop_extrapolate = match extrapolate.as_ref() {
            "Zero" => eop::EOPExtrapolation::Zero,
            "Hold" => eop::EOPExtrapolation::Hold,
            "Error" => eop::EOPExtrapolation::Error,
            _ => return Err(PyRuntimeError::new_err(format!("Unknown extrapolation type '{}'. Must \
            be 'Zero', 'Hold', or 'Error'", extrapolate)))
        };

        let eop_type = match eop_type.as_ref() {
            "StandardBulletinA" => eop::EOPType::StandardBulletinA,
            "StandardBulletinB" => eop::EOPType::StandardBulletinB,
            _ => return Err(PyRuntimeError::new_err(format!("Unknown EOP type '{}'. Must \
                be 'StandardBulletinA' or 'StandardBulletinB'", eop_type)))
        };

        match eop::EarthOrientationData::from_standard_file(filepath.as_ref(), eop_extrapolate,
                                                            interpolate, eop_type) {
            Ok(eop_obj) => Ok(EarthOrientationData{obj:eop_obj}),
            _ => Err(PyRuntimeError::new_err(format!("Error loading file as C04 EOP data: {}",
                                                     filepath.as_ref() as &str)))
        }
    }

    /// Load package-default C04 Earth orientation data.
    ///
    /// Parses the Earth orientation data packaged with the RAstro library return a valid
    /// `EarthOrientationData`.
    ///
    /// Args:
    ///     extrapolate (`str`): Set EOP Extrapolation behavior for resulting EarthOrientationData
    /// object. Can be `"Zero"`, `"Hold"`, or `"Error"`.
    ///     interpolate (`bool`): Set EOP interpolation behavior for resulting EarthOrientationData
    /// object.
    ///     eop_type (`str`): Type to parse data file as. Can be `"StandardBulletinA"` or `"EOPType::StandardBulletinB"`
    ///
    /// Returns:
    ///     `EarthOrientationData`: On successful parse returns `EarthOrientationData` object
    #[staticmethod]
    #[pyo3(text_signature = "(extrapolate, interpolate, type)")]
    fn from_default_standard(extrapolate: &str, interpolate: bool, eop_type: &str) ->
    PyResult<EarthOrientationData> {
        let eop_extrapolate = match extrapolate.as_ref() {
            "Zero" => eop::EOPExtrapolation::Zero,
            "Hold" => eop::EOPExtrapolation::Hold,
            "Error" => eop::EOPExtrapolation::Error,
            _ => return Err(PyRuntimeError::new_err(format!("Unknown extrapolation type '{}'. Must \
            be 'Zero', 'Hold', or 'Error'", extrapolate)))
        };

        let eop_type = match eop_type.as_ref() {
            "StandardBulletinA" => eop::EOPType::StandardBulletinA,
            "StandardBulletinB" => eop::EOPType::StandardBulletinB,
            _ => return Err(PyRuntimeError::new_err(format!("Unknown EOP type '{}'. Must \
                be 'StandardBulletinA' or 'StandardBulletinB'", eop_type)))
        };

        let eop_obj = eop::EarthOrientationData::from_default_standard(eop_extrapolate,
                                                                       interpolate, eop_type);

        Ok(EarthOrientationData{obj:eop_obj})
    }

    /// Get UT1-UTC offset set for specified date.
    ///
    /// Function will return the UT1-UTC time scale for the given date.
    /// Function is guaranteed to return a value. If the request value is beyond the end of the
    /// loaded Earth orientation data set the behavior is specified by the `extrapolate` setting of
    /// the underlying `EarthOrientationData` object. The possible behaviors for the returned
    /// data are:
    /// - `Zero`: Returned values will be `0.0` where data is not available
    /// - `Hold`: Will return the last available returned value when data is not available
    /// - `Error`: Function call will panic and terminate the program
    ///
    /// If the date is in between data points, which typically are at integer day intervals, the
    /// function will linearly interpolate between adjacent data points if `interpolate` was set
    /// to `true` for the `EarthOrientationData` object or will return the value from the most
    /// recent data point if `false`.
    ///
    /// Args
    ///     mjd (`float`): Modified Julian date to get Earth orientation parameters for
    ///
    /// Returns
    ///     ut1_utc (`float`): Offset of UT1 time scale from UTC time scale. Units: (seconds)
    #[pyo3(text_signature = "(mjd)")]
    fn get_ut1_utc(&self, mjd: f64) -> f64 {
        self.obj.get_ut1_utc(mjd)
    }

    /// Get polar motion offset set for specified date.
    ///
    /// Function will return the pm-x and pm-y for the given date.
    /// Function is guaranteed to return a value. If the request value is beyond the end of the
    /// loaded Earth orientation data set the behavior is specified by the `extrapolate` setting of
    /// the underlying `EarthOrientationData` object. The possible behaviors for the returned
    /// data are:
    /// - `Zero`: Returned values will be `0.0` where data is not available
    /// - `Hold`: Will return the last available returned value when data is not available
    /// - `Error`: Function call will panic and terminate the program
    ///
    /// If the date is in between data points, which typically are at integer day intervals, the
    /// function will linearly interpolate between adjacent data points if `interpolate` was set
    /// to `true` for the `EarthOrientationData` object or will return the value from the most
    /// recent data point if `false`.
    ///
    /// Args:
    ///     mjd (`float`): Modified Julian date to get Earth orientation parameters for
    ///
    /// Returns:
    ///     pm_x (`float`): x-component of polar motion correction. Units: (radians)
    ///     pm_y (`float`): y-component of polar motion correction. Units: (radians)
    #[pyo3(text_signature = "(mjd)")]
    fn get_pm(&self, mjd: f64) -> (f64, f64) {
        self.obj.get_pm(mjd)
    }

    /// Get precession-nutation for specified date.
    ///
    /// Function will return the dX and dY for the given date.
    /// Function is guaranteed to return a value. If the request value is beyond the end of the
    /// loaded Earth orientation data set the behavior is specified by the `extrapolate` setting of
    /// the underlying `EarthOrientationData` object. The possible behaviors for the returned
    /// data are:
    /// - `Zero`: Returned values will be `0.0` where data is not available
    /// - `Hold`: Will return the last available returned value when data is not available
    /// - `Error`: Function call will panic and terminate the program
    ///
    /// If the date is in between data points, which typically are at integer day intervals, the
    /// function will linearly interpolate between adjacent data points if `interpolate` was set
    /// to `true` for the `EarthOrientationData` object or will return the value from the most
    /// recent data point if `false`.
    ///
    /// # Arguments
    ///     mjd (`float`): Modified Julian date to get Earth orientation parameters for
    ///
    /// # Returns
    ///     dX (`float`): "X" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
    ///     dY (`float`): "Y" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
    #[pyo3(text_signature = "(mjd)")]
    fn get_dxdy(&self, mjd: f64) -> (f64, f64) {
        self.obj.get_dxdy(mjd)
    }

    /// Get length of day offset set for specified date.
    ///
    /// Function will return the LOD offset for the given date.
    /// Function is guaranteed to return a value. If the request value is beyond the end of the
    /// loaded Earth orientation data set the behavior is specified by the `extrapolate` setting of
    /// the underlying `EarthOrientationData` object. The possible behaviors for the returned
    /// data are:
    /// - `Zero`: Returned values will be `0.0` where data is not available
    /// - `Hold`: Will return the last available returned value when data is not available
    /// - `Error`: Function call will panic and terminate the program
    ///
    /// If the date is in between data points, which typically are at integer day intervals, the
    /// function will linearly interpolate between adjacent data points if `interpolate` was set
    /// to `true` for the `EarthOrientationData` object or will return the value from the most
    /// recent data point if `false`.
    ///
    /// # Arguments
    ///     mjd (`float`): Modified Julian date to get Earth orientation parameters for
    ///
    /// # Returns
    ///     lod (`float`): Difference between length of astronomically determined solar day and 86400 second
    ///         TAI day. Units: (seconds)
    #[pyo3(text_signature = "(mjd)")]
    fn get_lod(&self, mjd: f64) -> f64 {
        self.obj.get_lod(mjd)
    }

    /// Get Earth orientation parameter set for specified date.
    ///
    /// Function will return the full set of Earth orientation parameters for the given date.
    /// Function is guaranteed to provide the full set of Earth Orientation parameters according
    /// to the behavior specified by the `extrapolate` setting of the underlying
    /// `EarthOrientationData` object. The possible behaviors for the returned data are:
    /// - `Zero`: Returned values will be `0.0` where data is not available
    /// - `Hold`: Will return the last available returned value when data is not available
    /// - `Error`: Function call will panic and terminate the program
    ///
    /// Note, if the type is `Hold` for an StandardBulletinB file which does not contain LOD data
    /// a value of `0.0` for LOD will be returned instead.
    ///
    /// If the date is in between data points, which typically are at integer day intervals, the
    /// function will linearly interpolate between adjacent data points if `interpolate` was set
    /// to `true` for the `EarthOrientationData` object or will return the value from the most
    /// recent data point if `false`.
    ///
    /// # Arguments
    ///     mjd (`float`): Modified Julian date to get Earth orientation parameters for
    ///
    /// # Returns
    ///     pm_x (`float`): x-component of polar motion correction. Units: (radians)
    ///     pm_y (`float`): y-component of polar motion correction. Units: (radians)
    ///     ut1_utc (`float`): Offset of UT1 time scale from UTC time scale. Units: (seconds)
    ///     dX (`float`): "X" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
    ///     dY (`float`): "Y" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
    ///     lod (`float`): Difference between length of astronomically determined solar day and
    ///         86400 second TAI day. Units: (seconds)
    #[pyo3(text_signature = "(mjd)")]
    fn get_eop(&self, mjd: f64) -> (f64, f64, f64, f64, f64, f64) {
        self.obj.get_eop(mjd)
    }
}

/// Download latest C04 Earth orientation parameter file.
///
///
/// Will attempt to download the latest parameter file to the specified location. Creating any
/// missing directories as required.
///
/// Download source: [https://datacenter.iers.org/data/latestVersion/9_FINALS.ALL_IAU2000_V2013_019.txt](https://datacenter.iers.org/data/latestVersion/9_FINALS.ALL_IAU2000_V2013_019.txt)
///
/// Args:
///     filepath (`str`): Path of desired output file
#[pyfunction]
#[pyo3(text_signature = "(filepath)")]
fn download_c04_eop_file(filepath: &str) -> PyResult<()> {
    eop::download_c04_eop_file(filepath);
    Ok(())
}

/// Download latest standard Earth orientation parameter file.
///
/// Will attempt to download the latest parameter file to the specified location. Creating any
/// missing directories as required.
///
/// Download source: [https://datacenter.iers.org/data/latestVersion/224_EOP_C04_14.62-NOW.IAU2000A224.txt](https://datacenter.iers.org/data/latestVersion/224_EOP_C04_14.62-NOW.IAU2000A224.txt)
///
/// Args:
///     filepath (`str`): Path of desired output file
#[pyfunction]
#[pyo3(text_signature = "(filepath)")]
fn download_standard_eop_file(filepath: &str) -> PyResult<()> {
    eop::download_standard_eop_file(filepath);
    Ok(())
}

#[pymodule]
pub fn eop(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<EarthOrientationData>()?;
    module.add_function(wrap_pyfunction!(download_c04_eop_file, module)?)?;
    module.add_function(wrap_pyfunction!(download_standard_eop_file, module)?)?;
    Ok(())
}

//////////
// Time //
//////////

/// Helper function to parse strings into appropriate time system enumerations
fn str_to_time_system(s:&str) -> Result<time::TimeSystem, PyErr> {
    match s.as_ref() {
        "GPS" => Ok(time::TimeSystem::GPS),
        "TAI" => Ok(time::TimeSystem::TAI),
        "TT" => Ok(time::TimeSystem::TT),
        "UTC" => Ok(time::TimeSystem::UTC),
        "UT1" => Ok(time::TimeSystem::UT1),
        _ => Err(PyRuntimeError::new_err(format!("Unkown time system string \"{}\"", s)))
    }
}

/// Helper function to convert time system enumerations into representative string
fn time_system_to_string(ts: time::TimeSystem) -> String {
    match ts {
        time::TimeSystem::GPS => String::from("GPS"),
        time::TimeSystem::TAI => String::from("TAI"),
        time::TimeSystem::TT => String::from("TT"),
        time::TimeSystem::UTC => String::from("UTC"),
        time::TimeSystem::UT1 => String::from("UT1"),
    }
}

/// Convert a Gregorian calendar date representation to the equivalent Julian Date
/// representation of that same instant in time.
///
/// Note: Due to the ambiguity of the nature of leap second insertion, this
/// method should not be used if a specific behavior for leap second insertion is expected. This
/// method treats leap seconds as if they don't exist.
///
/// Arguments:
///     year (`float`): Year
///     month (`float`): Month
///     day (`float`): Day
///     hour (`float`): Hour
///     minute (`float`): Minute
///     second (`float`): Second
///
/// Returns:
///     jd (`float`) Julian date of epoch
#[pyfunction]
#[pyo3(text_signature = "(year, month, day, hour, minute, second, nanosecond)")]
fn datetime_to_jd(year:u32, month:u8, day:u8, hour:u8, minute:u8, second:f64,
                  nanosecond:f64) -> PyResult<f64> {
    Ok(time::datetime_to_jd(year, month, day,hour,minute,second,nanosecond))
}

/// Convert a Gregorian calendar date representation to the equivalent Modified Julian Date
/// representation of that same instant in time.
///
/// Note: Due to the ambiguity of the nature of leap second insertion, this
/// method should not be used if a specific behavior for leap second insertion is expected. This
/// method treats leap seconds as if they don't exist.
///
/// Arguments:
///     year (`float`): Year
///     month (`float`): Month
///     day (`float`): Day
///     hour (`float`): Hour
///     minute (`float`): Minute
///     second (`float`): Second
///
/// Returns:
///     mjd (`float`) Modified Julian date of epoch
#[pyfunction]
#[pyo3(text_signature = "(year, month, day, hour, minute, second, nanosecond)")]
fn datetime_to_mjd(year:u32, month:u8, day:u8, hour:u8, minute:u8, second:f64,
                   nanosecond:f64) -> PyResult<f64> {
    Ok(time::datetime_to_mjd(year, month, day,hour,minute,second,nanosecond))
}

/// Convert a Julian Date representation to the equivalent Gregorian calendar date representation
/// of that same instant in time.
///
/// Note: Due to the ambiguity of the nature of leap second insertion, this
/// method should not be used if a specific behavior for leap second insertion is expected. This
/// method treats leap seconds as if they don't exist.
///
/// Arguments:
///     mjd (`float`) Modified Julian date of epoch
///
/// Returns:
///     year (`float`): Year
///     month (`float`): Month
///     day (`float`): Day
///     hour (`float`): Hour
///     minute (`float`): Minute
///     second (`float`): Second
#[pyfunction]
#[pyo3(text_signature = "(year, month, day, hour, minute, second, nanosecond)")]
fn jd_to_datetime(jd:f64) -> PyResult<(u32, u8, u8, u8, u8, f64, f64)> {
    Ok(time::jd_to_datetime(jd))
}

/// Convert a Modified Julian Date representation to the equivalent Gregorian calendar date representation
/// of that same instant in time.
///
/// Note: Due to the ambiguity of the nature of leap second insertion, this
/// method should not be used if a specific behavior for leap second insertion is expected. This
/// method treats leap seconds as if they don't exist.
///
/// Arguments:
///     mjd (`float`) Modified Julian date of epoch
///
/// Returns:
///     year (`float`): Year
///     month (`float`): Month
///     day (`float`): Day
///     hour (`float`): Hour
///     minute (`float`): Minute
///     second (`float`): Second
#[pyfunction]
#[pyo3(text_signature = "(year, month, day, hour, minute, second, nanosecond)")]
fn mjd_to_datetime(mjd:f64) -> PyResult<(u32, u8, u8, u8, u8, f64, f64)> {
    Ok(time::mjd_to_datetime(mjd))
}

#[pyclass]
struct TestClass {
    obj: eop::TestClass
}

#[pymethods]
impl TestClass {
    #[staticmethod]
    fn CreateTestClass(v:f64) -> PyResult<TestClass> {
        Ok(TestClass{obj: eop::TestClass{value:v}})
    }
}

/// Compute the offset between two time systems at a given Epoch.
///
/// The offset (in seconds) is computed as:
///     time_system_offset = time_system_dst - time_system_src
///
/// The value returned is the number of seconds that musted be added to the
/// source time system given the input epoch, to get the equivalent epoch.
///
/// Conversions are accomplished using SOFA C library calls.
///
/// Arguments
///     jd (`float`): Part 1 of two-part date (Julian days)
///     fd (`float`): Part 2 of two-part date (Fractional days)
///     time_system_src (`str`): Base time system
///     time_system_dest (`str`): Destination time system
///
/// Returns
///     offset (`float`): Offset between soruce and destination time systems in seconds.
#[pyfunction]
#[pyo3(text_signature = "(jd, fd, time_system_src, time_system_dest, eop)")]
fn time_system_offset(jd:f64, fd:f64, time_system_src:&str, time_system_dest:&str,
                      eop: &TestClass) -> PyResult<f64> {

    let ts_src = match str_to_time_system(time_system_src) {
        Ok(ts) => ts,
        Err(e) => return Err(e)
    };

    let ts_dst = match str_to_time_system(time_system_dest) {
        Ok(ts) => ts,
        Err(e) => return Err(e)
    };

    Ok(eop.obj.value)
    // Ok(0.0)
    // Ok(time::time_system_offset(jd, fd, ts_src, ts_dst, &eop.obj))
}

/// `Epoch` representing a specific instant in time.
///
/// The Epoch structure is the primary and preferred mechanism for representing
/// time in the Rastro library. It is designed to be able to accurately represent,
/// track, and compare instants in time accurately.
///
/// Internally, the Epoch structure stores time in terms of `days`, `seconds`, and
/// `nanoseconds`. This representation was chosen so that underlying time system
/// conversions and comparisons can be performed using the IAU SOFA library, which
/// has an API that operations in days and fractional days. However a day-based representation
/// does not accurately handle small changes in time (subsecond time) especially when
/// propagating or adding small values over long periods. Therefore, the Epoch structure
/// internall stores time in terms of seconds and nanoseconds and converts converts changes to
/// seconds and days when required. This enables the best of both worlds. Accurate
/// time representation of small differences and changes in time (nanoseconds) and
/// validated conversions between time systems.
///
/// Internally, the structure
/// uses [Kahan summation](https://en.wikipedia.org/wiki/Kahan_summation_algorithm) to
/// accurate handle running sums over long periods of time without losing accuracy to
/// floating point representation of nanoseconds.
///
/// All arithmetic operations (addition, substracion) that the structure supports
/// use seconds as the default value and return time differences in seconds.
struct Epoch<'a> {
    /// Stored object for underlying EOP
    obj: time::Epoch<'a>,
}

// #[pymethods]
// impl Epoch {
//     // Define attribute access methods
//     /// `str`: Time system of Epoch. One of: "GPS", "TAI", "TT", "UTC", "UT1"
//     #[getter]
//     fn time_system(&self) -> String {
//         match self.robj.eop_type {
//             eop::EOPType::GPS => String::from("GPS"),
//             eop::EOPType::TAI => String::from("TAI"),
//             eop::EOPType::TT => String::from("TT"),
//             eop::EOPType::UTC => String::from("UTC"),
//             eop::EOPType::UT1 => String::from("UT1"),
//         }
//     }
//
//     // pub fn from_date(year:u32, month:u8, day:u8, time_system: TimeSystem, eop: &'a EarthOrientationData)
//     // pub fn from_datetime(year:u32, month:u8, day:u8, hour:u8, minute:u8, second:f64,
//     //                      nanosecond:f64, time_system: TimeSystem, eop: &'a EarthOrientationData) -> Self {}
//     // pub fn from_string(datestr: &str, eop: &'a EarthOrientationData) -> Option<Self> {
//     //
//     // }
//     // pub fn from_jd(jd: f64, time_system:TimeSystem, eop: &'a EarthOrientationData) -> Self {
//     //
//     // }
//     // pub fn from_mjd(mjd: f64, time_system:TimeSystem, eop: &'a EarthOrientationData) -> Self {
//     //
//     // }
//     // pub fn from_gps_date(week: u32, seconds: f64, eop: &'a EarthOrientationData) -> Self {
//     //
//     // }
//     // pub fn from_gps_seconds(gps_seconds: f64, eop: &'a EarthOrientationData) -> Self {
//     //
//     // }
//     // pub fn from_gps_nanoseconds(gps_nanoseconds: u64, eop: &'a EarthOrientationData) -> Self {
//     // pub fn to_datetime_as_tsys(&self, time_system:TimeSystem) -> (u32, u8, u8, u8, u8, f64, f64) {}
//     // pub fn to_datetime(&self) -> (u32, u8, u8, u8, u8, f64, f64) {}
//     // pub fn jd_as_tsys(&self, time_system:TimeSystem) -> f64 {}
//     // pub fn jd(&self) -> f64 {}
//     // pub fn mjd_as_tsys(&self, time_system:TimeSystem) -> f64 {}
//     // pub fn mjd(&self) -> f64 {}
//     // pub fn gps_date(&self) -> (u32, f64) {}
//     // pub fn gps_seconds(&self) -> f64 {}
//     // pub fn gps_nanoseconds(&self) -> f64 {}
//     // pub fn isostring(&self) -> String {}
//     // pub fn isostringd(&self, decimals: usize) -> String {}
//     // pub fn to_string_as_tsys(&self, time_system:TimeSystem) -> String {}
//     // pub fn gast(&self, as_degrees: bool) -> f64 {}
//     // pub fn gmst(&self, as_degrees: bool) -> f64 {}
// }

#[pymodule]
pub fn time(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<TestClass>()?;
    module.add_function(wrap_pyfunction!(datetime_to_jd, module)?)?;
    module.add_function(wrap_pyfunction!(datetime_to_mjd, module)?)?;
    module.add_function(wrap_pyfunction!(mjd_to_datetime, module)?)?;
    module.add_function(wrap_pyfunction!(jd_to_datetime, module)?)?;
    module.add_function(wrap_pyfunction!(time_system_offset, module)?)?;

    Ok(())
}

//////////////
//  Orbits  //
//////////////

/// Computes the orbital period of an object around Earth.
///
/// Uses rastro.constants.GM_EARTH as the standard gravitational parameter for the calculation.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: (m)
///
/// Returns:
///     period (`float`): The orbital period of the astronomical object. Units: (s)
#[pyfunction]
#[pyo3(text_signature = "(a)")]
fn orbital_period(a: f64) -> PyResult<f64> {
    Ok(orbits::orbital_period(a))
}

/// Computes the orbital period of an astronomical object around a general body.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: (m)
///     gm (`float`): The standard gravitational parameter of primary body. Units: [m^3/s^2]
///
/// Returns:
///     period (`float`): The orbital period of the astronomical object. Units: (s)
#[pyfunction]
#[pyo3(text_signature = "(a, gm)")]
fn orbital_period_general(a: f64, gm: f64) -> PyResult<f64> {
    Ok(orbits::orbital_period_general(a, gm))
}

/// Computes the mean motion of an astronomical object around Earth.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: (m)
///     as_degrees (`bool`): Return output in degrees instead of radians
///
/// Returns:
///     n (`float`): The mean motion of the astronomical object. Units: (rad) or (deg)
#[pyfunction(as_degrees="true")]
#[pyo3(text_signature = "(a, as_degrees)")]
fn mean_motion(a: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::mean_motion(a, as_degrees))
}

/// Computes the mean motion of an astronomical object around a general body
/// given a semi-major axis.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: (m)
///     gm (`float`): The standard gravitational parameter of primary body. Units: [m^3/s^2]
///     as_degrees (`bool`): Return output in degrees instead of radians
///
/// Returns:
///     n (`float`): The mean motion of the astronomical object. Units: (rad) or (deg)
#[pyfunction(as_degrees="true")]
#[pyo3(text_signature = "(a, gm, as_degrees)")]
fn mean_motion_general(a: f64, gm: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::mean_motion_general(a, gm, as_degrees))
}

/// Computes the semi-major axis of an astronomical object from Earth
/// given the object's mean motion.
///
/// Arguments:
///     n (`float`): The mean motion of the astronomical object. Units: (rad) or (deg)
///     as_degrees (`bool`): Interpret mean motion as degrees if `true` or radians if `false`
///
/// Returns:
///     a (`float`): The semi-major axis of the astronomical object. Units: (m)
#[pyfunction(as_degrees="true")]
#[pyo3(text_signature = "(a, as_degrees)")]
fn semimajor_axis(n: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::semimajor_axis(n, as_degrees))
}

/// Computes the semi-major axis of an astronomical object from a general body
/// given the object's mean motion.
///
/// Arguments:
///     n (`float`): The mean motion of the astronomical object. Units: (rad) or (deg)
///     gm (`float`): The standard gravitational parameter of primary body. Units: [m^3/s^2]
///     as_degrees (`float`): Interpret mean motion as degrees if `true` or radians if `false`
///
/// Returns:
///     a (`float`): The semi-major axis of the astronomical object. Units: (m)
#[pyfunction(as_degrees="true")]
#[pyo3(text_signature = "(a, gm, as_degrees)")]
fn semimajor_axis_general(n: f64, gm: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::semimajor_axis_general(n, gm, as_degrees))
}

/// Computes the perigee velocity of an astronomical object around Earth.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: (m)
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///
/// Returns:
///     v (`float`): The magnitude of velocity of the object at perigee. Units: (m/s)
#[pyfunction]
#[pyo3(text_signature = "(a, e)")]
fn perigee_velocity(a: f64, e: f64) -> PyResult<f64> {
    Ok(orbits::perigee_velocity(a, e))
}

/// Computes the periapsis velocity of an astronomical object around a general body.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: (m)
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     gm (`float`): The standard gravitational parameter of primary body. Units: [m^3/s^2]
///
/// Returns:
///     v (`float`): The magnitude of velocity of the object at periapsis. Units: (m/s)
#[pyfunction]
#[pyo3(text_signature = "(a, e)")]
fn periapsis_velocity(a: f64, e: f64, gm: f64) -> PyResult<f64> {
    Ok(orbits::periapsis_velocity(a, e, gm))
}

/// Calculate the distance of an object at its periapsis
///
/// # Arguments
///
/// * `a`: The semi-major axis of the astronomical object. Units: (m)
/// * `e`: The eccentricity of the astronomical object's orbit. Dimensionless
///
/// # Returns
///
/// * `r`: The distance of the object at periapsis. Units (s)
#[pyfunction]
#[pyo3(text_signature = "(a, e)")]
fn periapsis_distance(a: f64, e: f64) -> PyResult<f64> {
    Ok(orbits::periapsis_distance(a, e))
}

/// Computes the apogee velocity of an astronomical object around Earth.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: (m)
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///
/// Returns:
///     v (`float`): The magnitude of velocity of the object at apogee. Units: (m/s)
#[pyfunction]
#[pyo3(text_signature = "(a, e)")]
fn apogee_velocity(a: f64, e: f64) -> PyResult<f64> {
    Ok(orbits::apogee_velocity(a, e))
}

/// Computes the apoapsis velocity of an astronomical object around a general body.
///
/// Arguments:
///     a (`float`): The semi-major axis of the astronomical object. Units: (m)
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     gm (`float`): The standard gravitational parameter of primary body. Units: [m^3/s^2]
///
/// Returns:
///     v (`float`): The magnitude of velocity of the object at apoapsis. Units: (m/s)
#[pyfunction]
#[pyo3(text_signature = "(a, e)")]
fn apoapsis_velocity(a: f64, e: f64, gm: f64) -> PyResult<f64> {
    Ok(orbits::apoapsis_velocity(a, e, gm))
}

/// Calculate the distance of an object at its apoapsis
///
/// # Arguments
///
/// * `a`: The semi-major axis of the astronomical object. Units: (m)
/// * `e`: The eccentricity of the astronomical object's orbit. Dimensionless
///
/// # Returns
///
/// * `r`: The distance of the object at apoapsis. Units (s)
#[pyfunction]
#[pyo3(text_signature = "(a, e)")]
fn apoapsis_distance(a: f64, e: f64) -> PyResult<f64> {
    Ok(orbits::apoapsis_distance(a, e))
}

/// Computes the inclination for a Sun-synchronous orbit around Earth based on
/// the J2 gravitational perturbation.
///
/// Arguments:
///     a (`float`) The semi-major axis of the astronomical object. Units: (m)
///     e (`float`) The eccentricity of the astronomical object's orbit. Dimensionless
///     as_degrees (`bool`) Return output in degrees instead of radians
///
/// Returns:
///     inc (`float`) Inclination for a Sun synchronous orbit. Units: (deg) or (rad)
#[pyfunction]
#[pyo3(text_signature = "(a, e, as_degrees)")]
fn sun_synchronous_inclination(a: f64, e: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::sun_synchronous_inclination(a, e, as_degrees))
}

/// Converts eccentric anomaly into mean anomaly.
///
/// Arguments:
///     anm_ecc (`float`): Eccentric anomaly. Units: (rad) or (deg)
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     as_degrees (`bool`): Interprets input and returns output in (deg) if `true` or (rad) if `false`
///
/// Returns:
///     anm_mean (`float`): Mean anomaly. Units: (rad) or (deg)
#[pyfunction]
#[pyo3(text_signature = "(anm_ecc, e, as_degrees)")]
fn anomaly_eccentric_to_mean(anm_ecc: f64, e: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::anomaly_eccentric_to_mean(anm_ecc, e, as_degrees))
}

/// Converts mean anomaly into eccentric anomaly
///
/// Arguments:
///     anm_mean (`float`): Mean anomaly. Units: (rad) or (deg)
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     as_degrees (`float`): Interprets input and returns output in (deg) if `true` or (rad) if `false`
///
/// Returns:
///     anm_ecc (`float`): Eccentric anomaly. Units: (rad) or (deg)
#[pyfunction]
#[pyo3(text_signature = "(anm_mean, e, as_degrees)")]
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
///     anm_true (`float`): true anomaly. Units: (rad) or (deg)
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     as_degrees (`bool`): Interprets input and returns output in (deg) if `true` or (rad) if `false`
///
/// Returns:
///     anm_ecc (`float`): Eccentric anomaly. Units: (rad) or (deg)
#[pyfunction]
#[pyo3(text_signature = "(anm_true, e, as_degrees))")]
fn anomaly_true_to_eccentric(anm_true: f64, e: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::anomaly_true_to_eccentric(anm_true, e, as_degrees))
}

/// Converts eccentric anomaly into true anomaly
///
/// # Arguments
///     anm_ecc (`float`): Eccentric anomaly. Units: (rad) or (deg)
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     as_degrees (`bool`): Interprets input and returns output in (deg) if `true` or (rad) if `false`
///
/// # Returns
///     anm_true (`float`): true anomaly. Units: (rad) or (deg)
#[pyfunction]
#[pyo3(text_signature = "(anm_ecc, e, as_degrees))")]
fn anomaly_eccentric_to_true(anm_ecc: f64, e: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::anomaly_eccentric_to_true(anm_ecc, e, as_degrees))
}

/// Converts true anomaly into mean anomaly.
///
/// Arguments:
///     anm_true (`float`): True anomaly. Units: (rad) or (deg)
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     as_degrees (`bool`): Interprets input and returns output in (deg) if `true` or (rad) if `false`
///
/// Returns:
///     anm_mean (`float`): Mean anomaly. Units: (rad) or (deg)
#[pyfunction]
#[pyo3(text_signature = "(anm_ecc, e, as_degrees))")]
fn anomaly_true_to_mean(anm_ecc: f64, e: f64, as_degrees: bool) -> PyResult<f64> {
    Ok(orbits::anomaly_true_to_mean(anm_ecc, e, as_degrees))
}

/// Converts mean anomaly into true anomaly
///
/// Arguments:
///     anm_mean (`float`): Mean anomaly. Units: (rad) or (deg)
///     e (`float`): The eccentricity of the astronomical object's orbit. Dimensionless
///     as_degrees (`bool`): Interprets input and returns output in (deg) if `true` or (rad) if `false`
///
/// Returns:
///     anm_true (`float`): True anomaly. Units: (rad) or (deg)
#[pyfunction]
#[pyo3(text_signature = "(anm_mean, e, as_degrees)")]
fn anomaly_mean_to_true(anm_mean: f64, e: f64, as_degrees: bool) -> PyResult<f64> {
    let res = orbits::anomaly_mean_to_true(anm_mean, e, as_degrees);
    if res.is_ok() {
        Ok(res.unwrap())
    } else {
        Err(PyRuntimeError::new_err(res.err().unwrap()))
    }
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
    module.add_function(wrap_pyfunction!(periapsis_distance, module)?)?;
    module.add_function(wrap_pyfunction!(apogee_velocity, module)?)?;
    module.add_function(wrap_pyfunction!(apoapsis_velocity, module)?)?;
    module.add_function(wrap_pyfunction!(apoapsis_distance, module)?)?;
    module.add_function(wrap_pyfunction!(sun_synchronous_inclination, module)?)?;
    module.add_function(wrap_pyfunction!(anomaly_eccentric_to_mean, module)?)?;
    module.add_function(wrap_pyfunction!(anomaly_mean_to_eccentric, module)?)?;
    module.add_function(wrap_pyfunction!(anomaly_true_to_eccentric, module)?)?;
    module.add_function(wrap_pyfunction!(anomaly_eccentric_to_true, module)?)?;
    module.add_function(wrap_pyfunction!(anomaly_true_to_mean, module)?)?;
    module.add_function(wrap_pyfunction!(anomaly_mean_to_true, module)?)?;

    Ok(())
}