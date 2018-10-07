use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;
use std::io::prelude::*;
use std::fmt;
use std::iter::Iterator;
use std::cmp::Ordering;
use std::cmp::min;
use std::error::Error;
use std::path::Path;


fn main() {
    println!("{}", "Opening file");
    //let f = File::open("/Users/drnielsen/Development/misc/english-words/words_alpha.txt").unwrap();
    let f = File::open("/usr/share/dict/words").unwrap();
    let buffered_reader = BufReader::new(f);

    let dictionary: HashSet<String> = buffered_reader.lines()
                                                     .map(|x| x.unwrap())
                                                     .map(|x| x.to_lowercase())
                                                     .filter(|x| {
                                                        if x.len() > 1 {
                                                            return true
                                                        } else if x == "a" {
                                                            return true
                                                        } else if x == "o" {
                                                            return true
                                                        } else if x == "i" {
                                                            return true
                                                        } else {
                                                            return false
                                                        }
                                                     })
                                                     .collect();

    let count = dictionary.iter().count();
    println!("Found {} word total.", count);

    let nine_letter_words: Vec<&String> = dictionary.iter().filter(|x| x.len() == 9).collect();
    println!("Found {} 9 letter words.", nine_letter_words.iter().count());

    let results = build_word_tree(&dictionary, &nine_letter_words, 8);

    println!("Filtered words:");
    results.iter().for_each(|word_lineage| println!("{}", word_lineage));

    println!("Words List:");
    let mut words: Vec<String> = results.iter().map(|x| x.get_original_word().to_string()).collect();
    words.sort_unstable();
    words.dedup();
    words.iter().for_each(|word| println!("{}", word));
    println!("Final count: {}", words.iter().count());

    let mut dotfile_instructions: Vec<String> = results.iter().flat_map(|x| x.get_dot_file_instructions()).collect();
    dotfile_instructions.sort_unstable();
    dotfile_instructions.dedup();
    //dotfile_instructions.iter().for_each(|word| println!("{}", word));

    let path = Path::new("word_graph.dot");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    match file.write_all("digraph G {\n".as_bytes()) {
        Err(why) => {
                panic!("couldn't write to {}: {}", display,
                                                why.description())
            },
            Ok(_) => (),
    }

    for instruction in dotfile_instructions {
        match file.write_all((instruction+"\n").as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display,
                                                why.description())
            },
            Ok(_) => (),
        }
    }

    match file.write_all("}\n".as_bytes()) {
        Err(why) => {
                panic!("couldn't write to {}: {}", display,
                                                why.description())
            },
            Ok(_) => (),
    }
}

fn build_word_tree(dictionary: &HashSet<String>, lineage_list: &Vec<&String>, iterations: i32) -> Vec<WordLineage> {
    let mut pipeline: Box<Iterator<Item=WordLineage>> = Box::new(lineage_list.iter().map(|x| WordLineage::new().init(x)));

    for _ in 0..iterations {
        pipeline = Box::new(pipeline.flat_map(|word| create_word_permutations(&word))
                .filter(|word_lineage| is_a_word(word_lineage.get_latest_word(), &dictionary)));
    }
    let mut result: Vec<WordLineage> = pipeline.collect();
    result.sort_unstable();
    result.dedup();
    result
}

fn create_word_permutations(lineage_to_permutate: &WordLineage) -> Vec<WordLineage> {
    let mut permutations: Vec<String> = Vec::new();
    let str_to_mutate = lineage_to_permutate.get_latest_word();

    for x in 0..str_to_mutate.len(){
        let mut substring = str_to_mutate[..x].to_string();
        substring.push_str(&str_to_mutate[x+1..]);
        permutations.push(substring);
    }

    permutations.into_iter().map(|word| {
        WordLineage::new().extend_lineage(lineage_to_permutate, &word)
    }).collect()
}

fn is_a_word(str_to_check: &str, list_of_words: &HashSet<String>) -> bool {
    list_of_words.contains(str_to_check)
}

#[derive(Eq)]
struct WordLineage {
    lineage: Vec<String>,
}

impl WordLineage {
    fn new() -> WordLineageBuilder {
        WordLineageBuilder{ word_lineage: WordLineage { lineage: Vec::new() } }
    }

    /// Get the most recent word from this lineage
    fn get_latest_word(&self) -> &str {
        &self.lineage[self.lineage.len()-1]
    }

    /// Get the original word from this lineage
    fn get_original_word(&self) -> &str {
        &self.lineage[0]
    }

    fn get_dot_file_instructions(&self) -> Vec<String> {
        let mut instructions = Vec::new();
        for x in 0..self.lineage.len()-1 {
            let mut instruct = self.lineage[x].to_string();
            instruct.push_str(" -> ");
            instruct.push_str(&self.lineage[x+1]);
            instructions.push(instruct);
        }
        instructions
    }
}

impl Ord for WordLineage {
    fn cmp(&self, other: &WordLineage) -> Ordering {
        for x in 0..min(self.lineage.len(), other.lineage.len()) {
            if self.lineage[x] != other.lineage[x] {
                return self.lineage[x].cmp(&other.lineage[x])
            }
        }
        return self.lineage.len().cmp(&other.lineage.len())
    }
}

impl PartialOrd for WordLineage {
    fn partial_cmp(&self, other: &WordLineage) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for WordLineage {
    fn eq(&self, other: &WordLineage) -> bool {
        if self.lineage.len() != other.lineage.len() {
            return false
        } else {
            for x in 0..self.lineage.len()-1 {
                if self.lineage[x] != other.lineage[x] {
                    return false
                }
            }
            return true
        }
    }
}

impl Clone for WordLineage {
    fn clone(&self) -> WordLineage {
        WordLineage { lineage: self.lineage.clone() }
    }
}

impl fmt::Display for WordLineage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut it = self.lineage.iter();
        let mut output = String::new();
        
        match it.next() {
            None => output.push_str("Lineage is empty"),
            Some(first_word) => {
                output.push_str(first_word);
                it.for_each(|x| {
                    output.push_str(" -> ");
                    output.push_str(x);
                });
            },
        };
        write!(f, "{}", output)
    }
}

struct WordLineageBuilder {
    word_lineage: WordLineage,
}

impl WordLineageBuilder {
    fn extend_lineage(mut self, previous_lineage: &WordLineage, next_word: &str) -> WordLineage {
        self.word_lineage.lineage.append(&mut previous_lineage.lineage.clone());
        self.word_lineage.lineage.push(next_word.to_string());
        self.word_lineage
    }

    fn init(mut self, initial_word: &str) -> WordLineage {
        self.word_lineage.lineage.push(initial_word.to_string());
        self.word_lineage
    }
}

