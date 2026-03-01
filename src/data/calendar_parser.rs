use regex::Regex;

pub struct CalendarParser;

impl CalendarParser {
    /// Parse calendar text and structure it into readable events
    pub fn parse_events(text: &str) -> Vec<String> {
        let mut events = Vec::new();
        
        // Split by common date/event separators first
        let sections = Self::split_into_sections(text);
        
        for section in sections {
            if section.trim().is_empty() {
                continue;
            }
            
            // Extract individual events from this section
            let parsed_events = Self::extract_events(&section);
            events.extend(parsed_events);
        }
        
        events
    }

    /// Split text into sections by month/date markers
    fn split_into_sections(text: &str) -> Vec<String> {
        let mut sections = Vec::new();
        let mut current_section = String::new();
        
        // Patterns to split on: months, recess, end of term markers
        let split_patterns = vec![
            "JANUARY", "FEBRUARY", "MARCH", "APRIL", "MAY", "JUNE",
            "JULY", "AUGUST", "SEPTEMBER", "OCTOBER", "NOVEMBER", "DECEMBER",
            "RECESS", "START OF TERM", "END OF TERM", "HOLIDAY"
        ];
        
        for word in text.split_whitespace() {
            // Check if this word is a split trigger
            if split_patterns.iter().any(|p| word.contains(p)) && !current_section.is_empty() {
                sections.push(current_section.trim().to_string());
                current_section = format!("{} ", word);
            } else {
                current_section.push_str(&format!("{} ", word));
            }
        }
        
        if !current_section.is_empty() {
            sections.push(current_section.trim().to_string());
        }
        
        sections
    }

    /// Extract individual events from a section
    fn extract_events(section: &str) -> Vec<String> {
        let mut events = Vec::new();
        
        // Simple approach: look for time-stamped events: "Text (HH:MM)"
        let time_event_re = Regex::new(r"([A-Z][A-Za-z\s&:,\.\-0-9]*?)\s*\((\d{2}:\d{2})\)").unwrap();
        for caps in time_event_re.captures_iter(section) {
            let event = caps.get(1).map_or("", |m| m.as_str()).trim().to_string();
            let time = caps.get(2).map_or("", |m| m.as_str());
            
            if !event.is_empty() && event.len() > 2 {
                let formatted_event = format!("{} ({})", event, time);
                if !events.contains(&formatted_event) {
                    events.push(formatted_event);
                }
            }
        }
        
        // Also extract key phrases: "START OF TERM", "END OF TERM", etc.
        let key_phrases = vec![
            "START OF TERM 1", "START OF TERM 2", 
            "END OF TERM 1", "END OF TERM 2",
            "WCED SCHOOLS OPEN", "WCED SCHOOLS CLOSE",
            "NEW YEAR'S DAY", "FAMILY DAY", "GOOD FRIDAY",
            "HUMAN RIGHTS DAY", "INTERNATIONAL",
            "RECESS"
        ];
        
        for phrase in key_phrases {
            if section.contains(phrase) {
                events.push(phrase.to_string());
            }
        }
        
        events
    }

    /// Format calendar events into a readable string with line breaks
    pub fn format_events(events: &[String]) -> String {
        if events.is_empty() {
            return "No events found in calendar.".to_string();
        }
        
        // Deduplicate events while preserving order
        let mut unique_events = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for event in events {
            let normalized = event.trim().to_lowercase();
            if !seen.contains(&normalized) && normalized.len() > 2 {
                seen.insert(normalized);
                unique_events.push(event.clone());
            }
        }
        
        // Format with one event per line
        unique_events
            .iter()
            .take(20) // Limit to 20 events for readability
            .map(|e| format!("• {}", e))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_events() {
        let text = "JANUARY 2024 Management Committee (09:00) Senate Library Committee (13:00)";
        let events = CalendarParser::parse_events(text);
        assert!(!events.is_empty());
    }

    #[test]
    fn test_format_events() {
        let events = vec![
            "Management Committee (09:00)".to_string(),
            "Senate Meeting (13:00)".to_string(),
        ];
        let formatted = CalendarParser::format_events(&events);
        assert!(formatted.contains("•"));
    }
}
