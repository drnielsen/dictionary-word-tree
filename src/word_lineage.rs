use std::fmt;
use std::iter::Iterator;
use std::cmp::Ordering;
use std::cmp::min;

#[derive(Eq)]
pub struct Lineage {
    lineage: Vec<String>,
}

impl Lineage {
    pub fn new() -> WordLineageBuilder {
        WordLineageBuilder{ word_lineage: Lineage { lineage: Vec::new() } }
    }

    /// Get the most recent word from this lineage
    pub fn get_latest_word(&self) -> &str {
        &self.lineage[self.lineage.len()-1]
    }

    /// Get the original word from this lineage
    pub fn get_original_word(&self) -> &str {
        &self.lineage[0]
    }

    pub fn get_dot_file_instructions(&self) -> Vec<String> {
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

impl Ord for Lineage {
    fn cmp(&self, other: &Lineage) -> Ordering {
        for x in 0..min(self.lineage.len(), other.lineage.len()) {
            if self.lineage[x] != other.lineage[x] {
                return self.lineage[x].cmp(&other.lineage[x])
            }
        }
        return self.lineage.len().cmp(&other.lineage.len())
    }
}

impl PartialOrd for Lineage {
    fn partial_cmp(&self, other: &Lineage) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Lineage {
    fn eq(&self, other: &Lineage) -> bool {
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

impl Clone for Lineage {
    fn clone(&self) -> Lineage {
        Lineage { lineage: self.lineage.clone() }
    }
}

impl fmt::Display for Lineage {
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

pub struct WordLineageBuilder {
    word_lineage: Lineage,
}

impl WordLineageBuilder {
    pub fn extend_lineage(mut self, previous_lineage: &Lineage, next_word: &str) -> Lineage {
        self.word_lineage.lineage.append(&mut previous_lineage.lineage.clone());
        self.word_lineage.lineage.push(next_word.to_string());
        self.word_lineage
    }

    pub fn init(mut self, initial_word: &str) -> Lineage {
        self.word_lineage.lineage.push(initial_word.to_string());
        self.word_lineage
    }
}
