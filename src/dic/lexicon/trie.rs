use pyo3::prelude::*;

#[pyclass]
pub struct Trie {
    array: Vec<u32>,
    size: u32, // number of elements
}

#[pymethods]
impl Trie {

    #[new]
    pub fn new(array: Vec<u32>, size: u32) -> Self {
        Trie { array, size }
    }

    pub fn total_size(&self) -> usize {
        4 * self.size as usize
    }

    pub fn common_prefix_search(&self, input: &[u8], offset: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        let mut node_pos: usize = 0;
        let mut unit: usize = *self.array.get(node_pos).unwrap() as usize;
        node_pos ^= Trie::offset(unit);

        for i in offset..input.len() {
            let k = input.get(i).unwrap();
            node_pos ^= *k as usize;
            unit = *self.array.get(node_pos).unwrap() as usize;
            if Trie::label(unit) != *k as usize {
                return result;
            }

            node_pos ^= Trie::offset(unit);
            if Trie::has_leaf(unit) {
                let r = (
                    Trie::value(*self.array.get(node_pos).unwrap() as usize),
                    i + 1,
                );
                result.push(r);
            }
        }

        result
    }

    #[staticmethod]
    fn has_leaf(unit: usize) -> bool {
        ((unit >> 8) & 1) == 1
    }

    #[staticmethod]
    fn value(unit: usize) -> usize {
        unit & ((1 << 31) - 1)
    }

    #[staticmethod]
    fn label(unit: usize) -> usize {
        unit & ((1 << 31) | 0xFF)
    }

    #[staticmethod]
    fn offset(unit: usize) -> usize {
        (unit >> 10) << ((unit & (1 << 9)) >> 6)
    }
}
