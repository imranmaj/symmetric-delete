use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use anyhow::{Context, Result};
use itertools::Itertools;

const WORDS_FILE: &str = "words.txt";
const MAX_EDIT_DISTANCE: usize = 2;
const UPDATE_INTERVAL: usize = 10000;

fn main() -> Result<()> {
    let words_file = File::open(WORDS_FILE).context("could not open words file")?;
    let words_reader = BufReader::new(words_file);

    // maps deletions to (a map of distances to correct spellings in the dictionary)
    let mut corrections = HashMap::new();

    // get words: trim whitespace, remove empty lines
    let words = words_reader
        .lines()
        .map(|line_result| line_result.map(|line| line.trim().to_lowercase().to_owned()))
        .filter(|line_result| line_result.as_ref().map_or(true, |line| !line.is_empty()))
        .collect::<Result<Vec<_>, _>>()
        .context("could not read from file")?;

    for distance in 0..=MAX_EDIT_DISTANCE {
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
                // if the current distance is the closest we have then this word is the closest
                // correct spelling for this subsequence
                corrections
                    .entry(subsequence.clone())
                    .or_insert_with(|| HashMap::with_capacity(1))
                    .entry(distance)
                    .or_insert_with(|| Vec::with_capacity(1))
                    .push(word.clone());
            }
        }
    }

    loop {
        print!("\n> Enter a word, can be misspelled: ");
        io::stdout().flush().context("could not flush stdout")?;
        let mut input_word = String::new();
        io::stdin()
            .read_line(&mut input_word)
            .context("could not read from stdin")?;
        let input_word = input_word.trim().to_lowercase().to_owned();

        // maps distances from input word to correct spellings
        let mut results = HashMap::new();
        for input_to_subseq_dist in 0..=MAX_EDIT_DISTANCE {
            // creating subsequences from this word at this distance will yield empty strings
            if input_word.len() - input_to_subseq_dist <= 0 {
                continue;
            }

            // find subsequences of input, and check those against the subsequences in corrections map
            for subsequence in subsequences_from_n_deletions(&input_word, input_to_subseq_dist) {
                if let Some(subseq_dist_to_correct_spelling) = corrections.get(&subsequence) {
                    for subseq_to_correct_dist in 0..=MAX_EDIT_DISTANCE {
                        if let Some(correct_spellings) =
                            subseq_dist_to_correct_spelling.get(&subseq_to_correct_dist)
                        {
                            results
                                // we use the max of distance from input to subsequence and distance from subsequence to correct spelling
                                // so that we don't favor the subsequence when it is itself a correct spelling
                                // eg, consider input "tubr", dictionary has "tube" and "tub"
                                // tubr -> tub = 1
                                // tub -> tube = 1
                                // since we're using the max, tubr is 1 away from both tube and tub, but if we were using a sum of distances,
                                // for example, tub would be 1 away while tube would be 1 + 1 = 2 away
                                .entry(input_to_subseq_dist.max(subseq_to_correct_dist))
                                .or_insert_with(|| HashSet::with_capacity(1))
                                .extend(correct_spellings);
                        }
                    }
                }
            }
        }
        if let Some(min_distance) = results.keys().min() {
            println!("\nFound correct spellings with distance {min_distance}:");
            for correct_spelling in &results[min_distance] {
                println!("{correct_spelling}");
            }
        } else {
            println!("Did not find any corrections for that word")
        }
    }
}

/// Returns all possible subsequences that can be created by deleting n characters from s
fn subsequences_from_n_deletions(s: &str, n: usize) -> Vec<String> {
    if n == 0 {
        return vec![s.to_owned()];
    }

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
