use dictionary::Dictionary;

use crate::{
    commands::{cd, ShellCommands},
    fs::tests::MockFs,
};

use super::*;

#[test]
fn dict_returns_iter_of_all_words_added() {
    let mut dict = Dictionary::new();
    assert_eq!(dict.words().collect::<Vec<&str>>(), Vec::<&str>::new());
    dict.add("hello");
    dict.add_all(["hi", "world"]);
    let mut words = dict.words().collect::<Vec<_>>();
    words.sort_unstable();
    let mut expected = ["hello", "hi", "world"];
    expected.sort_unstable();
    assert_eq!(words, expected);
}

#[test]
fn init_dict_from_router_and_file_system() {
    let mut router = ShellCommands::new(Vec::new());
    router.add(cd::Cd);
    let fs = MockFs::new(Vec::new(), Vec::new());
    let dict = Dictionary::init(&router, &fs);
    let mut words = dict.words().collect::<Vec<_>>();
    words.sort_unstable();
    let mut expected = ["cd"];
    expected.sort_unstable();
    assert_eq!(words, expected);
}

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
