use crate::token_similarity::are_tokens_similar;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::collections::HashMap;

const FUZZY_MATCH_DISCOUNT: f32 = 0.9;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BOW {
    pub entity_id: String,
    pub words: HashMap<String, i32>,
    pub words_count: i32,
}

impl BOW {
    pub fn add_word(&mut self, word: String) {
        self.words
            .entry(word)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        self.words_count += 1;
    }
}

pub fn compute_similarity(a: &BOW, b: &BOW) -> f32 {
    if a.words == b.words {
        return 1.0;
    }

    let longest_len = cmp::max(a.words_count, b.words_count) as f32;
    let mut similar_tokens = 0.0;

    for (token_a, count_a) in &a.words {
        match b.words.get(token_a) {
            Some(count_b) => {
                similar_tokens += common_count(count_a, count_b);
            }
            None => {
                for token_b in b.words.keys() {
                    if are_tokens_similar(token_a, token_b) {
                        let count_b = b.words.get(token_b).unwrap();
                        similar_tokens += common_count(count_a, count_b) * FUZZY_MATCH_DISCOUNT;
                        break;
                    }
                }
            }
        }
    }

    similar_tokens / longest_len
}

fn common_count(a: &i32, b: &i32) -> f32 {
    (cmp::max(a, b) - (a - b).abs()) as f32
}

#[test]
fn test_compute_similarity() {
    let mut a = BOW::default();
    a.add_word("word".to_string());
    a.add_word("word".to_string());
    a.add_word("other".to_string());

    let mut b = BOW::default();
    b.add_word("word".to_string());
    b.add_word("word".to_string());
    b.add_word("other".to_string());

    assert_eq!(compute_similarity(&a, &b), 1.0);

    let mut a = BOW::default();
    a.add_word("word".to_string());
    a.add_word("word".to_string());
    a.add_word("word".to_string());
    a.add_word("word".to_string());

    let mut b = BOW::default();
    b.add_word("word".to_string());
    b.add_word("word".to_string());
    b.add_word("word".to_string());
    b.add_word("other".to_string());

    assert_eq!(compute_similarity(&a, &b), 0.75);

    let mut a = BOW::default();
    a.add_word("product".to_string());

    let mut b = BOW::default();
    b.add_word("products".to_string());

    assert_eq!(compute_similarity(&a, &b), 0.9);

    let mut a = BOW::default();
    a.add_word("one".to_string());
    a.add_word("product".to_string());

    let mut b = BOW::default();
    b.add_word("two".to_string());
    b.add_word("products".to_string());

    assert_eq!(compute_similarity(&a, &b), 0.45);

    let mut a = BOW::default();
    a.add_word("word".to_string());
    a.add_word("product".to_string());

    let mut b = BOW::default();
    b.add_word("word".to_string());
    b.add_word("products".to_string());

    assert_eq!(compute_similarity(&a, &b), 0.95);
}

#[test]
fn test_initalize_new_bow() {
    let new_bow = BOW::default();

    assert_eq!(new_bow.entity_id, "".to_string());
    assert_eq!(new_bow.words, HashMap::new());
}

#[test]
fn test_add_word() {
    let mut bow = BOW::default();
    let word = "word".to_string();
    let word2 = "other".to_string();

    assert_eq!(bow.words_count, 0);
    assert_eq!(bow.words.len(), 0);

    bow.add_word(word.clone());

    assert_eq!(bow.words_count, 1);
    assert_eq!(bow.words.len(), 1);
    assert_eq!(bow.words.get(&word), Some(1).as_ref());

    bow.add_word(word2.clone());

    assert_eq!(bow.words_count, 2);
    assert_eq!(bow.words.len(), 2);
    assert_eq!(bow.words.get(&word2), Some(1).as_ref());

    bow.add_word(word2.clone());

    assert_eq!(bow.words_count, 3);
    assert_eq!(bow.words.len(), 2);
    assert_eq!(bow.words.get(&word2), Some(2).as_ref());
}
