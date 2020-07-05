use nom::{le_u32, le_u8};

use pyo3::prelude::*;

#[pyclass]
pub struct WordIdTable {
    bytes: Vec<u8>,
    size: u32,
    offset: usize,
}

#[pymethods]
impl WordIdTable {
    
    #[new]
    pub fn new(bytes: Vec<u8>, size: u32, offset: usize) -> Self {
        WordIdTable {
            bytes,
            size,
            offset,
        }
    }

    pub fn storage_size(&self) -> usize {
        4 + self.size as usize
    }

    pub fn get(&self, index: usize) -> Vec<u32> {
        let (_rest, result) = word_id_table_parser(&self.bytes, self.offset, index).unwrap();
        result
    }
}

named_args!(
    word_id_table_parser(offset: usize, index: usize)<&[u8], Vec<u32>>,
    do_parse!(
        _seek: take!(offset + index) >>
        length: le_u8 >>
        result: count!(le_u32, length as usize) >>

        (result)
    )
);
