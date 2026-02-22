use std::fs;
use std::path::Path;

pub struct DocumentLoader;

impl DocumentLoader {
    /// Load text from a .docx file (simplified version)
    pub fn load_docx(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    // Read file as plain text (works for both .txt and .docx)
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(e) => {
            eprintln!("Error reading file {:?}: {}", path, e);
            Ok(String::new())
        }
    }
}

    /// Load all .docx files from a directory
    pub fn load_documents(dir: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut documents = Vec::new();

        if !dir.exists() {
            return Ok(documents);
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

         if path.extension().map_or(false, |ext| ext == "txt" || ext == "docx") {
                match Self::load_docx(&path) {
                    Ok(text) => {
                        if !text.is_empty() {
                            documents.push(text);
                        }
                    }
                    Err(e) => eprintln!("Warning: Failed to load {:?}: {}", path, e),
                }
            }
        }

        Ok(documents)
    }
}
