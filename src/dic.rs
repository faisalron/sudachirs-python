pub mod grammar;
pub mod header;
pub mod lexicon;

use crate::tokenizer::Tokenizer;

use pyo3::prelude::*;

#[pyclass]
pub struct Dictionary {
    bytes: Vec<u8>
}

#[pymethods]
impl Dictionary {
    #[new]
    pub fn new() -> Self {
      let bytes = include_bytes!("resources/system.dic");
      
      Dictionary {
        bytes: bytes.to_vec()
      }
    }

    pub fn create(&self) -> Tokenizer {
      let tokenizer = Tokenizer::new(self.bytes.clone());

      tokenizer
    }
}