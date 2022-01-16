use std::fmt;
use std::fs::{File, OpenOptions};
use std::str::FromStr;
use std::io::{BufReader, Read};
use std::io::prelude::*;
use std::collections::HashMap;

use std::fs;
use std::io::Write;
use std::path::Path;
use ureq;
use crate::constants::AS2RAD;

// Package EOP data as part of crate
/// Packaged C04 EOP Data File
static PACKAGED_C04_FILE: &'static [u8] = include_bytes!("../data/iau2000A_c04_14.txt");
/// Packaged Finals 2000A Data File
static PACKAGED_FINALS2000_FILE: &'static [u8] = include_bytes!("../data/iau2000A_finals_ab.txt");

/// Enumerated value that indicates the preferred behavior of the Earth Orientation Data provider
/// when the desired time point is not present.
///
/// # Values
/// - `Zero`: Return a value of zero for the missing data
/// - `Hold`: Return the last value prior to the requested date
/// - `Error`: Panics current execution thread, immediately terminating the program
#[derive(Debug,Clone,PartialEq,Copy)]
pub enum EOPExtrapolation {
    Zero,
    Hold,
    Error
}

impl fmt::Display for EOPExtrapolation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EOPExtrapolation::Zero => write!(f, "EOPExtrapolation::Zero"),
            EOPExtrapolation::Hold => write!(f, "EOPExtrapolation::Hold"),
            EOPExtrapolation::Error => write!(f, "EOPExtrapolation::Error"),
        }
    }
}

/// Enumerates type of Earth Orientation data loaded. All models assumed to be
/// consistent with IAU2000 precession Nutation Model
///
/// # Values
/// - `C04`: IERS Long Term Data Product EOP 14 C04
/// - `StandardBulletinA`: IERS Standard Data Bulletin A from finals2000 file
/// - `StandardBulletinB`: IERS Standard Data Bulletin B from finals2000 file
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum EOPType {
    C04,
    StandardBulletinA,
    StandardBulletinB
}

impl fmt::Display for EOPType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EOPType::C04 => write!(f, "C04"),
            EOPType::StandardBulletinA => write!(f, "Bulletin A"),
            EOPType::StandardBulletinB => write!(f, "Bulletin B"),
        }
    }
}

/// Stores Earth orientation parameter data.
///
/// The structure assumes the input data uses the IAU 2010/2000A conventions. That is the
/// precession/nutation parameter values are in terms of `dX` and `dY`, not `dPsi` and `dEps`.
#[derive(Clone)]
pub struct EarthOrientationData {
    /// Type of Earth orientation data loaded
    pub eop_type: EOPType,
    /// Primary data structure storing loaded Earth orientation parameter data.
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
    pub data: HashMap<u32, (f64, f64, f64, Option<f64>, Option<f64>, Option<f64>)>,
    /// Defines desired behavior for out-of-bounds Earth Orientation data access
    pub extrapolate: EOPExtrapolation,
    /// Defines interpolation behavior of data for requests between data points in table.
    ///
    /// When set to `true` data will be linearly interpolated to the desired time.
    /// When set to `false` data will be given as the value as the closest previous data entry
    /// present.
    pub interpolate: bool,
    /// Minimum date of stored data. This is the value of the smallest key stored in the `data`
    /// HashMap. Value is a modified Julian date.
    pub mjd_min: u32,
    /// Maximum date of stored data. This is the value of the largest key stored in the `data`
    /// HashMap. Behavior
    /// of data retrieval for dates larger than this will be defined by the `extrapolate` value.
    /// Babylon's Fall
    pub mjd_max: u32,
    /// Modified Julian date of last valid Length of Day (LOD) value. Only applicable for
    /// Bulletin A EOP data. Will be 0 for Bulletin B data and the same as `mjd_max` for C04 data.
    pub mjd_last_lod: u32,
    /// Modified Julian date of last valid precession/nutation dX/dY correction values. Only
    /// applicable for Bulletin A. Will always be the sam as `mjd_max` for Bulletin B and C04 data.
    pub mjd_last_dxdy: u32,
}

impl fmt::Display for EarthOrientationData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EOP Object - type: {}, {} entries, mjd_min: {}, mjd_max: {},  mjd_last_lod: \
        {}, mjd_last_dxdy: {}, extrapolate: {}, \
        interpolate: {}", self.eop_type, self.data.len(), self.mjd_min, self.mjd_max,
               self.mjd_last_lod, self.mjd_last_dxdy, self.extrapolate, self.interpolate)
    }
}

impl fmt::Debug for EarthOrientationData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EOP Object - type: {}, {} entries, mjd_min: {}, mjd_max: {},  mjd_last_lod: \
        {}, mjd_last_dxdy: {}, extrapolate: {}, \
        interpolate: {}", self.eop_type, self.data.len(), self.mjd_min, self.mjd_max,
               self.mjd_last_lod, self.mjd_last_dxdy, self.extrapolate, self.interpolate)
    }
}

/// Parse a line out of a C04 file and return the resulting data.
///
/// # Arguments
/// - `line`: Reference to string to attempt to parse as a C04 formatted line
///
/// # Returns
/// On successful parse returns tuple containing:
/// - `mjd`: Modified Julian date of data point
/// - `pm_x`: x-component of polar motion correction. Units: (radians)
/// - `pm_y`: y-component of polar motion correction. Units: (radians)
/// - `ut1_utc`: Offset of UT1 time scale from UTC time scale. Units: (seconds)
/// - `dX`: "X" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
/// - `dY`: "Y" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
/// - `lod`: Difference between astronomically determined length of day and 86400 second TAI. Units: (seconds)
#[allow(non_snake_case)]
fn parse_c04_line(line: &str) -> Result<(u32, f64, f64, f64, Option<f64>, Option<f64>,
                                             Option<f64>), String> {

    let mjd = match u32::from_str(&line[12..19].trim()) {
        Ok(mjd) => mjd,
        Err(e) => return Err(format!("Failed to parse mjd from '{}': {}", &line[12..19], e))
    };
    let pm_x = match f64::from_str(&line[19..30].trim()) {
        Ok(pm_x) => pm_x * AS2RAD,
        Err(e) => return Err(format!("Failed to parse pm_x from '{}': {}", &line[19..30], e))
    };
    let pm_y = match f64::from_str(&line[30..41].trim()) {
        Ok(pm_y) => pm_y * AS2RAD,
        Err(e) => return Err(format!("Failed to parse pm_y from '{}': {}", &line[30..41], e))
    };
    let ut1_utc = match f64::from_str(&line[41..53].trim()) {
        Ok(ut1_utc) => ut1_utc,
        Err(e) => return Err(format!("Failed to parse ut1_utc from '{}': {}", &line[41..53], e))
    };
    let lod = match  f64::from_str(&line[53..65].trim()) {
        Ok(lod) => lod,
        Err(e) => return Err(format!("Failed to parse lod from '{}': {}", &line[53..65], e))
    };
    let dX = match f64::from_str(&line[65..76].trim()) {
        Ok(dX) => dX * AS2RAD,
        Err(e) => return Err(format!("Failed to parse dX from '{}': {}", &line[65..76], e))
    };
    let dY = match f64::from_str(&line[76..87].trim()) {
        Ok(dY) => dY * AS2RAD,
        Err(e) => return Err(format!("Failed to parse dY from '{}': {}", &line[76..87], e))
    };

    Ok((mjd, pm_x, pm_y, ut1_utc, Some(dX), Some(dY), Some(lod)))
}

/// Take in a `BufReader` object and attempt to parse reader as a C04-type EOP data stream and
/// return a EarthOrientationData structure from it.
///
/// # Arguments
/// - `reader`: BufReader object to parse. Can be either a File object or a BufReader wrapping a
/// char-byte array.
/// - `extrapolate`: Set EOP Extrapolation behavior for resulting EarthOrientationData object.
/// - `interpolate`: Set EOP interpolation behavior for resulting EarthOrientationData object.
fn eop_c04_from_bufreader<T: Read>(reader: BufReader<T>, extrapolate: EOPExtrapolation,
                                   interpolate: bool)
    -> Result<EarthOrientationData, String> {
    let mut mjd_min:u32 = 0;
    let mut mjd_max:u32 = 0;

    let mut data: HashMap<u32, (f64, f64, f64, Option<f64>, Option<f64>, Option<f64>)> =
        HashMap::new();


    for (lineno, linestr) in reader.lines().enumerate() {
        // Skip first 14 lines of C04 data file header
        if lineno < 14 {
            continue
        }

        let line = match linestr {
            Ok(l) => l,
            Err(e) => return Err(format!("Failed to parse EOP file on line {}: {}", lineno, e))
        };
        let eop_data = match parse_c04_line(&line) {
            Ok(eop_data) => eop_data,
            Err(e) => return Err(format!("Failed to parse EOP file on line {}: {}", lineno, e))
        };

        // Update record or min and max data entry encountered
        // This is kind of hacky since it assumes the EOP data files are sorted,
        // But there are already a number of assumptions on input data formatting.
        if mjd_min == 0 {
            mjd_min = eop_data.0;
        }

        if (lineno == 0) || (eop_data.0 > mjd_max) {
            mjd_max = eop_data.0;
        }

        data.insert(
            eop_data.0,
            (eop_data.1, eop_data.2, eop_data.3, eop_data.4, eop_data.5, eop_data.6)
        );
    }

    Ok(EarthOrientationData {
        eop_type:EOPType::C04,
        data,
        extrapolate,
        interpolate,
        mjd_min,
        mjd_max,
        mjd_last_lod: mjd_max,  // Same as mjd_max for C04 data format
        mjd_last_dxdy: mjd_max  // Same as mjd_max for C04 data format
    })
}

/// Parse a line out of a standard EOP file and return the resulting data.
///
/// # Arguments
/// - `line`: Reference to string to attempt to parse as a C04 formatted line
/// - `eop_type`: Type to parse data file as. Can be `EOPType::StandardBulletinA` or
/// `EOPType::StandardBulletinB`
///
/// # Returns
/// On successful parse returns tuple containing:
/// - `mjd`: Modified Julian date of data point
/// - `pm_x`: x-component of polar motion correction. Units: (radians)
/// - `pm_y`: y-component of polar motion correction. Units: (radians)
/// - `ut1_utc`: Offset of UT1 time scale from UTC time scale. Units: (seconds)
/// - `dX`: "X" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
/// - `dY`: "Y" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
/// - `lod`: Difference between astronomically determined length of day and 86400 second TAI. Units: (seconds)
#[allow(non_snake_case)]
fn parse_standard_eop_line(line: &str, eop_type: EOPType) -> Result<(u32, f64, f64, f64,
                             Option<f64>, Option<f64>, Option<f64>), String> {

    let pm_x: f64;
    let pm_y: f64;
    let ut1_utc: f64;
    let lod: Option<f64>;
    let dX: Option<f64>;
    let dY: Option<f64>;

    // Finals files like to have a trailing new-line which breaks this parsing.
    // We perform a check for minimum line length we would expect to find primary values in
    if line.len() >= 68 {
       let mjd = match u32::from_str(&line[6..12].trim()) {
            Ok(mjd) => mjd,
            Err(e) => return Err(format!("Failed to parse mjd from '{}': {}", &line[6..12], e))
        };

        match eop_type {
            EOPType::StandardBulletinA => {
                pm_x = match f64::from_str(&line[17..27].trim()) {
                    Ok(pm_x) => pm_x * AS2RAD,
                    Err(e) => return Err(format!("Failed to parse pm_x from '{}': {}", &line[18..27], e))
                };
                pm_y = match f64::from_str(&line[37..46].trim()) {
                    Ok(pm_y) => pm_y * AS2RAD,
                    Err(e) => return Err(format!("Failed to parse pm_y from '{}': {}", &line[37..46], e))
                };
                ut1_utc = match f64::from_str(&line[58..68].trim()) {
                    Ok(ut1_utc) => ut1_utc,
                    Err(e) => return Err(format!("Failed to parse ut1_utc from '{}': {}", &line[58..68], e))
                };
                lod = match  f64::from_str(&line[78..86].trim()) {
                    Ok(lod) => Some(lod),
                    Err(_) => None
                };
                dX = match f64::from_str(&line[97..106].trim()) {
                    Ok(dX) => Some(dX * AS2RAD),
                    Err(_) => None
                };
                dY = match f64::from_str(&line[116..125].trim()) {
                    Ok(dY) => Some(dY * AS2RAD),
                    Err(_) => None
                };
            },
            EOPType::StandardBulletinB => {
                pm_x = match f64::from_str(&line[134..144].trim()) {
                    Ok(pm_x) => pm_x * AS2RAD,
                    Err(e) => return Err(format!("Failed to parse pm_x from '{}': {}", &line[134..144], e))
                };
                pm_y = match f64::from_str(&line[144..154].trim()) {
                    Ok(pm_y) => pm_y * AS2RAD,
                    Err(e) => return Err(format!("Failed to parse pm_y from '{}': {}", &line[144..154], e))
                };
                ut1_utc = match f64::from_str(&line[154..165].trim()) {
                    Ok(ut1_utc) => ut1_utc,
                    Err(e) => return Err(format!("Failed to parse ut1_utc from '{}': {}", &line[154..165], e))
                };
                lod = Some(0.0);
                dX = match f64::from_str(&line[165..175].trim()) {
                    Ok(dX) => Some(dX * AS2RAD),
                    Err(e) => return Err(format!("Failed to parse dX from '{}': {}", &line[165..175], e))
                };
                dY = match f64::from_str(&line[175..185].trim()) {
                    Ok(dY) => Some(dY * AS2RAD),
                    Err(e) => return Err(format!("Failed to parse dY from '{}': {}", &line[175..185], e))
                };
            },
            _ => {
                return Err(format!("Invalid EOPType for standard parsing: {}", eop_type))
            }
        }

        Ok((mjd, pm_x, pm_y, ut1_utc, dX, dY, lod))
    } else {
        Err(format!("Unable to parse line. Line too short."))
    }
}

/// Take in a `BufReader` object and attempt to parse reader as a C04-type EOP data stream and
/// return a EarthOrientationData structure from it.
///
/// # Arguments
/// - `reader`: BufReader object to parse. Can be either a File object or a BufReader wrapping a
/// char-byte array.
/// - `extrapolate`: Set EOP Extrapolation behavior for resulting EarthOrientationData object.
/// - `interpolate`: Set EOP interpolation behavior for resulting EarthOrientationData object.
/// - `eop_type`: Type to parse data file as. Can be `EOPType::StandardBulletinA` or
/// `EOPType::StandardBulletinB`
///
/// # Returns
/// On successful parse
/// - `eop`: Returns `EarthOrientationData` object
fn eop_standard_eop_from_bufreader<T: Read>(reader: BufReader<T>, extrapolate: EOPExtrapolation,
                                   interpolate: bool, eop_type: EOPType)
                                   -> Result<EarthOrientationData, String> {
    let mut mjd_min:u32 = 0;
    let mut mjd_max:u32 = 0;
    let mut mjd_last_lod: u32 = 0;
    let mut mjd_last_dxdy:u32 = 0;

    let mut data: HashMap<u32, (f64, f64, f64, Option<f64>, Option<f64>, Option<f64>)> =
        HashMap::new();


    for (lineno, linestr) in reader.lines().enumerate() {
        let line = match linestr {
            Ok(l) => l,
            Err(e) => return Err(format!("Failed to parse EOP file on line {}: {}", lineno, e))
        };
        let eop_data = match parse_standard_eop_line(&line, eop_type) {
            Ok(eop_data) => eop_data,
            Err(_) => continue // There is probably a better way to handle this but we just
            // continue reading data until the end of the file is reached. For bad lines we just
            // skip updating fields or data
        };

        // Update record or min and max data entry encountered
        // This is kind of hacky since it assumes the EOP data files are sorted,
        // But there are already a number of assumptions on input data formatting.
        if mjd_min == 0 {
            mjd_min = eop_data.0;
        }

        if (lineno == 0) || (eop_data.0 > mjd_max) {
            mjd_max = eop_data.0;
        }

        // Advance last valid MJD of LOD data if Bulletin A and a value was parsed
        if eop_type == EOPType::StandardBulletinA && eop_data.6 != None {
            mjd_last_lod = eop_data.0;
        }

        // Advance last valid MJD of dX/dY data if Bulletin A and a value was parsed
        if (eop_data.4 != None) && (eop_data.5 != None) {
            mjd_last_dxdy = eop_data.0;
        }

        data.insert(
            eop_data.0,
            (eop_data.1, eop_data.2, eop_data.3, eop_data.4, eop_data.5, eop_data.6)
        );
    }

    Ok(EarthOrientationData {
        eop_type,
        data,
        extrapolate,
        interpolate,
        mjd_min,
        mjd_max,
        mjd_last_lod,
        mjd_last_dxdy
    })
}

impl EarthOrientationData {
    /// Load C04 Earth orientation data from file.
    ///
    /// Takes a path to a given file which will be read on the assumption that it is an Earth
    /// orientation parameter data file formatted according to [IERS C04 formatting standards](https://www.iers.org/IERS/EN/DataProducts/EarthOrientationData/eop.html)
    ///
    /// # Arguments
    /// - `filepath`: Path of input data file
    /// - `extrapolate`: Set EOP Extrapolation behavior for resulting EarthOrientationData object.
    /// - `interpolate`: Set EOP interpolation behavior for resulting EarthOrientationData object.
    ///
    /// # Returns
    /// - `eop`: On successful parse returns `EarthOrientationData` object
    ///
    /// # Examples
    /// ```rust
    /// use std::env;
    /// use std::path::Path;
    /// use rastro::eop::*;
    ///
    /// // Get crate root directly to provide consistent path to test data file
    /// let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    /// // Create filepath object of desired Earth orientation data to load
    /// let filepath = Path::new(&manifest_dir).join("test_assets").join("iau2000A_c04_14.txt");
    /// // Set EOP extrapolation behavior will hold the last value
    /// let eop_extrapolation = EOPExtrapolation::Hold;
    /// // Set EOP interpolation behavior -> will interpolate between points
    /// let eop_interpolation = true;
    /// // Create Earth orientation data object loading the
    /// let eop = EarthOrientationData::from_c04_file(filepath.to_str().unwrap(),
    ///     eop_extrapolation, eop_interpolation).unwrap();
    /// // Display the properties of the loaded data including the number of data points
    /// println!("{}", eop);
    /// ```
    pub fn from_c04_file(filepath: &str, extrapolate: EOPExtrapolation, interpolate: bool) ->
                                                                                            Result<Self, String> {
        let f = match File::open(filepath) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}", e))
        };
        let reader = BufReader::new(f);

        eop_c04_from_bufreader(reader, extrapolate, interpolate)
    }

    /// Load package-default C04 Earth orientation data.
    ///
    /// Parses the Earth orientation data packaged with the RAstro library return a valid
    /// `EarthOrientationData`.
    ///
    /// # Arguments
    /// - `extrapolate`: Set EOP Extrapolation behavior for resulting EarthOrientationData object.
    /// - `interpolate`: Set EOP interpolation behavior for resulting EarthOrientationData object.
    ///
    /// # Returns
    /// - `eop`: Returns `EarthOrientationData` object
    /// # Examples
    /// ```rust
    /// use std::env;
    /// use std::path::Path;
    /// use rastro::eop::*;
    ///
    /// // Set EOP extrapolation behavior will hold the last value
    /// let eop_extrapolation = EOPExtrapolation::Hold;
    /// // Set EOP interpolation behavior -> will interpolate between points
    /// let eop_interpolation = true;
    /// // Load packaged C04 data
    /// let eop = EarthOrientationData::from_default_c04(eop_extrapolation, eop_interpolation);
    /// // Display the properties of the loaded data including the number of data points
    /// println!("{}", eop);
    /// ```
    pub fn from_default_c04(extrapolate: EOPExtrapolation, interpolate: bool) -> Self {
        let reader = BufReader::new(PACKAGED_C04_FILE);
        eop_c04_from_bufreader(reader, extrapolate, interpolate).expect("Failed to parse and \
        load packed C04 Earth Orientation Data.")
    }

    /// Load standard Earth orientation data from file.
    ///
    /// Takes a path to a given file which will be read on the assumption that it is an Earth
    /// orientation parameter data file formatted according to [IERS Standard EOP Data format](https://www.iers.org/IERS/EN/DataProducts/EarthOrientationData/eop.html)
    ///
    /// # Arguments
    /// - `filepath`: Path of input data file
    /// - `extrapolate`: Set EOP Extrapolation behavior for resulting EarthOrientationData object.
    /// - `interpolate`: Set EOP interpolation behavior for resulting EarthOrientationData object.
    /// - `eop_type`: Type to parse data file as. Can be `EOPType::StandardBulletinA` or
    /// `EOPType::StandardBulletinB`
    ///
    /// # Returns
    /// - `eop`: On successful parse returns `EarthOrientationData` object
    ///
    /// # Examples
    /// ```rust
    /// use std::env;
    /// use std::path::Path;
    /// use rastro::eop::*;
    ///
    /// // Get crate root directly to provide consistent path to test data file
    /// let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    /// // Create filepath object of desired Earth orientation data to load
    /// let filepath = Path::new(&manifest_dir).join("test_assets").join("iau2000A_finals_ab.txt");
    /// // Set EOP extrapolation behavior will hold the last value
    /// let eop_extrapolation = EOPExtrapolation::Hold;
    /// // Set EOP interpolation behavior -> will interpolate between points
    /// let eop_interpolation = true;
    /// // Set type of EOP data to load
    /// let eop_type = EOPType::StandardBulletinA;
    /// // Load standard Earth orientation file. Typically a "Finals2000" file
    /// let eop = EarthOrientationData::from_standard_file(filepath.to_str().unwrap(),
    ///             eop_extrapolation, eop_interpolation, eop_type).unwrap();
    /// // Display the properties of the loaded data including the number of data points
    /// println!("{}", eop);
    /// ```
    pub fn from_standard_file(filepath: &str, extrapolate: EOPExtrapolation, interpolate: bool, eop_type: EOPType) ->
    Result<Self, String> {
        let f = match File::open(filepath) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}", e))
        };
        let reader = BufReader::new(f);

        eop_standard_eop_from_bufreader(reader, extrapolate, interpolate, eop_type)
    }

    /// Load package-default C04 Earth orientation data.
    ///
    /// Parses the Earth orientation data packaged with the RAstro library return a valid
    /// `EarthOrientationData`.
    ///
    /// # Arguments
    /// - `extrapolate`: Set EOP Extrapolation behavior for resulting EarthOrientationData object.
    /// - `interpolate`: Set EOP interpolation behavior for resulting EarthOrientationData object.
    /// - `eop_type`: Type to parse data file as. Can be `EOPType::StandardBulletinA` or
    /// `EOPType::StandardBulletinB`
    ///
    /// # Returns
    /// - `eop`: Returns `EarthOrientationData` object
    ///
    /// # Examples
    /// ```rust
    /// use rastro::eop::*;
    ///
    /// // Set EOP extrapolation behavior will hold the last value
    /// let eop_extrapolation = EOPExtrapolation::Hold;
    /// // Set EOP interpolation behavior -> will interpolate between points
    /// let eop_interpolation = true;
    /// // Set type of EOP data to load
    /// let eop_type = EOPType::StandardBulletinA;
    /// // Load packaged standard EOP data
    /// let eop = EarthOrientationData::from_default_standard(eop_extrapolation, eop_interpolation, eop_type);
    /// // Display the properties of the loaded data including the number of data points
    /// println!("{}", eop);
    /// ```
    pub fn from_default_standard(extrapolate: EOPExtrapolation, interpolate: bool, eop_type: EOPType) -> Self {
        let reader = BufReader::new(PACKAGED_FINALS2000_FILE);
        eop_standard_eop_from_bufreader(reader, extrapolate, interpolate, eop_type).expect("Failed to \
        parse and \
        load packed Standard Earth Orientation Data.")
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
    /// # Arguments
    /// - `mjd`: Modified Julian date to get Earth orientation parameters for
    ///
    /// # Returns
    /// - `ut1_utc`: Offset of UT1 time scale from UTC time scale. Units: (seconds)
    ///
    /// # Examples
    /// use rastro::eop::*;
    ///
    /// ```rust
    /// use rastro::eop::*;
    ///
    /// // Load Standard EOP
    /// let eop_extrapolation = EOPExtrapolation::Hold;
    /// let eop_interpolation = true;
    /// let eop_type = EOPType::StandardBulletinA;
    /// let eop = EarthOrientationData::from_default_standard(eop_extrapolation, eop_interpolation, eop_type);
    ///
    /// // Get EOP for 36 hours before the end of the table
    /// let ut1_utc = eop.get_ut1_utc(eop.mjd_max as f64 - 1.5);
    /// ```
    pub fn get_ut1_utc(&self, mjd: f64) -> f64 {
        // Check if time is beyond bounds of data table
        if mjd < self.mjd_max as f64 {
            if self.interpolate == true {
                // Get Time points
                let t1:f64 = mjd.floor();
                let t2:f64 = mjd.floor() + 1.0;

                // Get Values
                let y1:f64 = self.data[&(mjd.floor() as u32)].2;
                let y2:f64 = self.data[&(mjd.floor() as u32 + 1)].2;

                // Interpolate
                (y2 - y1) / (t2 - t1) * (mjd - t1) + y1
            } else {
                // Prior value
                self.data[&(mjd.floor() as u32)].2
            }
        } else {
            match self.extrapolate {
                EOPExtrapolation::Zero => {
                    0.0
                },
                EOPExtrapolation::Hold => {
                    // UT1-UTC is guaranteed to be present through `mjd_max`
                    self.data[&self.mjd_max].2
                },
                EOPExtrapolation::Error => {
                    panic!("Attempted ut1-utc beyond end of loaded EOP data. Accessed: {}, Max \
                    MJD: {}", mjd, self.mjd_max)
                }
            }
        }
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
    /// # Arguments
    /// - `mjd`: Modified Julian date to get Earth orientation parameters for
    ///
    /// # Returns
    /// - `pm_x`: x-component of polar motion correction. Units: (radians)
    /// - `pm_y`: y-component of polar motion correction. Units: (radians)
    ///
    /// # Examples
    /// use rastro::eop::*;
    ///
    /// ```rust
    /// use rastro::eop::*;
    ///
    /// // Load Standard EOP
    /// let eop_extrapolation = EOPExtrapolation::Hold;
    /// let eop_interpolation = true;
    /// let eop_type = EOPType::StandardBulletinA;
    /// let eop = EarthOrientationData::from_default_standard(eop_extrapolation, eop_interpolation, eop_type);
    ///
    /// // Get EOP for 36 hours before the end of the table
    /// let (pm_x, pm_y) = eop.get_pm(eop.mjd_max as f64 - 1.5);
    /// ```
    pub fn get_pm(&self, mjd: f64) -> (f64, f64) {
        // Check if time is beyond bounds of data table
        if mjd < self.mjd_max as f64 {
            if self.interpolate == true {
                // Get Time points
                let t1:f64 = mjd.floor();
                let t2:f64 = mjd.floor() + 1.0;

                // Get Values
                let pmx1:f64 = self.data[&(mjd.floor() as u32)].0;
                let pmx2:f64 = self.data[&(mjd.floor() as u32 + 1)].0;

                let pmy1:f64 = self.data[&(mjd.floor() as u32)].1;
                let pmy2:f64 = self.data[&(mjd.floor() as u32 + 1)].1;

                // Interpolate
                (
                    (pmx2 - pmx1) / (t2 - t1) * (mjd - t1) + pmx1,
                    (pmy2 - pmy1) / (t2 - t1) * (mjd - t1) + pmy1
                )
            } else {
                // Prior value
                (self.data[&(mjd.floor() as u32)].0, self.data[&(mjd.floor() as u32)].1)
            }
        } else {
            match self.extrapolate {
                EOPExtrapolation::Zero => {
                    (0.0, 0.0)
                },
                EOPExtrapolation::Hold => {
                    // pm-x and pm-y are guaranteed to be present through `mjd_max`
                    (self.data[&self.mjd_max].0, self.data[&self.mjd_max].1)
                },
                EOPExtrapolation::Error => {
                    panic!("Attempted pm-x,pm-y beyond end of loaded EOP data. Accessed: {}, Max \
                    MJD: {}", mjd, self.mjd_max)
                }
            }
        }
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
    /// - `mjd`: Modified Julian date to get Earth orientation parameters for
    ///
    /// # Returns
    /// - `dX`: "X" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
    /// - `dY`: "Y" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
    ///
    /// # Examples
    /// use rastro::eop::*;
    ///
    /// ```rust
    /// use rastro::eop::*;
    ///
    /// // Load Standard EOP
    /// let eop_extrapolation = EOPExtrapolation::Hold;
    /// let eop_interpolation = true;
    /// let eop_type = EOPType::StandardBulletinA;
    /// let eop = EarthOrientationData::from_default_standard(eop_extrapolation, eop_interpolation, eop_type);
    ///
    /// // Get EOP for 36 hours before the end of the table
    /// let (dx, dy) = eop.get_dxdy(eop.mjd_last_dxdy as f64 - 1.5);
    /// ```
    pub fn get_dxdy(&self, mjd: f64) -> (f64, f64) {
        // Check if time is beyond bounds of data table
        if mjd < self.mjd_last_dxdy as f64 {
            if self.interpolate == true {
                // Get Time points
                let t1:f64 = mjd.floor();
                let t2:f64 = mjd.floor() + 1.0;

                // Get Values
                let dx1:f64 = self.data[&(mjd.floor() as u32)].3.unwrap();
                let dx2:f64 = self.data[&(mjd.floor() as u32 + 1)].3.unwrap();

                let dy1:f64 = self.data[&(mjd.floor() as u32)].4.unwrap();
                let dy2:f64 = self.data[&(mjd.floor() as u32 + 1)].4.unwrap();

                // Interpolate
                (
                    (dx2 - dx1) / (t2 - t1) * (mjd - t1) + dx1,
                    (dy2 - dy1) / (t2 - t1) * (mjd - t1) + dy1
                )
            } else {
                // Prior value
                (self.data[&(mjd.floor() as u32)].3.unwrap(), self.data[&(mjd.floor() as u32)].4.unwrap())
            }
        } else {
            match self.extrapolate {
                EOPExtrapolation::Zero => {
                    (0.0, 0.0)
                },
                EOPExtrapolation::Hold => {
                    // dX,dY are guaranteed to be present through `mjd_last_dxdy`
                    (self.data[&self.mjd_last_dxdy].3.unwrap(), self.data[&self.mjd_last_dxdy].4.unwrap())
                },
                EOPExtrapolation::Error => {
                    panic!("Attempted dX,dY beyond end of loaded EOP data. Accessed: {}, Max \
                    MJD: {}", mjd, self.mjd_last_dxdy)
                }
            }
        }
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
    /// - `mjd`: Modified Julian date to get Earth orientation parameters for
    ///
    /// # Returns
    /// - `lod`: Difference between length of astronomically determined solar day and 86400 second
    ///     TAI day. Units: (seconds)
    ///
    /// # Examples
    /// use rastro::eop::*;
    ///
    /// ```rust
    /// use rastro::eop::*;
    ///
    /// // Load Standard EOP
    /// let eop_extrapolation = EOPExtrapolation::Hold;
    /// let eop_interpolation = true;
    /// let eop_type = EOPType::StandardBulletinA;
    /// let eop = EarthOrientationData::from_default_standard(eop_extrapolation, eop_interpolation, eop_type);
    ///
    /// // Get EOP for 36 hours before the end of the table
    /// let lod = eop.get_lod(eop.mjd_last_lod as f64 - 1.5);
    /// ```
    pub fn get_lod(&self, mjd: f64) -> f64 {
        // Check if time is beyond bounds of data table
        if mjd < self.mjd_last_lod as f64 {
            if self.interpolate == true {
                // Get Time points
                let t1:f64 = mjd.floor();
                let t2:f64 = mjd.floor() + 1.0;

                // Get Values
                let y1:f64 = self.data[&(mjd.floor() as u32)].5.unwrap();
                let y2:f64 = self.data[&(mjd.floor() as u32 + 1)].5.unwrap();

                // Interpolate
                (y2 - y1) / (t2 - t1) * (mjd - t1) + y1
            } else {
                // Prior value
                self.data[&(mjd.floor() as u32)].5.unwrap()
            }
        } else {
            match self.extrapolate {
                EOPExtrapolation::Zero => {
                    0.0
                },
                EOPExtrapolation::Hold => {
                    // LOD is guaranteed to be present through `mjd_last_lod`
                    self.data[&self.mjd_last_lod].5.unwrap()
                },
                EOPExtrapolation::Error => {
                    panic!("Attempted LOD beyond end of loaded EOP data. Accessed: {}, Max \
                    MJD: {}", mjd, self.mjd_last_lod)
                }
            }
        }
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
    /// - `mjd`: Modified Julian date to get Earth orientation parameters for
    ///
    /// # Returns
    /// - `pm_x`: x-component of polar motion correction. Units: (radians)
    /// - `pm_y`: y-component of polar motion correction. Units: (radians)
    /// - `ut1_utc`: Offset of UT1 time scale from UTC time scale. Units: (seconds)
    /// - `dX`: "X" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
    /// - `dY`: "Y" component of Celestial Intermediate Pole (CIP) offset. Units: (radians)
    /// - `lod`: Difference between length of astronomically determined solar day and 86400 second
    ///    TAI day. Units: (seconds)
    ///
    /// # Examples
    /// ```rust
    /// use rastro::eop::*;
    ///
    /// // Load Standard EOP
    /// let eop_extrapolation = EOPExtrapolation::Hold;
    /// let eop_interpolation = true;
    /// let eop_type = EOPType::StandardBulletinA;
    /// let eop = EarthOrientationData::from_default_standard(eop_extrapolation, eop_interpolation, eop_type);
    ///
    /// // Get EOP for 36 hours before the end of the table
    /// let eop_params = eop.get_eop(eop.mjd_max as f64 - 1.5);
    /// ```
    #[allow(non_snake_case)]
    pub fn get_eop(&self, mjd: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (pm_x, pm_y) = self.get_pm(mjd);
        let ut1_utc = self.get_ut1_utc(mjd);
        let (dX, dY) = self.get_dxdy(mjd);
        let lod = self.get_lod(mjd);
        (pm_x, pm_y, ut1_utc, dX, dY, lod)
    }
}

impl Default for EarthOrientationData {
    /// Initialize default Earth orientation parameter data.
    ///
    /// Loads Standard Bulletin A data with extrapolation set to `EOPExtrapolation::Zero` and
    /// interpolation enabled. Loads from package-included default.
    fn default() -> Self { EarthOrientationData::from_default_standard
(EOPExtrapolation::Zero, true, EOPType::StandardBulletinA) }
}

/// Download latest C04 Earth orientation parameter file.
///
///
/// Will attempt to download the latest parameter file to the specified location. Creating any
/// missing directories as required.
///
/// Download source: [https://datacenter.iers.org/data/latestVersion/9_FINALS.ALL_IAU2000_V2013_019.txt](https://datacenter.iers.org/data/latestVersion/9_FINALS.ALL_IAU2000_V2013_019.txt)
///
/// # Arguments
/// - `filepath`: Path of desired output file
pub fn download_c04_eop_file(filepath: &str) -> Result<(), &str> {
    // Create parent directory
    let filepath = Path::new(filepath);
    let parent_dir = filepath.parent().expect("Failed to identify parent directory.");

    fs::create_dir_all(parent_dir).expect(&*format!("Failed to create directory {}",
                                                    parent_dir.display()));

    let body = ureq::get("https://datacenter.iers.org/data/latestVersion/224_EOP_C04_14.62-NOW\
    .IAU2000A224.txt").call().expect("Download Request fialed").into_string().expect("Failed to \
    parse response into string");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filepath).expect(&*format!("Failed to create file: {}", filepath.display()));
    writeln!(&mut file, "{}", body).unwrap();

    Ok(())
}

/// Download latest standard Earth orientation parameter file.
///
/// Will attempt to download the latest parameter file to the specified location. Creating any
/// missing directories as required.
///
/// Download source: [https://datacenter.iers.org/data/latestVersion/9_FINALS.ALL_IAU2000_V2013_019.txt](https://datacenter.iers.org/data/latestVersion/9_FINALS.ALL_IAU2000_V2013_019.txt)
///
/// # Arguments
/// - `filepath`: Path of desired output file
pub fn download_standard_eop_file(filepath: &str) -> Result<(), &str> {
    // Create parent directory
    let filepath = Path::new(filepath);
    let parent_dir = filepath.parent().expect("Failed to identify parent directory.");

    fs::create_dir_all(parent_dir).expect(&*format!("Failed to create directory {}",
                                                    parent_dir.display()));

    let body = ureq::get("https://datacenter.iers.org/data/latestVersion/9_FINALS.ALL_IAU2000_V2013_019.txt")
        .call().expect("Download Request fialed").into_string().expect("Failed to \
    parse response into string");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filepath).expect(&*format!("Failed to create file: {}", filepath.display()));
    writeln!(&mut file, "{}", body).unwrap();

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::env;
    use crate::constants::AS2RAD;
    use crate::eop::*;

    #[test]
    fn test_parse_c04_line() {
        let good_str = "2021  11  23  59541   0.129614   0.247350  -0.1067281  -0.0005456   0\
        .000265  -0.000031   0.000026   0.000019  0.0000079  0.0000069    0.000055    0.000044";
        assert_eq!((59541, 0.129614 * AS2RAD, 0.247350 * AS2RAD, -0.1067281,
                    Some(0.000265 * AS2RAD), Some(-0.000031 * AS2RAD), Some(-0.0005456)),
                   parse_c04_line(good_str).unwrap());

        let bad_str = "2021  11  23  59541   0.abc614   0.247350  -0.1067281  -0.0005456   0\
        .000265  -0.000031   0.000026   0.000019  0.0000079  0.0000069    0.000055    0.000044";
        assert_eq!(parse_c04_line(bad_str).is_err(), true);
    }

    #[test]
    fn test_from_c04_file() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_c04_14.txt");
        let eop_return = EarthOrientationData::from_c04_file(filepath.to_str().unwrap(),
                                                             EOPExtrapolation::Hold,
                                                             true);
        assert_eq!(eop_return.is_err(), false);

        let eop = eop_return.unwrap();
        assert_eq!(eop.data.len(), 21877);
        assert_eq!(eop.mjd_min, 37665);
        assert_eq!(eop.mjd_max, 59541);
        assert_eq!(eop.eop_type, EOPType::C04);
        assert_eq!(eop.extrapolate, EOPExtrapolation::Hold);
        assert_eq!(eop.interpolate, true);
    }

    #[test]
    fn test_from_default_c04() {
        let eop = EarthOrientationData::from_default_c04(EOPExtrapolation::Hold, true);

        // These need to be structured slightly differently since the
        // default package data is regularly updated.
        assert_ne!(eop.data.len(), 0);
        assert_eq!(eop.mjd_min, 37665);
        assert!(eop.mjd_max >= 59541);
        assert_eq!(eop.eop_type, EOPType::C04);
        assert_eq!(eop.extrapolate, EOPExtrapolation::Hold);
        assert_eq!(eop.interpolate, true);
    }

    #[test]
    fn test_parse_standard_eop_line_bulletin_a() {
        // Test good parse
        let good_str = "741231 42412.00 I -0.043558 0.029749  0.265338 0.028736  I-0.2891063 0.0002710  2.9374 0.1916  P    -0.259    0.199    -0.869    0.300  -.039000   .281000  -.2908000   -16.159    -1.585";
        assert_eq!((42412, -0.043558 * AS2RAD, 0.265338 * AS2RAD, -0.2891063,
                    Some(-0.259 * AS2RAD), Some(-0.869 * AS2RAD), Some(2.9374)),
                   parse_standard_eop_line(good_str, EOPType::StandardBulletinA).unwrap());

        // Test prediction w/o LOD data
        let no_lod_str = "22 224 59634.00 P  0.012311 0.006394  0.360715 0.008161  P-0.1074307 0\
        .0063266                 P     0.195    0.128     0.056    0.160                                                     ";
        assert_eq!((59634, 0.012311 * AS2RAD, 0.360715 * AS2RAD, -0.1074307,
                    Some(0.195 * AS2RAD), Some(0.056 * AS2RAD), None),
                   parse_standard_eop_line(no_lod_str, EOPType::StandardBulletinA).unwrap());


        // Test prediction without LOD, dX, dY
        let min_str = "22 327 59665.00 P  0.028851 0.008032  0.417221 0.010886  P-0.1127678 0\
        .0087497                                                                                                             ";
        assert_eq!((59665, 0.028851 * AS2RAD, 0.417221 * AS2RAD, -0.1127678,
                    None, None, None),
                   parse_standard_eop_line(min_str, EOPType::StandardBulletinA).unwrap());


        // Test bad parse
        let bad_str = "75 1 1 42413.00 I -0.043k02 0.024593  0.265903 0.023470  I 0.7078620 0\
        .0002710  3.1173 0.1916  P    -0.267    0.199    -0.880    0.300  -.039000   .281000   \
        .7065000   -16.126    -1.815";
        assert_eq!(parse_standard_eop_line(bad_str, EOPType::StandardBulletinA).is_err(), true);

        // Test parsing wrong type
        assert_ne!((42413, -0.043802 * AS2RAD, 0.265903 * AS2RAD, 0.7078620,
                    Some(-0.267 * AS2RAD), Some(-0.880 * AS2RAD), Some(3.1173)),
                   parse_standard_eop_line(good_str, EOPType::StandardBulletinB).unwrap());
    }

    #[test]
    fn test_parse_standard_eop_line_bulletin_b() {
        // Test good parse
        let good_str = "741231 42412.00 I -0.043558 0.029749  0.265338 0.028736  I-0.2891063 0.0002710  2.9374 0.1916  P    -0.259    0.199    -0.869    0.300  -.039000   .281000  -.2908000   -16.159    -1.585";
        assert_eq!((42412, -0.039000 * AS2RAD, 0.281000 * AS2RAD, -0.2908000,
                    Some(-16.159 * AS2RAD), Some(-1.585 * AS2RAD), Some(0.0)),
                   parse_standard_eop_line(good_str, EOPType::StandardBulletinB).unwrap());

        // Test bad parse
        let bad_str = "75 1 1 42413.00 I -0.043002 0.024593  0.265903 0.023470  I 0.7078620 0\
        .0002710  3.1173 0.1916  P    -0.267    0.199    -0.880    0.300  -.039000   .281000   \
        .7065000   -16.126    -1.81c";
        assert_eq!(parse_standard_eop_line(bad_str, EOPType::StandardBulletinB).is_err(), true);

        // Test parsing wrong type
        assert_ne!((42412, -0.039000 * AS2RAD, 0.281000 * AS2RAD, -0.2908000,
                    Some(-16.159 * AS2RAD), Some(-1.585 * AS2RAD), Some(0.0)),
                   parse_standard_eop_line(good_str, EOPType::StandardBulletinA).unwrap());
    }

    #[test]
    fn test_from_standard_file_bulletin_a() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_finals_ab.txt");
        let eop_return = EarthOrientationData::from_standard_file(filepath.to_str().unwrap(),
                                                                  EOPExtrapolation::Hold,
                                                                  true, EOPType::StandardBulletinA);
        assert_eq!(eop_return.is_err(), false);

        let eop = eop_return.unwrap();
        assert_eq!(eop.data.len(), 18261);
        assert_eq!(eop.mjd_min, 41684);
        assert_eq!(eop.mjd_max, 59944);
        assert_eq!(eop.eop_type, EOPType::StandardBulletinA);
        assert_eq!(eop.extrapolate, EOPExtrapolation::Hold);
        assert_eq!(eop.interpolate, true);
        assert_eq!(eop.mjd_last_lod, 59570);
        assert_eq!(eop.mjd_last_dxdy, 59648);
    }

    #[test]
    fn test_from_default_standard_bulletin_a() {
        let eop = EarthOrientationData::from_default_standard(EOPExtrapolation::Hold, true, EOPType::StandardBulletinA);

        // These need to be structured slightly differently since the
        // default package data is regularly updated.
        assert_ne!(eop.data.len(), 0);
        assert_eq!(eop.mjd_min, 41684);
        assert!(eop.mjd_max >= 59519);
        assert!(eop.mjd_last_lod >= 59570);
        assert!(eop.mjd_last_dxdy >= 59648);
        assert_eq!(eop.eop_type, EOPType::StandardBulletinA);
        assert_eq!(eop.extrapolate, EOPExtrapolation::Hold);
        assert_eq!(eop.interpolate, true);
    }

    #[test]
    fn test_from_standard_file_bulletin_b() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_finals_ab.txt");
        let eop_return = EarthOrientationData::from_standard_file(filepath.to_str().unwrap(),
                                                                  EOPExtrapolation::Hold,
                                                                  true, EOPType::StandardBulletinB);
        assert_eq!(eop_return.is_err(), false);

        let eop = eop_return.unwrap();
        assert_eq!(eop.data.len(), 17836);
        assert_eq!(eop.mjd_min, 41684);
        assert_eq!(eop.mjd_max, 59519);
        assert_eq!(eop.eop_type, EOPType::StandardBulletinB);
        assert_eq!(eop.extrapolate, EOPExtrapolation::Hold);
        assert_eq!(eop.interpolate, true);
        assert_eq!(eop.mjd_last_lod, 0);
        assert_eq!(eop.mjd_last_dxdy, 59519);
    }

    #[test]
    fn test_from_default_standard_bulletin_b() {
        let eop = EarthOrientationData::from_default_standard(EOPExtrapolation::Hold, true,
                                                              EOPType::StandardBulletinB);

        // These need to be structured slightly differently since the
        // default package data is regularly updated.
        assert_ne!(eop.data.len(), 0);
        assert_eq!(eop.mjd_min, 41684);
        assert!(eop.mjd_max >= 59519);
        assert_eq!(eop.mjd_last_lod, 0);
        assert!(eop.mjd_last_dxdy >= 59519);
        assert_eq!(eop.mjd_last_dxdy, eop.mjd_max);
        assert_eq!(eop.eop_type, EOPType::StandardBulletinB);
        assert_eq!(eop.extrapolate, EOPExtrapolation::Hold);
        assert_eq!(eop.interpolate, true);
    }

    #[test]
    fn test_get_ut1_utc() {
        let eop_extrapolation = EOPExtrapolation::Hold;
        let eop_interpolation = true;
        let eop_type = EOPType::StandardBulletinA;
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_finals_ab.txt");
        let eop = EarthOrientationData::from_standard_file(filepath.to_str().unwrap(),
                                                                  eop_extrapolation,
                                                                  eop_interpolation, eop_type).unwrap();

        // Test getting exact point in table
        let ut1_utc = eop.get_ut1_utc(59569.0);
        assert_eq!(ut1_utc, -0.1079838);

        // Test interpolating within table
        let ut1_utc = eop.get_ut1_utc(59569.5);
        assert_eq!(ut1_utc, (-0.1079838 + -0.1075832)/2.0);

        // Test extrapolation hold
        let ut1_utc = eop.get_ut1_utc(59950.0);
        assert_eq!(ut1_utc, -0.0278563);

        // Test extrapolation zero
        let eop_extrapolation = EOPExtrapolation::Zero;
        let eop = EarthOrientationData::from_standard_file(filepath.to_str().unwrap(),
                                                          eop_extrapolation,
                                                          eop_interpolation, eop_type).unwrap();

        let ut1_utc = eop.get_ut1_utc(59950.0);
        assert_eq!(ut1_utc, 0.0);
    }


    #[test]
    fn test_get_pm_xy() {
        let eop_extrapolation = EOPExtrapolation::Hold;
        let eop_interpolation = true;
        let eop_type = EOPType::StandardBulletinA;
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_finals_ab.txt");
        let eop = EarthOrientationData::from_standard_file(filepath.to_str().unwrap(),
                                                                  eop_extrapolation,
                                                                  eop_interpolation, eop_type).unwrap();

        // Test getting exact point in table
        let (pm_x, pm_y) = eop.get_pm(59569.0);
        assert_eq!(pm_x, 0.075367*AS2RAD);
        assert_eq!(pm_y, 0.263430*AS2RAD);

        // Test interpolating within table
        let (pm_x, pm_y) = eop.get_pm(59569.5);
        assert_eq!(pm_x, (0.075367*AS2RAD + 0.073151*AS2RAD)/2.0);
        assert_eq!(pm_y, (0.263430*AS2RAD + 0.264294*AS2RAD)/2.0);

        // Test extrapolation hold
        let (pm_x, pm_y) = eop.get_pm(59950.0);
        assert_eq!(pm_x, 0.096178*AS2RAD);
        assert_eq!(pm_y, 0.252770*AS2RAD);

        // Test extrapolation zero
        let eop_extrapolation = EOPExtrapolation::Zero;
        let eop = EarthOrientationData::from_standard_file(filepath.to_str().unwrap(),
                                                           eop_extrapolation,
                                                           eop_interpolation, eop_type).unwrap();

        let (pm_x, pm_y) = eop.get_pm(59950.0);
        assert_eq!(pm_x, 0.0);
        assert_eq!(pm_y, 0.0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_get_dxdy() {
        let eop_extrapolation = EOPExtrapolation::Hold;
        let eop_interpolation = true;
        let eop_type = EOPType::StandardBulletinA;
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_finals_ab.txt");
        let eop = EarthOrientationData::from_standard_file(filepath.to_str().unwrap(),
                                                                  eop_extrapolation,
                                                                  eop_interpolation, eop_type).unwrap();


        // Test getting exact point in table
        let (dX, dY) = eop.get_dxdy(59569.0);
        assert_eq!(dX, 0.088*AS2RAD);
        assert_eq!(dY, 0.057*AS2RAD);

        // Test interpolating within table
        let (dX, dY) = eop.get_dxdy(59569.5);
        assert_eq!(dX, (0.088*AS2RAD + 0.086*AS2RAD)/2.0);
        assert_eq!(dY, (0.057*AS2RAD + 0.058*AS2RAD)/2.0);

        // Test extrapolation hold
        let (dX, dY) = eop.get_dxdy(59950.0);
        assert_eq!(dX, 0.283*AS2RAD);
        assert_eq!(dY, 0.104*AS2RAD);

        // Test extrapolation zero
        let eop_extrapolation = EOPExtrapolation::Zero;
        let eop = EarthOrientationData::from_standard_file(filepath.to_str().unwrap(),
                                                           eop_extrapolation,
                                                           eop_interpolation, eop_type).unwrap();

        let (dX, dY) = eop.get_dxdy(59950.0);
        assert_eq!(dX, 0.0);
        assert_eq!(dY, 0.0);
    }


    #[test]
    fn test_get_lod() {
        let eop_extrapolation = EOPExtrapolation::Hold;
        let eop_interpolation = true;
        let eop_type = EOPType::StandardBulletinA;
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_finals_ab.txt");
        let eop = EarthOrientationData::from_standard_file(filepath.to_str().unwrap(),
                                                                  eop_extrapolation,
                                                                  eop_interpolation, eop_type).unwrap();

        // Test getting exact point in table
        let lod = eop.get_lod(59569.0);
        assert_eq!(lod, -0.4288);

        // Test interpolating within table
        let lod = eop.get_lod(59569.5);
        assert_eq!(lod, (-0.4288 + -0.3405)/2.0);

        // Test extrapolation hold
        let lod = eop.get_lod(59950.0);
        assert_eq!(lod, -0.3405);

        // Test extrapolation zero
        let eop_extrapolation = EOPExtrapolation::Zero;
        let eop = EarthOrientationData::from_standard_file(filepath.to_str().unwrap(),
                                                           eop_extrapolation,
                                                           eop_interpolation, eop_type).unwrap();

        let lod = eop.get_lod(59950.0);
        assert_eq!(lod, 0.0);
    }

    #[test]
    #[ignore]
    fn test_eop_extrapolation_error() {
        let eop_extrapolation = EOPExtrapolation::Error;
        let eop_interpolation = true;
        let eop_type = EOPType::StandardBulletinA;
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath = Path::new(&manifest_dir).join("test_assets")
            .join("iau2000A_finals_ab.txt");
        let eop = EarthOrientationData::from_standard_file(filepath.to_str().unwrap(),
                                                           eop_extrapolation,
                                                           eop_interpolation, eop_type).unwrap();

        let result = std::panic::catch_unwind(|| eop.get_ut1_utc(59950.0));
        assert!(result.is_err());

        // Polar Motion
        let result = std::panic::catch_unwind(|| eop.get_pm(59950.0));
        assert!(result.is_err());

        // dX, dY
        let result = std::panic::catch_unwind(|| eop.get_dxdy(59950.0));
        assert!(result.is_err());

        // LOD
        let result = std::panic::catch_unwind(|| eop.get_lod(59950.0));
        assert!(result.is_err());
    }
}
