use custom_logger as log;
use regex::Regex;
use std::{fs, path::Path};

pub trait DocumentformInterface {
    fn save_document(path: String, data: String) -> Result<(), Box<dyn std::error::Error>>;
    fn get_document(key: String) -> Result<String, Box<dyn std::error::Error>>;
    fn search_folder(search: String) -> Result<String, Box<dyn std::error::Error>>;
    fn regex_search_documents(
        search: String,
        pattern: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct Document {}

impl DocumentformInterface for Document {
    fn save_document(path: String, data: String) -> Result<(), Box<dyn std::error::Error>> {
        let updated_path = format!("store/{}", path);
        let p = Path::new(&updated_path);
        let directory = p.parent().unwrap_or(Path::new("general"));
        fs::create_dir_all(directory)?;
        log::debug!("[save_document] path {:?}", directory);
        Ok(fs::write(&p, data)?)
    }

    fn get_document(path: String) -> Result<String, Box<dyn std::error::Error>> {
        let updated_path = format!("store/{}", path);
        log::debug!("[get_document] {}", updated_path);
        Ok(fs::read_to_string(&updated_path)?)
    }

    fn search_folder(path: String) -> Result<String, Box<dyn std::error::Error>> {
        let updated_path = format!("store/{}", path);
        let files: Vec<String> = fs::read_dir(updated_path)?
            .map(|x| {
                let entry = x?;
                Ok(entry.file_name().to_string_lossy().into_owned())
            })
            .collect::<Result<Vec<String>, std::io::Error>>()?;
        let mut result = files.join("\n");
        if !result.is_empty() {
            result.push('\n');
        }
        Ok(result)
    }

    fn regex_search_documents(
        path: String,
        pattern: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let re = Regex::new(pattern)?;
        let updated_path = format!("store/{}", path);
        let mut matching_files = Vec::new();

        for entry in fs::read_dir(updated_path)? {
            let entry = entry?;
            let file_path = entry.path();

            // Skip subdirectories so read_to_string doesn't fail
            if file_path.is_file() {
                // Skip non-UTF8/binary files gracefully without crashing the search
                if let Ok(contents) = fs::read_to_string(&file_path)
                    && re.is_match(&contents)
                {
                    matching_files.push(entry.file_name().to_string_lossy().into_owned());
                }
            }
        }

        let mut result = matching_files.join("\n");
        if !result.is_empty() {
            result.push('\n');
        }
        Ok(result)
    }
}
