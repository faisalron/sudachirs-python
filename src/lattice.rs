pub mod node;

use std::i32;

use self::node::Node;
use crate::dic::grammar::Grammar;

use pyo3::prelude::*;

#[pyclass]
pub struct Lattice {
    size: usize,
    pub end_lists: Vec<Vec<Node>>,
}

#[pymethods]
impl Lattice {

    #[new]
    pub fn new(size: usize) -> Self {
        let mut end_lists = vec![Vec::<Node>::new(); size + 1];

        let bos_node = Node::new_bos();
        end_lists[0].push(bos_node);

        Lattice {
            size,
            end_lists,
        }
    }

    fn connect_node(&self, r_node: &mut Node, grammar: &Grammar) {
        let begin = r_node.begin;
        r_node.total_cost = i32::MAX;

        for (i, l_node) in self.end_lists[begin].iter().enumerate() {
            if !l_node.is_connected_to_bos {
                continue;
            }

            let connect_cost = grammar
                            .get_connect_cost(l_node.right_id, r_node.left_id);
            let cost = l_node.total_cost + connect_cost as i32;
            if cost < r_node.total_cost {
                r_node.total_cost = cost;
                r_node.best_previous_node_index = Some((begin, i));
            }
        }
        r_node.total_cost += r_node.cost as i32;

        r_node.is_connected_to_bos = match r_node.best_previous_node_index {
            Some(_) => true,
            None => false,
        };
    }

    pub fn insert(&mut self, begin: usize, end: usize, mut node: Node, grammar: &Grammar) {
        node.set_range(begin, end);
        self.connect_node(&mut node, grammar);
        self.end_lists[end].push(node);
    }

    pub fn connect_eos_node(&mut self, grammar: &Grammar) {
        let eos_node = Node::new_eos(self.size);
        self.insert(eos_node.begin, eos_node.end, eos_node, grammar);
    }

    pub fn get_best_path(&self) -> Vec<Node> {
        // TODO: reference of eos_node in struct `Lattice` ?
        let eos_node = self.end_lists[self.size].last().unwrap();

        if !eos_node.is_connected_to_bos {
            panic!("EOS isn't connected to BOS");
        }

        let mut path = Vec::new();

        let mut node = eos_node;
        // todo
        let (i, j) = node.best_previous_node_index.unwrap();
        let mut i = i;
        let mut j = j;
        while (i, j) != (0, 0) {
            path.push((*node).clone());

            // todo
            let (a, b) = node.best_previous_node_index.unwrap();
            i = a;
            j = b;
            node = &self.end_lists[i][j];
        }

        path.reverse();
        path.pop(); // EOS

        path
    }
}
