use super::parser::{parse_paper_toml, Paper};
use std::collections::VecDeque;
use wl_clipboard_rs::copy::{MimeType, Options, Source};

/// In the given `filedir`, look for all the files of the valid INT.toml format and
/// store the path to them in a vector that is returned.
pub fn get_all_valid_filepaths(
    folderdir: &std::path::PathBuf,
    tag_filter: &Option<String>,
) -> Vec<std::path::PathBuf> {
    // Initialise vector
    let mut all_file_paths: Vec<std::path::PathBuf> = Vec::new();
    // Loop trough all files in the directory.
    let paths = std::fs::read_dir(folderdir).expect("Error obtaining file paths.");
    // Define the expected form of the files.
    let re = regex::Regex::new(r"^*\.toml$").expect("Error building Regex.");
    // Loop through all the paths, and add them to the vector
    for path in paths {
        let (file_name_os_string, file_path) = match path {
            Ok(p) => (p.file_name(), p.path()),
            Err(_) => {
                continue;
            }
        };
        let file_name = file_name_os_string.to_str().unwrap();
        if re.is_match(&file_name) {
            match tag_filter {
                Some(tag) => {
                    // Parse the paper
                    let paper = match parse_paper_toml(&file_path) {
                        Some(p) => p,
                        None => {
                            continue;
                        }
                    };
                    // Check if any of the tags matches the given tag
                    for tag_paper in &paper.tags {
                        if *tag == tag_paper.label {
                            // If the tag is present, include it in the valid paths
                            all_file_paths.push(file_path);
                            break;
                        }
                    }
                }
                None => {
                    all_file_paths.push(file_path);
                }
            }
        }
    }
    return all_file_paths;
}

/// Loader struct that makes sure not every paper is loaded at the beginning of the
/// program and only loads a new paper if and when we need it.
///
/// The `valid_paths` field contains the paths to all the paper files that could be loaded.
/// The `loaded_paths` field contains index pointers to `valid_paths` with the currently loaded
/// paths.
/// The `paper` field contains the parsed `Paper` structs of the loaded paths specified by
/// `loaded_paths`.
#[derive(Clone, Debug)]
pub struct Loader {
    pub valid_paths: Vec<std::path::PathBuf>,
    pub loaded_paths: VecDeque<usize>,
    pub papers: VecDeque<Paper>,
}

impl Loader {
    /// Given a load size and the directory where the paper information
    /// files are stored, create a new instance of `Loader`. The
    /// `loaded_paths` will be as large as possible, bounded by the
    /// `load` parameter.
    pub fn load(load: i32, folderdir: &std::path::PathBuf, tag_filter: &Option<String>) -> Self {
        // Initialise the `Loader` fields.
        let valid_paths = get_all_valid_filepaths(folderdir, tag_filter);
        let mut loaded_paths: VecDeque<usize> = VecDeque::new();
        let mut papers: VecDeque<Paper> = VecDeque::new();

        for i in 0..=load {
            let file_path = match valid_paths.get(i as usize) {
                Some(p) => p,
                None => {
                    continue;
                }
            };
            let paper = match parse_paper_toml(file_path) {
                Some(p) => p,
                None => {
                    continue;
                }
            };
            loaded_paths.push_back(i as usize);
            papers.push_back(paper);
        }

        return Loader {
            valid_paths,
            loaded_paths,
            papers,
        };
    }
    /// Load the next paper, if there is another valid paper to load.
    pub fn load_next(&mut self) {
        // Check if there is even anything to load, if not, exit.
        if self.valid_paths.is_empty() {
            return;
        }
        // Get the last loaded index
        let last_load: usize = match self.loaded_paths.back() {
            Some(v) => *v,
            None => {
                return;
            }
        };
        // Get the next load path if possible.
        let new_load_path = match self.valid_paths.get(last_load + 1) {
            Some(p) => p,
            None => {
                return;
            }
        };
        // Load the paper
        let paper = match parse_paper_toml(new_load_path) {
            Some(p) => p,
            None => {
                return;
            }
        };
        // Keep the load size by unloading the first elements and
        // adding the new elements
        self.loaded_paths.pop_front();
        self.papers.pop_front();
        self.loaded_paths.push_back(last_load + 1);
        self.papers.push_back(paper);
    }
    /// Load the previous paper, if there is a previous paper to load.
    pub fn load_previous(&mut self) {
        // Check if there is even anything to load, if not, exit.
        if self.valid_paths.is_empty() {
            return;
        }
        // Get the first loaded index
        let first_load: usize = match self.loaded_paths.front() {
            Some(v) => *v,
            None => {
                return;
            }
        };
        // Get the new load path if possible
        if first_load <= 0 {
            return;
        }
        let new_load_path = match self.valid_paths.get(first_load - 1) {
            Some(p) => p,
            None => {
                return;
            }
        };
        // Load the new paper
        let paper = match parse_paper_toml(new_load_path) {
            Some(p) => p,
            None => {
                return;
            }
        };
        // Check if there is even anything to load, if not, exit.
        if self.valid_paths.is_empty() {
            return;
        }
        // Keep the load size by unloading the last entries and adding
        // the new elements to the front.
        self.loaded_paths.pop_back();
        self.papers.pop_back();
        self.loaded_paths.push_front(first_load - 1);
        self.papers.push_front(paper);
    }
    /// Given an index in the `Loader.papers` vector, copy the
    /// contents of the `Paper.bibtex` field to the system clipboard.
    /// The method assumes `wl-clipboard` is installed and uses the
    /// `wl_clipboard_rs` crate.
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
            Ok(_) => {}
            Err(_) => {
                return;
            }
        }
        match std::process::Command::new("notify-send")
            .arg("Bibtex copied")
            .status()
        {
            Ok(_) => {
                return;
            }
            Err(_) => {
                return;
            }
        }
    }
    /// Open the paper pointed at by `selected_idx` in Neovim.
    /// Note that this function assumes `kitty` and `nvim` are
    /// installed.
    pub fn open_file_in_editor(&self, selected_idx: usize) {
        // Get the file path pointer
        let fp_pointer = match self.loaded_paths.get(selected_idx) {
            Some(i) => *i,
            None => {
                return;
            }
        };
        // Get the file path
        let file_path = match self.valid_paths.get(fp_pointer) {
            Some(p) => p.clone(),
            None => {
                return;
            }
        };
        // Open the file in Neovim
        match std::process::Command::new("kitty")
            .arg("--detach")
            .arg("nvim")
            .arg(file_path)
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
    /// Open the PDF file stored in the currently selected paper
    /// struct. The `pdf_dir` tells us in which directory to look
    /// for the file name of the PDF and `pdf_viewer` tells us which
    /// PDF viewer to use in opening the PDF file.
    pub fn open_file_in_pdfviewer(
        &self,
        selected_idx: usize,
        pdf_viewer: &String,
        pdf_dir: &String,
    ) {
        // Get the document name of the currently selected paper.
        let file_name = match self.papers.get(selected_idx) {
            Some(p) => p.docname.clone(),
            None => {
                return;
            }
        };
        // Create path to that file using the given `pdf_dir`
        let mut file_path = std::path::PathBuf::from(pdf_dir);
        file_path.push(file_name);
        // Check that the file exists, then open it in the provided
        // `pdf_viewer`.
        if file_path.exists() {
            match std::process::Command::new(pdf_viewer)
                .arg(file_path)
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
    }
    /// Remove the currently selected file, and remove it from the
    /// loader
    ///
    /// Note: Currently it is not removed from the `valid_paths` field,
    /// this can cause some weird behaviour in some instances. We
    /// will add this later, however, this requires some further
    /// attention as changes to the `loaded_paths` field will also need
    /// to be changed.
    pub fn remove_file(&mut self, selected_idx: usize) {
        // Get the file path pointer
        let fp_pointer = match self.loaded_paths.get(selected_idx) {
            Some(i) => *i,
            None => {
                return;
            }
        };
        // Get the file path
        let file_path = match self.valid_paths.get(fp_pointer) {
            Some(p) => p.clone(),
            None => {
                return;
            }
        };
        // Check if the file exists
        if file_path.exists() {
            // Delete the file
            std::fs::remove_file(file_path).expect("Error attempting to remove file");
            // Remove it from the loader
            self.loaded_paths.remove(selected_idx);
            self.papers.remove(selected_idx);
        }
    }
}
