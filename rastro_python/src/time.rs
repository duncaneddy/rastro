use std::error::Error;
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use pyo3::wrap_pyfunction;
use rastro::time as time;
use rastro::time::TimeSystem;

use crate::eop::EarthOrientationData;

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
fn time_system_to_string(ts:TimeSystem) -> String {
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
                          eop: &PyAny) -> PyResult<f64> {

    // let res: PyResult<PyRef<EarthOrientationData>> = eop.extract();
    // if res.is_err() {
    //     println!("Error parsing object");
    // }
    // let eop: EarthOrientationData = eop.extract()?;
    let eop: EarthOrientationData = match eop.extract() {
        Ok(e) => e,
        Err(e) => {
            let typ = eop.get_type();
            println!("Object is of type: {}", typ);
            println!("isinstance(obj, EarthOrientationData): {}", eop.is_instance(typ).unwrap());
            Python::with_gil(|py| {
                println!("Could not convert object! {:?}", e);
                assert!(e.traceback(py).is_some());
                e.print(py);
            });
            return Ok(0.0)
            // return Ok(0.0)
        }
    };
    // let eop = match eop.extract::<'a, EarthOrientationData>() {
    //     Ok(i) => 0.0,
    //     Err(e) => {
    //         Python::with_gil(|py| {
    //                 e.print(py);
    //         });
    //         return Ok(0.0)
    //     }
    // };

    let ts_src = match str_to_time_system(time_system_src) {
        Ok(ts) => ts,
        Err(e) => return Err(e)
    };

    let ts_dst = match str_to_time_system(time_system_dest) {
        Ok(ts) => ts,
        Err(e) => return Err(e)
    };

    Ok(0.0)
    // Ok(time::time_system_offset(jd, fd, ts_src, ts_dst, &eop.robj))
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
    robj: time::Epoch<'a>,
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
    module.add_function(wrap_pyfunction!(datetime_to_jd, module)?)?;
    module.add_function(wrap_pyfunction!(datetime_to_mjd, module)?)?;
    module.add_function(wrap_pyfunction!(mjd_to_datetime, module)?)?;
    module.add_function(wrap_pyfunction!(jd_to_datetime, module)?)?;
    module.add_function(wrap_pyfunction!(time_system_offset, module)?)?;

    Ok(())
}