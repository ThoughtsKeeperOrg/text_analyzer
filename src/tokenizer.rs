use crate::bow::*;
use regex::Regex;

pub fn bow_from_text(text: String) -> BOW {
    let mut bow = BOW::default();
    let tokens = tokens_from_text(&text);

    for token in tokens.iter() {
        bow.add_word(token.to_string());
    }

    bow
}

pub fn tokens_from_text(text: &String) -> Vec<&str> {
    let pattern = Regex::new(r"\b\w{3,20}\b*").unwrap();

    pattern.find_iter(text).map(|m| m.as_str()).collect()
}

#[test]
fn test_tokens_from_text() {
    let text = "two words".to_string();
    let tokens = tokens_from_text(&text);

    assert_eq!(tokens[0], "two");
    assert_eq!(tokens[1], "words");

    let text = "two-words".to_string();
    let tokens = tokens_from_text(&text);

    assert_eq!(tokens[0], "two");
    assert_eq!(tokens[1], "words");

    let text = "two'words".to_string();
    let tokens = tokens_from_text(&text);

    assert_eq!(tokens[0], "two");
    assert_eq!(tokens[1], "words");

    let text = "two?words".to_string();
    let tokens = tokens_from_text(&text);

    assert_eq!(tokens[0], "two");
    assert_eq!(tokens[1], "words");

    let text = "two.words".to_string();
    let tokens = tokens_from_text(&text);

    assert_eq!(tokens[0], "two");
    assert_eq!(tokens[1], "words");

    let text = "two words 333".to_string();
    let tokens = tokens_from_text(&text);

    assert_eq!(tokens[0], "two");
    assert_eq!(tokens[1], "words");
    assert_eq!(tokens[2], "333");

    let text = "an ape".to_string();
    let tokens = tokens_from_text(&text);

    assert_eq!(tokens[0], "ape");

    let text = "two ? words".to_string();
    let tokens = tokens_from_text(&text);

    assert_eq!(tokens[0], "two");
    assert_eq!(tokens[1], "words");
}

#[test]
fn test_bow_from_text() {
    let text1 = "two words".to_string();
    let bow = bow_from_text(text1.clone());

    assert_eq!(bow.entity_id, "".to_string());
    assert_eq!(bow.words.len(), 2);
}
