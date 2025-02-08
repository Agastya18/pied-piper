use std::collections::{BinaryHeap, HashMap};

// Huffman Tree Node
#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub freq: usize,
    pub character: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Build Huffman Tree
pub fn build_huffman_tree(freq_map: &HashMap<char, usize>) -> Option<Box<Node>> {
    let mut heap = BinaryHeap::new();

    for (&char, &freq) in freq_map {
        heap.push(Node {
            freq,
            character: Some(char),
            left: None,
            right: None,
        });
    }

    while heap.len() > 1 {
        let left = Box::new(heap.pop().unwrap());
        let right = Box::new(heap.pop().unwrap());

        let parent = Node {
            freq: left.freq + right.freq,
            character: None,
            left: Some(left),
            right: Some(right),
        };

        heap.push(parent);
    }

    heap.pop().map(Box::new)
}

// Generate Huffman codes recursively
pub fn generate_codes(node: &Node, prefix: String, codes: &mut HashMap<char, String>) {
    if let Some(c) = node.character {
        codes.insert(c, prefix);
    } else {
        if let Some(ref left) = node.left {
            generate_codes(left, format!("{}0", prefix), codes);
        }
        if let Some(ref right) = node.right {
            generate_codes(right, format!("{}1", prefix), codes);
        }
    }
}

// Encode text
pub fn encode(text: &str, codes: &HashMap<char, String>) -> String {
    text.chars()
        .map(|c| codes.get(&c).unwrap().clone())
        .collect::<Vec<String>>()
        .join("")
}

// Decode text
pub fn decode(encoded_text: &str, root: &Node) -> String {
    let mut decoded_text = String::new();
    let mut current = root;

    for bit in encoded_text.chars() {
        current = if bit == '0' {
            current.left.as_ref().unwrap()
        } else {
            current.right.as_ref().unwrap()
        };

        if let Some(c) = current.character {
            decoded_text.push(c);
            current = root;
        }
    }

    decoded_text
}
