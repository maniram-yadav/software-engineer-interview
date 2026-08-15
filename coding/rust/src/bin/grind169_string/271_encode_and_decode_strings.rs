//! Grind 169 — LeetCode #271 Encode and Decode Strings (Medium, Premium)
//!
//! Design an algorithm to encode a list of strings into one string, and
//! decode it back into the original list of strings. Uses a
//! length-prefixed format ("<len>#<content>") so embedded delimiters or
//! special characters in the content never cause ambiguity.
//!
//! Example:
//!   Input: ["lint","code","love","you"]
//!   Output (encoded then decoded): ["lint","code","love","you"]

struct Codec;

impl Codec {
    fn encode(&self, strs: Vec<String>) -> String {
        let mut result = String::new();
        for s in &strs {
            result.push_str(&s.len().to_string());
            result.push('#');
            result.push_str(s);
        }
        result
    }

    fn decode(&self, s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let mut result = Vec::new();
        let mut i = 0;
        while i < bytes.len() {
            let mut j = i;
            while bytes[j] != b'#' {
                j += 1;
            }
            let len: usize = s[i..j].parse().unwrap();
            let start = j + 1;
            result.push(s[start..start + len].to_string());
            i = start + len;
        }
        result
    }
}

fn v(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

fn main() {
    let codec = Codec;
    let words = v(&["lint", "code", "love", "you"]);
    let encoded = codec.encode(words.clone());
    println!("{:?}", codec.decode(encoded));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_roundtrip() {
        let codec = Codec;
        let words = v(&["lint", "code", "love", "you"]);
        let encoded = codec.encode(words.clone());
        assert_eq!(codec.decode(encoded), words);
    }

    #[test]
    fn empty_list() {
        let codec = Codec;
        let encoded = codec.encode(vec![]);
        assert_eq!(codec.decode(encoded), Vec::<String>::new());
    }

    #[test]
    fn strings_with_embedded_delimiter_chars() {
        let codec = Codec;
        let words = v(&["a#b", "1#2#3", "", "#"]);
        let encoded = codec.encode(words.clone());
        assert_eq!(codec.decode(encoded), words);
    }

    #[test]
    fn single_empty_string() {
        let codec = Codec;
        let words = v(&[""]);
        let encoded = codec.encode(words.clone());
        assert_eq!(codec.decode(encoded), words);
    }
}
