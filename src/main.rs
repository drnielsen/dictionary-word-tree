use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;
use std::io::prelude::*;
use std::iter::Iterator;
use std::error::Error;
use std::path::Path;

mod word_lineage;


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

fn build_word_tree(dictionary: &HashSet<String>, lineage_list: &Vec<&String>, iterations: i32) -> Vec<word_lineage::Lineage> {
    let mut pipeline: Box<Iterator<Item=word_lineage::Lineage>> = Box::new(lineage_list.iter().map(|x| word_lineage::Lineage::new().init(x)));

    for _ in 0..iterations {
        pipeline = Box::new(pipeline.flat_map(|word| create_word_permutations(&word))
                .filter(|word_lineage| is_a_word(word_lineage.get_latest_word(), &dictionary)));
    }
    let mut result: Vec<word_lineage::Lineage> = pipeline.collect();
    result.sort_unstable();
    result.dedup();
    result
}

fn create_word_permutations(lineage_to_permutate: &word_lineage::Lineage) -> Vec<word_lineage::Lineage> {
    let mut permutations: Vec<String> = Vec::new();
    let str_to_mutate = lineage_to_permutate.get_latest_word();

    for x in 0..str_to_mutate.len(){
        let mut substring = str_to_mutate[..x].to_string();
        substring.push_str(&str_to_mutate[x+1..]);
        permutations.push(substring);
    }

    permutations.into_iter().map(|word| {
        word_lineage::Lineage::new().extend_lineage(lineage_to_permutate, &word)
    }).collect()
}

fn is_a_word(str_to_check: &str, list_of_words: &HashSet<String>) -> bool {
    list_of_words.contains(str_to_check)
}
