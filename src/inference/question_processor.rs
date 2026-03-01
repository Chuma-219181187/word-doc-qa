use crate::data::TextCleaner;
use std::collections::HashMap;

pub struct QuestionProcessor;

impl QuestionProcessor {
    pub fn process(question: &str) -> String {
        let cleaned = TextCleaner::clean(question);
        cleaned.trim().to_string()
    }

    /// Filter out common stopwords to focus on meaningful terms
    fn is_stopword(word: &str) -> bool {
        matches!(
            word.to_lowercase().as_str(),
            "a" | "an" | "and" | "are" | "as" | "at" | "be" | "by" | "for" | "from"
                | "has" | "he" | "in" | "is" | "it" | "its" | "of" | "on" | "or" | "that"
                | "the" | "to" | "was" | "will" | "with" | "what" | "when" | "where" | "why"
                | "how" | "who" | "which" | "this" | "these" | "those" | "should" | "would"
                | "could" | "can" | "do" | "does" | "did" | "have" | "has" | "tell" | "me"
                | "about" | "give" | "provide" | "show" | "list" | "get" | "put" | "find"
                | "i" | "you" | "we" | "they" | "them" | "their"
        )
    }

    /// Get domain-specific synonym groups for the academic calendar domain
    fn get_synonyms() -> HashMap<&'static str, Vec<&'static str>> {
        let mut synonyms = HashMap::new();
        
        // Academic breaks and holidays
        synonyms.insert("break", vec!["recess", "holiday", "vacation", "closure"]);
        synonyms.insert("breaks", vec!["recess", "holiday", "vacation", "closure"]);
        synonyms.insert("recess", vec!["break", "breaks", "holiday", "vacation"]);
        synonyms.insert("recesses", vec!["break", "breaks", "holiday", "vacation"]);
        synonyms.insert("holiday", vec!["break", "breaks", "recess", "vacation"]);
        synonyms.insert("holidays", vec!["break", "breaks", "recess", "vacation"]);
        synonyms.insert("vacation", vec!["break", "breaks", "holiday", "recess"]);
        synonyms.insert("vacations", vec!["break", "breaks", "holiday", "recess"]);
        
        // Graduation and ceremonies
        synonyms.insert("graduation", vec!["graduate", "commencement", "convocation"]);
        synonyms.insert("graduations", vec!["graduate", "commencement", "convocation"]);
        synonyms.insert("graduate", vec!["graduation", "graduations", "commencement"]);
        synonyms.insert("graduates", vec!["graduation", "graduations", "commencement"]);
        synonyms.insert("graduating", vec!["graduation", "commencement"]);
        synonyms.insert("commencement", vec!["graduation", "graduations", "convocation"]);
        synonyms.insert("convocation", vec!["graduation", "graduations", "ceremony"]);
        synonyms.insert("ceremony", vec!["graduation", "convocation", "graduation"]);
        synonyms.insert("ceremonies", vec!["graduation", "convocation"]);
        
        // Academic terms
        synonyms.insert("term", vec!["semester", "session"]);
        synonyms.insert("terms", vec!["semester", "session"]);
        synonyms.insert("semester", vec!["term", "terms", "session"]);
        synonyms.insert("semesters", vec!["term", "terms", "session"]);
        
        // Meetings and committees
        synonyms.insert("committee", vec!["board", "council", "committee"]);
        synonyms.insert("committees", vec!["board", "council"]);
        synonyms.insert("board", vec!["committee", "committees", "council"]);
        synonyms.insert("boards", vec!["committee", "committees", "council"]);
        synonyms.insert("council", vec!["committee", "committees", "board"]);
        synonyms.insert("councils", vec!["committee", "committees", "board"]);
        synonyms.insert("meeting", vec!["committee", "forum", "session"]);
        synonyms.insert("meetings", vec!["committee", "forum", "session"]);
        
        synonyms
    }

    /// Expand keywords with synonyms
    fn expand_keywords(keywords: &[String]) -> Vec<String> {
        let synonyms = Self::get_synonyms();
        let mut expanded = keywords.to_vec();
        
        for keyword in keywords {
            if let Some(syn_list) = synonyms.get(keyword.as_str()) {
                for syn in syn_list {
                    if !expanded.contains(&syn.to_string()) {
                        expanded.push(syn.to_string());
                    }
                }
            }
        }
        
        expanded
    }

    /// Extract meaningful keywords from question, filtering stopwords
    fn extract_keywords(text: &str) -> Vec<String> {
        let basic_keywords: Vec<String> = text
            .split_whitespace()
            .filter(|word| !Self::is_stopword(word) && word.len() > 2)
            .map(|word| word.to_lowercase())
            .collect();
        
        // Expand with synonyms
        Self::expand_keywords(&basic_keywords)
    }

    /// Calculate semantic relevance score between question and chunk
    fn calculate_relevance_score(question: &str, chunk: &str) -> f32 {
        let question_keywords = Self::extract_keywords(question);
        
        if question_keywords.is_empty() {
            return 0.0;
        }

        let chunk_lower = chunk.to_lowercase();
        let mut keywords_found = 0;
        let mut total_occurrences = 0;

        // Count keywords that appear in the chunk and their occurrences
        for keyword in &question_keywords {
            if chunk_lower.contains(keyword) {
                keywords_found += 1;
                
                // Count occurrences for density bonus (capped at 3)
                let occurrences = chunk_lower.matches(keyword).count().min(3);
                total_occurrences += occurrences;
            }
        }

        if keywords_found == 0 {
            return 0.0;
        }

        // Base score: what fraction of original question keywords were found
        // (count original keywords only, not synonyms)
        let original_keywords: Vec<String> = question
            .split_whitespace()
            .filter(|word| !Self::is_stopword(word) && word.len() > 2)
            .map(|word| word.to_lowercase())
            .collect();
        
        let original_keyword_coverage = if original_keywords.is_empty() {
            0.5  // fallback for edge case
        } else {
            let original_found = original_keywords
                .iter()
                .filter(|kw| chunk_lower.contains(&kw.to_lowercase()))
                .count();
            original_found as f32 / original_keywords.len() as f32
        };
        
        // Density bonus: reward chunks with multiple keyword mentions
        let occurrence_bonus = (total_occurrences as f32) / (question_keywords.len() as f32 * 3.0);
        
        // Combine: weight keyword coverage at 70%, occurrence frequency at 30%
        let mut base_score = (original_keyword_coverage * 0.7) + (occurrence_bonus * 0.3);

        // Chunk length factor: penalize overly long chunks slightly
        let chunk_words = chunk.split_whitespace().count();
        let length_factor = if chunk_words > 500 {
            0.8
        } else if chunk_words > 200 {
            0.9
        } else {
            1.0
        };

        // Final score: ensure it's always between 0.0 and 1.0
        (base_score * length_factor).min(1.0)
    }

    pub fn find_relevant_chunks(
        question: &str,
        chunks: &[String],
        top_k: usize,
    ) -> Vec<(String, f32)> {
        let mut scored_chunks: Vec<(String, f32)> = chunks
            .iter()
            .map(|chunk| {
                let score = Self::calculate_relevance_score(question, chunk);
                (chunk.clone(), score)
            })
            .collect();

        // Sort by score in descending order
        scored_chunks.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Filter out zero-score chunks and return top_k
        scored_chunks
            .into_iter()
            .filter(|(_, score)| *score > 0.0)
            .take(top_k)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_question() {
        let question = "What is the GRADUATION date?";
        let processed = QuestionProcessor::process(question);
        assert!(processed.len() > 0);
        assert!(processed.contains("graduation"));
    }

    #[test]
    fn test_find_relevant_chunks() {
        let question = "graduation ceremony";
        let chunks = vec![
            "The graduation ceremony is on June 15".to_string(),
            "The weather is sunny today".to_string(),
        ];
        let relevant = QuestionProcessor::find_relevant_chunks(&question, &chunks, 1);
        assert_eq!(relevant.len(), 1);
    }
}
