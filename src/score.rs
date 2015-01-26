
extern crate regex;
extern crate core;

use std::str;
use std::slice::SliceConcatExt;
use self::core::str::StrExt;
use self::regex::Regex;

pub fn score(choice: &str, query: &str) -> usize {
    println!("HELLO!");
    if choice == "" || query.char_len() > choice.char_len() {
        println!("RETURNING 0");
        0us
    } else if query == "" {
        println!("RETURNING 1");
        1us
    } else {
        println!("COMPUTING MATCH LEN");
        compute_match_length(choice, query)
    }
}

// Find the length of the shortest substring matching the given characters.
fn compute_match_length(string: &str, query: &str) -> usize {
    println!("WTF 1");
    let re_string = make_query_regex(query);
    println!("REGEX: {}", re_string);
    let re = match Regex::new(re_string.as_slice()) {
        Ok(re)   => re,
        Err(err) => panic!("{}", err.msg),
    };

    if re.is_match(string) {
        let caps = re.captures(string).unwrap();
        match caps.at(0) {
            Some(s) => return s.char_len(),
            None    => { println!("DID NOT MATCH"); return 0us }
        };
    } else {
        println!("NOT A MATCH!"); 
        return 0us;
    }
}

// Creates a regex for performing a non-greedy fuzzy match.
// Turns "abc" into ".*a.*?b.*?c.*?".
fn make_query_regex(query: &str) -> String {
    return query
        .chars()
        .map(|ch| ch.to_string())
        .collect::<Vec<String>>()
        .connect(".*?");
}

#[cfg(test)]
mod test {

    #[test]
    fn scores_zero_when_the_choice_is_empty() {
        assert_eq!(super::score("", ""), 0);
    }

    #[test]
    fn scores_one_when_the_query_is_empty() {
        assert_eq!(super::score("a", ""), 1);
    }

    #[test]
    fn scores_zero_when_the_query_is_longer_than_the_choice() {
        assert_eq!(super::score("short", "longer"), 0);
    }

    #[test]
    fn scores_zero_when_only_a_prefix_of_the_query_matches() {
        assert_eq!(super::score("ab", "ac"), 0us);
    }

    #[test]
    fn scores_greater_than_zero_when_it_matches() {
        assert!(super::score("a", "a") > 0us);
        assert!(super::score("ab", "a") > 0us);
        assert!(super::score("ba", "a") > 0us);
        assert!(super::score("bab", "a") > 0us);
        println!("SCORE: {}", super::score("babababab", "aaaa"));
        assert!(super::score("babababab", "aaaa") > 0us);
    }
}


