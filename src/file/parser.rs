use serde::{Deserialize, Serialize};
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
        Err(_) => {
            return None;
        }
    };

    // Read the contents of the file
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => {
            return None;
        }
    }

    // Parse the file contents
    let parsed_toml: toml::Value = match toml::from_str(&contents) {
        Ok(v) => v,
        Err(_) => {
            return None;
        }
    };

    // Convert parsed file contents into Rust struct, using default values
    // if needed.
    let paper: Paper = match parsed_toml.try_into() {
        Ok(v) => v,
        Err(_) => {
            return None;
        }
    };
    return Some(paper);
}

/// Given a `Paper` Rust struct and a file path, write the struct
/// to a Toml file specified by the file path.
pub fn write_paper_to_toml(paper: &Paper, filepath: &std::path::PathBuf) {
    let toml_str = toml::to_string(paper).expect("Error parsing Paper struct to Toml string");
    std::fs::write(filepath, toml_str).expect("Error writing Toml file.");
}

/// Effectively the same as `write_paper_to_toml`, but it does some
/// additional things. It mainly checks the directory where they file
/// would be safed, and gives the Toml file that will be created a
/// standardized name.
pub fn write_new_paper(paper: &Paper, filedir: &std::path::PathBuf) {
    let paths = match std::fs::read_dir(filedir) {
        Ok(p) => p,
        Err(_) => {
            println!("Error trying to obtain paper information file paths.");
            std::process::exit(1);
        }
    };
    let num_files = paths.count();

    // Create file path for new paper entry
    let mut new_paper_filepath = filedir.clone();
    if num_files == 0 {
        new_paper_filepath.push("1.toml");
    } else {
        new_paper_filepath.push(format!("{}.toml", num_files + 1));
    }

    // Write the paper entry
    write_paper_to_toml(paper, &new_paper_filepath);
}
