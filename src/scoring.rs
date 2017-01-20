// Helpers for cryptopals
use std::collections::HashMap;

pub fn compare_with_training(test_string: & Vec<u8>, training_result: & HashMap<char, f32>) -> f32 {

    // Let's score the sentence
    let mut char_scoring: HashMap<char, f32>  = HashMap::new();
    score_plaintext(&test_string,&mut char_scoring);
    return compare_hash_char_freq(& char_scoring, training_result);
}

pub fn compare_hash_char_freq(char_scoring: & HashMap<char, f32>, training_result: & HashMap<char, f32>) -> f32{

    let mut score = 0.0f32;

    for (key,value) in training_result {
        match char_scoring.get(&key) {
            Some(&number) => {
                let diff = (value - number).abs();
                score += diff;
            },
            _ =>(score += 10.0),
        }
    }

    return score;
}

// score_plaintext is used in challenge3. Be warned, code is dummy
pub fn score_plaintext(text: &Vec<u8>, letters_score: &mut HashMap<char, f32>) {

    let mut counter = 0.0f32;

    // Second loop is increment per char the HashMap.
    for character in text {
        match (*character as char).to_lowercase().next() {
            Some(_) => {
                if (*character as char).is_alphabetic() {
                    *letters_score.entry(*character as char).or_insert(0.0f32) += 1.0f32;
                    counter += 1.0f32;
                }
            },
            _ => (),
        }
    }

    // iterate over everything.
    for (_, score) in letters_score {
        *score /= counter;
    }
}
