pub mod args;
pub mod configuration;
pub mod file;
pub mod key;
pub mod ui;

use args::parser::{parse_arguments, ProgArgs};
use configuration::config::Config;
use file::loader::Loader;
use file::parser::write_new_paper;
use ui::window::create_window;

fn main() {
    // Create cache directory if it doesn't exist.
    let mut folderdir = dirs::cache_dir().expect("Error obtaining $HOME/.cache/");
    folderdir.push("dumpling");
    if !folderdir.exists() {
        std::fs::create_dir(folderdir.clone()).expect("Error creating $HOME/.cache/dumpling/");
    }

    // Get configuration file
    let mut config_path = dirs::config_dir().expect("Error obtaining $HOME/.config/");
    config_path.push("dumpling");
    config_path.push("dumpling.toml");
    let config = Config::from_config_file(&config_path);

    // Get program arguments
    let prog_args: ProgArgs = parse_arguments();

    // Load first section of existing papers.
    let mut file_load: Loader = Loader::load_tomls(1, config.general.load_size, &mut folderdir);

    if prog_args.flags.open {
        create_window(&mut file_load, &config, &folderdir);
    } else {
        if prog_args.flags.list_tags {
            println!("This has not been implemented yet.");
        } else {
            // If we get here, it is assumed a new entry is added.
            let paper = prog_args.to_paper();
            write_new_paper(&paper, &folderdir);
        }
    }
}
