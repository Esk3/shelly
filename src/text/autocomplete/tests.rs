use super::*;

#[test]
fn autocomplete_word_in_dict_reaturns_word() {
    let dict = vec!["hello", "world"];
    let autocomplete = AutoComplete::new(dict);
    let result = autocomplete.prefix_matching("hello");
    assert_eq!(result.first().unwrap().word().unwrap(), "hello");
}

#[test]
fn autocomplete_returns_sorted_list_of_matching_words() {
    let dict = ["123", "abc", "abc_xyz", "ab"];
    let autocomplete = AutoComplete::new(dict);
    let result = autocomplete.prefix_matching("a");
    let expected = ["ab", "abc", "abc_xyz"];
    assert_eq!(
        result
            .into_iter()
            .map(|r| r.word().unwrap().to_string())
            .collect::<Vec<_>>(),
        expected
    );
}
