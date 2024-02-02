mod myimpl;

use pyo3::prelude::*;
use wasm_bindgen::prelude::*;
use crate::myimpl::MyType;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn create_objp() -> PyResult<MyType> {
    Ok(MyType::new(2, "hello python".to_string()))
}

#[wasm_bindgen]
pub fn create_objs() -> MyType{
    MyType::new(5, "hello js".to_string())
}

#[pyfunction]
#[wasm_bindgen]
pub fn create_obj() -> MyType{
    MyType::new(6, "Hello world".to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn multilang_playground(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_objp, m)?)?;
    Ok(())
}
