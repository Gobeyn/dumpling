use super::parser::{parse_paper_toml, Paper};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::collections::VecDeque;

/// In the given `filedir`, look for all the files of the valid *.toml format and
/// store the path to them in a vector that is returned.
pub fn get_all_valid_filepaths(
    folderdir: &std::path::PathBuf,
    tag_filter: &Option<String>,
) -> Vec<std::path::PathBuf> {
    // Initialise vector
    let mut all_file_paths: Vec<std::path::PathBuf> = Vec::new();
    // Loop trough all files in the directory.
    let paths = match std::fs::read_dir(folderdir) {
        Ok(p) => p,
        Err(err) => {
            log::error!("Error obtaining paths to files in {folderdir:?}: {err}");
            std::process::exit(1);
        }
    };
    // Define the expected form of the files.
    let re = match regex::Regex::new(r"^*\.toml$") {
        Ok(r) => r,
        Err(err) => {
            log::error!("Error creating *.toml regex: {err}");
            std::process::exit(1);
        }
    };
    // Loop through all the paths, and add them to the vector
    for path in paths {
        let (file_name_os_string, file_path) = match path {
            Ok(p) => (p.file_name(), p.path()),
            Err(err) => {
                log::warn!("Error extracting `DirEntry` from a path, skipping this path: {err}");
                continue;
            }
        };
        let file_name = match file_name_os_string.to_str() {
            Some(s) => s,
            None => {
                log::error!("Error converting `OsString` to `&str`");
                std::process::exit(1);
            }
        };
        if re.is_match(&file_name) {
            match tag_filter {
                Some(tag) => {
                    // Parse the paper
                    let paper = match parse_paper_toml(&file_path) {
                        Some(p) => p,
                        None => {
                            log::warn!("Contents of {file_path:?} could not be deserialised into `Paper` struct, continuing to next paper to check tag of");
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
                    // If we get here, the load is probably larger than the amount of valid paths.
                    // We could break out of the loop here, but just to be safe we'll continue on
                    // to the next iteration.
                    continue;
                }
            };
            let paper = match parse_paper_toml(file_path) {
                Some(p) => p,
                None => {
                    log::warn!("Contents of {file_path:?} could not be deserialised into `Paper` struct, continuing to load next paper");
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
    pub fn load_next(&mut self, file_pointer: usize) -> usize {
        let mut fp: usize = file_pointer;
        // Check if there is even anything to load, if not, return a file pointer of zero.
        if self.valid_paths.is_empty() {
            return 0;
        }
        // Get the last loaded index
        let last_load: usize = match self.loaded_paths.back() {
            Some(v) => *v,
            None => {
                // There should be at least one element, the empty case was
                // handled before. So we should never get here. In case we do,
                // just return zero.
                log::warn!("Attempting to load non-existent back of `VecDeque`.");
                return 0;
            }
        };
        // Get the next load path if possible.
        let new_load_path = match self.valid_paths.get(last_load + 1) {
            Some(p) => p,
            None => {
                // If we were unable to load the next path, we are out of bounds meaning there is
                // nothing further to load and the file pointer needs to be updated.
                if file_pointer >= self.papers.len() - 1 {
                    fp = self.papers.len() - 1;
                } else {
                    fp += 1;
                }
                return fp;
            }
        };
        // Load the paper
        let paper = match parse_paper_toml(new_load_path) {
            Some(p) => p,
            None => {
                // If the program is being used correctly, we should never get here.
                log::error!("Error loading the next `Paper`.");
                std::process::exit(1);
            }
        };
        // Keep the load size by unloading the first elements and
        // adding the new elements
        self.loaded_paths.pop_front();
        self.papers.pop_front();
        self.loaded_paths.push_back(last_load + 1);
        self.papers.push_back(paper);
        // If we've gotten here, a new file has been loaded and the file pointer should remain as
        // is.
        return fp;
    }
    /// Load the previous paper, if there is a previous paper to load.
    pub fn load_previous(&mut self, file_pointer: usize) -> usize {
        let mut fp = file_pointer;
        // Check if there is even anything to load, if not, return a file pointer of zero.
        if self.valid_paths.is_empty() {
            return 0;
        }
        // Get the first loaded index
        let first_load: usize = match self.loaded_paths.front() {
            Some(v) => *v,
            None => {
                // There should be at least one element, the empty case was
                // handled before. So we should never get here. In case we do,
                // just return zero.
                log::warn!("Attempting to load non-existent front of `VecDeque`.");
                return 0;
            }
        };
        // If first_load is zero, then subtracting can cause an overflow. Which needs to be handled
        // before trying to get first_load - 1. This also means we are out of bounds for the
        // valid_paths array and hence there is nothing more previous to load. The file pointer
        // should be updated in this case.
        if first_load <= 0 {
            if file_pointer <= 0 {
                fp = 0;
            } else {
                fp -= 1;
            }
            return fp;
        }

        let new_load_path = match self.valid_paths.get(first_load - 1) {
            Some(p) => p,
            None => {
                // We should never get here because the usize overflow check above already handles
                // it.
                log::error!(
                    "Attempting to access outside the bounds of the `Loader.valid_paths` array."
                );
                std::process::exit(1);
            }
        };
        // Load the new paper
        let paper = match parse_paper_toml(new_load_path) {
            Some(p) => p,
            None => {
                // If the program is being used correctly, we should never get here.
                log::error!("Error loading the previous `Paper`.");
                std::process::exit(1);
            }
        };
        // Keep the load size by unloading the last entries and adding
        // the new elements to the front.
        self.loaded_paths.pop_back();
        self.papers.pop_back();
        self.loaded_paths.push_front(first_load - 1);
        self.papers.push_front(paper);
        // If we've gotten here, a new file has been loaded and the file pointer should remain as
        // is.
        return fp;
    }

    /// Given an index in the `Loader.papers` vector, copy the
    /// contents of the `Paper.bibtex` field to the system clipboard.
    /// The method uses the `cli-clipboard` crate, which should make
    /// this function on Linux (Wayland and X11), MacOS and Windows.
    pub fn bibtex_entry_to_clipboard(&self, selected_idx: usize) {
        // Get the entry
        let bibtex_entry = match self.papers.get(selected_idx) {
            Some(p) => p.bibtex.clone(),
            None => {
                log::warn!(
                    "Currently selected paper does not exist in the `Loader.papers` `VecDeque`. Stop copying bibtex contents to clipboard."
                );
                return;
            }
        };
        // Get clipboard context
        let mut ctx = match ClipboardContext::new() {
            Ok(c) => c,
            Err(err) => {
                log::warn!("Error obtaining clipboard context: {err}");
                return;
            }
        };
        // Set contents
        match ctx.set_contents(bibtex_entry) {
            Ok(_) => {}
            Err(err) => {
                log::warn!("Error setting clipboard contents: {err}");
            }
        }
        // Send notification
        match std::process::Command::new("notify-send")
            .arg("Bibtex copied")
            .status()
        {
            Ok(_) => {}
            Err(err) => {
                log::warn!("Error sending notification with `notify-send`: {err}");
            }
        }
    }
    /// Open the paper pointed at by `selected_idx` by prepending the given `command` to the file
    /// path. For example `command = "kitty --detach nvim"` will open a new kitty terminal, and
    /// open the selected file in Neovim. Another example, `command = "code"` will open the
    /// selected file in VS Code. This is of course assuming that the respective programs are
    /// installed on your system
    pub fn open_file_in_editor(&self, selected_idx: usize, command: &String) {
        // Get the file path pointer
        let fp_pointer = match self.loaded_paths.get(selected_idx) {
            Some(i) => *i,
            None => {
                log::warn!("Currently selected paper does not exists in the `Loader.loaded_paths` `VecDeque`. Stop opening editor.");
                return;
            }
        };
        // Get the file path
        let file_path = match self.valid_paths.get(fp_pointer) {
            Some(p) => p.clone(),
            None => {
                log::warn!("File pointer does not point to an existing element of the `Loader.valid_paths` vector. Stop opening editor.");
                return;
            }
        };
        // Separate the command into parts by separating by whitespace
        let mut command_parts = command.split_whitespace();
        let program = match command_parts.next() {
            Some(c) => c,
            None => {
                log::warn!(
                    "The `Config.general.editor_command` should not be empty. Stop opening editor"
                );
                return;
            }
        };
        let command_args: Vec<&str> = command_parts.collect();
        // Run the command
        match std::process::Command::new(program)
            .args(&command_args)
            .arg(file_path)
            .spawn()
        {
            Ok(_) => {}
            Err(err) => {
                log::warn!("Error executing command to open editor, check that your setting for `Config.general.editor_command` does what you think it does: {err}");
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
                log::warn!("Currently selected paper does not exist in the `Loader.papers` `VecDeque`. Stop opening PDF viewer.");
                return;
            }
        };
        // Create path to that file using the given `pdf_dir`
        let mut file_path = std::path::PathBuf::from(pdf_dir);
        file_path.push(file_name);
        // Expand $HOME or ~ aliases.
        file_path = expand_filepath(&file_path);
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
                Err(err) => {
                    log::warn!("Error executing command to open PDF viewer: {err}");
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
                log::warn!("Currently selected paper does not exist in the `Loader.loaded_paths` `VecDeque`. Stop removing file.");
                return;
            }
        };
        // Get the file path
        let file_path = match self.valid_paths.get(fp_pointer) {
            Some(p) => p.clone(),
            None => {
                log::warn!("File pointer does not point to an existing element of the `Loader.valid_paths` vector. Stop removing file.");
                return;
            }
        };
        // Check if the file exists
        if file_path.exists() {
            // Delete the file
            match std::fs::remove_file(file_path) {
                Ok(_) => {}
                Err(err) => {
                    log::warn!("Error attempting to remove file. Stop removing file: {err}");
                    return;
                }
            }
            // All the pointers in `loaded_paths` larger than the removed `fp_pointer` need to be
            // shifted down by one
            self.loaded_paths = self
                .loaded_paths
                .iter()
                .map(|&x| if x > fp_pointer { x - 1 } else { x })
                .collect();
            // Remove it from the loader
            self.valid_paths.remove(fp_pointer);
            self.loaded_paths.remove(selected_idx);
            self.papers.remove(selected_idx);
        }
    }
}

/// Compute the loader size based on the terminal size.
pub fn compute_loader_size() -> i32 {
    // First: -1.0 because of master block title
    // Second: -2.0*2.0 because margin(2) on top and bottom
    // Third: * 85.0 / 100.0 because explorer takes up 85% of the window
    // Fourth: -1.0 because of block title.
    // Fifth: -1.0 as a buffer so we are certainly not overloaded.
    match termsize::get() {
        Some(size) => {
            let tui_rows = ((size.rows as f32 - 1.0) - 2.0 * 2.0) * 85.0 / 100.0 - 1.0 - 1.0;
            let tui_rows = tui_rows.floor() as i32;
            return tui_rows;
        }
        None => {
            log::warn!("Could not obtain terminal size. Default value will be used.");
            return 20;
        }
    }
}

/// Given a file path that, check if the `$HOME` or `~` are used, and if so,
/// replace them with their actual value. The use of [`std::path::MAIN_SEPARATOR`] makes sure we
/// use the right path separator depending on whether the OS is UNIX based or Windows.
pub fn expand_filepath(path: &std::path::PathBuf) -> std::path::PathBuf {
    // Check if the `~` alias is used
    if path.starts_with("~") {
        match dirs::home_dir() {
            Some(home_dir) => {
                let path_str = path.to_string_lossy();
                return home_dir.join(
                    path_str
                        .trim_start_matches('~')
                        .trim_start_matches(std::path::MAIN_SEPARATOR),
                );
            }
            None => {
                log::warn!("Unable to obtain $HOME as `PathBuf`.");
                return path.to_path_buf();
            }
        }
    // Check if the "$HOME" alias is used
    } else if path.starts_with("$HOME") {
        match dirs::home_dir() {
            Some(home_dir) => {
                let path_str = path.to_string_lossy();
                return home_dir.join(
                    path_str
                        .trim_start_matches("$HOME")
                        .trim_start_matches(std::path::MAIN_SEPARATOR),
                );
            }
            None => {
                log::warn!("Unable to obtain $HOME as `PathBuf`.");
                return path.to_path_buf();
            }
        }
    } else {
        return path.to_path_buf();
    }
}
