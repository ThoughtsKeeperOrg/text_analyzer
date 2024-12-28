use regex::Regex;

pub fn tokens_from_text(text: &String) -> Vec<&str> {
    let pattern = Regex::new(r"\b\w{3,20}\b*").unwrap();

    pattern.find_iter(text).map(|mtch| mtch.as_str()).collect()
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
