#[macro_use]
extern crate nom;

pub mod morpheme;
pub mod tokenizer;

pub mod dic;
pub mod lattice;

use crate::tokenizer::Tokenizer;
use crate::dic::Dictionary;
use crate::morpheme::Morpheme;

use pyo3::prelude::*;


#[pymodule]
fn sudachirs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Tokenizer>()?;
    m.add_class::<Dictionary>()?;
    m.add_class::<Morpheme>()?;
    Ok(())
}