use pyo3::pyclass;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[pyclass]
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
