use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[derive(Debug,Clone,PartialEq,Copy)]
pub enum RClassEnum {
    Apple,
    Watermelon
}

#[derive(Clone,Debug)]
pub struct HiddenRClass {
    pub secret: u32,
    pub data: HashMap<u32, f64>,
    pub val: RClassEnum,
}

impl HiddenRClass {
    fn new(secret:u32, data: HashMap<u32, f64>) -> Self {
        Self{secret, data, val: RClassEnum::Apple}
    }

    fn get_data(&self) -> f64 {
        self.data[&0]
    }
}

#[pyclass]
#[derive(Clone,Debug)]
pub struct RClass {
    pub internal: HiddenRClass,
    pub count: u32,
    pub truth: bool,
}

#[pymethods]
impl RClass {
    #[new]
    fn new(count: u32, truth: bool) -> Self {
        let mut data = HashMap::new();
        data.insert(0, 1.0);
        let internal= HiddenRClass{
            secret: 0,
            data,
            val: RClassEnum::Watermelon
        };
        Self{internal, count, truth}
    }

    #[getter]
    fn count(&self) -> u32 {
        self.count
    }

    #[getter]
    fn truth(&self) -> bool {
        self.truth
    }
}

#[pyfunction]
#[pyo3(text_signature = "(rclass)")]
fn count_class(r: RClass) -> PyResult<u32> {
    Ok(r.count + 1)
}

#[pyfunction]
#[pyo3(text_signature = "(rclass)")]
fn access_internal(r: RClass) -> PyResult<f64> {
    println!("{:?}", r.internal);
    println!("{:p}", &r.internal.data);
    Ok(r.internal.get_data())
}

#[pyfunction]
#[pyo3(text_signature = "(rclass)")]
fn add_internal(r: &mut RClass) -> PyResult<()> {
    println!("{:?}", r.internal);
    println!("{:p}", &r.internal.data);
    r.internal.data.insert(0, r.internal.get_data() + 1.0);
    println!("{:?}", r.internal);
    println!("{:p}", &r.internal.data);
    Ok(())
}

#[pymodule]
pub fn test(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<RClass>()?;
    module.add_function(wrap_pyfunction!(count_class, module)?)?;
    module.add_function(wrap_pyfunction!(access_internal, module)?)?;
    module.add_function(wrap_pyfunction!(add_internal, module)?)?;
    Ok(())
}