
use core::str::StrExt;
use regex;

pub fn score(choice: &String, query: &String) -> f32 {
    if choice.len() == 0 {
        0.0
    } else if query.len() == 0 {
        1.0
    } else {
        if compute_match_length(choice, query) > 0 {
            1.0 / choice.len() as f32
        } else {
            0.0
        }
    }
}

// Find the length of the shortest substring matching the given characters.
fn compute_match_length(string: &String, query: &String) -> usize {
    let re_string = &make_query_regex(query);
    let re = match regex::Regex::new(re_string.as_slice()) {
        Ok(re)   => re,
        Err(err) => panic!("{}", err.msg),
    };
    match re.captures(string.as_slice()) {
        Some(caps) => caps.at(0).unwrap_or("").char_len(),
        None       => 0
    }
}

// Creates a regex for performing a case-insensitive non-greedy fuzzy match.
// Turns "abc" into "(?i)a.*?b.*?c.*?".
fn make_query_regex(query: &String) -> String {
    let mut parts = query
        .chars()
        .map(|ch| regex::quote(ch.to_string().as_slice()))
        .collect::<Vec<String>>();

    parts.insert(0, "(?i)".to_string());

    parts.connect(".*?")
}

#[cfg(test)]
mod test {

    use score::score;
    use core::str::StrExt;

    pub fn do_score(choice: &str, query: &str) -> f32 {
       let choice_string = choice.to_string();
       let query_string = query.to_string();
       score(&choice_string,  &query_string)
    }

    #[test]
    fn scores_zero_when_the_choice_is_empty() {
        assert_eq!(do_score("", ""), 0.0);
    }

    #[test]
    fn scores_one_when_the_query_is_empty() {
        assert_eq!(do_score("a", ""), 1.0);
    }

    #[test]
    fn scores_zero_when_the_query_is_longer_than_the_choice() {
        assert_eq!(do_score("short", "longer"), 0.0);
    }

    #[test]
    fn scores_zero_when_only_a_prefix_of_the_query_matches() {
        assert_eq!(do_score("ab", "ac"), 0.0);
    }

    #[test]
    fn scores_greater_than_zero_when_it_matches() {
        assert!(do_score("a", "a") > 0.0);
        assert!(do_score("ab", "a") > 0.0);
        assert!(do_score("ba", "a") > 0.0);
        assert!(do_score("bab", "a") > 0.0);
        assert!(do_score("babababab", "aaaa") > 0.0);
    }

    #[test]
    fn scores_one_normalized_to_length_when_the_query_equals_the_choice() {
        assert_eq!(do_score("a", "a"), 1.0);
        assert_eq!(do_score("ab", "ab"), 0.5f32);
        assert_eq!(do_score("a long string", "a long string"),
            1.0 / ("a long string".char_len() as f32));

        assert_eq!(do_score("spec/search_spec.rb", "sear"),
            1.0 / ("spec/search_spec.rb".char_len() as f32));
    }

    // Character matching

    #[test]
    fn it_matches_punctuation() {
        assert!(do_score("/! symbols $^", "/!$^") > 0.0);
    }

    #[test]
    fn it_is_case_insensitive() {
        assert_eq!(do_score("a", "A"), 1.0);
        assert_eq!(do_score("A", "a"), 1.0);
    }

    #[test]
    fn it_doesnt_match_when_the_same_letter_is_repeated_in_the_choice() {
        assert_eq!(do_score("a", "aa"), 0.0);
    }

    #[test]
    fn it_scores_higher_for_better_matches() {
        assert!(do_score("selecta.gemspec", "asp")
                > do_score("algorithm4_spec.rb", "asp"));
        assert!(do_score("README.md", "em")
                > do_score("benchmark.rb", "em"));
        assert!(do_score("search.rb", "sear")
                > do_score("spec/search_spec.rb", "sear"));
    }
}


