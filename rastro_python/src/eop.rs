use pyo3::{exceptions::PyRuntimeError, prelude::*, PyObjectProtocol};
use pyo3::wrap_pyfunction;

use std::collections::HashMap;
use rastro::eop as eop;

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
struct EarthOrientationData {
    /// Stored object for underlying EOP
    robj: eop::EarthOrientationData,
}

// Define python class attributes
#[pyproto]
impl PyObjectProtocol for EarthOrientationData {
    fn __repr__(&self) -> String {
        format!("EarthOrientationData<type: {}, {} entries, mjd_min: {}, mjd_max: {},  \
        mjd_last_lod: \
        {}, mjd_last_dxdy: {}, extrapolate: {}, \
        interpolate: {}>", self.robj.eop_type, self.robj.data.len(), self.robj.mjd_min, self.robj
            .mjd_max,
                self.robj.mjd_last_lod, self.robj.mjd_last_dxdy, self.robj.extrapolate, self.robj.interpolate)
    }

    fn __str__(&self) -> String {
        format!("EarthOrientationData<type: {}, {} entries, mjd_min: {}, mjd_max: {},  \
        mjd_last_lod: \
        {}, mjd_last_dxdy: {}, extrapolate: {}, \
        interpolate: {}>", self.robj.eop_type, self.robj.data.len(), self.robj.mjd_min, self.robj.mjd_max,
                self.robj.mjd_last_lod, self.robj.mjd_last_dxdy, self.robj.extrapolate, self.robj.interpolate)
    }
}

#[pymethods]
impl EarthOrientationData {
    // Define attribute access methods
    /// `str`: Type of Earth orientation data loaded. Can be "C04", "StandardBulletinA", or "StandardBulletinB"
    #[getter]
    fn eop_type(&self) -> String {
        match self.robj.eop_type {
            eop::EOPType::C04 => String::from("C04"),
            eop::EOPType::StandardBulletinA => String::from("StandardBulletinA"),
            eop::EOPType::StandardBulletinB => String::from("StandardBulletinB")
        }
    }

    #[getter]
    /// `str`: Extrapolation setting. Can be "Zero", "Hold", or "Error"
    fn extrapolate(&self) -> String {
        match self.robj.extrapolate {
            eop::EOPExtrapolation::Zero => String::from("Zero"),
            eop::EOPExtrapolation::Hold => String::from("Hold"),
            eop::EOPExtrapolation::Error => String::from("Error")
        }
    }

    /// `bool`: Whether to interpolate for time between loaded data set
    #[getter]
    fn interpolate(&self) -> bool {
        self.robj.interpolate
    }

    /// mjd_min (`float`): Minimum date of stored data. This is the value of the smallest
    /// accessible date
    #[getter]
    fn mjd_min(&self) -> u32 {
        self.robj.mjd_min
    }

    /// mjd_min (`float`): Minimum date of stored data. This is the value of the largest
    /// accessible date
    #[getter]
    fn mjd_max(&self) -> u32 {
        self.robj.mjd_max
    }

    /// mjd_last_lod (`float`): Minimum date of stored data. This is the value of the last date
    /// which will return a LOD value not given by the `extrapolate` value
    #[getter]
    fn mjd_last_lod(&self) -> u32 {
        self.robj.mjd_last_lod
    }

    /// mjd_last_lod (`float`): Minimum date of stored data. This is the value of the last date
    /// which will return dX and dY values not given by the `extrapolate` value
    #[getter]
    fn mjd_last_dxdy(&self) -> u32 {
        self.robj.mjd_last_dxdy
    }

    /// Return length of stored data array
    ///
    /// Returns:
    ///     len (`int`): Number of data points in loaded Earth orientation data file.
    fn len(&self) -> usize {
        self.robj.data.len()
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
            Ok(eop_obj) => Ok(EarthOrientationData{robj:eop_obj}),
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

        Ok(EarthOrientationData{robj:eop_obj})
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
            Ok(eop_obj) => Ok(EarthOrientationData{robj:eop_obj}),
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

        Ok(EarthOrientationData{robj:eop_obj})
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
        self.robj.get_ut1_utc(mjd)
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
        self.robj.get_pm(mjd)
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
        self.robj.get_dxdy(mjd)
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
        self.robj.get_lod(mjd)
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
        self.robj.get_eop(mjd)
    }
}

#[pymodule]
pub fn eop(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<EarthOrientationData>()?;
    Ok(())
}