use super::parser::{parse_paper_toml, Paper};
use std::collections::VecDeque;
use wl_clipboard_rs::copy::{MimeType, Options, Source};

#[derive(Clone, Debug)]
pub struct Loader {
    pub file_paths: VecDeque<std::path::PathBuf>,
    pub papers: VecDeque<Paper>,
}

impl Loader {
    pub fn load_tomls(start: i32, load: i32, filedir: &mut std::path::PathBuf) -> Self {
        let end = start + load;
        let mut file_paths: VecDeque<std::path::PathBuf> = VecDeque::new();
        let mut papers: VecDeque<Paper> = VecDeque::new();

        for i in start..=end {
            let mut file_path = filedir.clone();
            file_path.push(format!("{}.toml", i));
            if file_path.exists() {
                let paper = match parse_paper_toml(&mut file_path) {
                    Some(p) => p,
                    None => {
                        continue;
                    }
                };
                papers.push_back(paper);
                file_paths.push_back(file_path);
            }
        }

        return Loader { file_paths, papers };
    }

    pub fn load_next(&mut self, filedir: &std::path::PathBuf) {
        // Check if file_paths is empty, if so, leave.
        if self.file_paths.is_empty() {
            return;
        }

        let last_load: &std::path::PathBuf = match self.file_paths.back() {
            Some(p) => p,
            None => {
                return;
            }
        };
        let num_last_load = match num_from_filepath(last_load) {
            Some(n) => n,
            None => {
                return;
            }
        };
        let mut path_next_load = filedir.clone();
        path_next_load.push(format!("{}.toml", num_last_load + 1));
        if path_next_load.exists() {
            // Create new paper
            let paper: Paper = match parse_paper_toml(&mut path_next_load) {
                Some(p) => p,
                None => {
                    return;
                }
            };
            // Pop first element out
            self.file_paths.pop_front();
            self.papers.pop_front();
            // Add new file path and new paper to the back
            self.file_paths.push_back(path_next_load);
            self.papers.push_back(paper);
        }
    }

    pub fn load_previous(&mut self, filedir: &std::path::PathBuf) {
        // Check if file_paths is empty, if so, leave.
        if self.file_paths.is_empty() {
            return;
        }
        // Get the first path
        let first_load: &std::path::PathBuf = match self.file_paths.front() {
            Some(p) => p,
            None => {
                return;
            }
        };
        // Extract number from it
        let num_first_load: i32 = match num_from_filepath(first_load) {
            Some(n) => n,
            None => {
                return;
            }
        };
        // Check if previous number is larger then the program minimum of 1.
        if num_first_load <= 1 {
            return;
        }
        // If not it is safe to proceed
        let mut path_prev_load = filedir.clone();
        path_prev_load.push(format!("{}.toml", num_first_load - 1));
        if path_prev_load.exists() {
            // Create new paper
            let paper = match parse_paper_toml(&mut path_prev_load) {
                Some(p) => p,
                None => {
                    return;
                }
            };
            // Remove path and paper from the back
            self.file_paths.pop_back();
            self.papers.pop_back();
            // Add new path and paper to the front
            self.file_paths.push_front(path_prev_load);
            self.papers.push_front(paper);
        }
    }

    pub fn bibtex_entry_to_clipboard(&self, selected_idx: usize) {
        let bibtex_entry = match self.papers.get(selected_idx) {
            Some(p) => p.bibtex.clone(),
            None => {
                return;
            }
        };
        let opts = Options::new();
        match opts.copy(
            Source::Bytes(bibtex_entry.into_bytes().into()),
            MimeType::Autodetect,
        ) {
            Ok(_) => {
                return;
            }
            Err(_) => {
                return;
            }
        }
    }

    pub fn open_file_in_editor(&self, selected_idx: usize) {
        // Get selected filepath
        let filepath = match self.file_paths.get(selected_idx) {
            Some(f) => f.clone(),
            None => {
                return;
            }
        };
        // Open file in editor in a new window, note that the terminal and editor are hard coded.
        match std::process::Command::new("kitty")
            .arg("--detach")
            .arg("nvim")
            .arg(filepath)
            .spawn()
        {
            Ok(_) => {
                return;
            }
            Err(_) => {
                return;
            }
        }
    }

    pub fn open_file_in_pdfviewer(
        &self,
        selected_idx: usize,
        pdf_viewer: &String,
        pdf_dir: &String,
    ) {
        // Get file name from the paper content
        let filename = match self.papers.get(selected_idx) {
            Some(p) => p.docname.clone(),
            None => {
                return;
            }
        };
        // Create path to that file with pdf_dir as directory
        let mut filepath = std::path::PathBuf::from(pdf_dir);
        filepath.push(filename);

        // Check if that file exists
        if filepath.exists() {
            // Open in given PDF viewer.
            match std::process::Command::new(pdf_viewer).arg(filepath).spawn() {
                Ok(_) => {
                    return;
                }
                Err(_) => {
                    return;
                }
            }
        }
    }

    pub fn remove_file(&mut self, selected_idx: usize) {
        // Get the file path
        let filepath = match self.file_paths.get(selected_idx) {
            Some(f) => f.clone(),
            None => {
                return;
            }
        };
        // Check if it exists
        if filepath.exists() {
            // Delete it
            std::fs::remove_file(filepath).expect("Error attempting to remove file.");
            // Remove file path and paper from the loader
            self.file_paths.remove(selected_idx);
            self.papers.remove(selected_idx);
        }
    }
}

pub fn num_from_filepath(filepath: &std::path::PathBuf) -> Option<i32> {
    let file_name_os_string = match filepath.file_name() {
        Some(f) => f,
        None => {
            return None;
        }
    };

    let file_name = match file_name_os_string.to_str() {
        Some(s) => s,
        None => {
            return None;
        }
    };

    let re = match regex::Regex::new(r"^(\d+)\.toml$") {
        Ok(r) => r,
        Err(_) => {
            return None;
        }
    };

    if re.is_match(&file_name) {
        let caps = match re.captures(&file_name) {
            Some(c) => c,
            None => {
                return None;
            }
        };
        match caps[1].parse::<i32>() {
            Ok(n) => {
                return Some(n);
            }
            Err(_) => {
                return None;
            }
        }
    } else {
        return None;
    }
}
