use serde::{Deserialize, Serialize};
use std::io::Read;
//use std::io::Write;

#[derive(Deserialize, Serialize, Debug)]
pub struct Author {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tag {
    pub label: String,
}

#[derive(Deserialize, Serialize, Debug)]
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
            description: String::from("Paper has not been read yet."),
            bibtex: String::new(),
            docname: String::new(),
            authors: Vec::new(),
            tags: Vec::new(),
        }
    }
}

pub fn parse_paper_toml(filepath: &mut std::path::PathBuf) -> Option<Paper> {
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

//pub fn write_paper_to_toml(paper: Paper, filepath: std::path::PathBuf) {
//
//}
