use crate::dic::grammar::Grammar;
use crate::dic::lexicon::word_infos::WordInfo;
use crate::dic::lexicon::Lexicon;

use pyo3::prelude::*;

#[pyclass]
pub struct Morpheme {
    word_info: WordInfo,
    _pos: Vec<String>,
}

#[pymethods]
impl Morpheme {

    #[new]
    pub fn new(word_id: usize, grammar: &Grammar, lexicon: &Lexicon) -> Self {
        let word_info = lexicon.get_word_info(word_id);
        let _pos = grammar.pos_list.get(word_info.pos_id as usize).unwrap().to_vec();
        Morpheme { word_info, _pos }
    }

    pub fn surface(&self) -> &String {
        &self.word_info.surface
    }

    pub fn pos(&self) -> Vec<String> {
        self._pos.clone()
    }

    pub fn normalized_form(&self) -> &String {
        &self.word_info.normalized_form
    }

    pub fn reading_form(&self) -> &String {
        &self.word_info.reading_form
    }

    pub fn dictionary_form(&self) -> &String {
        &self.word_info.dictionary_form
    }
}
