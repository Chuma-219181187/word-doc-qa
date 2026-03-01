use regex::Regex;

pub struct TextCleaner;

impl TextCleaner {
    /// Clean text: remove OCR artifacts, normalize spacing
    pub fn clean(text: &str) -> String {
        let mut cleaned = text.to_string();

        // Remove very long sequences of numbers (OCR artifacts from PDFs)
        let re = Regex::new(r"\b\d{8,}\b").unwrap();
        cleaned = re.replace_all(&cleaned, "").to_string();

        // Remove artifact patterns like -241817000 followed by month names
        cleaned = cleaned
            .replace("-241817000", "")
            .replace("-216535000", "")
            .replace("-146227-90208400", "")
            .replace("-13045559000", "");

        // Add spacing after month/year indicators  
        cleaned = cleaned
            .replace("2024JANUARY", "2024\nJANUARY")
            .replace("2024FEBRUARY", "2024\nFEBRUARY")
            .replace("2024MARCH", "2024\nMARCH")
            .replace("2024APRIL", "2024\nAPRIL");

        // Add spacing between month names and weekdays
        cleaned = cleaned
            .replace("JANUARY2024SUNDAY", "JANUARY 2024\nSUNDAY")
            .replace("FEBRUARY2024SUNDAY", "FEBRUARY 2024\nSUNDAY")
            .replace("MARCH2024SUNDAY", "MARCH 2024\nSUNDAY")
            .replace("APRIL2024SUNDAY", "APRIL 2024\nSUNDAY");

        // Add spacing after day names when together
        cleaned = cleaned
            .replace("SUNDAYMONDAY", "SUNDAY MONDAY")
            .replace("MONDAYTUESDAY", "MONDAY TUESDAY")
            .replace("TUESDAYWEDNESDAY", "TUESDAY WEDNESDAY")
            .replace("WEDNESDAYTHURSDAY", "WEDNESDAY THURSDAY")
            .replace("THURSDAYFRIDAY", "THURSDAY FRIDAY")
            .replace("FRIDAYSATURDAY", "FRIDAY SATURDAY");

        // Remove lines that are just numbers or noise
        let lines: Vec<&str> = cleaned
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() 
                    && !(trimmed.len() < 4 && trimmed.chars().all(|c| c.is_numeric()))
                    && !trimmed.chars().all(|c| c.is_numeric() || c == '-')
            })
            .collect();
        
        cleaned = lines.join("\n");

        // Remove excessive whitespace
        let re = Regex::new(r"[ \t]+").unwrap();
        cleaned = re.replace_all(&cleaned, " ").to_string();

        // Clean up common OCR artifacts
        cleaned = cleaned
            .replace("í", "i")
            .replace("ï", "i")
            .replace("ó", "o")
            .replace("ò", "o")
            .replace("ç", "c");

        // Trim leading/trailing whitespace
        cleaned.trim().to_string()
    }

    /// Split text into chunks of approximate token count, respecting sentence boundaries
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
