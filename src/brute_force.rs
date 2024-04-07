use std::collections::HashMap;
use std::io;
use crate::brute_force::statistical_analysis::{calculate_and_save_ngram_probability, calculate_t, count_ngrams, read_probabilities, save_ngram_counts, sum_values_in_file};

mod statistical_analysis;
const BASE_PATH: &str = "src/resource/";
const FILE_PATH: &str = "grams.txt";

fn run_stats(text: &str) -> io::Result<()> {
    for n in 1..4
    {
        let counts = count_ngrams(text, n);
        let file = BASE_PATH.to_owned() + &*n.to_string() + FILE_PATH;
        save_ngram_counts(&*file, &counts)?;
        let sum = sum_values_in_file(&file).unwrap();
        let probability_file = file + "probabilities";
        calculate_and_save_ngram_probability(&file, &*probability_file)?;
        let probabilities = read_probabilities(&probability_file).unwrap();
        let total_ngrams = counts.iter().map(|(_, count)| *count).sum::<u32>();

        let t_value = calculate_t(&counts.into_iter().collect::<HashMap<_, _>>(), total_ngrams, &probabilities);
    }

    Ok(())
}

