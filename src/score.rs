
extern crate regex;
extern crate core;

use std::str;
use std::slice::SliceConcatExt;
use self::core::str::StrExt;
use self::regex::Regex;

pub fn score(choice: &str, query: &str) -> u32 {
    if choice == "" || query.char_len() > choice.char_len() {
        0
    } else if query == "" {
        1
    } else {
        compute_match_length(choice, query)
    }
}

// Find the length of the shortest substring matching the given characters.
fn compute_match_length(string: &str, query: &str) -> u32 {
    let re_string = make_query_regex(query);
    let re = match Regex::new(re_string.as_slice()) {
        Ok(re)   => re,
        Err(err) => panic!("{}", err.msg),
    };

    1
}


// Creates a regex for performing a non-greedy match.
// Transforms a user-supplied query such as "abc" into "a.*?b.*?c.*?".
fn make_query_regex(query: &str) -> String {
    let s = String::from_str(query);
    let v = vec!(s);
    let c = v.connect(",");
    return String::from_str(c.as_slice());
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
        assert_eq!(super::score("ab", "ac"), 0);
    }

    #[test]
    fn scores_greater_than_zero_when_it_matches() {
        //assert!(super::score("a", "a") > 0);


        //expect(score("a", "a")).to be > 0
        //expect(score("ab", "a")).to be > 0
        //expect(score("ba", "a")).to be > 0
        //expect(score("bab", "a")).to be > 0
        //expect(score("babababab", "aaaa")).to be > 0
    }
}


