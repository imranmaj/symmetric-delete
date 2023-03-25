use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};
use itertools::Itertools;

const WORDS_FILE: &str = "words.txt";
const MAX_EDIT_DISTANCE: usize = 2;
const UPDATE_INTERVAL: usize = 10000;

fn main() -> Result<()> {
    let words_file = File::open(WORDS_FILE).context("could not open words file")?;
    let words_reader = BufReader::new(words_file);

    // maps deletions to correct spellings in the dictionary
    let mut corrections = HashMap::new();
    // maps deletions to the minimum distance to a dictionary word
    let mut min_distance_to_correct = HashMap::new();

    // get words: trim whitespace, remove empty lines
    let words = words_reader
        .lines()
        .map(|line_result| line_result.map(|line| line.trim().to_owned()))
        .filter(|line_result| line_result.as_ref().map_or(true, |line| !line.is_empty()))
        .collect::<Result<Vec<_>, _>>()
        .context("could not read from file")?;

    // dictionary words have distance 0; they are corrected to themself
    for word in &words {
        corrections.insert(word.clone(), vec![word.clone()]);
        min_distance_to_correct.insert(word.clone(), 0);
    }

    for distance in 1..=MAX_EDIT_DISTANCE {
        println!("\nCalculating subsequences with distance {distance}");
        for (i, word) in words.iter().enumerate() {
            if i % UPDATE_INTERVAL == 0 {
                println!("Processing word {i}: {word}");
            }

            // creating subsequences from this word at this distance will yield empty strings
            if word.len() - distance <= 0 {
                continue;
            }

            for subsequence in subsequences_from_n_deletions(word, distance) {
                // set distance from generated subsequence to the word the subsequence came from;
                // if there is already an entry then the existing entry is closer
                // because we are processing distances only in increasing order
                let existing_distance = min_distance_to_correct
                    .entry(subsequence.clone())
                    .or_insert(distance);

                // if the current distance is the closest we have then this word is the closest
                // correct spelling for this subsequence
                if distance == *existing_distance {
                    corrections
                        .entry(subsequence.clone())
                        .or_insert_with(|| Vec::with_capacity(1))
                        .push(word.clone());
                }
            }
        }
    }

    Ok(())
}

fn subsequences_from_n_deletions(s: &str, n: usize) -> Vec<String> {
    let combinations = (0..s.len()).combinations(n);

    let mut subsequences = Vec::new();
    for indices in combinations {
        let new_word = s
            .chars()
            .enumerate()
            .filter(|(i, _)| !indices.contains(i))
            .map(|(_, c)| c)
            .collect();
        subsequences.push(new_word);
    }

    return subsequences;
}
