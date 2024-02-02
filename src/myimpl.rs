#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(all(feature = "wasm-bindgen", not(feature="pyo3")))]
#[wasm_bindgen]
#[derive(Debug)]
pub struct MyType{
    info: u32,
    name: String,
}

#[cfg(feature = "pyo3")]
use pyo3::pyclass;

#[cfg(all(feature = "pyo3", not(feature="wasm-bindgen")))]
#[pyclass]
#[derive(Debug)]
pub struct MyType{
    info: u32,
    name: String,
}

#[cfg(all(not(feature = "wasm-bindgen"), not(feature = "pyo3")))]
#[derive(Debug)]
pub struct MyType{
    info: u32,
    name: String,
}

impl MyType{
    pub fn new(i: u32, n: String)->Self{
        return Self{
            info: i,
            name: n
        }
    }
    pub fn do_stuff(&self)->String{
        format!("Hello world {}", self.name).to_string()
    }
    pub fn do_other_stuff(&self)->u32{
        self.info+self.name.len() as u32
    }
}
