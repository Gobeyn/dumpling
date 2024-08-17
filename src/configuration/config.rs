use ratatui::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct GeneralFromFile {
    pub load_size: i32,
    pub pdf_viewer: String,
    pub pdf_dir: String, // Cannot contain $HOME or ~, must be a direct path
    pub selection_icon: String,
}

impl Default for GeneralFromFile {
    fn default() -> Self {
        let mut pdf_dir = dirs::home_dir().expect("Error obtaining $HOME directory");
        pdf_dir.push(".paper");
        GeneralFromFile {
            load_size: 30,
            pdf_viewer: String::from("zathura"),
            pdf_dir: String::from(pdf_dir.to_str().unwrap()),
            selection_icon: String::from(" "),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct ColorsFromFile {
    pub master_block_title: Vec<u8>,
    pub master_block_border: Vec<u8>,
    pub explorer_unselected_fg: Vec<u8>,
    pub explorer_unselected_bg: Vec<u8>,
    pub explorer_selected_fg: Vec<u8>,
    pub explorer_selected_bg: Vec<u8>,
    pub content_block_title: Vec<u8>,
    pub content_block_border: Vec<u8>,
    pub title_content: Vec<u8>,
    pub author_content: Vec<u8>,
    pub description_content: Vec<u8>,
}

impl Default for ColorsFromFile {
    fn default() -> Self {
        ColorsFromFile {
            master_block_title: vec![255, 255, 255],
            master_block_border: vec![255, 255, 255],
            explorer_unselected_fg: vec![0, 0, 255],
            explorer_unselected_bg: vec![0, 0, 0],
            explorer_selected_fg: vec![0, 0, 255],
            explorer_selected_bg: vec![48, 48, 48],
            content_block_title: vec![255, 255, 255],
            content_block_border: vec![255, 255, 255],
            title_content: vec![255, 255, 255],
            author_content: vec![255, 255, 255],
            description_content: vec![255, 255, 255],
        }
    }
}

#[derive(Clone, Debug)]
pub struct TuiColors {
    pub master_block_title: Color,
    pub master_block_border: Color,
    pub explorer_unselected_fg: Color,
    pub explorer_unselected_bg: Color,
    pub explorer_selected_fg: Color,
    pub explorer_selected_bg: Color,
    pub content_block_title: Color,
    pub content_block_border: Color,
    pub title_content: Color,
    pub author_content: Color,
    pub description_content: Color,
}

impl TuiColors {
    pub fn from_config_file_colors(cff: &ColorsFromFile) -> Self {
        TuiColors {
            master_block_title: Color::Rgb(
                cff.master_block_title[0],
                cff.master_block_title[1],
                cff.master_block_title[2],
            ),
            master_block_border: Color::Rgb(
                cff.master_block_border[0],
                cff.master_block_border[1],
                cff.master_block_border[2],
            ),
            explorer_unselected_fg: Color::Rgb(
                cff.explorer_unselected_fg[0],
                cff.explorer_unselected_fg[1],
                cff.explorer_unselected_fg[2],
            ),
            explorer_unselected_bg: Color::Rgb(
                cff.explorer_unselected_bg[0],
                cff.explorer_unselected_bg[1],
                cff.explorer_unselected_bg[2],
            ),
            explorer_selected_fg: Color::Rgb(
                cff.explorer_selected_fg[0],
                cff.explorer_selected_fg[1],
                cff.explorer_selected_fg[2],
            ),
            explorer_selected_bg: Color::Rgb(
                cff.explorer_selected_bg[0],
                cff.explorer_selected_bg[1],
                cff.explorer_selected_bg[2],
            ),
            content_block_title: Color::Rgb(
                cff.content_block_title[0],
                cff.content_block_title[1],
                cff.content_block_title[2],
            ),
            content_block_border: Color::Rgb(
                cff.content_block_border[0],
                cff.content_block_border[1],
                cff.content_block_border[2],
            ),
            title_content: Color::Rgb(
                cff.title_content[0],
                cff.title_content[1],
                cff.title_content[2],
            ),
            author_content: Color::Rgb(
                cff.author_content[0],
                cff.author_content[1],
                cff.author_content[2],
            ),
            description_content: Color::Rgb(
                cff.description_content[0],
                cff.description_content[1],
                cff.description_content[2],
            ),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct KeybindsFromFile {
    pub quit: char,
    pub next: char,
    pub previous: char,
    pub bibtex_to_clipboard: char,
    pub edit: char,
    pub delete: char,
    pub open_in_pdfviewer: char,
}

impl Default for KeybindsFromFile {
    fn default() -> Self {
        KeybindsFromFile {
            quit: 'q',
            next: 'j',
            previous: 'k',
            bibtex_to_clipboard: 'b',
            edit: 'e',
            delete: 'd',
            open_in_pdfviewer: 'o',
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConfigFromFile {
    pub general: GeneralFromFile,
    pub colors: ColorsFromFile,
    pub keybinds: KeybindsFromFile,
}

impl Default for ConfigFromFile {
    fn default() -> Self {
        ConfigFromFile {
            general: GeneralFromFile::default(),
            colors: ColorsFromFile::default(),
            keybinds: KeybindsFromFile::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub general: GeneralFromFile,
    pub colors: TuiColors,
    pub keybinds: KeybindsFromFile,
}

impl Config {
    pub fn from_config_file(filepath: &std::path::PathBuf) -> Self {
        let config_from_file = parse_config_file(filepath);
        let config = Config {
            general: config_from_file.general,
            colors: TuiColors::from_config_file_colors(&config_from_file.colors),
            keybinds: config_from_file.keybinds,
        };
        return config;
    }
}

pub fn parse_config_file(filepath: &std::path::PathBuf) -> ConfigFromFile {
    // Check if the config file exists
    if !filepath.exists() {
        return ConfigFromFile::default();
    }

    // Open the file
    let mut file = match std::fs::File::open(filepath) {
        Ok(v) => v,
        Err(_) => {
            return ConfigFromFile::default();
        }
    };
    // Read contents from the file
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(_) => {
            return ConfigFromFile::default();
        }
    };
    // Parse the file contents
    let parsed_toml: toml::Value = match toml::from_str(&contents) {
        Ok(v) => v,
        Err(_) => {
            return ConfigFromFile::default();
        }
    };
    // Convert config Toml to struct
    let config_struct: ConfigFromFile = match parsed_toml.try_into() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error converting to struct: {}", e);
            return ConfigFromFile::default();
        }
    };
    return config_struct;
}
