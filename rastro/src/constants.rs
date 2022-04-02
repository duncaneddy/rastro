#![allow(dead_code)]

/// Constant to convert degrees to radians. Units: [rad/deg]
pub const DEG2RAD:f64 = std::f64::consts::PI/180.0;

/// Constant to convert radians to degrees. Units: [deg/rad]
pub const RAD2DEG:f64 = 180.0/std::f64::consts::PI;

/// Constant to convert arc seconds to radians. Units: [rad/as]
pub const AS2RAD:f64 = DEG2RAD / 3600.0;

/// Constant to convert radians to arc seconds. Units: [ad/rad]
pub const RAD2AS:f64 = RAD2DEG * 3600.0;

// Time Constants //

/// Offset of Modified Julian Days representation with respect to Julian Days.
/// For a time, t, MJD_ZERO is equal to:
///
/// `MJD_ZERO = t_jd - t_mjd`
///
/// Where `t_jd` is the epoch represented in Julian Days, and `t_mjd` is the
/// epoch in Modified Julian Days.
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and Applications*, 2012.
pub const MJD_ZERO:f64 = 2400000.5;


/// Modified Julian Date of January 1, 2000 12:00:00. Value is independent of time
/// system.
///
/// # References:
/// TODO: Fix Reference
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const MJD2000:f64 = 51544.5;

/// Offset of GPS time system with respect to TAI time system. Units: (s)
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///   Applications*, 2012.
pub const GPS_TAI:f64 = -19.0;

/// Offset of TAI time system with respect to GPS time system. Units: (s)
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const TAI_GPS:f64  = -GPS_TAI;

/// Offset of TT time system with respect to TAI time system. Units (s)
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const TT_TAI:f64 = 32.184;

/// Offset of TAI time system with respect to TT time system. Units: (s)
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const TAI_TT:f64 = -TT_TAI;

/// Offset of GPS time system with respect to TT time system. Units: (s)
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const GPS_TT:f64 = GPS_TAI + TAI_TT;

/// Offset of TT time system with respect to GPS time system. Units: (s)
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const TT_GPS:f64 = -GPS_TT;

/// Modified Julian Date of the start of the GPS time system in the GPS time
/// system. This date was January 6, 1980 0H as reckoned in the UTC time
/// system. Units: (s)
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const GPS_ZERO:f64 = 44244.0;

/// Physical Constants //

/// Speed of light in vacuum. Units: (m/s)
///
/// # References:
/// 1. D. Vallado, *Fundamentals of Astrodynamics and Applications (4th Ed.)*, 2010
pub const C_LIGHT:f64 = 299792458.0;

/// Astronomical Unit. Equal to the mean distance of the Earth from the sun.
/// TDB-compatible value. Units: (m)
///
/// # References:
/// 1. P. Gérard and B. Luzum, *IERS Technical Note 36*, 2010
pub const AU:f64 = 1.49597870700e11;


/// Earth's equatorial radius. Units: (m)
///
/// # References:
///  1. J. Ries, S. Bettadpur, R. Eanes, Z. Kang, U. Ko, C. McCullough, P. Nagel, N. Pie, S. Poole, T. Richter, H. Save, and B. Tapley, Development and Evaluation of the Global Gravity Model GGM05, 2016
pub const R_EARTH:f64 = 6.378136300e6;

/// Earth's semi-major axis as defined by the WGS84 geodetic system.
/// Units: (m)
///
/// # References:
///  1. NIMA Technical Report TR8350.2, Department of Defense World Geodetic System 1984, Its Definition and Relationships With Local Geodetic Systems
pub const WGS84_A:f64 = 6378137.0;

/// Earth's ellipsoidal flattening. WGS84 Value. Units: (m)
///
/// # References:
///  1. NIMA Technical Report TR8350.2, Department of Defense World Geodetic System 1984, Its Definition and Relationships With Local Geodetic Systems
pub const WGS84_F:f64 = 1.0/298.257223563;

/// Earth's Gravitational constant. Units: [m^3/s^2]
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const GM_EARTH:f64 = 3.986004415e14;

/// Earth's first eccentricity. WGS84 Value. Units: (dimensionless)
///
/// # References:
///  1. NIMA Technical Report TR8350.2
pub const ECC_EARTH:f64 = 8.1819190842622e-2;

/// Earth's first zonal harmonic. Units: (dimensionless)
///
/// # References:
///  1. GGM05s Gravity Model.
pub const J2_EARTH:f64 = 0.0010826358191967;

/// Earth axial rotation rate. Units: Units: [rad/s]
///
/// # References:
///  1. D. Vallado, *Fundamentals of Astrodynamics and Applications (4th Ed.)*, p. 222, 2010
pub const OMEGA_EARTH:f64 = 7.292115146706979e-5;

/// Gravitational constant of the Sun. Units: [m^3/s^2]
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const GM_SUN:f64 = 132712440041.939400*1e9;

/// Nominal solar photosphere radius. Units: (m)
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const R_SUN:f64 = 6.957*1e8;

/// Nominal solar radiation pressure at 1 AU. Units: [N/m^2]
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const P_SUN:f64 = 4.560E-6;

/// Nominal lunar radius. Units: (m)
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const R_MOON:f64 = 1738.0*1e3;

/// Gravitational constant of the Moon. Units: [m^3/s^2]
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const GM_MOON:f64 = 4902.800066*1e9;

/// Gravitational constant of the Mercury. Units: [m^3/s^2]
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const GM_MERCURY:f64 = 22031.780000*1e9;

/// Gravitational constant of the Venus. Units: [m^3/s^2]
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const GM_VENUS:f64 = 324858.592000*1e9;

/// Gravitational constant of the Mars. Units: [m^3/s^2]
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const GM_MARS:f64 = 42828.37521*1e9;

/// Gravitational constant of the Jupiter. Units: [m^3/s^2]
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const GM_JUPITER:f64 = 126712764.8*1e9;

/// Gravitational constant of the Saturn. Units: [m^3/s^2]
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const GM_SATURN:f64 = 37940585.2*1e9;

/// Gravitational constant of the Uranus. Units: [m^3/s^2]
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const GM_URANUS:f64 = 5794548.6*1e9;

/// Gravitational constant of the Neptune. Units: [m^3/s^2]
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const GM_NEPTUNE:f64 = 6836527.100580*1e9;

/// Gravitational constant of the Pluto. Units: [m^3/s^2]
///
/// # References:
///  1. O. Montenbruck, and E. Gill, *Satellite Orbits: Models, Methods and
///  Applications*, 2012.
pub const GM_PLUTO:f64 = 977.000000*1e9;
