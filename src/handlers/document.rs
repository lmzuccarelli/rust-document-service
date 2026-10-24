use std::{fs, path::Path};

pub trait DocumentformInterface {
    fn save_document(path: String, data: String) -> Result<(), Box<dyn std::error::Error>>;
    fn get_document(key: String) -> Result<String, Box<dyn std::error::Error>>;
    fn search_document(search: String) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct Document {}

impl DocumentformInterface for Document {
    fn save_document(path: String, data: String) -> Result<(), Box<dyn std::error::Error>> {
        let updated_path = format!("store/{}", path);
        let p = Path::new(&updated_path);
        let directory = p.parent().unwrap_or(Path::new("general"));
        fs::create_dir_all(directory)?;
        Ok(fs::write(&path, data)?)
    }
    fn get_document(path: String) -> Result<String, Box<dyn std::error::Error>> {
        let updated_path = format!("store/{}", path);
        Ok(fs::read_to_string(&updated_path)?)
    }
    fn search_document(path: String) -> Result<String, Box<dyn std::error::Error>> {
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
}
