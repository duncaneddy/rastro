use std::os::raw::{c_char, c_int};
use std::ffi::CString;
use rsofa;
use crate::constants::{MJD_ZERO, TAI_GPS, GPS_TAI, TAI_TT, TT_TAI, GPS_TT, TT_GPS};
use crate::eop::EarthOrientationData;

/// Convert a Gregorian calendar date representation to the equivalent Modified Julian Date
/// representation of that same instant in time.
///
/// Note: Due to the ambiguity of the nature of leap second insertion, this
/// method should not be used if a specific behavior for leap second insertion is expected. This
/// method treats leap seconds as if they don't exist.
///
/// # Arguments
/// - `year`: Year
/// - `month`: Month
/// - `day`: Day
/// - `hour`: Hour
/// - `minute`: Minute
/// - `second`: Second
///
/// # Returns
/// - `jd` Julian date of epoch
///
/// # Examples
/// ```rust
/// use rastro::time::caldate_to_jd;
/// let jd = caldate_to_jd(2000, 1, 1, 12, 0, 0.0, 0.0);
///
/// assert!(jd == 2451545.0);
/// ```
#[allow(temporary_cstring_as_ptr)]
pub fn caldate_to_jd(year:u32, month:u8, day:u8, hour:u8, minute:u8, second:f64,
                     nanosecond:f64) -> f64 {

    let mut jd:f64 = 0.0;
    let mut fd:f64 = 0.0;


    unsafe {
        rsofa::iauDtf2d(CString::new("TAI").unwrap().as_ptr() as *const c_char,
                        year as i32, month as i32, day as i32, hour as i32, minute as i32,
                        second + nanosecond/1.0e9, &mut jd as *mut f64, &mut fd as *mut f64);
    }

    jd + fd
}

/// Convert a Gregorian calendar date representation to the equivalent Modified Julian Date
/// representation of that same instant in time.
///
/// Note: Due to the ambiguity of the nature of leap second insertion, this
/// method should not be used if a specific behavior for leap second insertion is expected. This
/// method treats leap seconds as if they don't exist.
///
/// # Arguments
/// - `year`: Year
/// - `month`: Month
/// - `day`: Day
/// - `hour`: Hour
/// - `minute`: Minute
/// - `second`: Second
///
/// # Returns
/// - `mjd` Modified Julian date of epoch
///
/// # Examples
/// ```rust
/// use rastro::time::caldate_to_mjd;
/// let mjd = caldate_to_mjd(2000, 1, 1, 12, 0, 0.0, 0.0);
///
/// assert!(mjd == 51544.5);
/// ```
#[allow(temporary_cstring_as_ptr)]
pub fn caldate_to_mjd(year:u32, month:u8, day:u8, hour:u8, minute:u8, second:f64,
                     nanosecond:f64) -> f64 {
    caldate_to_jd(year, month, day, hour, minute, second, nanosecond) - MJD_ZERO
}

/// Convert a Julian Date representation to the equivalent Gregorian calendar date representation
/// of that same instant in time.
///
/// Note: Due to the ambiguity of the nature of leap second insertion, this
/// method should not be used if a specific behavior for leap second insertion is expected. This
/// method treats leap seconds as if they don't exist.
///
/// # Arguments
/// - `jd` Modified Julian date of epoch
///
/// # Returns
/// - `year`: Year
/// - `month`: Month
/// - `day`: Day
/// - `hour`: Hour
/// - `minute`: Minute
/// - `second`: Second
///
/// # Examples
/// ```rust
/// use rastro::time::jd_to_caldate;
/// let (year, month, day, hour, minute, second, nanosecond) = jd_to_caldate(2451545.0);
///
/// assert!(year == 2000);
/// assert!(month == 1);
/// assert!(day == 1);
/// assert!(hour == 12);
/// assert!(minute == 0);
/// assert!(second == 0.0);
/// assert!(nanosecond == 0.0);
/// ```
#[allow(temporary_cstring_as_ptr)]
pub fn jd_to_caldate(jd: f64) -> (u32, u8, u8, u8, u8, f64, f64) {
    let mut iy:i32 = 0;
    let mut im:i32 = 0;
    let mut id:i32 = 0;
    let mut ihmsf: [c_int; 4] = [0; 4];

    unsafe {
        rsofa::iauD2dtf(CString::new("TAI").unwrap().as_ptr() as *const c_char,
            9, jd, 0.0, &mut iy, &mut im, &mut id, &mut ihmsf as *mut i32
        );
    }

    (iy as u32, im as u8, id as u8, ihmsf[0] as u8, ihmsf[1] as u8, ihmsf[2] as f64, ihmsf[3] as f64)
}

/// Convert a Julian Date representation to the equivalent Gregorian calendar date representation
/// of that same instant in time.
///
/// Note: Due to the ambiguity of the nature of leap second insertion, this
/// method should not be used if a specific behavior for leap second insertion is expected. This
/// method treats leap seconds as if they don't exist.
///
/// # Arguments
/// - `mjd` Modified Julian date of epoch
///
/// # Returns
/// - `year`: Year
/// - `month`: Month
/// - `day`: Day
/// - `hour`: Hour
/// - `minute`: Minute
/// - `second`: Second
///
/// # Examples
/// ```rust
/// use rastro::time::mjd_to_caldate;
/// let (year, month, day, hour, minute, second, nanosecond) = mjd_to_caldate(51544.5);
///
/// assert!(year == 2000);
/// assert!(month == 1);
/// assert!(day == 1);
/// assert!(hour == 12);
/// assert!(minute == 0);
/// assert!(second == 0.0);
/// assert!(nanosecond == 0.0);
/// ```
pub fn mjd_to_caldate(mjd: f64) -> (u32, u8, u8, u8, u8, f64, f64) {
    jd_to_caldate(mjd + MJD_ZERO)
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
/// Args:
///     jd (float): Part 1 of two-part date (Julian days)
///     fd (float): Part 2 of two-part date (Fractional days)
///     tsys_src (str): Base time system
///     tsys_dest (str): Destination time system
///
/// Returns:
///     offset (float): Offset between soruce and destination time systems in seconds.
///
/// Example:
/// ```rust
/// use rastro::constants::MJD_ZERO;
/// use rastro::eop::{EarthOrientationData, EOPExtrapolation, EOPType};
/// use rastro::time::{time_system_offset, TimeSystem};
///
/// // Load Standard EOP
/// let eop_extrapolation = EOPExtrapolation::Hold;
/// let eop_interpolation = true;
/// let eop_type = EOPType::StandardBulletinA;
/// let eop = EarthOrientationData::from_default_standard(eop_extrapolation, eop_interpolation, eop_type);
///
/// // Get offset between GPS time and UT1 for 0h 2020-03-01
/// let offset = time_system_offset(58909.0 + MJD_ZERO, 0.0, TimeSystem::GPS, TimeSystem::UT1, &eop);
/// ```
#[allow(temporary_cstring_as_ptr)]
pub fn time_system_offset(jd: f64, fd: f64,
                          time_system_src: TimeSystem, time_system_dst: TimeSystem,
                          eop: &EarthOrientationData) -> f64 {
    if time_system_src == time_system_dst {
        return 0.0
    }

    let mut offset: f64 = 0.0;

    // Convert from source representation to TAI time system
    match time_system_src {
        TimeSystem::GPS => {
            offset += TAI_GPS;
        },
        TimeSystem::TAI => {
            offset += 0.0;
        },
        TimeSystem::TT => {
            offset += TAI_TT;
        },
        TimeSystem::UTC => {
            let mut iy:i32 = 0;
            let mut im:i32 = 0;
            let mut id:i32 = 0;
            let mut ihmsf: [c_int; 4] = [0; 4];
            let mut dutc: f64 = 0.0;

            // Convert jd/fd to year, month, day hour, minute, second.
            unsafe {
                // Get year, month, day and hour, minute, second correctly given UTC
                rsofa::iauD2dtf(CString::new("UTC").unwrap().as_ptr() as *const c_char,
                                9, jd, fd, &mut iy, &mut im, &mut id, &mut ihmsf as *mut i32
                );

                // Get utc offset
                let seconds = (ihmsf[0]*3600 + ihmsf[1]*60 + ihmsf[2]) as f64 + (ihmsf[3] as f64)/1.0e9;
                rsofa::iauDat(iy, im, id, seconds/86400.0, &mut dutc);
            }

            offset += dutc;
        },
        TimeSystem::UT1 => {
            let mut iy:i32 = 0;
            let mut im:i32 = 0;
            let mut id:i32 = 0;
            let mut ihmsf: [c_int; 4] = [0; 4];
            let mut dutc: f64 = 0.0;

            // Convert jd/fd to year, month, day hour, minute, second.
            unsafe {
                // Get year, month, day and hour, minute, second correctly given UTC
                rsofa::iauD2dtf(CString::new("UTC").unwrap().as_ptr() as *const c_char,
                                9, jd, fd, &mut iy, &mut im, &mut id, &mut ihmsf as *mut i32
                );

                // Get utc offset
                let seconds = (ihmsf[0]*3600 + ihmsf[1]*60 + ihmsf[2]) as f64 + (ihmsf[3] as f64)/1.0e9;
                rsofa::iauDat(iy, im, id, seconds/86400.0, &mut dutc);
            }

            // UTC -> TAI offset
            offset += dutc;

            // UT1 -> UTC offset
            offset -= eop.get_ut1_utc((jd - MJD_ZERO) + fd);
        }
    }

    match time_system_dst {
        TimeSystem::GPS => {
            offset += GPS_TAI;
        },
        TimeSystem::TAI => {
            offset += 0.0;
        },
        TimeSystem::TT => {
            offset += TT_TAI;
        },
        TimeSystem::UTC => {
            // Initial UTC guess
            let mut u1 = jd;
            let mut u2 = fd + offset/86400.0;

            for i in 0..3 {
                let mut d1 = 0.0;
                let mut d2 = 0.0;

                unsafe {
                    rsofa::iauUtctai(u1, u2, &mut d1, &mut d2);
                }
            }

            let mut iy:i32 = 0;
            let mut im:i32 = 0;
            let mut id:i32 = 0;
            let mut ihmsf: [c_int; 4] = [0; 4];
            let mut dutc: f64 = 0.0;

            unsafe {
                // Compute calendar date from two-part date
                rsofa::iauD2dtf(CString::new("UTC").unwrap().as_ptr() as *const c_char,
                                9, jd, fd, &mut iy, &mut im, &mut id, &mut ihmsf as *mut i32
                );

                // Get utc offset
                let seconds = (ihmsf[0]*3600 + ihmsf[1]*60 + ihmsf[2]) as f64 + (ihmsf[3] as f64)/1.0e9;
                rsofa::iauDat(iy, im, id, seconds/86400.0, &mut dutc);
            }

            offset -= dutc;

        },
        TimeSystem::UT1 => {
            // Initial UTC guess
            let mut u1 = jd;
            let mut u2 = fd + offset/86400.0;

            for i in 0..3 {
                let mut d1 = 0.0;
                let mut d2 = 0.0;

                unsafe {
                    rsofa::iauUtctai(u1, u2, &mut d1, &mut d2);
                }
            }

            let mut iy:i32 = 0;
            let mut im:i32 = 0;
            let mut id:i32 = 0;
            let mut ihmsf: [c_int; 4] = [0; 4];
            let mut dutc: f64 = 0.0;

            unsafe {
                // Compute calendar date from two-part date
                rsofa::iauD2dtf(CString::new("UTC").unwrap().as_ptr() as *const c_char,
                                9, jd, fd, &mut iy, &mut im, &mut id, &mut ihmsf as *mut i32
                );

                // Get utc offset
                let seconds = (ihmsf[0]*3600 + ihmsf[1]*60 + ihmsf[2]) as f64 + (ihmsf[3] as f64)/1.0e9;
                rsofa::iauDat(iy, im, id, seconds/86400.0, &mut dutc);
            }

            offset -= dutc;

            offset += eop.get_ut1_utc(u1 + u2 + offset/86400.0 - MJD_ZERO);
        }
    }

    offset
}

/// Enumeration of different time systems
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TimeSystem {
    GPS,
    TAI,
    TT,
    UTC,
    UT1
}

/// Epoch representing a specific instant in time.
struct Epoch<'a> {
    /// Time system used to instantiate the Epoch. The time system will be used to
    /// format the display of results on output
    pub time_system: TimeSystem,
    /// Number of integer Julian Days in the TAI time system of the instant
    days: u32,
    /// Elapsed seconds in the TAI time system of the instant
    nanoseconds: f64,
    /// Running compensation term from Kahan summation algorithm to account for lost low-order
    /// bits on long-running sums.
    nanoseconds_kc: f64,
    /// Reference to an Earth orientation data instance. This instance will be used to perform
    /// time system conversions
    eop: &'a EarthOrientationData
}

// impl Epoch {
//     pub fn from_caldate(year:u32, month:u8, day:u8, hour:u8, minute:u8, second:f64,
//                         nanosecond:f64, time_system: TimeSystem, eop: &EarthOrientationData)
//     -> Self {
//
//     }
//
//     pub fn from_ymd(year:u32, month:u8, day:u8, time_system: TimeSystem, eop:
// &EarthOrientationData) -> Self {
//
//     }
//
//     pub fn from_string(datestr: &str) -> Self {
//
//     }
//
//     pub fn from_mjd(mjd: f64) -> Self {
//
//     }
//
//     pub fn from_jd(jd: f64) -> Self {
//
//     }
//
//     pub fn from_gps_weekseconds(week: u32, seconds: f64) -> Self {
//
//     }
// }

#[cfg(test)]
mod tests {
    use approx::{assert_abs_diff_eq};
    use std::path::Path;
    use std::env;

    // use crate::constants::*;
    use crate::eop::{EarthOrientationData, EOPExtrapolation};
    use crate::time::*;



    #[test]
    fn test_caldate_to_jd() {
        assert_eq!(caldate_to_jd(2000, 1, 1, 12, 0, 0.0, 0.0), 2451545.0);
    }

    #[test]
    fn test_caldate_to_mjd() {
        assert_eq!(caldate_to_mjd(2000, 1, 1, 12, 0, 0.0, 0.0), 51544.5);
    }

    #[test]
    fn test_jd_to_caldate() {
        let (year, month, day, hour, minute, second, nanosecond) = jd_to_caldate(2451545.0);

        assert_eq!(year, 2000);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 12);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 0.0);
    }

    #[test]
    fn test_mjd_to_caldate() {
        let (year, month, day, hour, minute, second, nanosecond) = mjd_to_caldate(51544.5);

        assert_eq!(year, 2000);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 12);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 0.0);
    }

    #[test]
    fn test_time_system_offset() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_c04_14.txt");
        let eop = EarthOrientationData::from_c04_file(filepath.to_str().unwrap(),
                                                             EOPExtrapolation::Hold, true).unwrap();

        // Test date
        let jd = caldate_to_jd(2018, 6, 1, 0, 0, 0.0, 0.0);

        // UTC - TAI offset
        let dutc = -37.0;
        let dut1 = 0.0769859;

        // GPS
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::GPS, TimeSystem::GPS, &eop), 0.0);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::GPS, TimeSystem::TT,  &eop), TT_GPS);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::GPS, TimeSystem::UTC, &eop), dutc + TAI_GPS);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::GPS, TimeSystem::UT1, &eop), dutc + TAI_GPS + dut1, epsilon=1e-6);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::GPS, TimeSystem::TAI, &eop), TAI_GPS);

        // TT
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TT, TimeSystem::GPS, &eop), GPS_TT);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TT, TimeSystem::TT,  &eop), 0.0);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TT, TimeSystem::UTC, &eop), dutc + TAI_TT);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TT, TimeSystem::UT1, &eop), dutc + TAI_TT + dut1, epsilon=1e-6);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TT, TimeSystem::TAI, &eop), TAI_TT);

        // UTC
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UTC, TimeSystem::GPS, &eop), -dutc + GPS_TAI);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UTC, TimeSystem::TT,  &eop), -dutc + TT_TAI);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UTC, TimeSystem::UTC, &eop), 0.0);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UTC, TimeSystem::UT1, &eop), dut1, epsilon=1e-6);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UTC, TimeSystem::TAI, &eop), -dutc);

        // UT1
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UT1, TimeSystem::GPS, &eop), -dutc + GPS_TAI - dut1, epsilon=1e-6);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UT1, TimeSystem::TT,  &eop), -dutc + TT_TAI - dut1, epsilon=1e-6);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UT1, TimeSystem::UTC, &eop), -dut1, epsilon=1e-6);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UT1, TimeSystem::UT1, &eop), 0.0, epsilon=1e-6);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::UT1, TimeSystem::TAI, &eop), -dutc - dut1, epsilon=1e-6);

        // TAI
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TAI, TimeSystem::GPS, &eop), GPS_TAI);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TAI, TimeSystem::TT,  &eop), TT_TAI);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TAI, TimeSystem::UTC, &eop), dutc);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TAI, TimeSystem::UT1, &eop), dutc + dut1, epsilon=1e-6);
        assert_abs_diff_eq!(time_system_offset(jd, 0.0, TimeSystem::TAI, TimeSystem::TAI, &eop), 0.0);
    }
}