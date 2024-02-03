mod myimpl;

#[cfg( feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(all(feature = "wasm-bindgen", feature = "pyo3"))]
compile_error!("feature \"wasm-bindgen\" and feature \"pyo3\" cannot be enabled at the same time");
pub use crate::myimpl::MyType;

/// Formats the sum of two numbers as string.

/// A Python module implemented in Rust.
#[cfg(feature="pyo3")]
#[pymodule]
fn multilang_playground(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyType>()?;
    Ok(())
}
