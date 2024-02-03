#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg_attr(feature = "pyo3", pyclass)]
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
#[derive(Debug)]
pub struct MyType{
    info: u32,
    name: String,
}

#[cfg_attr(feature = "pyo3", pymethods)]
#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
impl MyType{
    #[cfg(feature = "pyo3")]
    #[new]
    pub fn new(i: u32, n: String)->Self{
        return Self{
            info: i,
            name: n
        }
    }
    #[cfg(feature = "wasm-bindgen")]
    #[wasm_bindgen(constructor)]
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
