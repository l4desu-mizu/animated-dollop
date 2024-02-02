mod myimpl;

#[cfg( feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(all(feature = "foo", feature = "bar"))]
compile_error!("feature \"foo\" and feature \"bar\" cannot be enabled at the same time");
use crate::myimpl::MyType;

/// Formats the sum of two numbers as string.
#[cfg(feature = "pyo3")]
#[pyfunction]
fn create_objp() -> PyResult<MyType> {
    Ok(MyType::new(2, "hello python".to_string()))
}

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
pub fn create_objs() -> MyType{
    MyType::new(5, "hello js".to_string())
}

/// A Python module implemented in Rust.
#[cfg(feature="pyo3")]
#[pymodule]
fn multilang_playground(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_objp, m)?)?;
    Ok(())
}
