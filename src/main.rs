use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Write},
    time::Instant,
};

use anyhow::{Context, Result};
use itertools::Itertools;

const WORDS_FILE: &str = "clean_words_alpha_revb.txt";
// we only consider subsequences that are at most MAX_EDIT_DISTANCE away from the input word
// and at most MAX_EDIT_DISTANCE away from a correct spelling
const MAX_EDIT_DISTANCE: usize = 2;
const UPDATE_INTERVAL: usize = 10000;

fn main() -> Result<()> {
    let words_file = File::open(WORDS_FILE).context("could not open words file")?;
    let words_reader = BufReader::new(words_file);

    // maps subsequences to (a map of distances to correct spellings in the dictionary)
    let mut dictionary_subsequences = HashMap::new();

    // get words: trim whitespace, lowercase, remove empty lines
    let words = words_reader
        .lines()
        .map(|line_result| line_result.map(|line| line.trim().to_lowercase()))
        .filter(|line_result| line_result.as_ref().map_or(true, |line| !line.is_empty()))
        .collect::<Result<Vec<_>, _>>()
        .context("could not read from file")?;

    let processing_start = Instant::now();

    // preprocess dictionary words by calculating their subsequences and storing the
    // subsequences along with distances to the original word
    //
    // we want to keep all subsequences and not just the closest ones because otherwise
    // we might miss valid corrections
    // eg, consider input "tubr", dictionary has "tube" and "tub"
    // tub -> tube = 1
    // tub -> tub = 0 (tub is already a valid word)
    // if we only kept the subsequences closest to correct words then we would only keep tub
    // and miss tube as a correction for tubr
    for distance in 0..=MAX_EDIT_DISTANCE {
        println!("\nCalculating subsequences with distance {distance}");
        for (i, word) in words.iter().enumerate() {
            if i % UPDATE_INTERVAL == 0 {
                println!("Processing word {i}: {word}");
            }

            // creating subsequences from this word at this distance will yield empty strings
            if word.len() as i32 - distance as i32 <= 0 {
                continue;
            }

            for subsequence in subsequences_from_n_deletions(word, distance) {
                dictionary_subsequences
                    .entry(subsequence.clone())
                    .or_insert_with(|| HashMap::with_capacity(1))
                    .entry(distance)
                    .or_insert_with(|| Vec::with_capacity(1))
                    .push(word.clone());
            }
        }
    }

    let processing_time_seconds = processing_start.elapsed().as_millis() as f64 / 1000_f64;
    println!("\nFinished processing dictionary in {processing_time_seconds:.3}s");

    loop {
        print!("\n> Enter a word, can be misspelled: ");
        io::stdout().flush().context("could not flush stdout")?;
        let mut input_word = String::new();
        io::stdin()
            .read_line(&mut input_word)
            .context("could not read from stdin")?;
        let input_word = input_word.trim().to_lowercase().clone();

        // maps (distances from input word to correct spellings) to correct spellings
        let mut results = HashMap::new();
        for dist_input_to_subseq in 0..=MAX_EDIT_DISTANCE {
            // creating subsequences from this word at this distance will yield empty strings
            if input_word.len() as i32 - dist_input_to_subseq as i32 <= 0 {
                continue;
            }

            // find subsequences of input, and update results with dictionary words that share a subsequence
            // with the input subsequence
            for input_subsequence in
                subsequences_from_n_deletions(&input_word, dist_input_to_subseq)
            {
                check_for_correct_spellings(
                    &mut results,
                    &dictionary_subsequences,
                    &input_subsequence,
                    dist_input_to_subseq,
                );
            }
        }
        if let Some(min_distance) = results.keys().min() {
            // sort by length then alphabetically
            let mut correct_spellings: Vec<&String> =
                results.get(min_distance).unwrap().iter().copied().collect();
            correct_spellings.sort_by(|a, b| match a.len().cmp(&b.len()) {
                Ordering::Equal => a.cmp(b),
                x => x,
            });

            println!("\nFound correct spellings with distance {min_distance}:");
            for correct_spelling in correct_spellings {
                println!("{correct_spelling}");
            }
        } else {
            println!("Did not find any corrections for that word");
        }
    }
}

/// Updates the map `results` from (distance from input word to possible correct spellings) to correct spellings
/// by checking if the `input_subsequence` (a subsequence of the input word) is also a subsequence of any dictionary words.
/// If it is, then those dictionary words are stored in results with the key being
/// max(`dist_input_to_subseq`: distance from the input word to `input_subsequence`, distance from `input_subsequence` to correct spellings)
fn check_for_correct_spellings<'a>(
    results: &mut HashMap<usize, HashSet<&'a String>>,
    dictionary_subsequences: &'a HashMap<String, HashMap<usize, Vec<String>>>,
    input_subsequence: &String,
    dist_input_to_subseq: usize,
) {
    if let Some(subseq_dist_to_correct_spelling) = dictionary_subsequences.get(input_subsequence) {
        for (dist_subseq_to_correction, correct_spellings) in subseq_dist_to_correct_spelling {
            results
                // we use the max of distance from input to subsequence and distance from subsequence to correct spelling
                // so that we don't favor the subsequence when it is itself a correct spelling
                // eg, consider input "tubr", dictionary has "tube" and "tub"
                // tubr -> tub = 1
                // tub -> tube = 1
                // since we're using the max, tubr is 1 away from both tube and tub, but if we were using a sum of distances,
                // for example, tub would be 1 away while tube would be 1 + 1 = 2 away
                .entry(dist_input_to_subseq.max(*dist_subseq_to_correction))
                .or_insert_with(|| HashSet::with_capacity(1))
                .extend(correct_spellings);
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

    subsequences
}
