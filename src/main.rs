mod huffman;
mod utils;

use huffman::{build_huffman_tree, decode, encode, generate_codes};
use utils::{build_frequency_map, read_file, write_to_file};
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_file> <compressed_file>", args[0]);
        return;
    }

    let input_file = &args[1];
    let compressed_file = &args[2];

    // Read the input file
    let text = match read_file(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let freq_map = build_frequency_map(&text);
    let huffman_tree = build_huffman_tree(&freq_map).expect("Failed to build Huffman tree");

    let mut codes = HashMap::new();
    generate_codes(&huffman_tree, String::new(), &mut codes);

    let encoded_text = encode(&text, &codes);

    // Save compressed data
    if let Err(e) = write_to_file(compressed_file, &encoded_text) {
        eprintln!("Error writing compressed file: {}", e);
    } else {
        println!("File successfully compressed: {}", compressed_file);
    }
}
