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

    for (i, line_result) in words_reader.lines().enumerate() {
        let line = line_result.context("could not read line from file")?;
        let word = line.trim().to_owned();
        if word.is_empty() {
            continue;
        }

        if i % UPDATE_INTERVAL == 0 {
            println!("Processing word {i}: {word}");
        }

        // insert dictionary words with distance 0
        corrections
            .entry(word.clone())
            .and_modify(|v: &mut Vec<String>| v.clear())
            .or_insert_with(|| Vec::with_capacity(1))
            .push(word.clone());
        min_distance_to_correct.insert(word.clone(), 0);

        // edit distance should not be more than number of letters in word
        for distance in 1..=MAX_EDIT_DISTANCE.min(word.len() - 1) {
            for subsequence in subsequences_from_n_deletions(&word, distance) {
                // update min distance to a correct spelling for this subsequence
                let existing_distance = min_distance_to_correct
                    .entry(subsequence.clone())
                    .or_insert(distance);

                // if this subsequence is now the closest distance to a correct spelling
                // then update the existing distance and clear the existing correct spellings
                if distance < *existing_distance {
                    *existing_distance = distance;
                    corrections
                        .entry(subsequence.clone())
                        .and_modify(|v| v.clear());
                }

                // if this subsequence has the minimum distance then add this correct spelling
                // to the corrections for this subsequence
                if min_distance_to_correct[&subsequence] == distance {
                    corrections
                        .entry(subsequence)
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
