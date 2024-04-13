use std::collections::HashMap;
use crate::affine::decrypt;
use crate::brute_force::statistical_analysis::{ count_ngrams, read_probabilities};
use crate::caesar;

mod statistical_analysis;

const PROBABILITY_FILE: &str = "src/resource/reference/expected-frequencies";
const M: i32 = 26;

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


pub fn brute_force_caesar(ciphertext: &str) -> (String, i32, f64) {
    let expected_frequencies = read_probabilities(PROBABILITY_FILE).unwrap();
    let mut best_match = (String::new(), 0, f64::MAX);

    for key in 1..=25 {
        let decrypted_text = caesar::cipher(ciphertext, -key);
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

pub fn brute_force_affine(ciphertext: &str) -> (String, i32, i32) {
    let valid_a = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];
    let mut best_score = f64::MAX;
    let mut best_plaintext = String::new();
    let mut best_a = 0;
    let mut best_b = 0;
    let expected_frequencies = read_probabilities(PROBABILITY_FILE).unwrap();

    for &a in valid_a.iter() {
        for b in 0..M {
            if let Some(plaintext) = decrypt(ciphertext, a, b) {
                let observed_frequencies = count_chi_for_encrypted_text(&plaintext);
                let score = calculate_chi_squared(&observed_frequencies, &expected_frequencies);
                if score < best_score {
                    best_score = score;
                    best_plaintext = plaintext;
                    best_a = a;
                    best_b = b;
                }
            }
        }
    }

    (best_plaintext, best_a, best_b)
}