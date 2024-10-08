use crate::file::parser::{parse_paper_toml, Paper};
use std::collections::HashMap;

/// Create a vector of all the parsed paper entries with the valid *.toml format looking only
/// in the given `folderdir` directory.
fn load_all_papers(folderdir: &std::path::PathBuf) -> Vec<Paper> {
    // Initialise vector
    let mut papers: Vec<Paper> = Vec::new();
    // Loop trough all files in the directory.
    let paths = match std::fs::read_dir(folderdir) {
        Ok(p) => p,
        Err(err) => {
            log::error!("Error obtaining paths to files in {folderdir:?}: {err}");
            std::process::exit(1);
        }
    };
    // Regex for the expected INT.toml format.
    let re = match regex::Regex::new(r"^*\.toml$") {
        Ok(r) => r,
        Err(err) => {
            log::error!("Error creating *.toml regex: {err}");
            std::process::exit(1);
        }
    };
    // Loop through the path.
    for path in paths {
        let (file_name_os_string, mut file_path) = match path {
            Ok(p) => {
                let fnos = p.file_name();
                let fp = p.path();
                (fnos, fp)
            }
            Err(err) => {
                log::warn!(
                    "Error extracting `DirEntry` from a path, continuing to next path: {err}"
                );
                continue;
            }
        };
        let file_name = match file_name_os_string.to_str() {
            Some(s) => s,
            None => {
                log::error!("Error converting `OsString` into `&str`.");
                std::process::exit(1);
            }
        };
        if re.is_match(&file_name) {
            let paper = match parse_paper_toml(&mut file_path) {
                Some(p) => p,
                None => {
                    log::warn!("Error deserialising {file_path:?} into `Paper` struct, continuing to next paper.");
                    continue;
                }
            };
            papers.push(paper);
        }
    }
    return papers;
}

/// Extract the tags listed in a given vector of `Paper` structs.
fn get_tags(papers: &Vec<Paper>) -> Vec<String> {
    // Initiate vector
    let mut tags: Vec<String> = Vec::new();
    // Loop through all the papers and get the tags listed in them.
    for paper in papers {
        for tag in &paper.tags {
            tags.push(tag.label.clone());
        }
    }
    return tags;
}

/// Convert a vector of tags into a `HashMap`, which keeps only
/// the unique entries in the given vector and keep count of
/// how many times that entry has appeared.
fn to_hash_map(tags: Vec<String>) -> HashMap<String, usize> {
    // Initiate hash map
    let mut map: HashMap<String, usize> = HashMap::new();
    // Add each unique element in the Vec to the HashMap.
    for tag in tags {
        let count = map.entry(tag).or_insert(0);
        *count += 1;
    }
    return map;
}

/// Given a `HashMap` with the present tags and how many times they
/// appear, format the text to show to the user.
fn show_tags_and_count(tags_hm: &HashMap<String, usize>) {
    for (tag, count) in tags_hm {
        println!("{}: Appears {} times", tag, count);
    }
}

/// Function that chains together `load_all_papers`, `get_tags`, `to_hash_map` and
/// `show_tags_and_count`.
pub fn list_tags(folderdir: &std::path::PathBuf) {
    let papers = load_all_papers(folderdir);
    let tags_vec = get_tags(&papers);
    let tags_hm = to_hash_map(tags_vec);
    show_tags_and_count(&tags_hm);
}
