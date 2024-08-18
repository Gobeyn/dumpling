pub mod args;
pub mod configuration;
pub mod file;
pub mod key;
pub mod listing;
pub mod ui;

use args::parser::{parse_arguments, ProgArgs};
use configuration::config::Config;
use file::loader::Loader;
use file::parser::write_new_paper;
use listing::tags::list_tags;
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

    if prog_args.flags.open {
        // Get the tag filter, if there is one. If the tag filter is "", use None.
        let tag_filter = {
            if prog_args.filter_by_tag.is_empty() {
                None
            } else {
                Some(prog_args.filter_by_tag.clone())
            }
        };
        // Load first section of existing papers.
        let mut file_load: Loader = Loader::load(config.general.load_size, &folderdir, &tag_filter);
        create_window(&mut file_load, &config);
    } else {
        if prog_args.flags.list_tags {
            list_tags(&folderdir);
        } else {
            // If we get here, it is assumed a new entry is added. We will only add it if
            // any program arguments were set.
            match prog_args.to_paper() {
                Some(p) => {
                    write_new_paper(&p, &folderdir);
                }
                None => {
                    return;
                }
            };
        }
    }
}
