use edit_distance::edit_distance;
use std::cmp;

const TOKEN_DIFF_TRESHOLD: f32 = 0.17;

pub fn are_tokens_similar(a: &String, b: &String) -> bool {
    let diff = edit_distance(a, b);

    if diff == 0 {
        return true;
    };

    let longest_len = cmp::max(a.len(), b.len()) as f32;
    let diff_percent = (diff as f32) / longest_len;

    TOKEN_DIFF_TRESHOLD > diff_percent
}

#[test]
fn test_are_tokens_similar() {
    let token1 = "aaa".to_string();
    let token2 = "aaa".to_string();
    let result = are_tokens_similar(&token1, &token2);

    assert_eq!(result, true);

    let token1 = "aab".to_string();
    let token2 = "aaa".to_string();
    let result = are_tokens_similar(&token1, &token2);

    assert_eq!(result, false);

    let token1 = "aaa".to_string();
    let token2 = "aaaa".to_string();
    let result = are_tokens_similar(&token1, &token2);

    assert_eq!(result, false);

    let token1 = "aaaa".to_string();
    let token2 = "aaab".to_string();
    let result = are_tokens_similar(&token1, &token2);

    assert_eq!(result, false);

    let token1 = "aaaaa".to_string();
    let token2 = "aaaab".to_string();
    let result = are_tokens_similar(&token1, &token2);

    assert_eq!(result, false);

    let token1 = "aaaaa".to_string();
    let token2 = "aaaaaa".to_string();
    let result = are_tokens_similar(&token1, &token2);

    assert_eq!(result, true);

    let token1 = "aaaaaaaa".to_string();
    let token2 = "aaaaaaab".to_string();
    let result = are_tokens_similar(&token1, &token2);

    assert_eq!(result, true);

    let token1 = "aaaaaaaa".to_string();
    let token2 = "aaaaaabb".to_string();
    let result = are_tokens_similar(&token1, &token2);

    assert_eq!(result, false);

    let token1 = "aaaaaaaaaaaa".to_string();
    let token2 = "aaaaaaaaaabb".to_string();
    let result = are_tokens_similar(&token1, &token2);

    assert_eq!(result, true);

    let token1 = "aaaaaaaaaaaa".to_string();
    let token2 = "aaaaaaaaabbb".to_string();
    let result = are_tokens_similar(&token1, &token2);

    assert_eq!(result, false);
}
