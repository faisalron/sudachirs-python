use crate::dic::grammar::Grammar;
use crate::dic::header::Header;
use crate::dic::lexicon::Lexicon;
use crate::lattice::node::Node;
use crate::lattice::Lattice;
use crate::morpheme::Morpheme;

use pyo3::prelude::*;

#[pyclass]
pub struct Tokenizer {
    pub grammar: Grammar,
    pub lexicon: Lexicon,
}

#[derive(PartialEq)]
pub enum Mode {
    A,
    B,
    C,
}

#[pymethods]
impl Tokenizer {
    
    #[new]
    pub fn new(bytes: Vec<u8>) -> Self {
        let mut offset = 0;

        let _header = Header::new(&bytes, offset);
        offset += Header::STORAGE_SIZE;

        let grammar = Grammar::new(bytes.clone(), offset);
        offset += grammar.storage_size;

        let lexicon = Lexicon::new(&bytes, offset);

        Tokenizer {
            grammar,
            lexicon,
        }
    }
    
    #[args(
        mode="\"A\"",
        enable_debug="false" 
    )]
    pub fn tokenize(&self, input: String, mode: &str, enable_debug: bool) -> Vec<Morpheme> {
        let input_bytes = input.as_bytes();

        // build_lattice
        let mut lattice = Lattice::new(input_bytes.len());

        for (i, b) in input_bytes.iter().enumerate() {
            // TODO: if (!input.canBow(i) || !lattice.hasPreviousNode(i)) { continue; }
            if (b & 0xC0) == 0x80 {
                continue;
            }

            for (word_id, end) in self.lexicon.lookup(&input_bytes, i) {
                let (left_id, right_id, cost) = self.lexicon.get_word_param(word_id as usize);
                let node = Node::new(left_id, right_id, cost, word_id);
                lattice.insert(i, end, node, &self.grammar);
            }
        }
        lattice.connect_eos_node(&self.grammar);

        // lattice dump
        if enable_debug {
            println!("=== Lattice dump:");
            let mut i = 0;
            for r_nodes in lattice.end_lists.iter().rev() {
                for r_node in r_nodes {
                    print!("{}: {}: ", i, r_node);
                    for l_node in &lattice.end_lists[r_node.begin] {
                        let connect_cost = self
                            .grammar
                            .get_connect_cost(l_node.right_id, r_node.left_id);
                        let cost = l_node.total_cost + connect_cost as i32;
                        print!("{} ", cost);
                    }
                    println!();
                    i += 1;
                }
            }
            println!("===");
        };

        let node_list = lattice.get_best_path();

        let mut word_id_list = Vec::new();
        if mode == "C" {
            word_id_list = node_list
                .iter()
                .map(|node| node.word_id.unwrap() as usize)
                .collect::<Vec<_>>();
        } else {
            for node in &node_list {
                let node_word_id = node.word_id.unwrap() as usize;
                let word_ids = match mode {
                    "A" => self.lexicon.get_word_info(node_word_id).a_unit_split,
                    "B" => self.lexicon.get_word_info(node_word_id).b_unit_split,
                    _ => panic!(),
                };

                if (word_ids.len() == 0) | (word_ids.len() == 1) {
                    word_id_list.push(node_word_id);
                } else {
                    for word_id in word_ids {
                        word_id_list.push(word_id as usize);
                    }
                }
            }
        };



        let morpheme_list = word_id_list
            .iter()
            .map(|word_id| Morpheme::new(*word_id, &self.grammar, &self.lexicon))
            .collect::<Vec<_>>();

        morpheme_list
    }
}
