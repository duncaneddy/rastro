use std::fmt;
use std::os::raw::{c_char, c_int};
use std::ffi::CString;
use std::ops;
use std::cmp::Ordering;
use std::f64::consts::PI;
use regex::Regex;
use rsofa;
use crate::constants::{MJD_ZERO, TAI_GPS, GPS_TAI, TAI_TT, TT_TAI, GPS_ZERO};
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

/// Align days, seconds, and nanoseconds to expected time ranges.
///
/// Given an input of any arbitrary integer days, floating point seconds, and
/// floating point nanoseconds, compute and updated set of days, seconds, and
/// nanoseconds that align the values to expected ranges stored in the Epoch class.
/// The expected ranges are:
///     - days [0, ∞)
///     - seconds [0, 86400)
///     - nanoseconds [0, 1_000_000_000)
///
/// Misalignment can occur during arthemetic operations on this date tuple (adding or subtracing
/// seconds for time system conversion or alignment.
fn align_dsns(days:u32, seconds:u32, nanoseconds:f64) -> (u32, u32, f64){
    let mut days = days;
    let mut seconds = seconds;
    let mut nanoseconds = nanoseconds;

    // First pass checking for values out-of-range below (negative)
    while nanoseconds < 0.0 {
        if seconds == 0 {
            days -= 1;
            seconds += 86400;
        }

        seconds -= 1;
        nanoseconds += 1.0e9;
    }

    // Second pass checking for things out of range above
    while nanoseconds >= 1.0e9 {
        nanoseconds -= 1.0e9;
        seconds += 1;
    }

    while seconds >= 86400 {
        seconds -= 86400;
        days += 1;
    }

    (days, seconds, nanoseconds)
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
///     time_system_src (str): Base time system
///     time_system_dest (str): Destination time system
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
#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Copy, Clone)]
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

    // Constructors
    //
    // Because Epoch internally stores the data representation in terms of days, seconds, and
    // nanoseconds as (u32, u32, f64). It is important to ensure that when initializing the
    // time representation that any subtraction due to time-system offset conversion or
    // from changes from arithmetic operations does not result in subtraction from a u32 below
    // 0. Additionally, when initializing the Epoch object it is important to ensure that
    // that factional date components are properly handled to retain resolution and assign
    // the time value to the appropriate storage range.
    //
    // The intended storage ranges are:
    //     - days [0, ∞)
    //     - seconds [0, 86400)
    //     - nanoseconds [0, 1_000_000_000)
    //
    // There when initializing or altering Epoch objects it is important to ensure that the
    // final object at the end of the operations results in a time representation with values
    // aligned to the above ranges

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

    pub fn from_jd(jd: f64, time_system:TimeSystem, eop: &'a EarthOrientationData) -> Self {
        // Get time system offset of JD to TAI
        let time_system_offset = time_system_offset(jd, 0.0, time_system, TimeSystem::TAI, eop);

        // Add offset to JD and split into days, seconds, and nano-seconds
        let jd = jd + time_system_offset/86400.0;

        let (days, fdays) = split_f64(jd);
        let total_seconds = fdays*86400.0;
        let (seconds, fseconds) = split_f64(total_seconds);
        let ns = fseconds * 1.0e9;


        Epoch {
            time_system,
            days: days as u32,
            seconds: seconds as u32,
            nanoseconds: ns,
            nanoseconds_kc: 0.0,
            eop
        }
    }

    pub fn from_mjd(mjd: f64, time_system:TimeSystem, eop: &'a EarthOrientationData) -> Self {
        Epoch::from_jd(mjd + MJD_ZERO, time_system, eop)
    }

    pub fn from_gps_date(week: u32, seconds: f64, eop: &'a EarthOrientationData) -> Self {
        // Get time system offset based on days and fractional days using SOFA
        let jd = MJD_ZERO + GPS_ZERO + 7.0*f64::from(week) + (seconds / 86400.0).floor();
        let mut days = f64::trunc(jd);
        let fd = (seconds % 86400.0) / 86400.0;
        let time_system_offset = time_system_offset(days, fd, TimeSystem::GPS, TimeSystem::TAI, eop);

        // Get days, seconds, nanoseconds
        let mut seconds = seconds % 86400.0 + f64::fract(jd)*86400.0 + time_system_offset;

        while seconds < 0.0 {
            days -= 1.0;
            seconds += 86400.0;
        }

        Epoch {
            time_system: TimeSystem::GPS,
            days: days as u32,
            seconds: f64::trunc(seconds) as u32,
            nanoseconds: f64::fract(seconds)*1.0e9,
            nanoseconds_kc: 0.0,
            eop
        }
    }

    pub fn from_gps_seconds(gps_seconds: f64, eop: &'a EarthOrientationData) -> Self {
        // Get time system offset based on days and fractional days using SOFA
        let jd = MJD_ZERO + GPS_ZERO + (gps_seconds / 86400.0).floor();
        let mut days = f64::trunc(jd);
        let fd = (gps_seconds % 86400.0) / 86400.0 + f64::fract(jd);
        let time_system_offset = time_system_offset(days, fd, TimeSystem::GPS, TimeSystem::TAI, eop);

        // Get days, seconds, nanoseconds
        let mut seconds = gps_seconds % 86400.0 + f64::fract(jd)*86400.0 + time_system_offset;

        while seconds < 0.0 {
            days -= 1.0;
            seconds += 86400.0;
        }

        Epoch {
            time_system: TimeSystem::GPS,
            days: days as u32,
            seconds: f64::trunc(seconds) as u32,
            nanoseconds: f64::fract(seconds)*1.0e9,
            nanoseconds_kc: 0.0,
            eop
        }
    }

    pub fn from_gps_nanoseconds(gps_nanoseconds: u64, eop: &'a EarthOrientationData) -> Self {
        let gps_seconds = (gps_nanoseconds / 1_000_000_000) as f64;
        let jd = MJD_ZERO + GPS_ZERO + (gps_seconds / 86400.0).floor();
        let mut days = f64::trunc(jd);
        let fd = (gps_seconds % 86400.0) / 86400.0 + f64::fract(jd);
        let time_system_offset = time_system_offset(days, fd, TimeSystem::GPS, TimeSystem::TAI, eop);

        // Get days, seconds, nanoseconds
        let mut seconds = gps_seconds % 86400.0 + f64::fract(jd)*86400.0 + time_system_offset;

        while seconds < 0.0 {
            days -= 1.0;
            seconds += 86400.0;
        }

        let mut ns  = f64::fract(seconds)*1.0e9;
        if gps_nanoseconds > 1_000_000_000 {
            ns += (gps_nanoseconds % 1_000_000_000) as f64;
        }

        Epoch {
            time_system: TimeSystem::GPS,
            days: days as u32,
            seconds: f64::trunc(seconds) as u32,
            nanoseconds: ns,
            nanoseconds_kc: 0.0,
            eop
        }
    }

    fn get_jdfd(&self, time_system:TimeSystem) -> (f64, f64) {
        // Get JD / FD from Epoch
        let jd = self.days as f64;
        let fd = ((self.nanoseconds)/1.0e9 + self.seconds as f64)/86400.0;

        let offset = time_system_offset(jd, fd, TimeSystem::TAI, time_system, self.eop);
        let fd = fd + offset/86400.0;

        (jd, fd)
    }

    #[allow(temporary_cstring_as_ptr)]
    pub fn to_datetime_as_tsys(&self, time_system:TimeSystem) -> (u32, u8, u8, u8, u8, f64, f64) {
        // Get JD / FD from Epoch
        let (jd, fd) = self.get_jdfd(time_system);


        let mut iy:i32 = 0;
        let mut im:i32 = 0;
        let mut id:i32 = 0;
        let mut ihmsf: [c_int; 4] = [0; 4];

        unsafe {
            rsofa::iauD2dtf(CString::new(time_system.to_string()).unwrap().as_ptr() as *const c_char,
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

    pub fn jd_as_tsys(&self, time_system:TimeSystem) -> f64 {
        let (jd, fd) = self.get_jdfd(time_system);

        jd + fd
    }

    pub fn jd(&self) -> f64 {
        self.jd_as_tsys(self.time_system)
    }

    pub fn mjd_as_tsys(&self, time_system:TimeSystem) -> f64 {
        let (jd, fd) = self.get_jdfd(time_system);

        (jd - MJD_ZERO) + fd
    }

    pub fn mjd(&self) -> f64 {
        self.mjd_as_tsys(self.time_system)
    }

    pub fn gps_date(&self) -> (u32, f64) {
        let mjd = self.mjd_as_tsys(TimeSystem::GPS);

        let gps_week = ((mjd - GPS_ZERO)/7.0).floor();
        let gps_seconds = mjd - GPS_ZERO - gps_week*7.0;

        (gps_week as u32, gps_seconds*86400.0)
    }

    pub fn gps_seconds(&self) -> f64 {
        let (jd, fd) = self.get_jdfd(TimeSystem::GPS);

        (jd - MJD_ZERO - GPS_ZERO + fd) * 86400.0
    }

    pub fn gps_nanoseconds(&self) -> f64 {
        self.gps_seconds() * 1.0e9
    }

    pub fn iso_string(&self) -> String {
        String::from("placeholder")
    }

    pub fn to_string_as_tsys(&self, time_system:TimeSystem) -> String {
        String::from("placeholder")
    }

    // pub fn to_string(&self) -> String {
    //     String::from("placeholder")
    // }

    pub fn gast(&self, as_degrees: bool) -> f64 {
        let (uta, utb) = self.get_jdfd(TimeSystem::UT1);
        let (tta, ttb) = self.get_jdfd(TimeSystem::TT);

        let gast;

        unsafe {
            gast = rsofa::iauGst06a(uta, utb, tta, ttb);
        }

        if as_degrees {
            gast * 180.0 / PI
        } else {
            gast
        }
    }

    pub fn gmst(&self, as_degrees: bool) -> f64 {
        let (uta, utb) = self.get_jdfd(TimeSystem::UT1);
        let (tta, ttb) = self.get_jdfd(TimeSystem::TT);

        let gast;

        unsafe {
            gast = rsofa::iauGmst06(uta, utb, tta, ttb);
        }

        if as_degrees {
            gast * 180.0 / PI
        } else {
            gast
        }
    }
}

//
// Epoch Arithmetic Operators
//

impl<'a> ops::AddAssign<f64> for Epoch<'a> {
    fn add_assign(&mut self, f: f64) {
        // Kahan summation algorithm to compensate for floating-point arthimetic errors
        let y = (f as f64)*1.0e9 + self.nanoseconds_kc;
        let t = self.nanoseconds + y;
        let nanoseconds_kc = y - (t - self.nanoseconds);
        let nanoseconds    = t;

        let (days, seconds, nanoseconds) = align_dsns(self.days, self.seconds, nanoseconds);


        *self = Self {
            time_system: self.time_system,
            days,
            seconds,
            nanoseconds,
            nanoseconds_kc,
            eop: self.eop
        };
    }
}

impl<'a> ops::AddAssign<f32> for Epoch<'a> {
    fn add_assign(&mut self, f: f32) {
        *self += f as f64;
    }
}

impl<'a> ops::AddAssign<u8> for Epoch<'a> {
    fn add_assign(&mut self, f: u8) {
        *self += f as f64;
    }
}

impl<'a> ops::AddAssign<u16> for Epoch<'a> {
    fn add_assign(&mut self, f: u16) {
        *self += f as f64;
    }
}

impl<'a> ops::AddAssign<u32> for Epoch<'a> {
    fn add_assign(&mut self, f: u32) {
        *self += f as f64;
    }
}

impl<'a> ops::AddAssign<u64> for Epoch<'a> {
    fn add_assign(&mut self, f: u64) {
        *self += f as f64;
    }
}

impl<'a> ops::AddAssign<i8> for Epoch<'a> {
    fn add_assign(&mut self, f: i8) {
        *self += f as f64;
    }
}

impl<'a> ops::AddAssign<i16> for Epoch<'a> {
    fn add_assign(&mut self, f: i16) {
        *self += f as f64;
    }
}

impl<'a> ops::AddAssign<i32> for Epoch<'a> {
    fn add_assign(&mut self, f: i32) {
        *self += f as f64;
    }
}

impl<'a> ops::AddAssign<i64> for Epoch<'a> {
    fn add_assign(&mut self, f: i64) {
        *self += f as f64;
    }
}

impl<'a> ops::SubAssign<f64> for Epoch<'a> {
    fn sub_assign(&mut self, f: f64) {
        *self += -(f as f64);
    }
}

impl<'a> ops::SubAssign<f32> for Epoch<'a> {
    fn sub_assign(&mut self, f: f32) {
        *self += -(f as f64);
    }
}

impl<'a> ops::SubAssign<u8> for Epoch<'a> {
    fn sub_assign(&mut self, f: u8) {
        *self += -(f as f64);
    }
}

impl<'a> ops::SubAssign<u16> for Epoch<'a> {
    fn sub_assign(&mut self, f: u16) {
        *self += -(f as f64);
    }
}

impl<'a> ops::SubAssign<u32> for Epoch<'a> {
    fn sub_assign(&mut self, f: u32) {
        *self += -(f as f64);
    }
}

impl<'a> ops::SubAssign<u64> for Epoch<'a> {
    fn sub_assign(&mut self, f: u64) {
        *self += -(f as f64);
    }
}

impl<'a> ops::SubAssign<i8> for Epoch<'a> {
    fn sub_assign(&mut self, f: i8) {
        *self += -(f as f64);
    }
}

impl<'a> ops::SubAssign<i16> for Epoch<'a> {
    fn sub_assign(&mut self, f: i16) {
        *self += -(f as f64);
    }
}

impl<'a> ops::SubAssign<i32> for Epoch<'a> {
    fn sub_assign(&mut self, f: i32) {
        *self += -(f as f64);
    }
}

impl<'a> ops::SubAssign<i64> for Epoch<'a> {
    fn sub_assign(&mut self, f: i64) {
        *self += -(f as f64);
    }
}

impl<'a> ops::Add<f64> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn add(self, f: f64) -> Epoch<'a> {
        // Kahan summation algorithm to compensate for floating-point arthimetic errors
        let y = (f as f64)*1.0e9 + self.nanoseconds_kc;
        let t = self.nanoseconds + y;
        let nanoseconds_kc = y - (t - self.nanoseconds);
        let nanoseconds    = t;

        let (days, seconds, nanoseconds) = align_dsns(self.days, self.seconds, nanoseconds);


        Epoch {
            time_system: self.time_system,
            days,
            seconds,
            nanoseconds,
            nanoseconds_kc,
            eop: self.eop
        }
    }
}

impl<'a> ops::Add<f32> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn add(self, f: f32) -> Epoch<'a> {
        self + (f as f64)
    }
}

impl<'a> ops::Add<u8> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn add(self, f: u8) -> Epoch<'a> {
        self + (f as f64)
    }
}

impl<'a> ops::Add<u16> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn add(self, f: u16) -> Epoch<'a> {
        self + (f as f64)
    }
}

impl<'a> ops::Add<u32> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn add(self, f: u32) -> Epoch<'a> {
        self + (f as f64)
    }
}

impl<'a> ops::Add<u64> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn add(self, f: u64) -> Epoch<'a> {
        self + (f as f64)
    }
}

impl<'a> ops::Add<i8> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn add(self, f: i8) -> Epoch<'a> {
        self + (f as f64)
    }
}

impl<'a> ops::Add<i16> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn add(self, f: i16) -> Epoch<'a> {
        self + (f as f64)
    }
}

impl<'a> ops::Add<i32> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn add(self, f: i32) -> Epoch<'a> {
        self + (f as f64)
    }
}

impl<'a> ops::Add<i64> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn add(self, f: i64) -> Epoch<'a> {
        self + (f as f64)
    }
}

impl<'a> ops::Sub<Epoch<'a>> for Epoch<'a> {
    type Output = f64;

    fn sub(self, other: Epoch) -> f64 {
        (((self.days as i64 - other.days as i64)*86400) as f64) + ((self.seconds as i64 - other.seconds as i64) as f64) +
        (self.nanoseconds - other.nanoseconds)*1.0e-9 +
        (self.nanoseconds_kc - other.nanoseconds_kc)*1.0e-9
    }
}

impl<'a> ops::Sub<f64> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn sub(self, f: f64) -> Epoch<'a> {
        self + -(f as f64)
    }
}

impl<'a> ops::Sub<f32> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn sub(self, f: f32) -> Epoch<'a> {
        self + -(f as f64)
    }
}

impl<'a> ops::Sub<u8> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn sub(self, f: u8) -> Epoch<'a> {
        self + -(f as f64)
    }
}

impl<'a> ops::Sub<u16> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn sub(self, f: u16) -> Epoch<'a> {
        self + -(f as f64)
    }
}

impl<'a> ops::Sub<u32> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn sub(self, f: u32) -> Epoch<'a> {
        self + -(f as f64)
    }
}

impl<'a> ops::Sub<u64> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn sub(self, f: u64) -> Epoch<'a> {
        self + -(f as f64)
    }
}

impl<'a> ops::Sub<i8> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn sub(self, f: i8) -> Epoch<'a> {
        self + -(f as f64)
    }
}

impl<'a> ops::Sub<i16> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn sub(self, f: i16) -> Epoch<'a> {
        self + -(f as f64)
    }
}

impl<'a> ops::Sub<i32> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn sub(self, f: i32) -> Epoch<'a> {
        self + -(f as f64)
    }
}

impl<'a> ops::Sub<i64> for Epoch<'a> {
    type Output = Epoch<'a>;

    fn sub(self, f: i64) -> Epoch<'a> {
        self + -(f as f64)
    }
}

//
// Epoch Arithmetic Operators
//

impl<'a> PartialEq for Epoch<'a> {
    fn eq(&self, other: &Self) -> bool {
        (self.days == other.days) &&
            (self.seconds == other.seconds) &&
            (((self.nanoseconds + self.nanoseconds_kc) - (other.nanoseconds + other.nanoseconds_kc)).abs() < 1.0e-6)
    }
}

impl<'a> Eq for Epoch<'a> { }

impl<'a> PartialOrd for Epoch<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Epoch<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.days < other.days) || ((self.days == other.days) && (self.seconds < other.seconds)) ||
            ((self.days == other.days) && (self.seconds == other.seconds) && ((self.nanoseconds + self.nanoseconds_kc) < (other.nanoseconds + other.nanoseconds_kc))) {
            Ordering::Less
        } else if (self.days > other.days) || ((self.days == other.days) && (self.seconds > other.seconds)) ||
            ((self.days == other.days) && (self.seconds == other.seconds) && ((self.nanoseconds + self.nanoseconds_kc) > (other.nanoseconds + other.nanoseconds_kc))) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

// EpochRange

struct EpochRange<'a> {
    epoch_current: Epoch<'a>,
    epoch_end: Epoch<'a>,
    step: f64,
    positive_step: bool
}

impl<'a> EpochRange<'a> {
    /// Constructor for OneToTen.
    fn new(epoch_start:Epoch<'a>, epoch_end:Epoch<'a>, step:f64) -> Self {

        Self {
            epoch_current: epoch_start.clone(),
            epoch_end,
            step: step.abs(),
            positive_step: epoch_end > epoch_start
        }
    }
}

impl<'a> Iterator for EpochRange<'a> {
    type Item = Epoch<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.epoch_end != self.epoch_current {
            // Grab current epoch to return prior to advancing
            let epc = self.epoch_current.clone();

            let rem = (self.epoch_end - self.epoch_current).abs();
            let h = if self.step < rem {self.step} else {rem};

            if self.positive_step {
                self.epoch_current += h;
            } else {
                self.epoch_current -= h;
            }

            Some(epc)
        } else {
            None
        }
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

    fn setup_eop() -> EarthOrientationData {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_c04_14.txt");
        let eop = EarthOrientationData::from_c04_file(filepath.to_str().unwrap(),
                                                      EOPExtrapolation::Hold, true).unwrap();

        eop
    }


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
        let eop = setup_eop();

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
        let eop = setup_eop();

        let epc = Epoch::from_datetime(2020, 2, 3, 4, 5, 6.0, 0.0, TimeSystem::GPS, &eop);

        assert_eq!(epc.to_string(), "2020-02-03 04:05:06.000 GPS")
    }

    #[test]
    fn test_epoch_debug() {
        let eop = setup_eop();

        let epc = Epoch::from_datetime(2020, 2, 3, 4, 5, 6.0, 0.0, TimeSystem::GPS, &eop);

        assert_eq!(format!("{:?}", epc), "Epoch<2458882, 57924, 999999999.9927241, 0, GPS>")
    }

    #[test]
    fn test_epoch_from_date() {
        let eop = setup_eop();

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
        let eop = setup_eop();

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
        assert_eq!(nanosecond, 0.5 * 1.0e9 + 1.2345);
    }

    #[test]
    fn test_epoch_from_string() {
        let eop = setup_eop();

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

    #[test]
    fn test_epoch_from_jd() {
        let eop = setup_eop();


        let epc = Epoch::from_jd(MJD_ZERO + MJD2000, TimeSystem::TAI, &eop);
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2000);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 12);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        let epc = Epoch::from_jd(MJD_ZERO + MJD2000, TimeSystem::GPS, &eop);
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime_as_tsys(TimeSystem::TAI);
        assert_eq!(year, 2000);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 12);
        assert_eq!(minute, 0);
        assert_eq!(second, 19.0);
        assert_eq!(nanosecond, 17643.974853515625); // Rounding error from floating point conversion
        assert_eq!(epc.time_system, TimeSystem::GPS);
    }

    #[test]
    fn test_epoch_from_mjd() {
        let eop = setup_eop();


        let epc = Epoch::from_mjd(MJD2000, TimeSystem::TAI, &eop);
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2000);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 12);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        let epc = Epoch::from_mjd(MJD2000, TimeSystem::GPS, &eop);
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime_as_tsys(TimeSystem::TAI);
        assert_eq!(year, 2000);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 12);
        assert_eq!(minute, 0);
        assert_eq!(second, 19.0);
        assert_eq!(nanosecond, 17643.974853515625); // Rounding error from floating point conversion
        assert_eq!(epc.time_system, TimeSystem::GPS);
    }

    #[test]
    fn test_epoch_from_gps_date() {
        let eop = setup_eop();

        let epc = Epoch::from_gps_date(0, 0.0, &eop);
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 1980);
        assert_eq!(month, 1);
        assert_eq!(day, 6);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::GPS);

        let epc = Epoch::from_gps_date(2194, 435781.5, &eop);
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 28);
        assert_eq!(hour, 1);
        assert_eq!(minute, 3);
        assert_eq!(second, 1.0);
        assert_eq!(nanosecond, 500000000.0);
        assert_eq!(epc.time_system, TimeSystem::GPS);
    }

    #[test]
    fn test_epoch_from_gps_seconds() {
        let eop = setup_eop();

        let epc = Epoch::from_gps_seconds(0.0, &eop);
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 1980);
        assert_eq!(month, 1);
        assert_eq!(day, 6);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::GPS);

        let epc = Epoch::from_gps_seconds(2194.0 * 7.0 * 86400.0 + 3.0 * 3600.0 + 61.5, &eop);
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 23);
        assert_eq!(hour, 3);
        assert_eq!(minute, 1);
        assert_eq!(second, 1.0);
        assert_eq!(nanosecond, 500000000.0);
        assert_eq!(epc.time_system, TimeSystem::GPS);
    }

    #[test]
    fn test_epoch_from_gps_nanoseconds() {
        let eop = setup_eop();

        let epc = Epoch::from_gps_nanoseconds(0, &eop);
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 1980);
        assert_eq!(month, 1);
        assert_eq!(day, 6);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::GPS);

        let gpsns: u64 = (2194 * 7 * 86400 + 3 * 3600 + 61) * 1_000_000_000 + 1;
        let epc = Epoch::from_gps_nanoseconds(gpsns, &eop);
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 23);
        assert_eq!(hour, 3);
        assert_eq!(minute, 1);
        assert_eq!(second, 1.0);
        assert_eq!(nanosecond, 1.0);
        assert_eq!(epc.time_system, TimeSystem::GPS);
    }

    #[test]
    fn test_epoch_to_jd() {
        let eop = setup_eop();

        let epc = Epoch::from_datetime(2000, 1, 1, 12, 0, 0.0, 0.0, TimeSystem::TAI, &eop);

        assert_eq!(epc.jd(), MJD_ZERO + MJD2000);

        let epc = Epoch::from_datetime(2000, 1, 1, 12, 0, 0.0, 0.0, TimeSystem::TAI, &eop);
        assert_eq!(epc.jd_as_tsys(TimeSystem::UTC), MJD_ZERO + MJD2000 - 32.0/86400.0)
    }

    #[test]
    fn test_epoch_to_mjd() {
        let eop = setup_eop();

        let epc = Epoch::from_datetime(2000, 1, 1, 12, 0, 0.0, 0.0, TimeSystem::TAI, &eop);

        assert_eq!(epc.mjd(), MJD2000);

        let epc = Epoch::from_datetime(2000, 1, 1, 12, 0, 0.0, 0.0, TimeSystem::TAI, &eop);
        assert_eq!(epc.mjd_as_tsys(TimeSystem::UTC), MJD2000 - 32.0/86400.0)
    }

    #[test]
    fn test_gps_date() {
        let eop = setup_eop();

        let epc = Epoch::from_date(2018, 3, 1, TimeSystem::GPS, &eop);
        let (gps_week, gps_seconds) = epc.gps_date();
        assert_eq!(gps_week, 1990);
        assert_eq!(gps_seconds, 4.0*86400.0);

        let epc = Epoch::from_date(2018, 3, 8, TimeSystem::GPS, &eop);
        let (gps_week, gps_seconds) = epc.gps_date();
        assert_eq!(gps_week, 1991);
        assert_eq!(gps_seconds, 4.0*86400.0);

        let epc = Epoch::from_date(2018, 3, 11, TimeSystem::GPS, &eop);
        let (gps_week, gps_seconds) = epc.gps_date();
        assert_eq!(gps_week, 1992);
        assert_eq!(gps_seconds, 0.0*86400.0);

        let epc = Epoch::from_date(2018, 3, 24, TimeSystem::GPS, &eop);
        let (gps_week, gps_seconds) = epc.gps_date();
        assert_eq!(gps_week, 1993);
        assert_eq!(gps_seconds, 6.0*86400.0);
    }

    #[test]
    fn test_gps_seconds() {
        let eop = setup_eop();

        let epc = Epoch::from_date(1980, 1, 6, TimeSystem::GPS, &eop);
        assert_eq!(epc.gps_seconds(), 0.0);

        let epc = Epoch::from_datetime(1980, 1, 7, 0, 0, 1.0, 0.0,TimeSystem::GPS, &eop);
        assert_eq!(epc.gps_seconds(), 86401.0);
    }

    #[test]
    fn test_gps_nanoseconds() {
        let eop = setup_eop();

        let epc = Epoch::from_date(1980, 1, 6, TimeSystem::GPS, &eop);
        assert_eq!(epc.gps_nanoseconds(), 0.0);

        let epc = Epoch::from_datetime(1980, 1, 7, 0, 0, 1.0, 0.0,TimeSystem::GPS, &eop);
        assert_eq!(epc.gps_nanoseconds(), 86401.0*1.0e9);
    }

    #[test]
    fn test_gmst() {
        let eop = setup_eop();

        let epc = Epoch::from_date(2000, 1, 1, TimeSystem::UTC, &eop);
        assert_abs_diff_eq!(epc.gmst(true), 99.969, epsilon=1.0e-3);

        let epc = Epoch::from_date(2000, 1, 1, TimeSystem::UTC, &eop);
        assert_abs_diff_eq!(epc.gmst(false), 99.969 * PI / 180.0, epsilon=1.0e-3);
    }

    #[test]
    fn test_gast() {
        let eop = setup_eop();

        let epc = Epoch::from_date(2000, 1, 1, TimeSystem::UTC, &eop);
        assert_abs_diff_eq!(epc.gast(true), 99.965, epsilon=1.0e-3);

        let epc = Epoch::from_date(2000, 1, 1, TimeSystem::UTC, &eop);
        assert_abs_diff_eq!(epc.gast(false), 99.965 * PI / 180.0, epsilon=1.0e-3);
    }

    #[test]
    fn test_ops_add_assign() {
        let eop = setup_eop();

        // Test Positive additions of different size
        let mut epc = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);
        epc += 1.0;
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 31);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 1.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        let mut epc = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);
        epc += 86400.5;
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 2);
        assert_eq!(day, 1);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 500_000_000.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        let mut epc = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);
        epc += 1.23456789e-9;
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 31);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 1.23456789);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        // Test subtractions of different size
        let mut epc = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);
        epc += -1.0;
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 30);
        assert_eq!(hour, 23);
        assert_eq!(minute, 59);
        assert_eq!(second, 59.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        let mut epc = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);
        epc += -86400.5;
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 29);
        assert_eq!(hour, 23);
        assert_eq!(minute, 59);
        assert_eq!(second, 59.0);
        assert_eq!(nanosecond, 500_000_000.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        // Test types
        let mut epc = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);
        epc += 1;
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 31);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 1.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        let mut epc = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);
        epc += -1;
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 30);
        assert_eq!(hour, 23);
        assert_eq!(minute, 59);
        assert_eq!(second, 59.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);
    }

    #[test]
    fn test_ops_sub_assign() {
        let eop = setup_eop();

        let mut epc = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);
        epc -= 1.23456789e-9;
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 30);
        assert_eq!(hour, 23);
        assert_eq!(minute, 59);
        assert_eq!(second, 59.0);
        assert_eq!(nanosecond, 999_999_999.7654321);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        // Test subtractions of different size
        let mut epc = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);
        epc -= 1.0;
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 30);
        assert_eq!(hour, 23);
        assert_eq!(minute, 59);
        assert_eq!(second, 59.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        let mut epc = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);
        epc -= 86400.5;
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 29);
        assert_eq!(hour, 23);
        assert_eq!(minute, 59);
        assert_eq!(second, 59.0);
        assert_eq!(nanosecond, 500_000_000.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        // Test types
        let mut epc = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);
        epc -= 1;
        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 30);
        assert_eq!(hour, 23);
        assert_eq!(minute, 59);
        assert_eq!(second, 59.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);
    }

    #[test]
    fn test_ops_add() {
        let eop = setup_eop();

        // Base epoch
        let epc = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);

        // Test Positive additions of different size
        let epc_2:Epoch = epc + 1.0;
        let (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 31);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 1.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        let epc_2:Epoch = epc + 86400.5;
        let (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 2);
        assert_eq!(day, 1);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 500_000_000.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        let epc_2:Epoch = epc + 1.23456789e-9;
        let (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 31);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 1.23456789);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        // Test subtractions of different size
        let epc_2:Epoch = epc + -1.0;
        let (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 30);
        assert_eq!(hour, 23);
        assert_eq!(minute, 59);
        assert_eq!(second, 59.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        let epc_2:Epoch = epc + -86400.5;
        let (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 29);
        assert_eq!(hour, 23);
        assert_eq!(minute, 59);
        assert_eq!(second, 59.0);
        assert_eq!(nanosecond, 500_000_000.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        // Test types
        let epc_2:Epoch = epc + 1;
        let (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 31);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 1.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        let epc_2:Epoch = epc + -1;
        let (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 30);
        assert_eq!(hour, 23);
        assert_eq!(minute, 59);
        assert_eq!(second, 59.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);
    }

    #[test]
    fn test_ops_sub() {
        let eop = setup_eop();

        // Base epoch
        let epc = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);

        // Test subtractions of different size
        let epc_2:Epoch = epc - 1.0;
        let (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 30);
        assert_eq!(hour, 23);
        assert_eq!(minute, 59);
        assert_eq!(second, 59.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        let epc_2:Epoch = epc - 86400.5;
        let (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 29);
        assert_eq!(hour, 23);
        assert_eq!(minute, 59);
        assert_eq!(second, 59.0);
        assert_eq!(nanosecond, 500_000_000.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);

        // Test types
        let epc_2:Epoch = epc - 1;
        let (year, month, day, hour, minute, second, nanosecond) = epc_2.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 30);
        assert_eq!(hour, 23);
        assert_eq!(minute, 59);
        assert_eq!(second, 59.0);
        assert_eq!(nanosecond, 0.0);
        assert_eq!(epc.time_system, TimeSystem::TAI);
    }

    #[test]
    fn test_ops_sub_epoch() {
        let eop = setup_eop();

        let epc_1 = Epoch::from_date(2022, 1, 31, TimeSystem::TAI, &eop);
        let epc_2 = Epoch::from_date(2022, 2, 1, TimeSystem::TAI, &eop);
        assert_eq!(epc_2 - epc_1, 86400.0);

        let epc_1 = Epoch::from_date(2021, 1, 1, TimeSystem::TAI, &eop);
        let epc_2 = Epoch::from_date(2022, 1, 1, TimeSystem::TAI, &eop);
        assert_eq!(epc_2 - epc_1, 86400.0*365.0);

        let epc_1 = Epoch::from_datetime(2022, 1, 1, 0, 0, 0.0, 0.0, TimeSystem::TAI, &eop);
        let epc_2 = Epoch::from_datetime(2022, 1, 1, 0, 0, 0.0, 1.0, TimeSystem::TAI, &eop);
        assert_eq!(epc_2 - epc_1, 1.0e-9);

        let epc_1 = Epoch::from_datetime(2022, 1, 1, 0, 0, 0.0, 0.0, TimeSystem::TAI, &eop);
        let epc_2 = Epoch::from_datetime(2022, 1, 2, 1, 1, 1.0, 1.0, TimeSystem::TAI, &eop);
        assert_eq!(epc_2 - epc_1, 86400.0 + 3600.0 + 60.0 + 1.0 + 1.0e-9);

        let epc_1 = Epoch::from_datetime(2022, 1, 1, 0, 0, 0.0, 0.0, TimeSystem::TAI, &eop);
        let epc_2 = Epoch::from_datetime(2022, 1, 1, 0, 0, 19.0, 0.0, TimeSystem::TAI, &eop);
        assert_eq!(epc_2 - epc_1, 19.0);
        assert_eq!(epc_1 - epc_2, -19.0);
        assert_eq!(epc_1 - epc_1, 0.0);
    }

    #[test]
    fn test_eq_epoch() {
        let eop = setup_eop();

        let epc_1 = Epoch::from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23456789, TimeSystem::TAI, &eop);
        let epc_2 = Epoch::from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23456789, TimeSystem::TAI, &eop);
        assert_eq!(epc_1 == epc_2, true);

        let epc_1 = Epoch::from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23456, TimeSystem::TAI, &eop);
        let epc_2 = Epoch::from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23455 , TimeSystem::TAI, &eop);
        assert_eq!(epc_1 != epc_2, true);

        // Check instant comparison against time systems works
        let epc_1 = Epoch::from_datetime(1980, 1, 6, 0, 0, 0.0, 0.0, TimeSystem::GPS, &eop);
        let epc_2 = Epoch::from_datetime(1980, 1, 6, 0, 0, 19.0, 0.0 , TimeSystem::TAI, &eop);
        assert_eq!(epc_1 == epc_2, true);
    }

    #[test]
    fn test_cmp_epoch() {
        let eop = setup_eop();

        let epc_1 = Epoch::from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23456, TimeSystem::TAI, &eop);
        let epc_2 = Epoch::from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23455, TimeSystem::TAI, &eop);
        assert_eq!(epc_1 > epc_2, true);
        assert_eq!(epc_1 >= epc_2, true);
        assert_eq!(epc_1 < epc_2, false);
        assert_eq!(epc_1 <= epc_2, false);

        let epc_1 = Epoch::from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23456, TimeSystem::TAI, &eop);
        let epc_2 = Epoch::from_datetime(2022, 1, 1, 12, 23, 59.9, 1.23456, TimeSystem::TAI, &eop);
        assert_eq!(epc_1 > epc_2, false);
        assert_eq!(epc_1 >= epc_2, true);
        assert_eq!(epc_1 < epc_2, false);
        assert_eq!(epc_1 <= epc_2, true);
    }

    #[test]
    #[ignore]
    fn test_nanosecond_addition_stability() {
        let eop = setup_eop();

        let mut epc = Epoch::from_datetime(2022, 1, 1, 0, 0, 0.0, 0.0, TimeSystem::TAI, &eop);

        for _i in 0..1_000_000_000 {
            epc += 1.0e-9;
        }

        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2022);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 1.0);
        assert_eq!(nanosecond, 0.0);
    }

    #[test]
    fn test_addition_stability() {
        let eop = setup_eop();

        let mut epc = Epoch::from_datetime(2022, 1, 1, 0, 0, 0.0, 0.0, TimeSystem::TAI, &eop);

        // Advance a year 1 second at a time
        for _i in 0..(86400*365) {
            epc += 1.0;
        }

        let (year, month, day, hour, minute, second, nanosecond) = epc.to_datetime();
        assert_eq!(year, 2023);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0.0);
        assert_eq!(nanosecond, 0.0);
    }

    #[test]
    fn test_epoch_range() {
        let eop = setup_eop();

        let mut epcv: Vec<Epoch> = Vec::new();
        let epcs = Epoch::from_datetime(2022, 1, 1, 0, 0, 0.0, 0.0, TimeSystem::TAI, &eop);
        let epcf = Epoch::from_datetime(2022, 1, 2, 0, 0, 0.0, 0.0, TimeSystem::TAI, &eop);
        let mut epc = Epoch::from_datetime(2022, 1, 1, 0, 0, 0.0, 0.0, TimeSystem::TAI, &eop);

        for e in EpochRange::new(epcs, epcf, 1.0) {
            assert_eq!(epc, e);
            epc += 1;
            epcv.push(e);
        }

        let epcl = Epoch::from_datetime(2022, 1, 1, 23, 59, 59.0, 0.0, TimeSystem::TAI, &eop);
        assert_eq!(epcv.len(), 86400);
        assert_eq!(epcv[epcv.len() - 1] != epcf, true);
        assert!((epcv[epcv.len() - 1] - epcl).abs() < 1.0e-9);
    }
}