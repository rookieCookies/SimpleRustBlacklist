use std::{fs::File, io::Read, collections::HashMap, str::Chars};

const CENSOR_TEXT : &str = "****";

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

    println!("{}", base_filter.filter_out("Never gonna give you up"));
    println!("{}", base_filter.filter_out("Never gonna let you down"));
    println!("{}", base_filter.filter_out("Never gonna run around and desert you"));
    println!("{}", base_filter.filter_out("Never gonna make you cry"));
    println!("{}", base_filter.filter_out("Never gonna say goodbye"));
    println!("{}", base_filter.filter_out("Never gonna tell a lie and hurt you"));


    print_in_hierarchy(&base_filter.trie, 0);
}

#[derive(Debug)]
struct Filter {
    trie: Node,
}

impl Filter {
    fn new() -> Self { 
        Self { trie: Node::new() }
    }

    fn filter_out(&self, message: &str) -> String {
        let mut new_message = String::new();

        for original_word in Parser::string_to_words(message, " ") {
            new_message += if Filter::is_blacklisted(&self.trie, &mut original_word.to_lowercase().chars()) {
                CENSOR_TEXT
            } else {
                &original_word
            };
            new_message += " ";
        }

        new_message
    }

    fn is_blacklisted(node: &Node, word_chars: &mut Chars) -> bool {
        let char = match word_chars.next() {
            Some(c) => c,
            None => return true
        };
        if node.0.contains_key(&char) {
            return Filter::is_blacklisted(node.0.get(&char).unwrap(), word_chars);
        }
        false
    }
}

#[derive(Debug)]
struct Node(HashMap<char, Node>);

impl Node {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn add_word(&mut self, word: &str) {
        let mut current_node = self;
        for char in word.chars() {
            current_node = current_node.0.entry(char).or_insert_with(Node::new);
        }
    }
}


struct Parser;

impl Parser {
    fn string_to_words(string: &str, separator: &str) -> Vec<String> {
        let mut vector  = Vec::new();
        for word in string.split(separator) {
            vector.push(word.to_string())
        }
        vector
    }
    fn compile_filter(filter: &mut Filter, list: &str) {
        for word in Parser::string_to_words(list, "\n") {
            filter.trie.add_word(&word.to_lowercase())
        }
    }
}

fn print_in_hierarchy(node: &Node, indent: usize) {
    for (section_char, section_node) in node.0.iter() {
        for i in 0..indent {
            if indent-1 != i {
                print!("-");
            } else {
                print!(">");
            }
        }
        println!("{:?}", section_char);
        print_in_hierarchy(section_node, indent + 1);
    }
}
