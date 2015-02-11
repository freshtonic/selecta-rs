extern crate core;

use std::collections::HashMap;

pub struct Configuration<'a> {
    visible_choices: usize,
    initial_search: &'a str,
    choices: Vec<&'a str>
}

impl<'a> Configuration<'a> {

    pub fn from_inputs<'b>(choices: Vec<&'b str>, options: HashMap<&'b str, &'b str>, screen_height: usize) -> Configuration<'b> {
        Configuration { visible_choices: 0us, initial_search: "", choices: choices }
    }

    pub fn default_options<'b>() -> HashMap<&'b str, &'b str> {
        HashMap::new()
    }

    pub fn choices(&self) -> Vec<&str> {
        self.choices.iter().map(|choice| choice.trim()).collect::<Vec<_>>()
    }
}


#[cfg(test)]
mod test {

    use super::Configuration;
    use std::old_io::File;

    // Choices

    #[test]
    fn it_removes_leading_and_trailing_whitespace() {
        let config = Configuration::from_inputs(
            vec![" a choice "],
            Configuration::default_options(),
            21us
        );
        assert_eq!(config.choices(), vec!["a choice"]);
    }

    #[test]
    fn it_silences_invalid_UTF_characters() {
        let path = Path::new("invalid_utf8.txt");
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.desc),
            Ok(file) => file
        };
        match file.read_to_string() {
            Err(why) => assert_eq!("invalid input", why.desc),
            Ok(string) => panic!("should not have been able to read invalid_utf8.txt successfully!") 
        }
    }
}
