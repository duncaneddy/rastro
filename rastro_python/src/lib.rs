use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::PyType;
/// This is the wrapper for the rastro script.
///
/// It is currently all in one file because of the PyO3 issues discussed in
/// https://github.com/PyO3/pyo3/issues/1444 which prevents sharing rust-defined
/// PyO3 classes between different modules. This leads to a need to keep the
/// entire wrapper in a single file until this is addressed.
///
/// While unfortunate, that's where we are at.
use pyo3::{exceptions, wrap_pyfunction};
use rastro::{constants, eop, orbits, time};

////////////////
//  Consants  //
////////////////

// Directly Added

/////////////////////////
//  Earth Orientation  //
/////////////////////////

/// Helper function to parse strings into appropriate EOPExtrapolation enumerations
fn string_to_eop_extrapolation(s: &str) -> Result<eop::EOPExtrapolation, PyErr> {
    match s.as_ref() {
        "Hold" => Ok(eop::EOPExtrapolation::Hold),
        "Zero" => Ok(eop::EOPExtrapolation::Zero),
        "Error" => Ok(eop::EOPExtrapolation::Error),
        _ => Err(exceptions::PyRuntimeError::new_err(format!(
            "Unknown EOP Extrapolation string \"{}\"",
            s
        ))),
    }
}

/// Helper function to convert EOPExtrapolation enumerations into representative string
fn eop_extrapolation_to_string(extrapolation: eop::EOPExtrapolation) -> String {
    match extrapolation {
        eop::EOPExtrapolation::Hold => String::from("Hold"),
        eop::EOPExtrapolation::Zero => String::from("Zero"),
        eop::EOPExtrapolation::Error => String::from("Error"),
    }
}

/// Helper function to parse strings into appropriate EOPType enumerations
fn string_to_eop_type(s: &str) -> Result<eop::EOPType, PyErr> {
    match s.as_ref() {
        "C04" => Ok(eop::EOPType::C04),
        "StandardBulletinA" => Ok(eop::EOPType::StandardBulletinA),
        "StandardBulletinB" => Ok(eop::EOPType::StandardBulletinB),
        "Static" => Ok(eop::EOPType::Static),
        _ => Err(exceptions::PyRuntimeError::new_err(format!(
            "Unknown EOP Type string \"{}\"",
            s
        ))),
    }
}

/// Helper function to convert EOPType enumerations into representative string
fn eop_type_to_string(eop_type: eop::EOPType) -> String {
    match eop_type {
        eop::EOPType::C04 => String::from("C04"),
        eop::EOPType::StandardBulletinA => String::from("StandardBulletinA"),
        eop::EOPType::StandardBulletinB => String::from("StandardBulletinB"),
        eop::EOPType::Static => String::from("Static"),
    }
}

/// Initializes the RAstro static (global) EOP zero values.
///
/// The static (global) Earth orientation variable is used internally by RAstro
/// time and reference frame conversion functions.
///
/// This initialization can be used to easily initialize Earth orientation data
/// required for Epoch time system and reference frame conversions. The results
/// will not be physically actuate when using this initialization method, however
/// it can be useful for simple analysis.
#[pyfunction]
pub fn set_global_eop_from_zero() {
    eop::set_global_eop_from_zero()
}

/// Initializes the RAstro static (global) EOP static values.
///
/// The static (global) Earth orientation variable is used internally by RAstro
/// time and reference frame conversion functions.
///
/// This can be used to set a single set of static Earth that will be held
/// used for all conversions. This is accomplished by instantiating a standard
/// EarthOrientationData object with a single entry containing the necessary
/// values with extrapolation set to EOPExtrapolation::Hold, so that they are
/// used for all dates.
///
/// Args:
///     - pm_x (`float`): x-component of polar motion correction. Units: (radians)
///     - pm_y (`float`): y-component of polar motion correction. Units: (radians)
///     - ut1_utc (`float`): Offset of UT1 time scale from UTC time scale. Units: (seconds)
///     - dX (`float`): "X" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
///     - dY (`float`): "Y" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
///     - lod (`float`): Difference between astronomically determined length of day and 86400 second TAI. Units: (seconds)
///
/// This method applies the `from_static_values` initialization method to the static
/// crate EOP table.
///
#[pyfunction]
#[pyo3(text_signature = "(pm_x, pm_y, ut1_utc, dX, dY, lod)")]
#[allow(non_snake_case)]
pub fn set_global_eop_from_static_values(
    pm_x: f64,
    pm_y: f64,
    ut1_utc: f64,
    dX: f64,
    dY: f64,
    lod: f64,
) {
    eop::set_global_eop_from_static_values(pm_x, pm_y, ut1_utc, dX, dY, lod)
}

/// Initializes the RAstro static (global) EOP from C04 Earth orientation data from file.
///
/// Takes a path to a given file which will be read on the assumption that it is an Earth
/// orientation parameter data file formatted according to [IERS C04 formatting standards](https://www.iers.org/IERS/EN/DataProducts/EarthOrientationData/eop.html)
///
/// The static (global) Earth orientation variable is used internally by RAstro
/// time and reference frame conversion functions.
///
/// Args:
///     - filepath (`str`): Path of input data file
///     - extrapolate (`str`): Set EOP Extrapolation behavior for resulting EarthOrientationData object.
///     - interpolate (`bool`): Set EOP interpolation behavior for resulting EarthOrientationData object.
#[pyfunction]
#[pyo3(text_signature = "(filepath, extrapolate, interpolate)")]
pub fn set_global_eop_from_c04_file(filepath: &str, extrapolate: &str, interpolate: bool) {
    eop::set_global_eop_from_c04_file(
        filepath,
        string_to_eop_extrapolation(extrapolate).unwrap(),
        interpolate,
    )
    .unwrap();
}

/// Initializes the RAstro static (global) EOP from package-default C04 Earth orientation data.
///
/// Parses the Earth orientation data packaged with the RAstro library return a valid
/// `EarthOrientationData`.
///
/// The static (global) Earth orientation variable is used internally by RAstro
/// time and reference frame conversion functions.
///
/// Args:
///     - extrapolate (`str`): Set EOP Extrapolation behavior for resulting EarthOrientationData object.
///     - interpolate (`bool`): Set EOP interpolation behavior for resulting EarthOrientationData object.
#[pyfunction]
#[pyo3(text_signature = "(extrapolate, interpolate)")]
pub fn set_global_eop_from_default_c04(extrapolate: &str, interpolate: bool) {
    eop::set_global_eop_from_default_c04(
        string_to_eop_extrapolation(extrapolate).unwrap(),
        interpolate,
    )
    .unwrap();
}

/// Initializes the RAstro static (global) EOP from C04 Earth orientation data from file.
///
/// Takes a path to a given file which will be read on the assumption that it is an Earth
/// orientation parameter data file formatted according to [IERS C04 formatting standards](https://www.iers.org/IERS/EN/DataProducts/EarthOrientationData/eop.html)
///
/// The static (global) Earth orientation variable is used internally by RAstro
/// time and reference frame conversion functions.
///
/// Args:
///     - filepath (`str`): Path of input data file
///     - extrapolate (`str`): Set EOP Extrapolation behavior for resulting EarthOrientationData object.
///     - interpolate (`bool`): Set EOP interpolation behavior for resulting EarthOrientationData object.
///     - eop_type (`str`): Type to parse data file as. Can be `EOPType::StandardBulletinA` or
///       `EOPType::StandardBulletinB`
#[pyfunction]
#[pyo3(text_signature = "(filepath, extrapolate, interpolate, eop_type)")]
pub fn set_global_eop_from_standard_file(
    filepath: &str,
    extrapolate: &str,
    interpolate: bool,
    eop_type: &str,
) {
    eop::set_global_eop_from_standard_file(
        filepath,
        string_to_eop_extrapolation(extrapolate).unwrap(),
        interpolate,
        string_to_eop_type(eop_type).unwrap(),
    )
    .unwrap();
}

/// Initializes the RAstro static (global) EOP from package-default C04 Earth orientation data.
///
/// Parses the Earth orientation data packaged with the RAstro library return a valid
/// `EarthOrientationData`.
///
/// The static (global) Earth orientation variable is used internally by RAstro
/// time and reference frame conversion functions.
///
/// Args:
///     - extrapolate (`str`): Set EOP Extrapolation behavior for resulting EarthOrientationData object.
///     - interpolate (`bool`): Set EOP interpolation behavior for resulting EarthOrientationData object.
///     - eop_type (`str`): Type to parse data file as. Can be `EOPType::StandardBulletinA` or
///       `EOPType::StandardBulletinB`
#[pyfunction]
#[pyo3(text_signature = "(extrapolate, interpolate, eop_type)")]
pub fn set_global_eop_from_default_standard(extrapolate: &str, interpolate: bool, eop_type: &str) {
    eop::set_global_eop_from_default_standard(
        string_to_eop_extrapolation(extrapolate).unwrap(),
        interpolate,
        string_to_eop_type(eop_type).unwrap(),
    )
    .unwrap();
}

/// Get UT1-UTC offset set for specified date from loaded static Earth orientation data.
///
/// Function will return the full set of Earth orientation parameters for the given date.
/// Function is guaranteed to provide the full set of Earth Orientation parameters according
/// to the behavior specified by the `extrapolate` setting of the underlying loaded global Earth
/// orientation data. The possible behaviors for the returned data are:
/// - `Zero`: Returned values will be `0.0` where data is not available
/// - `Hold`: Will return the last available returned value when data is not available
/// - `Error`: Function call will panic and terminate the program
///
/// Note, if the type is `Hold` for an StandardBulletinB file which does not contain LOD data
/// a value of `0.0` for LOD will be returned instead.
///
/// If the date is in between data points, which typically are at integer day intervals, the
/// function will linearly interpolate between adjacent data points if `interpolate` was set
/// to `true` for the loaded global Earth orientation data or will return the value from the most
/// recent data point if `false`.
///
/// Args:
///     - mjd (`float`): Modified Julian date to get Earth orientation parameters for
///
/// Returns:
///     - ut1_utc (`float`): Offset of UT1 time scale from UTC time scale. Units: (seconds)
#[pyfunction]
#[pyo3(text_signature = "(mjd)")]
pub fn get_global_ut1_utc(mjd: f64) -> f64 {
    eop::get_global_ut1_utc(mjd).unwrap()
}

/// Get polar motion offset set for specified date from loaded static Earth orientation data.
///
/// Function will return the full set of Earth orientation parameters for the given date.
/// Function is guaranteed to provide the full set of Earth Orientation parameters according
/// to the behavior specified by the `extrapolate` setting of the underlying loaded global Earth
/// orientation data. The possible behaviors for the returned data are:
/// - `Zero`: Returned values will be `0.0` where data is not available
/// - `Hold`: Will return the last available returned value when data is not available
/// - `Error`: Function call will panic and terminate the program
///
/// Note, if the type is `Hold` for an StandardBulletinB file which does not contain LOD data
/// a value of `0.0` for LOD will be returned instead.
///
/// If the date is in between data points, which typically are at integer day intervals, the
/// function will linearly interpolate between adjacent data points if `interpolate` was set
/// to `true` for the loaded global Earth orientation data or will return the value from the most
/// recent data point if `false`.
///
/// Args:
///     - mjd (`float`): Modified Julian date to get Earth orientation parameters for
///
/// Returns:
///     - pm_x (`float`): x-component of polar motion correction. Units: (radians)
///     - pm_y (`float`): y-component of polar motion correction. Units: (radians)
#[pyfunction]
#[pyo3(text_signature = "(mjd)")]
pub fn get_global_pm(mjd: f64) -> (f64, f64) {
    eop::get_global_pm(mjd).unwrap()
}

/// Get precession-nutation for specified date from loaded static Earth orientation data.
///
/// Function will return the full set of Earth orientation parameters for the given date.
/// Function is guaranteed to provide the full set of Earth Orientation parameters according
/// to the behavior specified by the `extrapolate` setting of the underlying loaded global Earth
/// orientation data. The possible behaviors for the returned data are:
/// - `Zero`: Returned values will be `0.0` where data is not available
/// - `Hold`: Will return the last available returned value when data is not available
/// - `Error`: Function call will panic and terminate the program
///
/// Note, if the type is `Hold` for an StandardBulletinB file which does not contain LOD data
/// a value of `0.0` for LOD will be returned instead.
///
/// If the date is in between data points, which typically are at integer day intervals, the
/// function will linearly interpolate between adjacent data points if `interpolate` was set
/// to `true` for the loaded global Earth orientation data or will return the value from the most
/// recent data point if `false`.
///
/// Args:
///     - mjd (`float`): Modified Julian date to get Earth orientation parameters for
///
/// Returns:
///     - dX (`float`): "X" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
///     - dY (`float`): "Y" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
///
#[pyfunction]
#[pyo3(text_signature = "(mjd)")]
pub fn get_global_dxdy(mjd: f64) -> (f64, f64) {
    eop::get_global_dxdy(mjd).unwrap()
}

/// Get length of day offset set for specified date from loaded static Earth orientation data.
///
/// Function will return the full set of Earth orientation parameters for the given date.
/// Function is guaranteed to provide the full set of Earth Orientation parameters according
/// to the behavior specified by the `extrapolate` setting of the underlying loaded global Earth
/// orientation data. The possible behaviors for the returned data are:
/// - `Zero`: Returned values will be `0.0` where data is not available
/// - `Hold`: Will return the last available returned value when data is not available
/// - `Error`: Function call will panic and terminate the program
///
/// Note, if the type is `Hold` for an StandardBulletinB file which does not contain LOD data
/// a value of `0.0` for LOD will be returned instead.
///
/// If the date is in between data points, which typically are at integer day intervals, the
/// function will linearly interpolate between adjacent data points if `interpolate` was set
/// to `true` for the loaded global Earth orientation data or will return the value from the most
/// recent data point if `false`.
///
/// Args:
///     - mjd (`float`): Modified Julian date to get Earth orientation parameters for
///
/// Returns:
///     - lod (`float`): Difference between length of astronomically determined solar day and 86400 second
///       TAI day. Units: (seconds)
///
#[pyfunction]
#[pyo3(text_signature = "(mjd)")]
pub fn get_global_lod(mjd: f64) -> f64 {
    eop::get_global_lod(mjd).unwrap()
}

/// Get Earth orientation parameter set for specified date from loaded static Earth orientation data.
///
/// Function will return the full set of Earth orientation parameters for the given date.
/// Function is guaranteed to provide the full set of Earth Orientation parameters according
/// to the behavior specified by the `extrapolate` setting of the underlying loaded global Earth
/// orientation data. The possible behaviors for the returned data are:
/// - `Zero`: Returned values will be `0.0` where data is not available
/// - `Hold`: Will return the last available returned value when data is not available
/// - `Error`: Function call will panic and terminate the program
///
/// Note, if the type is `Hold` for an StandardBulletinB file which does not contain LOD data
/// a value of `0.0` for LOD will be returned instead.
///
/// If the date is in between data points, which typically are at integer day intervals, the
/// function will linearly interpolate between adjacent data points if `interpolate` was set
/// to `true` for the loaded global Earth orientation data or will return the value from the most
/// recent data point if `false`.
///
/// Args:
///     - mjd (`float`): Modified Julian date to get Earth orientation parameters for
///
/// Returns:
///     - pm_x (`float`): x-component of polar motion correction. Units: (radians)
///     - pm_y (`float`): y-component of polar motion correction. Units: (radians)
///     - ut1_utc (`float`): Offset of UT1 time scale from UTC time scale. Units: (seconds)
///     - dX (`float`): "X" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
///     - dY (`float`): "Y" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
///     - lod (`float`): Difference between length of astronomically determined solar day and 86400 second
///       TAI day. Units: (seconds)
#[pyfunction]
#[pyo3(text_signature = "(mjd)")]
pub fn get_global_eop(mjd: f64) -> (f64, f64, f64, f64, f64, f64) {
    eop::get_global_eop(mjd).unwrap()
}

/// Returns initialization state of global Earth orientation data
///
/// Returns:
/// - initialized (`bool`): Boolean, which if `true` indicates that the global static variable has been properly initialized.
#[pyfunction]
pub fn get_global_eop_initialization() -> bool {
    eop::get_global_eop_initialization()
}

/// Return length of loaded global Earth orientation data
///
/// Returns:
/// - len (`int`): length of number of loaded EOP data points
#[pyfunction]
pub fn get_global_eop_len() -> usize {
    eop::get_global_eop_len()
}

/// Return eop_type value of loaded global Earth orientation data
///
/// Returns:
/// - eop_type (`str`): Type of loaded Earth Orientation data
#[pyfunction]
pub fn get_global_eop_type() -> String {
    eop_type_to_string(eop::get_global_eop_type())
}

/// Return extrapolation value of loaded global Earth orientation data
///
/// Returns:
/// - extrapolation (`str`): Extrapolation setting of loaded Earth Orientation data
#[pyfunction]
pub fn get_global_eop_extrapolate() -> String {
    eop_extrapolation_to_string(eop::get_global_eop_extrapolate())
}

/// Return interpolation value of loaded global Earth orientation data
///
/// Returns:
/// - interpolation (`bool`): Interpolation setting of loaded Earth Orientation data
#[pyfunction]
pub fn get_global_eop_interpolate() -> bool {
    eop::get_global_eop_interpolate()
}

/// Return mjd_min value of loaded global Earth orientation data
///
/// Returns:
/// - mjd_min (`int`): Minimum MJD of loaded EOP data points
#[pyfunction]
pub fn get_global_eop_mjd_min() -> u32 {
    eop::get_global_eop_mjd_min()
}

/// Return mjd_max value of loaded global Earth orientation data
///
/// Returns:
/// - mjd_max (`int`): Maximum MJD of loaded EOP data points
#[pyfunction]
pub fn get_global_eop_mjd_max() -> u32 {
    eop::get_global_eop_mjd_max()
}

/// Return mjd_last_lod value of loaded global Earth orientation data
///
/// Returns:
/// - mjd_last_lod (`int`): MJD of latest chronological EOP data points with a valid LOD value
#[pyfunction]
pub fn get_global_eop_mjd_last_lod() -> u32 {
    eop::get_global_eop_mjd_last_lod()
}

/// Return mjd_last_dxdy value of loaded global Earth orientation data
///
/// Returns:
/// - mjd_last_dxdy (`int`): MJD of latest chronological EOP data points with valid dX, dY values
#[pyfunction]
pub fn get_global_eop_mjd_last_dxdy() -> u32 {
    eop::get_global_eop_mjd_last_dxdy()
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
    eop::download_c04_eop_file(filepath).unwrap();
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
    eop::download_standard_eop_file(filepath).unwrap();
    Ok(())
}

//////////
// Time //
//////////

/// Helper function to parse strings into appropriate time system enumerations
fn string_to_time_system(s: &str) -> Result<time::TimeSystem, PyErr> {
    match s.as_ref() {
        "GPS" => Ok(time::TimeSystem::GPS),
        "TAI" => Ok(time::TimeSystem::TAI),
        "TT" => Ok(time::TimeSystem::TT),
        "UTC" => Ok(time::TimeSystem::UTC),
        "UT1" => Ok(time::TimeSystem::UT1),
        _ => Err(exceptions::PyRuntimeError::new_err(format!(
            "Unknown time system string \"{}\"",
            s
        ))),
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
fn datetime_to_jd(
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: f64,
    nanosecond: f64,
) -> PyResult<f64> {
    Ok(time::datetime_to_jd(
        year, month, day, hour, minute, second, nanosecond,
    ))
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
fn datetime_to_mjd(
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: f64,
    nanosecond: f64,
) -> PyResult<f64> {
    Ok(time::datetime_to_mjd(
        year, month, day, hour, minute, second, nanosecond,
    ))
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
fn jd_to_datetime(jd: f64) -> PyResult<(u32, u8, u8, u8, u8, f64, f64)> {
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
fn mjd_to_datetime(mjd: f64) -> PyResult<(u32, u8, u8, u8, u8, f64, f64)> {
    Ok(time::mjd_to_datetime(mjd))
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
fn time_system_offset(
    jd: f64,
    fd: f64,
    time_system_src: &str,
    time_system_dest: &str,
) -> PyResult<f64> {
    let ts_src = match string_to_time_system(time_system_src) {
        Ok(ts) => ts,
        Err(e) => return Err(e),
    };

    let ts_dst = match string_to_time_system(time_system_dest) {
        Ok(ts) => ts,
        Err(e) => return Err(e),
    };

    Ok(time::time_system_offset(jd, fd, ts_src, ts_dst))
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
#[pyclass]
struct Epoch {
    /// Stored object for underlying EOP
    obj: time::Epoch,
}

#[pymethods]
impl Epoch {
    fn __repr__(&self) -> String {
        format!("{:?}", self.obj)
    }

    fn __str__(&self) -> String {
        self.obj.to_string()
    }

    // Define attribute access methods
    /// `str`: Time system of Epoch. One of: "GPS", "TAI", "TT", "UTC", "UT1"
    #[getter]
    fn time_system(&self) -> String {
        time_system_to_string(self.obj.time_system)
    }

    #[classmethod]
    fn from_date(
        _cls: &PyType,
        year: u32,
        month: u8,
        day: u8,
        time_system: &str,
    ) -> PyResult<Epoch> {
        Ok(Epoch {
            obj: time::Epoch::from_date(
                year,
                month,
                day,
                string_to_time_system(time_system).unwrap(),
            ),
        })
    }

    #[classmethod]
    pub fn from_datetime(
        _cls: &PyType,
        year: u32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: f64,
        nanosecond: f64,
        time_system: &str,
    ) -> PyResult<Epoch> {
        Ok(Epoch {
            obj: time::Epoch::from_datetime(
                year,
                month,
                day,
                hour,
                minute,
                second,
                nanosecond,
                string_to_time_system(time_system).unwrap(),
            ),
        })
    }

    #[classmethod]
    pub fn from_string(_cls: &PyType, datestr: &str) -> PyResult<Epoch> {
        Ok(Epoch {
            obj: time::Epoch::from_string(datestr).unwrap(),
        })
    }

    #[classmethod]
    pub fn from_jd(_cls: &PyType, jd: f64, time_system: &str) -> PyResult<Epoch> {
        Ok(Epoch {
            obj: time::Epoch::from_jd(jd, string_to_time_system(time_system).unwrap()),
        })
    }

    #[classmethod]
    pub fn from_mjd(_cls: &PyType, mjd: f64, time_system: &str) -> PyResult<Epoch> {
        Ok(Epoch {
            obj: time::Epoch::from_mjd(mjd, string_to_time_system(time_system).unwrap()),
        })
    }

    #[classmethod]
    pub fn from_gps_date(_cls: &PyType, week: u32, seconds: f64) -> PyResult<Epoch> {
        Ok(Epoch {
            obj: time::Epoch::from_gps_date(week, seconds),
        })
    }

    #[classmethod]
    pub fn from_gps_seconds(_cls: &PyType, gps_seconds: f64) -> PyResult<Epoch> {
        Ok(Epoch {
            obj: time::Epoch::from_gps_seconds(gps_seconds),
        })
    }

    #[classmethod]
    pub fn from_gps_nanoseconds(_cls: &PyType, gps_nanoseconds: u64) -> PyResult<Epoch> {
        Ok(Epoch {
            obj: time::Epoch::from_gps_nanoseconds(gps_nanoseconds),
        })
    }

    pub fn to_datetime_as_tsys(&self, time_system: &str) -> (u32, u8, u8, u8, u8, f64, f64) {
        self.obj
            .to_datetime_as_tsys(string_to_time_system(time_system).unwrap())
    }

    pub fn to_datetime(&self) -> (u32, u8, u8, u8, u8, f64, f64) {
        self.obj.to_datetime()
    }

    pub fn jd_as_tsys(&self, time_system: &str) -> f64 {
        self.obj
            .jd_as_tsys(string_to_time_system(time_system).unwrap())
    }

    pub fn jd(&self) -> f64 {
        self.obj.jd()
    }

    pub fn mjd_as_tsys(&self, time_system: &str) -> f64 {
        self.obj
            .mjd_as_tsys(string_to_time_system(time_system).unwrap())
    }

    pub fn mjd(&self) -> f64 {
        self.obj.mjd()
    }

    pub fn gps_date(&self) -> (u32, f64) {
        self.obj.gps_date()
    }

    pub fn gps_seconds(&self) -> f64 {
        self.obj.gps_seconds()
    }

    pub fn gps_nanoseconds(&self) -> f64 {
        self.obj.gps_nanoseconds()
    }

    pub fn isostring(&self) -> String {
        self.obj.isostring()
    }

    pub fn isostringd(&self, decimals: usize) -> String {
        self.obj.isostringd(decimals)
    }

    pub fn to_string_as_tsys(&self, time_system: &str) -> String {
        self.obj
            .to_string_as_tsys(string_to_time_system(time_system).unwrap())
    }

    pub fn gast(&self, as_degrees: bool) -> f64 {
        self.obj.gast(as_degrees)
    }

    pub fn gmst(&self, as_degrees: bool) -> f64 {
        self.obj.gmst(as_degrees)
    }

    pub fn __add__(&self, other: f64) -> PyResult<Epoch> {
        Ok(Epoch {
            obj: self.obj + other,
        })
    }

    pub fn __iadd__(&mut self, other: f64) -> () {
        self.obj += other;
    }

    pub fn __sub__(&self, other: &Epoch) -> f64 {
        self.obj - other.obj
    }

    // pub fn __sub__(&self, other: f64) -> PyResult<Epoch> {
    //     Ok(Epoch {
    //         obj: self.obj - other,
    //     })
    // }

    // pub fn __sub__(&self, other: &PyAny) -> PyResult<PyAny> {
    //     if other.is_instance_of::<&Epoch>().unwrap() {
    //         let epc: Epoch = other.extract().unwrap();
    //         Ok((self.obj - epc.obj))
    //     } else {
    //         Err(TypeError::py_err(
    //             "Epoch subtractraction not implemented for this type.",
    //         ))
    //     }
    // }

    pub fn __isub__(&mut self, other: f64) -> () {
        self.obj -= other;
    }

    fn __richcmp__(&self, other: &Epoch, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => (self.obj == other.obj),
            CompareOp::Ne => (self.obj != other.obj),
            CompareOp::Ge => (self.obj >= other.obj),
            CompareOp::Gt => (self.obj > other.obj),
            CompareOp::Le => (self.obj <= other.obj),
            CompareOp::Lt => (self.obj < other.obj),
        }
    }
}

#[pyclass]
struct EpochRange {
    obj: time::EpochRange,
}

#[pymethods]
impl EpochRange {
    #[new]
    fn new(epoch_start: &Epoch, epoch_end: &Epoch, step: f64) -> Self {
        Self {
            obj: time::EpochRange::new(epoch_start.obj, epoch_end.obj, step),
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Epoch> {
        match slf.obj.next() {
            Some(e) => Some(Epoch { obj: e }),
            None => None,
        }
    }
}

////////////
// Frames //
////////////

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
// fn orbital_period(a: f64) -> PyResult<f64> {
//     Ok(orbits::orbital_period(a))
// }

// pub fn bias_precession_nutation(e: &Epoch)
// pub fn earth_rotation
// pub fn polar_motion
// pub fn rotation_eci_to_ecef
// pub fn rotation_ecef_to_eci

/////////////////////
// Transformations //
/////////////////////

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
#[pyfunction(as_degrees = "true")]
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
#[pyfunction(as_degrees = "true")]
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
#[pyfunction(as_degrees = "true")]
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
#[pyfunction(as_degrees = "true")]
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
        Err(exceptions::PyRuntimeError::new_err(res.err().unwrap()))
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
        Err(exceptions::PyRuntimeError::new_err(res.err().unwrap()))
    }
}

////////////
// Module //
////////////

// NOTE: All imports have to be defined as a single module otherwise PyO3 runs
// into conversion errors when classes are passed between modules

#[pymodule]
pub fn module(_py: Python, module: &PyModule) -> PyResult<()> {
    // Constants
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

    // EOP
    module.add_function(wrap_pyfunction!(download_c04_eop_file, module)?)?;
    module.add_function(wrap_pyfunction!(download_standard_eop_file, module)?)?;
    module.add_function(wrap_pyfunction!(set_global_eop_from_zero, module)?)?;
    module.add_function(wrap_pyfunction!(set_global_eop_from_static_values, module)?)?;
    module.add_function(wrap_pyfunction!(set_global_eop_from_c04_file, module)?)?;
    module.add_function(wrap_pyfunction!(set_global_eop_from_default_c04, module)?)?;
    module.add_function(wrap_pyfunction!(set_global_eop_from_standard_file, module)?)?;
    module.add_function(wrap_pyfunction!(
        set_global_eop_from_default_standard,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(get_global_ut1_utc, module)?)?;
    module.add_function(wrap_pyfunction!(get_global_pm, module)?)?;
    module.add_function(wrap_pyfunction!(get_global_dxdy, module)?)?;
    module.add_function(wrap_pyfunction!(get_global_lod, module)?)?;
    module.add_function(wrap_pyfunction!(get_global_eop, module)?)?;
    module.add_function(wrap_pyfunction!(get_global_eop_initialization, module)?)?;
    module.add_function(wrap_pyfunction!(get_global_eop_len, module)?)?;
    module.add_function(wrap_pyfunction!(get_global_eop_type, module)?)?;
    module.add_function(wrap_pyfunction!(get_global_eop_extrapolate, module)?)?;
    module.add_function(wrap_pyfunction!(get_global_eop_interpolate, module)?)?;
    module.add_function(wrap_pyfunction!(get_global_eop_mjd_min, module)?)?;
    module.add_function(wrap_pyfunction!(get_global_eop_mjd_max, module)?)?;
    module.add_function(wrap_pyfunction!(get_global_eop_mjd_last_lod, module)?)?;
    module.add_function(wrap_pyfunction!(get_global_eop_mjd_last_dxdy, module)?)?;

    // Time
    module.add_function(wrap_pyfunction!(datetime_to_jd, module)?)?;
    module.add_function(wrap_pyfunction!(datetime_to_mjd, module)?)?;
    module.add_function(wrap_pyfunction!(mjd_to_datetime, module)?)?;
    module.add_function(wrap_pyfunction!(jd_to_datetime, module)?)?;
    module.add_function(wrap_pyfunction!(time_system_offset, module)?)?;
    module.add_class::<Epoch>()?;
    module.add_class::<EpochRange>()?;

    // Orbits
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
