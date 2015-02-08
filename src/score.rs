
extern crate regex;
extern crate core;

use std::str;
use std::slice::SliceConcatExt;
use self::core::str::StrExt;
use self::regex::Regex;
use std::num::ToPrimitive;

pub fn score(choice: &str, query: &str) -> f64 {
    if choice == "" || query.char_len() > choice.char_len() {
        0f64
    } else if query == "" {
        1f64
    } else {
        let match_length = compute_match_length(choice, query);

        if match_length > 0 {
            1f64 / choice.char_len().to_f64().unwrap()
        } else {
            0f64
        }
    }
}

// Find the length of the shortest substring matching the given characters.
fn compute_match_length(string: &str, query: &str) -> usize {
    let re_string = make_query_regex(query);
    let re = match Regex::new(re_string.as_slice()) {
        Ok(re)   => re,
        Err(err) => panic!("{}", err.msg),
    };

    if re.is_match(string) {
        let caps = re.captures(string).unwrap();
        match caps.at(0) {
            Some(s) => return s.char_len().to_uint().unwrap(),
            None    => return 0us
        };
    } else {
        0us
    }
}

// Creates a regex for performing a case-insensitive non-greedy fuzzy match.
// Turns "abc" into "(?i)a.*?b.*?c.*?".
fn make_query_regex(query: &str) -> String {
    let mut parts = query
        .chars()
        .map(|ch| regex::quote(ch.to_string().as_slice()))
        .collect::<Vec<String>>();

    parts.insert(0, String::from_str("(?i)"));

    parts.connect(".*?")
}

#[cfg(test)]
mod test {

    use super::score;
    use super::core::str::StrExt;
    use std::num::ToPrimitive;

    #[test]
    fn scores_zero_when_the_choice_is_empty() {
        assert_eq!(score("", ""), 0f64);
    }

    #[test]
    fn scores_one_when_the_query_is_empty() {
        assert_eq!(score("a", ""), 1f64);
    }

    #[test]
    fn scores_zero_when_the_query_is_longer_than_the_choice() {
        assert_eq!(score("short", "longer"), 0f64);
    }

    #[test]
    fn scores_zero_when_only_a_prefix_of_the_query_matches() {
        assert_eq!(score("ab", "ac"), 0f64);
    }

    #[test]
    fn scores_greater_than_zero_when_it_matches() {
        assert!(score("a", "a") > 0f64);
        assert!(score("ab", "a") > 0f64);
        assert!(score("ba", "a") > 0f64);
        assert!(score("bab", "a") > 0f64);
        assert!(score("babababab", "aaaa") > 0f64);
    }

    #[test]
    fn scores_one_normalized_to_length_when_the_query_equals_the_choice() {
        assert_eq!(score("a", "a"), 1.0f64);
        assert_eq!(score("ab", "ab"), 0.5f64);
        assert_eq!(score("a long string", "a long string"),
            1.0f64 / ("a long string".char_len().to_f64().unwrap()));

        assert_eq!(score("spec/search_spec.rb", "sear"),
            1.0f64 / ("spec/search_spec.rb".char_len().to_f64().unwrap()));
    }

    // Character matching

    #[test]
    fn it_matches_punctuation() {
        assert!(score("/! symbols $^", "/!$^") > 0f64);
    }

    #[test]
    fn it_is_case_insensitive() {
        assert_eq!(score("a", "A"), 1f64);
        assert_eq!(score("A", "a"), 1f64);
    }

    #[test]
    fn it_doesnt_match_when_the_same_letter_is_repeated_in_the_choice() {
        assert_eq!(score("a", "aa"), 0f64);
    }

    #[test]
    fn it_scores_higher_for_better_matches() {
        assert!(score("selecta.gemspec", "asp")
                > score("algorithm4_spec.rb", "asp"));
        assert!(score("README.md", "em")
                > score("benchmark.rb", "em"));
        assert!(score("search.rb", "sear")
                > score("spec/search_spec.rb", "sear"));
    }
}


