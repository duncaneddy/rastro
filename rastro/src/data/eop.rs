use std::fmt;
use std::fs::File;
use std::str::FromStr;
use std::io::{BufReader, Read};
use std::io::prelude::*;
use std::collections::HashMap;
use crate::constants::AS2RAD;

// Package EOP data as part of crate
/// Packaged C04 EOP Data File
static PACKAGED_C04_FILE: &'static [u8] = include_bytes!("../../data/iau2000A_c04_14.txt");
/// Packaged Finals 2000A Data File
static PACKAGED_FINALS2000_FILE: &'static [u8] = include_bytes!("../../data/iau2000A_finals_ab.txt");

/// Enumerated value that indicates the preferred behavior of the Earth Orientation Data provider
/// when the desired time point is not present.
///
/// # Values
/// - `Zero`: Return a value of zero for the missing data
/// - `Hold`: Return the last value prior to the requested date
/// - `Error`: Throw an
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
    eop_type: EOPType,
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
    /// - `lod`: Difference between astronomically determined length of day and 86400 second TAI
    ///   day. Units: (seconds)
    data: HashMap<u32, (f64, f64, f64, Option<f64>, Option<f64>, Option<f64>)>,
    /// Defines desired behavior for out-of-bounds Earth Orientation data access
    extrapolate: EOPExtrapolation,
    /// Defines interpolation behavior of data for requests between data points in table.
    ///
    /// When set to `true` data will be linearly interpolated to the desired time.
    /// When set to `false` data will be given as the value as the closest previous data entry
    /// present.
    interpolate: bool,
    /// Minimum date of stored data. This is the value of the smallest key stored in the `data`
    /// HashMap. Value is a modified Julian date.
    mjd_min: u32,
    /// Maximum date of stored data. This is the value of the largest key stored in the `data`
    /// HashMap. Behavior
    /// of data retrieval for dates larger than this will be defined by the `extrapolate` value.
    /// Babylon's Fall
    mjd_max: u32,
    /// Modified Julian date of last valid Length of Day (LOD) value. Only applicable for
    /// Bulletin A EOP data. Will be 0 for Bulletin B data and the same as `mjd_max` for C04 data.
    mjd_last_lod: u32,
    /// Modified Julian date of last valid precession/nutation dX/dY correction values. Only
    /// applicable for Bulletin A. Will always be the sam as `mjd_max` for Bulletin B and C04 data.
    mjd_last_dxdy: u32,
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
/// - `lod`: Difference between astronomically determined length of day and 86400 second TAI
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
/// - `lod`: Difference between astronomically determined length of day and 86400 second TAI
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
    pub fn from_default_standard(extrapolate: EOPExtrapolation, interpolate: bool, eop_type: EOPType) -> Self {
        let reader = BufReader::new(PACKAGED_FINALS2000_FILE);
        eop_standard_eop_from_bufreader(reader, extrapolate, interpolate, eop_type).expect("Failed to \
        parse and \
        load packed Standard Earth Orientation Data.")
    }
}

// impl Default for EarthOrientationData {
//     fn default() -> Self { EarthOrientationData::from_default_standard_eop() }
// }

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::env;
    use crate::constants::AS2RAD;
    use crate::data::eop::*;

    #[test]
    fn test_parse_c04_line() {
        let good_str = "2021  11  23  59541   0.129614   0.247350  -0.1067281  -0.0005456   0\
        .000265  -0.000031   0.000026   0.000019  0.0000079  0.0000069    0.000055    0.000044";
        assert_eq!((59541, 0.129614*AS2RAD, 0.247350*AS2RAD, -0.1067281,
                    Some(0.000265*AS2RAD),  Some(-0.000031*AS2RAD), Some(-0.0005456)),
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
        assert_eq!((42412, -0.043558*AS2RAD, 0.265338*AS2RAD, -0.2891063,
                    Some(-0.259*AS2RAD),  Some(-0.869*AS2RAD), Some(2.9374)),
                   parse_standard_eop_line(good_str, EOPType::StandardBulletinA).unwrap());

        // Test prediction w/o LOD data
        let no_lod_str = "22 224 59634.00 P  0.012311 0.006394  0.360715 0.008161  P-0.1074307 0\
        .0063266                 P     0.195    0.128     0.056    0.160                                                     ";
        assert_eq!((59634, 0.012311*AS2RAD, 0.360715*AS2RAD, -0.1074307,
                    Some(0.195*AS2RAD),  Some(0.056*AS2RAD), None),
                   parse_standard_eop_line(no_lod_str, EOPType::StandardBulletinA).unwrap());


        // Test prediction without LOD, dX, dY
        let min_str = "22 327 59665.00 P  0.028851 0.008032  0.417221 0.010886  P-0.1127678 0\
        .0087497                                                                                                             ";
        assert_eq!((59665, 0.028851*AS2RAD, 0.417221*AS2RAD, -0.1127678,
                    None,  None, None),
                   parse_standard_eop_line(min_str, EOPType::StandardBulletinA).unwrap());


        // Test bad parse
        let bad_str = "75 1 1 42413.00 I -0.043k02 0.024593  0.265903 0.023470  I 0.7078620 0\
        .0002710  3.1173 0.1916  P    -0.267    0.199    -0.880    0.300  -.039000   .281000   \
        .7065000   -16.126    -1.815";
        assert_eq!(parse_standard_eop_line(bad_str, EOPType::StandardBulletinA).is_err(), true);

        // Test parsing wrong type
        assert_ne!((42413, -0.043802*AS2RAD, 0.265903*AS2RAD, 0.7078620,
                    Some(-0.267*AS2RAD),  Some(-0.880*AS2RAD), Some(3.1173)),
                   parse_standard_eop_line(good_str, EOPType::StandardBulletinB).unwrap());
    }

    #[test]
    fn test_parse_standard_eop_line_bulletin_b() {
        // Test good parse
        let good_str = "741231 42412.00 I -0.043558 0.029749  0.265338 0.028736  I-0.2891063 0.0002710  2.9374 0.1916  P    -0.259    0.199    -0.869    0.300  -.039000   .281000  -.2908000   -16.159    -1.585";
        assert_eq!((42412, -0.039000*AS2RAD, 0.281000*AS2RAD, -0.2908000,
                    Some(-16.159*AS2RAD),  Some(-1.585*AS2RAD), Some(0.0)),
                   parse_standard_eop_line(good_str, EOPType::StandardBulletinB).unwrap());

        // Test bad parse
        let bad_str = "75 1 1 42413.00 I -0.043002 0.024593  0.265903 0.023470  I 0.7078620 0\
        .0002710  3.1173 0.1916  P    -0.267    0.199    -0.880    0.300  -.039000   .281000   \
        .7065000   -16.126    -1.81c";
        assert_eq!(parse_standard_eop_line(bad_str, EOPType::StandardBulletinB).is_err(), true);

        // Test parsing wrong type
        assert_ne!((42412, -0.039000*AS2RAD, 0.281000*AS2RAD, -0.2908000,
                    Some(-16.159*AS2RAD),  Some(-1.585*AS2RAD), Some(0.0)),
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
        assert_ne!(eop.data.len(), 0);
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
}