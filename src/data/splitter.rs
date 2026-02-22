pub struct Splitter;

impl Splitter {
    /// Split text into overlapping chunks
    pub fn split_overlapping(text: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut chunks = Vec::new();

        let mut i = 0;
        while i < words.len() {
            let end = std::cmp::min(i + chunk_size, words.len());
            let chunk = words[i..end].join(" ");
            chunks.push(chunk);

            if end >= words.len() {
                break;
            }
            i += chunk_size - overlap;
        }

        chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_overlapping() {
        let text = "word1 word2 word3 word4 word5 word6 word7 word8";
        let chunks = Splitter::split_overlapping(text, 3, 1);
        assert!(chunks.len() > 0);
    }
}
