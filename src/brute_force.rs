use std::collections::HashMap;
use crate::brute_force::statistical_analysis::{ count_ngrams, read_probabilities};
use crate::ceasar;

mod statistical_analysis;

const PROBABILITY_FILE: &str = "src/resource/reference/expected-frequencies";


fn count_chi_for_encrypted_text(encrypted: &str) -> HashMap<String, f64> {
    let ngrams_encrypted_text = count_ngrams(encrypted, 1);
    let ngrams_quantity = ngrams_encrypted_text.len() as f64;
    let mut encrypted_text_ngrams_frequencies = HashMap::new();
    for (key, value) in ngrams_encrypted_text.iter() {
        let single_frequency = *value as f64 / ngrams_quantity;
        encrypted_text_ngrams_frequencies.insert(key.clone(), single_frequency);
    }
    encrypted_text_ngrams_frequencies
}

fn calculate_chi_squared(observed: &HashMap<String, f64>, expected: &HashMap<String, f64>) -> f64 {
    let mut chi_squared = 0.0;
    for (ngram, &obs_freq) in observed.iter() {
        if let Some(&exp_freq) = expected.get(ngram) {
            let difference = obs_freq - exp_freq;
            chi_squared += difference.powi(2) / exp_freq;
        }
    }
    chi_squared
}


pub fn brute_force(ciphertext: &str) -> (String, i32, f64) {
    let expected_frequencies = read_probabilities(PROBABILITY_FILE).unwrap();
    let mut best_match = (String::new(), 0, f64::MAX);

    for key in 1..=25 {
        let decrypted_text = ceasar::cipher(ciphertext, -key);
        let observed_frequencies_borrowed = count_chi_for_encrypted_text(&decrypted_text);
        let observed_frequencies: HashMap<String, f64> = observed_frequencies_borrowed
            .iter()
            .map(|(k, &v)| (k.to_string(), v))
            .collect();
        let chi = calculate_chi_squared(&observed_frequencies, &expected_frequencies);
        if chi < best_match.2 {
            best_match = (decrypted_text, key, chi);
        }
    }
    best_match
}
