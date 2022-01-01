use pyo3::{exceptions::PyRuntimeError, prelude::*, PyObjectProtocol};
use pyo3::wrap_pyfunction;
use rastro::eop as eop;

#[pyclass]
struct EarthOrientationData {
    robj: eop::EarthOrientationData,
}

#[pymethods]
impl EarthOrientationData {
    // the signature for the constructor is attached
    // to the struct definition instead.
    // #[new]
    // fn new(c: i32, d: &str) -> Self {
    // }

    // #[staticmethod]
    // #[text_signature = "(e, f)"]
    // fn from_c04_file(e: i32, f: i32) -> i32 {
    //     e + f
    // }

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

    // #[staticmethod]
    // #[text_signature = "(e, f)"]
    // fn from_standard_file(e: i32, f: i32) -> i32 {
    //     e + f
    // }

    // #[staticmethod]
    // #[text_signature = "(e, f)"]
    // fn from_default_standard(e: i32, f: i32) -> i32 {
    //     e + f
    // }
}

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

#[pymodule]
pub fn eop(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<EarthOrientationData>()?;
    Ok(())
}