use std::collections::HashMap;
use serde::{Deserialize, Serialize};



// #[derive(Serialize, Deserialize)]
#[derive(Default, Serialize, Deserialize)]
pub struct BOW {
    pub id: String,
    pub words: HashMap<String, u32>,
}


impl BOW {
    pub fn add_word(&mut self, word: String) {
        self.words.entry(word)
                  .and_modify(|counter| *counter += 1)
                  .or_insert(1);
    }
}

#[test]
fn test_initalize_new_bow() {
    let new_bow = BOW::default();

    assert_eq!(new_bow.id, "".to_string());
    assert_eq!(new_bow.words, HashMap::new());
}

#[test]
fn test_add_word() {
    let mut bow = BOW::default();
    let word = "word".to_string();
    let word2 = "other".to_string();

    assert_eq!(bow.words.len(), 0);

    bow.add_word(word.clone());

    assert_eq!(bow.words.len(), 1);
    assert_eq!(bow.words.get(&word), Some(1).as_ref());
    
    bow.add_word(word2.clone());

    assert_eq!(bow.words.len(), 2);
    assert_eq!(bow.words.get(&word2), Some(1).as_ref());
}