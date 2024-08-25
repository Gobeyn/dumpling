use crate::file::loader::expand_filepath;
use crate::file::parser::{parse_paper_toml, Paper};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Pdf {
    pub valid_paths: Vec<std::path::PathBuf>,
    pub invalid_paths: Vec<std::path::PathBuf>,
}

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
    // Loop through the paths
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

fn get_paper_pdf_paths(papers: &Vec<Paper>, pdf_dir: &std::path::PathBuf) -> Pdf {
    // Initialise vector
    let mut valid_paths: Vec<std::path::PathBuf> = Vec::new();
    let mut invalid_paths: Vec<std::path::PathBuf> = Vec::new();
    // Loop through all the papers, extract the docname, append to the pdf_dir and put into
    // mentioned_paths vector
    for paper in papers {
        let mut file_path = pdf_dir.to_path_buf();
        file_path.push(&paper.docname);
        if file_path.exists() {
            valid_paths.push(file_path);
        } else {
            invalid_paths.push(file_path);
        }
    }

    return Pdf {
        valid_paths,
        invalid_paths,
    };
}

fn get_stored_pdf_paths(pdf_dir: &std::path::PathBuf) -> Vec<std::path::PathBuf> {
    let mut pdf_paths: Vec<std::path::PathBuf> = Vec::new();
    //let folder_path = std::path::PathBuf::from(pdf_dir);

    let paths = match std::fs::read_dir(&pdf_dir) {
        Ok(p) => p,
        Err(err) => {
            log::error!("Error obtaining PDF file paths: {err}");
            std::process::exit(1);
        }
    };
    for path in paths {
        let file_path = match path {
            Ok(p) => p.path(),
            Err(err) => {
                log::warn!("Error extracting `DirEntry` from path, continuing to next path: {err}");
                continue;
            }
        };
        if file_path.exists() {
            let extension = match file_path.extension() {
                Some(e) => e,
                None => {
                    log::warn!(
                        "Could not extract extension from {file_path:?}, continuing to next path."
                    );
                    continue;
                }
            };
            if extension == "pdf" {
                pdf_paths.push(file_path);
            }
        }
    }

    return pdf_paths;
}

fn get_unmatched_pdf_paths(
    paths_from_paper: &Vec<std::path::PathBuf>,
    stored_paths: &Vec<std::path::PathBuf>,
) -> Vec<std::path::PathBuf> {
    // Convert the paths from the paper entries into a HashSet for fast lookup.
    let from_paper_set: HashSet<_> = paths_from_paper.into_iter().collect();
    // Collect the elements from the stored_paths that do not appear in the paths_from_paper
    let unique_paths: Vec<std::path::PathBuf> = stored_paths
        .into_iter()
        .filter(|path| !from_paper_set.contains(path))
        .cloned()
        .collect();
    return unique_paths;
}

fn show_invalid_and_not_used_paths(
    invalid_paths: &Vec<std::path::PathBuf>,
    not_used_paths: &Vec<std::path::PathBuf>,
) {
    // Show the invalid PDF file paths
    println!("The following PDF file paths were mentioned, but do not exists: ");
    if invalid_paths.is_empty() {
        println!("No invalid paths found.");
    } else {
        for path in invalid_paths {
            println!("{:?}", path);
        }
    }
    // Show the not used paths
    println!("The following PDF file paths exist, but no paper entry has been made for it: ");
    if not_used_paths.is_empty() {
        println!("All file paths are used.");
    } else {
        for path in not_used_paths {
            println!("{:?}", path);
        }
    }
}

pub fn pdf_diagnostic(folderdir: &std::path::PathBuf, pdf_dir: &String) {
    let pdf_dir: std::path::PathBuf = expand_filepath(&std::path::PathBuf::from(pdf_dir));
    // Load the pdf files mentioned by the paper files
    let papers = load_all_papers(folderdir);
    let paper_pdfs = get_paper_pdf_paths(&papers, &pdf_dir);
    // Get the existing PDF file paths
    let stored_pdfs = get_stored_pdf_paths(&pdf_dir);
    // Get the PDF file paths not mentioned by any paper file
    let not_used_paths = get_unmatched_pdf_paths(&paper_pdfs.valid_paths, &stored_pdfs);
    // Print the diagnostic
    show_invalid_and_not_used_paths(&paper_pdfs.invalid_paths, &not_used_paths);
}
