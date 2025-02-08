# **Huffman Compression in Rust**

This project implements **Huffman Encoding and Decoding** in Rust to **compress and decompress text files** efficiently.

## **🚀 Features**
- Reads a file and compresses its content using **Huffman encoding**.
- Generates an optimized binary tree for compression.
- Supports **file decompression** to restore the original content.
- Efficient **bitwise encoding** (can be improved using `bitvec`).
- Clean, modular structure with separate files for Huffman logic and utilities.

---

## **📂 Project Structure**
huffman_compression/ │── src/ │ ├── main.rs # Entry point │ ├── huffman.rs # Huffman encoding/decoding logic │ ├── utils.rs # File handling utilities │── Cargo.toml # Rust project dependencies │── input.txt # Sample file to compress │── README.md # Project documentation


---


