use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn count_ngrams(text: &str, n: u32) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    let chars = text.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_uppercase().next().unwrap())
        .collect::<Vec<_>>();
    for window in chars.windows(n as usize) {
        let ngram = window.iter().collect::<String>();
        let ngram_owned = ngram.to_string();
        *counts.entry(ngram_owned).or_insert(0) += 1;
    }
    counts
}

// pub fn save_ngram_counts(filename: &str, counts: &[(String, u32)]) -> io::Result<()> {
//     let mut file = File::create(filename)?;
//     for (ngram, count) in counts.iter() {
//         writeln!(file, "{} {}", ngram, count)?;
//     }
//     Ok(())
// }
// pub fn sum_values_in_file(filename: &str) -> io::Result<u32> {
//     let file = File::open(filename)?;
//     let reader = BufReader::new(file);
//
//     let mut sum = 0;
//     for line in reader.lines() {
//         let line = line?;
//         let parts: Vec<&str> = line.split_whitespace().collect();
//         if parts.len() >= 2 {
//             if let Ok(value) = parts[1].parse::<u32>() {
//                 sum += value;
// //             }
// //         }
// //     }
//
//     Ok(sum)
// }
// pub fn calculate_and_save_ngram_probability(input_file: &str, output_file: &str) -> io::Result<()> {
//     let total_count = sum_values_in_file(input_file)?;
//     let inputfile = File::open(input_file)?;
//     let reader = BufReader::new(inputfile);
//     let mut outputfile = File::create(output_file)?;
//
//     for line in reader.lines() {
//         let line = line?;
//         let parts: Vec<&str> = line.split_whitespace().collect();
//
//         if parts.len()>=2 {
//             if let Ok(value) = parts[1].parse::<u32>() {
//                 let probability = value as f64 / total_count as f64;
//                 writeln!(outputfile, "{} {:.10}", parts[0], probability)?;
//             }
//         }
//     }
//
//     Ok(())
// }
// pub fn calculate_t(n_grams: &HashMap<String, u32>, total_ngrams: u32, probabilities: &HashMap<String, f64>) -> f64 {
//     let mut t = 0.0;
//     for (n_gram, &count) in n_grams {
//         if let Some(&probability) = probabilities.get(n_gram) {
//             let expected_count = total_ngrams as f64 * probability;
//             t += (count as f64 - expected_count).powi(2) / expected_count;
//         }
//     }
//
//     t
// }

pub fn read_probabilities(filename: &str) -> io::Result<HashMap<String, f64>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut probabilities = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() == 2 {
            if let Ok(prob) = parts[1].parse::<f64>() {
                probabilities.insert(parts[0].to_string(), prob);
            }
        }
    }

    Ok(probabilities)
}
