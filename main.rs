use std::{fs::File, io::Read, collections::HashMap};

const CENSOR_TEXT : &'static str = "****";

fn main() {
    let mut blacklist_file = match File::open("blacklist.txt") {
        Ok(file) => file,
        Err(..) => {
            File::create("blacklist.txt")
                .expect("Could not create nor find blacklist.txt")
        }
    };
    let mut blacklisted_word_string = String::new();
    blacklist_file.read_to_string(&mut blacklisted_word_string).expect("There was an error reading the blacklist");

    let mut base_filter = Filter::new();
    Parser::compile_filter(&mut base_filter, &blacklisted_word_string);

    println!("{}", base_filter.filter_out(&"Never gonna give you up".to_string()));
    println!("{}", base_filter.filter_out(&"Never gonna let you down".to_string()));
    println!("{}", base_filter.filter_out(&"Never gonna run around and desert you".to_string()));
    println!("{}", base_filter.filter_out(&"Never gonna make you cry".to_string()));
    println!("{}", base_filter.filter_out(&"Never gonna say goodbye".to_string()));
    println!("{}", base_filter.filter_out(&"Never gonna tell a lie and hurt you".to_string()));


    print_in_hierarchy(&base_filter, 0);
}

#[derive(Debug)]
struct Filter {
    map: HashMap<char, Filter>,
}

impl Filter {
    fn new() -> Self { 
        Self { map: HashMap::new() }
    }
    
    fn add_word(&mut self, word: &str) {
        let mut current_filter = self;
        for c in word.chars() {
            current_filter = current_filter.map.entry(c).or_insert(Filter::new());
        }
    }

    fn filter_out(&self, message: &String) -> String {
        let mut new_message = String::new();

        for s in Parser::string_to_words(message, " ") {
            new_message += if Filter::is_blacklisted(self, &s, 0) {
                CENSOR_TEXT
            } else {
                &s
            };
            new_message += " ";
        }

        new_message
    }

    fn is_blacklisted(filter: &Filter, word: &String, current_location: usize) -> bool {
        let converted_word = word.to_lowercase();
        let char = match converted_word.chars().nth(current_location) {
            Some(c) => c,
            None => return true
        };
        if filter.map.contains_key(&char) {
            return Filter::is_blacklisted(filter.map.get(&char).unwrap(), &converted_word, current_location + 1);
        }
        false

    }
}

struct Parser;

impl Parser {
    fn string_to_words(string: &String, separator: &str) -> Vec<String> {
        let mut v  = Vec::new();
        for s in string.split(separator) {
            v.push(s.to_string())
        }
        v
    }
    fn compile_filter(filter: &mut Filter, list: &String) {
        for s in Parser::string_to_words(list, "\n") {
            filter.add_word(&s.to_lowercase())
        }
    }
}

fn print_in_hierarchy(filter: &Filter, indent: usize) {
    for (c, f) in filter.map.iter() {
        for i in 0..indent {
            if indent-1 != i {
                print!("-");
            } else {
                print!(">");
            }
        }
        println!("{:?}", c);
        print_in_hierarchy(f, indent + 1);
    }
}
