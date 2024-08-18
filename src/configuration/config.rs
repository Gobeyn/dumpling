use ratatui::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Read;

/// Summary of the `[general]` section of the configuration file.
///
/// Any field not specified by the configuration takes a default as
/// specified by the Default implementation.
///
/// By default `pdf_dir` is set to $HOME/.paper/, this can be changed
/// in the configuration file, however, a direct path will need to be
/// provided as the program cannot interpret the $HOME and ~ aliases.
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

/// Summary of the [colors] section of the configuration file.
///
/// Any field not specified by the configuration takes a default as
/// specified by the Default implementation.
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
    pub tag_content: Vec<u8>,
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
            tag_content: vec![255, 255, 255],
        }
    }
}

/// To use the colors provided by `ColorsFromFile`, they need to be
/// converted to `ratatui::style::Color`. The result of that conversions
/// is summarized into this struct.
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
    pub tag_content: Color,
}

/// Macro that takes an instance of `ColorFromFile`, the `TuiColors`
/// type and all the field names. It repeatedly puts the RGB contents
/// stored in the Vec<u8> type into the `Color::Rgb` function to
/// generate a `ratatui::style::Color`. It does so for all the field
/// names provided. This way it generates a new instance of `TuiColors`
/// which is returned at the end.
macro_rules! convert_colors {
    // Take ColorFromFile instance, TuiColors type and all the
    // field names as input. Repeat the same conversion operation
    // on all the given fields. Create a new TuiColors struct
    // by doing so.
    ($src:ident, $dst:ident, $( $field:ident ),+) => {
        $dst {
            $(
                $field: Color::Rgb(
                    $src.$field[0],
                    $src.$field[1],
                    $src.$field[2],
                ),
            )+
        }
    };
}

impl TuiColors {
    /// Conversion from `ColorsFromFile` to `TuiColors` since the
    /// program needs `ratatui::style::Color` for rendering the TUI.
    /// The conversion uses the `convert_colors` macro.
    pub fn from_config_file_colors(cff: &ColorsFromFile) -> Self {
        let tui_colors = convert_colors!(
            cff,
            TuiColors,
            master_block_title,
            master_block_border,
            explorer_unselected_fg,
            explorer_unselected_bg,
            explorer_selected_fg,
            explorer_selected_bg,
            content_block_title,
            content_block_border,
            title_content,
            author_content,
            description_content,
            tag_content
        );
        return tui_colors;
    }
}

/// Summary of the [keybinds] section of the configuration file.
///
/// Any field not specified by the configuration takes a default as
/// specified by the Default implementation.
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

/// Representation of the configuration Toml file as a Rust struct.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
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

/// Identical to `ConfigFromFile`, but with `ColorsFromFile` replaced
/// by the `TuiColors` which the program needs for rendering.
#[derive(Debug, Clone)]
pub struct Config {
    pub general: GeneralFromFile,
    pub colors: TuiColors,
    pub keybinds: KeybindsFromFile,
}

impl Config {
    /// Given file path to configuration file, create `Config` struct.
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

/// Given file path to configuration file, parse the file contents
/// into the `ConfigFromFile` Rust struct.
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
