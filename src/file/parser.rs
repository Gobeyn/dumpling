use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::Read;

/// Deserialization struct for parsing the paper Toml files.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Author {
    pub name: String,
}

/// Deserialization struct for parsing the paper Toml files.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Tag {
    pub label: String,
}

/// Main deserialization struct for parsing the paper Toml files.
/// This includes vectors of the `Author` and `Tag` structs.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct Paper {
    pub title: String,
    pub year: i32,
    pub journal: String,
    pub description: String,
    pub bibtex: String,
    pub docname: String,
    pub authors: Vec<Author>,
    pub tags: Vec<Tag>,
}

impl Default for Paper {
    fn default() -> Self {
        Paper {
            title: String::new(),
            year: 0,
            journal: String::new(),
            description: String::new(),
            bibtex: String::new(),
            docname: String::new(),
            authors: Vec::new(),
            tags: Vec::new(),
        }
    }
}

/// Given the file path to a paper Toml file, parse the file contents into
/// the `Paper` Rust struct. If any step failed, `None` is returned.
pub fn parse_paper_toml(filepath: &std::path::PathBuf) -> Option<Paper> {
    // Attempt to open the file
    let mut file = match std::fs::File::open(filepath) {
        Ok(v) => v,
        Err(err) => {
            log::warn!("Error opening {filepath:?}, `None` is returned: {err}");
            return None;
        }
    };

    // Read the contents of the file
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(err) => {
            log::warn!("Error reading contents of {filepath:?}, `None` is returned: {err}");
            return None;
        }
    }

    // Parse the file contents
    let parsed_toml: toml::Value = match toml::from_str(&contents) {
        Ok(v) => v,
        Err(err) => {
            log::warn!("Error parsing contents of {filepath:?} into `toml::Value`, `None` is returned: {err}");
            return None;
        }
    };

    // Convert parsed file contents into Rust struct, using default values
    // if needed.
    let paper: Paper = match parsed_toml.try_into() {
        Ok(v) => v,
        Err(err) => {
            log::warn!("Error deserializing contents of {filepath:?} into `Paper` struct, `None` is returned: {err}");
            return None;
        }
    };
    return Some(paper);
}

/// Given a particular instance of `Paper` and a file path to a folder,
/// create a unique file name using SHA256 encoding on the file content bytes and
/// store the serialized `Paper` instance as a *.toml file in the specified directory.
pub fn write_new_paper(paper: &Paper, folderdir: &std::path::PathBuf) {
    // Convert the `Paper` struct into a toml formatted string.
    let toml_str = match toml::to_string(paper) {
        Ok(s) => s,
        Err(err) => {
            log::error!("Error serializing `Paper` struct into Toml formatted string: {err}");
            std::process::exit(1);
        }
    };
    // Create unique file name to store the file under by hashing the byte content.
    // Create new hasher instance
    let mut hasher = Sha256::new();
    // Hash the serialized Toml string
    hasher.update(toml_str.as_bytes());
    // Finalize hash and get result as byte array
    let hash_bytes = hasher.finalize();
    // Convert byte array to hexadecimal string
    let file_name = format!("{}.toml", hex::encode(hash_bytes));
    // Create file path for the saved paper
    let mut file_path = folderdir.clone();
    file_path.push(file_name);
    // Save the file
    match std::fs::write(file_path, toml_str) {
        Ok(_) => {}
        Err(err) => {
            log::error!("Error writing contents of `Paper` struct: {err}");
            std::process::exit(1);
        }
    }
}
