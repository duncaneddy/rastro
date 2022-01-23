use std::fmt;
use std::os::raw::{c_char, c_int};
use std::ffi::CString;
use regex::Regex;
use rsofa;
use crate::constants::{MJD_ZERO, TAI_GPS, GPS_TAI, TAI_TT, TT_TAI};
use crate::eop::EarthOrientationData;

const VALID_EPOCH_REGEX: [&str; 5] = [
    r"^(\d{4})\-(\d{2})\-(\d{2})$",
    r"^(\d{4})\-(\d{2})\-(\d{2})[T](\d{2}):(\d{2}):(\d{2})[Z]$",
    r"^(\d{4})\-(\d{2})\-(\d{2})[T](\d{2}):(\d{2}):(\d{2})[.](\d*)[Z]$",
    r"^(\d{4})(\d{2})(\d{2})[T](\d{2})(\d{2})(\d{2})[Z]$",
    r"^(\d{4})\-(\d{2})\-(\d{2})\s(\d{2}):(\d{2}):(\d{2})\.*\s*(\d*)\s*([A-Z]*)$",
];

/// Split f64 floating point number into whole and fractional part
fn split_f64(num:f64) -> (f64, f64) {
    (f64::trunc(num), f64::fract(num))
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
/// use rastro::time::jd_to_datetime;
/// let (year, month, day, hour, minute, second, nanosecond) = jd_to_datetime(2451545.0);
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
pub fn jd_to_datetime(jd: f64) -> (u32, u8, u8, u8, u8, f64, f64) {
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
/// use rastro::time::mjd_to_datetime;
/// let (year, month, day, hour, minute, second, nanosecond) = mjd_to_datetime(51544.5);
///
/// assert!(year == 2000);
/// assert!(month == 1);
/// assert!(day == 1);
/// assert!(hour == 12);
/// assert!(minute == 0);
/// assert!(second == 0.0);
/// assert!(nanosecond == 0.0);
/// ```
pub fn mjd_to_datetime(mjd: f64) -> (u32, u8, u8, u8, u8, f64, f64) {
    jd_to_datetime(mjd + MJD_ZERO)
}

/// Based on a JD/FD pair in the UTC time frame compute and return the UTC-TAI
/// offset
#[allow(temporary_cstring_as_ptr)]
fn utc_jdfd_to_utc_offset(jd:f64, fd:f64) -> f64 {
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

    dutc
}

/// Take initial JD/FD in the TAI time scale and return the offset to the UTC time scale
#[allow(temporary_cstring_as_ptr)]
fn tai_jdfd_to_utc_offset(jd:f64, fd:f64) -> f64 {
    // Initial UTC guess
    let mut u1 = jd;
    let mut u2 = fd;

    for _ in 0..3 {
        let mut d1 = 0.0;
        let mut d2 = 0.0;

        unsafe {
            rsofa::iauUtctai(u1, u2, &mut d1, &mut d2);
        }

        // Adjust UTC guess
        u1 += jd - d1;
        u2 += fd - d2;
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

    dutc
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
            offset += utc_jdfd_to_utc_offset(jd, fd);
        },
        TimeSystem::UT1 => {
            let dut1 = eop.get_ut1_utc((jd - MJD_ZERO) + fd);

            // UTC -> TAI offset
            offset += utc_jdfd_to_utc_offset(jd, fd - dut1);
            offset -= dut1;
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
            // Add TAI -> UTC correction to offset
            offset -= tai_jdfd_to_utc_offset(jd, fd + offset/86400.0);
        },
        TimeSystem::UT1 => {
            // Add TAI -> UTC correction to offset
            offset -= tai_jdfd_to_utc_offset(jd, fd + offset/86400.0);

            // Add UTC -> UT1 correction to offset
            offset += eop.get_ut1_utc(jd + fd + offset/86400.0 - MJD_ZERO);
        }
    }

    offset
}

fn align_epoch_data(days:u32, seconds:u32, nanoseconds:f64) -> (u32, u32, f64) {
    let mut d = days;
    let mut s = seconds;
    let mut ns = nanoseconds;

    while ns < 0.0 {
        ns += 1.0e9;

        // Ensure that there are seconds to remove from
        if s > 0 {
            s -= 1;
        } else {
            s += 86400;
            d -= 1;
        }
    }

    while ns >= 1.0e9 {
        ns -= 1.0e9;
        s += 1;
    }

    while s >= 86400 {
        s -= 86400;
        d += 1;
    }

    (d, s, ns)
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

impl fmt::Display for TimeSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeSystem::GPS => write!(f, "GPS"),
            TimeSystem::TAI => write!(f, "TAI"),
            TimeSystem::TT => write!(f, "TT"),
            TimeSystem::UTC => write!(f, "UTC"),
            TimeSystem::UT1 => write!(f, "UT1"),
        }
    }
}

/// Epoch representing a specific instant in time.
struct Epoch<'a> {
    /// Time system used to instantiate the Epoch. The time system will be used to
    /// format the display of results on output
    pub time_system: TimeSystem,
    /// Number of integer Julian Days in the TAI time system of the instant
    days: u32,
    /// Elapsed seconds. Possible values [0, 86400)
    seconds:u32,
    /// Elapsed fractional in the TAI time system of the instant. Possible values: (-1.0e-9, 1.0e9)
    nanoseconds: f64,
    /// Running compensation term from Kahan summation algorithm to account for lost low-order
    /// bits on long-running sums.
    nanoseconds_kc: f64,
    /// Reference to an Earth orientation data instance. This instance will be used to perform
    /// time system conversions
    eop: &'a EarthOrientationData
}

impl fmt::Display for Epoch<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (y,m,d,hh,mm,ss,ns) = self.to_datetime();
        write!(f, "{:4}-{:02}-{:02} {:02}:{:02}:{:06.3} {}", y, m, d, hh, mm, ss + ns/1.0e9, self.time_system.to_string())
    }
}

impl fmt::Debug for Epoch<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Epoch<{}, {}, {}, {}, {}>", self.days, self.seconds, self.nanoseconds, self.nanoseconds_kc, self.time_system.to_string())
    }
}

impl<'a> Epoch<'a> {
    pub fn from_date(year:u32, month:u8, day:u8, time_system: TimeSystem, eop: &'a EarthOrientationData)
                         -> Self {
        Epoch::from_datetime(year, month, day, 0, 0, 0.0, 0.0, time_system, eop)
    }

    #[allow(temporary_cstring_as_ptr)]
    pub fn from_datetime(year:u32, month:u8, day:u8, hour:u8, minute:u8, second:f64,
                        nanosecond:f64, time_system: TimeSystem, eop: &'a EarthOrientationData)
    -> Self {

        let mut jd:f64 = 0.0;
        let mut fd:f64 = 0.0;

        unsafe {
            // Seconds are not passed here due to addition of rounding errors
            // Their parsing is handled separately below
            rsofa::iauDtf2d(CString::new("TAI").unwrap().as_ptr() as *const c_char,
                            year as i32, month as i32, day as i32, hour as i32, minute as i32,
                            0.0, &mut jd as *mut f64, &mut fd as *mut f64);
        }

        // Get time system offset based on days and fractional days using SOFA
        let time_system_offset = time_system_offset(jd, fd, time_system, TimeSystem::TAI, eop);

        // Get whole seconds and fractional seconds part of offset
        let (woffset, foffset) = split_f64(time_system_offset);


        // Parse jd and fd separate whole and fractional days
        let (wjd, fjd) = split_f64(jd);
        let (wfd, ffd) = split_f64(fd);

        // Covert fractional days into total seconds while retaining fractional part
        let (ws, fs) = split_f64((fjd+ffd)*86400.0);

        // Aggregate Component pieces
        let mut days = (wjd + wfd) as u32; // This will always be positive
        let seconds:u32;

        if (ws + woffset + f64::trunc(second)) >= 0.0 {
            seconds = (ws + woffset + f64::trunc(second)) as u32;
        } else {
            days -= 1;
            seconds = (86400.0 + (ws + woffset + f64::trunc(second))) as u32;
        }

        let nanoseconds = nanosecond + (fs + foffset + f64::fract(second))*1.0e9;

        let (d, s, ns) = align_epoch_data(days, seconds, nanoseconds);

        Epoch {
            time_system,
            days: d,
            seconds: s,
            nanoseconds: ns,
            nanoseconds_kc: 0.0,
            eop
        }
    }

    pub fn from_string(datestr: &str, eop: &'a EarthOrientationData) -> Option<Self> {
        let year:u32;
        let month:u8;
        let day:u8;
        let hour:u8;
        let minute:u8;
        let second:f64;
        let nanosecond:f64;
        let time_system: TimeSystem;

        for regex in VALID_EPOCH_REGEX.into_iter() {
            if let Some(caps) = Regex::new(regex).unwrap().captures(datestr) {
                year = caps.get(1).map_or("", |s| s.as_str()).parse::<u32>().unwrap();
                month = caps.get(2).map_or("", |s| s.as_str()).parse::<u8>().unwrap();
                day = caps.get(3).map_or("", |s| s.as_str()).parse::<u8>().unwrap();

                if caps.len() >= 6 {
                    hour = caps.get(4).map_or("", |s| s.as_str()).parse::<u8>().unwrap();
                    minute = caps.get(5).map_or("", |s| s.as_str()).parse::<u8>().unwrap();
                    second = caps.get(6).map_or("", |s| s.as_str()).parse::<f64>().unwrap();

                    if caps.len() >= 8 {
                        let mut ns_str = caps.get(7).map_or("0.0", |s| s.as_str());
                        if ns_str.len() == 0 { ns_str = "0.0" }; // Some parses return a "" which causes issues for the below
                        nanosecond = ns_str.parse::<f64>().unwrap()*10_f64.powi((9 - ns_str.len() as u32).try_into().unwrap());

                        if caps.len() >= 9 {
                            time_system = match caps.get(8).map_or("", |s| s.as_str()) {
                                "GPS" => TimeSystem::GPS ,
                                "TAI" => TimeSystem::TAI ,
                                "TT" => TimeSystem::TT ,
                                "UTC" => TimeSystem::UTC ,
                                "UT1" => TimeSystem::UT1 ,
                                _ => return None
                            }
                        } else {
                            time_system = TimeSystem::UTC;
                        }
                    } else {
                        nanosecond = 0.0;
                        time_system = TimeSystem::UTC;
                    }
                } else {
                    hour = 0;
                    minute = 0;
                    second = 0.0;
                    nanosecond = 0.0;

                    // Valid ISO formatted regex strings are all UTC.
                    time_system = TimeSystem::UTC;
                }

                return Some(Epoch::from_datetime(year, month, day, hour, minute, second, nanosecond, time_system, eop))
            }
        }

        // If we have reached this point no match has been found
        None
    }

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

    #[allow(temporary_cstring_as_ptr)]
    pub fn to_datetime_as_tsys(&self, tsys:TimeSystem) -> (u32, u8, u8, u8, u8, f64, f64) {
        // Get JD / FD from Epoch
        let jd = self.days as f64;
        let fd = ((self.nanoseconds + self.nanoseconds_kc)/1.0e9 + self.seconds as f64)/86400.0;

        let offset = time_system_offset(jd, fd, TimeSystem::TAI, tsys, self.eop);
        let fd = fd + offset/86400.0;


        let mut iy:i32 = 0;
        let mut im:i32 = 0;
        let mut id:i32 = 0;
        let mut ihmsf: [c_int; 4] = [0; 4];

        unsafe {
            rsofa::iauD2dtf(CString::new(tsys.to_string()).unwrap().as_ptr() as *const c_char,
                            9, jd, fd, &mut iy, &mut im, &mut id, &mut ihmsf as *mut i32
            );
        }

        // Since ihmsf[3] returns an interger it does not represent time at a resolution finer than
        // nanoseconds. Therefore we directly add the fractional part of the nanoseconds fields
        let ns = ihmsf[3] as f64 + f64::fract(self.nanoseconds + self.nanoseconds_kc);
        (iy as u32, im as u8, id as u8, ihmsf[0] as u8, ihmsf[1] as u8, ihmsf[2] as f64, ns)
    }

    ///
    pub fn to_datetime(&self) -> (u32, u8, u8, u8, u8, f64, f64) {
        self.to_datetime_as_tsys(self.time_system)
    }
}

#[cfg(test)]
mod tests {
    use approx::{assert_abs_diff_eq};
    use std::path::Path;
    use std::env;

    use crate::constants::*;
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
    fn test_jd_to_datetime() {
        let (year, month, day, hour, minute, second, nanosecond) = jd_to_datetime(2451545.0);

        assert_eq!(year, 2000);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 12);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 0.0);
    }

    #[test]
    fn test_mjd_to_datetime() {
        let (year, month, day, hour, minute, second, nanosecond) = mjd_to_datetime(51544.5);

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

    #[test]
    fn test_epoch_display() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_c04_14.txt");
        let eop = EarthOrientationData::from_c04_file(filepath.to_str().unwrap(),
                                                      EOPExtrapolation::Hold, true).unwrap();

        let epc = Epoch::from_datetime(2020, 2, 3, 4, 5, 6.0, 0.0, TimeSystem::GPS, &eop);

        assert_eq!(epc.to_string(), "2020-02-03 04:05:06.000 GPS")
    }

    #[test]
    fn test_epoch_debug() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_c04_14.txt");
        let eop = EarthOrientationData::from_c04_file(filepath.to_str().unwrap(),
                                                      EOPExtrapolation::Hold, true).unwrap();

        let epc = Epoch::from_datetime(2020, 2, 3, 4, 5, 6.0, 0.0, TimeSystem::GPS, &eop);

        assert_eq!(format!("{:?}", epc), "Epoch<2458882, 57924, 999999999.9927241, 0, GPS>")
    }

    #[test]
    fn test_epoch_from_date() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_c04_14.txt");
        let eop = EarthOrientationData::from_c04_file(filepath.to_str().unwrap(),
                                                      EOPExtrapolation::Hold, true).unwrap();

        let epc = Epoch::from_date(2020, 1, 1, TimeSystem::GPS, &eop);

        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();

        assert_eq!(year, 2020);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 0.0);
    }

    #[test]
    fn test_epoch_from_datetime() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_c04_14.txt");
        let eop = EarthOrientationData::from_c04_file(filepath.to_str().unwrap(),
                                                      EOPExtrapolation::Hold, true).unwrap();

        // Test date initialization
        let epc = Epoch::from_datetime(2020, 1, 1, 0, 0, 0.0, 0.0, TimeSystem::TAI, &eop);

        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();

        assert_eq!(year, 2020);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 0.0);

        // Test initialization with seconds and nanoseconds
        let epc = Epoch::from_datetime(2020, 1, 1, 0, 0, 0.5, 1.2345, TimeSystem::TAI, &eop);

        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();

        assert_eq!(year, 2020);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 0.5*1.0e9 + 1.2345);
    }

    #[test]
    fn test_epoch_from_string() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_c04_14.txt");
        let eop = EarthOrientationData::from_c04_file(filepath.to_str().unwrap(),
                                                      EOPExtrapolation::Hold, true).unwrap();

        let epc = Epoch::from_string("2018-12-20", &eop).unwrap();
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2018);
        assert_eq!(month, 12);
        assert_eq!(day, 20);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::UTC);

        let epc = Epoch::from_string("2018-12-20T16:22:19.0Z", &eop).unwrap();
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2018);
        assert_eq!(month, 12);
        assert_eq!(day, 20);
        assert_eq!(hour, 16);
        assert_eq!(minute, 22);
        assert_eq!(second, 19.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::UTC);

        let epc = Epoch::from_string("2018-12-20T16:22:19.123Z", &eop).unwrap();
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2018);
        assert_eq!(month, 12);
        assert_eq!(day, 20);
        assert_eq!(hour, 16);
        assert_eq!(minute, 22);
        assert_eq!(second, 19.0);
        assert_eq!(nanosecond, 123000000.0);
        assert_eq!(epc.time_system, TimeSystem::UTC);

        let epc = Epoch::from_string("2018-12-20T16:22:19.123456789Z", &eop).unwrap();
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2018);
        assert_eq!(month, 12);
        assert_eq!(day, 20);
        assert_eq!(hour, 16);
        assert_eq!(minute, 22);
        assert_eq!(second, 19.0);
        assert_eq!(nanosecond, 123456789.0);
        assert_eq!(epc.time_system, TimeSystem::UTC);

        let epc = Epoch::from_string("2018-12-20T16:22:19Z", &eop).unwrap();
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2018);
        assert_eq!(month, 12);
        assert_eq!(day, 20);
        assert_eq!(hour, 16);
        assert_eq!(minute, 22);
        assert_eq!(second, 19.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::UTC);

        let epc = Epoch::from_string("20181220T162219Z", &eop).unwrap();
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2018);
        assert_eq!(month, 12);
        assert_eq!(day, 20);
        assert_eq!(hour, 16);
        assert_eq!(minute, 22);
        assert_eq!(second, 19.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::UTC);

        let epc = Epoch::from_string("2018-12-01 16:22:19 GPS", &eop).unwrap();
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2018);
        assert_eq!(month, 12);
        assert_eq!(day, 1);
        assert_eq!(hour, 16);
        assert_eq!(minute, 22);
        assert_eq!(second, 19.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::GPS);

        let epc = Epoch::from_string("2018-12-01 16:22:19.0 GPS", &eop).unwrap();
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2018);
        assert_eq!(month, 12);
        assert_eq!(day, 1);
        assert_eq!(hour, 16);
        assert_eq!(minute, 22);
        assert_eq!(second, 19.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::GPS);

        let epc = Epoch::from_string("2018-12-01 16:22:19.123 GPS", &eop).unwrap();
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2018);
        assert_eq!(month, 12);
        assert_eq!(day, 1);
        assert_eq!(hour, 16);
        assert_eq!(minute, 22);
        assert_eq!(second, 19.0);
        assert_eq!(nanosecond, 123000000.0);
        assert_eq!(epc.time_system, TimeSystem::GPS);

        let epc = Epoch::from_string("2018-12-01 16:22:19.123456789 GPS", &eop).unwrap();
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2018);
        assert_eq!(month, 12);
        assert_eq!(day, 1);
        assert_eq!(hour, 16);
        assert_eq!(minute, 22);
        assert_eq!(second, 19.0);
        assert_eq!(nanosecond, 123456789.0);
        assert_eq!(epc.time_system, TimeSystem::GPS);
    }
}