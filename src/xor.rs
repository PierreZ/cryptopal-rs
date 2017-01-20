// Helpers for cryptopals
extern crate rustc_serialize;

use std::collections::HashMap;


use std::str;


// apply_xor is taking two vect of bytes, and applying XOR on it.
// working on single or multiple char key
pub fn apply_xor(text: &mut Vec<u8>, key: & Vec<u8>) {

    match key.len() {
        1 => {
            for x in text.iter_mut() {
                // The BitXor trait is used to specify the functionality of ^. Example:
                // println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
                *x ^= key[0];
            }
        }
        _ => {
            // zip is an iterator that iterates two other iterators simultaneously.
            for (x, y) in text.iter_mut().zip(key) {
                // The BitXor trait is used to specify the functionality of ^. Example:
                // println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
                *x ^= *y;
            }
        }
    }
}
