use regex::Regex;

pub struct TextCleaner;

impl TextCleaner {
    /// Clean text: lowercase, remove extra spaces, remove special chars
    pub fn clean(text: &str) -> String {
        let mut cleaned = text.to_lowercase();

        // Remove extra whitespace
        let re = Regex::new(r"\s+").unwrap();
        cleaned = re.replace_all(&cleaned, " ").to_string();

        // Remove non-alphanumeric but keep spaces and basic punctuation
        let re = Regex::new(r"[^\w\s\.\,\?\!\:\;\-]").unwrap();
        cleaned = re.replace_all(&cleaned, "").to_string();

        // Trim leading/trailing whitespace
        cleaned.trim().to_string()
    }

    /// Split text into chunks of approximate token count
    pub fn split_into_chunks(text: &str, chunk_size: usize) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut word_count = 0;

        for word in words {
            if word_count >= chunk_size && !current_chunk.is_empty() {
                chunks.push(current_chunk.clone());
                current_chunk.clear();
                word_count = 0;
            }
            if !current_chunk.is_empty() {
                current_chunk.push(' ');
            }
            current_chunk.push_str(word);
            word_count += 1;
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean() {
        let text = "Hello   WORLD!!!   Test";
        let cleaned = TextCleaner::clean(text);
        assert!(cleaned.contains("hello") && cleaned.contains("world"));
    }

    #[test]
    fn test_split() {
        let text = "word1 word2 word3 word4 word5";
        let chunks = TextCleaner::split_into_chunks(text, 2);
        assert!(chunks.len() > 0);
    }
}
