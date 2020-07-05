use nom::le_i16;

use pyo3::prelude::*;

#[pyclass]
pub struct WordParams {
    bytes: Vec<u8>,
    size: u32,
    offset: usize,
}

#[pymethods]
impl WordParams {
    const ELEMENT_SIZE: usize = 2 * 3;

    #[new]
    pub fn new(bytes: Vec<u8>, size: u32, offset: usize) -> Self {
        WordParams {
            bytes,
            size,
            offset,
        }
    }

    pub fn storage_size(&self) -> usize {
        4 + WordParams::ELEMENT_SIZE * self.size as usize
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn get_left_id(&self, word_id: usize) -> i16 {
        let (_rest, num) =
            i16_parser(&self.bytes, self.offset + WordParams::ELEMENT_SIZE * word_id).unwrap();
        num
    }

    pub fn get_right_id(&self, word_id: usize) -> i16 {
        let (_rest, num) = i16_parser(
            &self.bytes,
            self.offset + WordParams::ELEMENT_SIZE * word_id + 2,
        )
        .unwrap();
        num
    }

    pub fn get_cost(&self, word_id: usize) -> i16 {
        let (_rest, num) = i16_parser(
            &self.bytes,
            self.offset + WordParams::ELEMENT_SIZE * word_id + 4,
        )
        .unwrap();
        num
    }
}

named_args!(
    i16_parser(offset: usize)<&[u8], i16>,
    do_parse!(
        _seek: take!(offset) >>
        num: le_i16 >>

        (num)
    )
);
