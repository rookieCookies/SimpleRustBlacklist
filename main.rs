use std::{fs::File, io::Read, str::Chars};
use rustc_hash::FxHashMap;

const CENSOR_TEXT : &str = "****";

fn main() {
    let filter : Filter;

    {
        // Creating the list of words
        let mut blacklist_file = match File::open("blacklist.txt") {
            Ok(file) => file,
            Err(..) => panic!("Could not open blacklist file"),
        };
        let mut blacklisted_word_string = String::new();
        blacklist_file.read_to_string(&mut blacklisted_word_string).expect("There was an error reading the blacklist");
        filter = Filter::new(&blacklisted_word_string.as_str());
    }
    print_filter(&filter);
    println!("{}", filter.apply("Hellohell hell how are you"));
}

struct Filter {
    map: Node,
}

impl Filter {
    fn new(list: &str) -> Self { 
        let mut filter = Self { map: Node::new() };
        for word in list.split("\n") {
            filter.map.add_word(&word.to_lowercase())
        }
        filter
    }

    fn apply(&self, message: &str) -> String {
        let mut new_message = String::new();
        for original_word in message.split(" ") {
            new_message += if Filter::is_blacklisted(&self.map, &mut original_word.to_lowercase().chars()) {
                CENSOR_TEXT
            } else {
                &original_word
            };
            new_message += " ";
        }
        new_message
    }

    #[inline(always)]
    fn is_blacklisted(node: &Node, word_chars: &mut Chars) -> bool {
        Filter::blacklisted_word(node, word_chars)
    }

    fn blacklisted_word(node: &Node, word_chars: &mut Chars) -> bool {
        let char = match word_chars.next() {
            Some(c) => c,
            None => return node.is_word
        };
        if node.map.contains_key(&char) {
            return Filter::blacklisted_word(node.map.get(&char).unwrap(), word_chars);
        }
        false
    }
}

struct Node {
    map: FxHashMap<char, Node>,
    is_word: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            map: FxHashMap::default(),
            is_word: false,
        }
    }
    fn add_word(&mut self, word: &str) {
        let mut current_node = self;
        for char in word.chars() {
            current_node = current_node.map.entry(char).or_insert_with(Node::new);
        }
        current_node.is_word = true
    }
}

fn print_filter(filter: &Filter) {
    print_in_hierarchy(&filter.map, 0);
}

fn print_in_hierarchy(node: &Node, indent: usize) {
    for (section_char, section_node) in node.map.iter() {
        for i in 0..indent {
            if indent-1 != i {
                print!("-");
            } else {
                print!(">");
            }
        }
        println!("'{}'", section_char);
        print_in_hierarchy(section_node, indent + 1);
    }
}
