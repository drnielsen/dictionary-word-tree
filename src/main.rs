use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;
use std::io::prelude::*;
use std::fmt;
use std::iter::Iterator;
use std::cmp::Ordering;
use std::cmp::min;

fn main() {
    println!("{}", "Opening file");
    let f = File::open("/Users/drnielsen/Development/misc/english-words/words.txt").unwrap();
    let buffered_reader = BufReader::new(f);

    let dictionary: HashSet<String> = buffered_reader.lines().map(|x| x.unwrap()).collect();

    let count = dictionary.iter().count();
    println!("Found {} word total.", count);

    let nine_letter_words: Vec<&String> = dictionary.iter().filter(|x| x.len() == 9).collect();
    println!("Found {} 9 letter words.", nine_letter_words.iter().count());

    let results = build_word_tree(&dictionary, &nine_letter_words, 8);

    println!("Final count: {}", results.iter().count());
    println!("Filtered words:");
    results.iter().for_each(|word_lineage| println!("{}", word_lineage));
}

fn build_word_tree(dictionary: &HashSet<String>, lineage_list: &Vec<&String>, iterations: i32) -> Vec<WordLineage> {
    let mut pipeline: Box<Iterator<Item=WordLineage>> = Box::new(lineage_list.iter().map(|x| WordLineage::new().init(x)));

    for _ in 0..iterations {
        pipeline = Box::new(pipeline.flat_map(|word| create_word_permutations(&word))
                .filter(|word_lineage| is_a_word(word_lineage.get_word(), &dictionary)));
    }
    let mut result: Vec<WordLineage> = pipeline.collect();
    result.sort_unstable();
    result.dedup();
    result
}

fn create_word_permutations(lineage_to_permutate: &WordLineage) -> Vec<WordLineage> {
    let mut permutations: Vec<String> = Vec::new();
    let str_to_mutate = lineage_to_permutate.get_word();

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
    fn get_word(&self) -> &str {
        &self.lineage[self.lineage.len()-1]
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
                output.push('\'');
                output.push_str(first_word);
                output.push('\'');
                it.for_each(|x| {
                    output.push_str(" -> ");
                    output.push('\'');
                    output.push_str(x);
                    output.push('\'');
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

