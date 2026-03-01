use std::fs;
use std::path::Path;
use std::io::Read;
use zip::ZipArchive;

pub struct DocumentLoader;

impl DocumentLoader {
    /// Load text from a .txt file
    fn load_txt(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        fs::read_to_string(path).map_err(|e| e.into())
    }

    /// Load text from a .docx file by extracting XML from zip
    fn load_docx_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let file = fs::File::open(path)?;
        let mut zip = ZipArchive::new(file)?;
        
        // DOCX files are ZIP archives. The main document content is in word/document.xml
        let mut file_content = zip.by_name("word/document.xml")?;
        let mut xml_string = String::new();
        file_content.read_to_string(&mut xml_string)?;
        
        // Extract text from XML by removing tags and decoding entities
        let text = Self::extract_text_from_xml(&xml_string);
        
        if !text.trim().is_empty() {
            Ok(text)
        } else {
            Err("No text content found in DOCX file".into())
        }
    }

    /// Simple XML text extraction - removes XML tags
    fn extract_text_from_xml(xml: &str) -> String {
        let mut text = String::new();
        let mut inside_tag = false;
        
        for ch in xml.chars() {
            match ch {
                '<' => inside_tag = true,
                '>' => {
                    inside_tag = false;
                }
                _ if !inside_tag => {
                    text.push(ch);
                }
                _ => {}
            }
        }
        
        // Decode XML entities
        text = text
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&")
            .replace("&quot;", "\"")
            .replace("&apos;", "'");
        
        // Clean up excessive whitespace
        let lines: Vec<&str> = text.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
        lines.join("\n")
    }

    /// Load text from a document file (.docx or .txt)
    pub fn load_docx(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("docx") => {
                match Self::load_docx_file(path) {
                    Ok(text) => Ok(text),
                    Err(e) => {
                        eprintln!("Error reading DOCX file {:?}: {}", path, e);
                        Ok(String::new())
                    }
                }
            }
            Some("txt") => {
                match Self::load_txt(path) {
                    Ok(text) => Ok(text),
                    Err(e) => {
                        eprintln!("Error reading TXT file {:?}: {}", path, e);
                        Ok(String::new())
                    }
                }
            }
            _ => {
                eprintln!("Unsupported file format: {:?}", path);
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
