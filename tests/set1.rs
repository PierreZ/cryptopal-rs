extern crate cryptopals;
use cryptopals::xor;
use cryptopals::scoring;

extern crate rustc_serialize;
use rustc_serialize::hex::FromHex;
use rustc_serialize::hex::ToHex;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;

use std::collections::HashMap;

#[test]
// https://cryptopals.com/sets/1/challenges/1
// The string:
// 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
// Should produce:
// SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
// So go ahead and make that happen. You'll need to use this code for the rest of the exercises.
fn challenge1() {

    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = hex.from_hex().unwrap();

    // base64::STANDARD is the configuration for RFC 4648 standard base64 encoding
    let result = bytes.to_base64(base64::STANDARD);

    let should_be = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(result, should_be);
}

#[test]
// https://cryptopals.com/sets/1/challenges/2
// Write a function that takes two equal-length buffers and produces their XOR combination.
// If your function works properly, then when you feed it the string:
// 1c0111001f010100061a024b53535009181c
// ... after hex decoding, and when XOR'd against:
// 686974207468652062756c6c277320657965
// ... should produce:
//746865206b696420646f6e277420706c6179
fn challenge2() {
    let text = "1c0111001f010100061a024b53535009181c";
    let mut vect_bytes_text= text.from_hex().unwrap();

    let key = "686974207468652062756c6c277320657965";
    let vect_bytes_key = key.from_hex().unwrap();

    cryptopals::xor::apply_xor(&mut vect_bytes_text,&vect_bytes_key);

    let should_be = "746865206b696420646f6e277420706c6179";
    assert_eq!(vect_bytes_text.to_hex(), should_be);
}


#[test]
// https://cryptopals.com/sets/1/challenges/3
// The hex encoded string:
// 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
// ... has been XOR'd against a single character. Find the key, decrypt the message.
// You can do this by hand. But don't: write code to do it for you.
// How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric.
// Evaluate each output and choose the one with the best score.
fn challenge3() {
    // Let's create a Vec holding some english sentences
    let training = String::from("The hex encoded string has been XOR'd against a single character. Find the key, decrypt the message. You can do this by hand. But don't: write code to do it for you. How? Devise some method for scoring a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.");

    let mut training_result: HashMap<char, f32>  = HashMap::new();

    cryptopals::scoring::score_plaintext(&training.into_bytes(), &mut training_result);

    let text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let mut vect_bytes_text = text.from_hex().unwrap();

    let begin = 'A' as u8;
    let end = 'Z' as u8;

    let mut best_answer = String::new();
    let mut best_score = 500.0f32;
    let mut score;

    // Bruteforce XOR, storing results in result_bruteforce
    for letter in begin..end {
        let mut try = vect_bytes_text.clone();
        let mut key = Vec::new();
        key.push(letter);

        cryptopals::xor::apply_xor(&mut try, &key);

        score = cryptopals::scoring::compare_with_training(&try, &training_result);

        if score < best_score {
            best_answer = String::from_utf8(try).unwrap();
            best_score = score;
            println!("new best score: {}", best_answer);
        }
    }
    assert_eq!("Cooking MC's like a pound of bacon", best_answer);
}

#[test]
// One of the 60-character strings in this file has been encrypted by single-character XOR.
// Find it.
// (Your code from #3 should help.)
fn challenge4() {
    // assert_eq!("Cooking MC's like a pound of bacon", "bacon");
}
