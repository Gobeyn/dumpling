use super::ui_wrapper;
use crate::configuration::config::Config;
use crate::file::loader::Loader;
use crate::key::event;

/// Main method for the program. It activates an alternate screen on top
/// of the current terminal session with `crossterm` and subsequently
/// render the TUI with `ratatui`. All the key binded actions are also
/// called here.
pub fn create_window(file_load: &mut Loader, config: &Config) {
    // Enable raw mode, disabling user input like typing
    match crossterm::terminal::enable_raw_mode() {
        Ok(_) => {}
        Err(err) => {
            log::error!("Error enabling raw mode: {err}");
            std::process::exit(1);
        }
    }

    // Create alternate screen on top of current terminal session
    match crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen) {
        Ok(_) => {}
        Err(err) => {
            log::error!("Error opening alternate screen on top of current terminal: {err}");
            std::process::exit(1);
        }
    }
    // Define ratatui terminal interface with crossterm
    let mut terminal =
        match ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout())) {
            Ok(t) => t,
            Err(err) => {
                log::error!("Error creating `ratatui` terminal interfaced with `crossterm`: {err}");
                std::process::exit(1);
            }
        };

    // Define UI drawing loop
    let mut run = true;
    let mut file_pointer: usize = 0;
    while run {
        let ui = ui_wrapper::ui_pre_args(file_load, config, file_pointer);
        match terminal.draw(ui) {
            Ok(_) => {}
            Err(err) => {
                log::error!("Error displaying TUI: {err}");
                std::process::exit(1);
            }
        }
        let key_event: event::KeyEvents = event::get_key_event(config);
        match key_event {
            event::KeyEvents::Quit => {
                run = false;
            }
            event::KeyEvents::Next => {
                file_pointer = file_load.load_next(file_pointer);
            }
            event::KeyEvents::Previous => {
                file_pointer = file_load.load_previous(file_pointer);
            }
            event::KeyEvents::Bibtex => {
                file_load.bibtex_entry_to_clipboard(file_pointer);
            }
            event::KeyEvents::Edit => {
                file_load.open_file_in_editor(file_pointer, &config.general.editor_command);
            }
            event::KeyEvents::Delete => {
                file_load.remove_file(file_pointer);
            }
            event::KeyEvents::Open => {
                file_load.open_file_in_pdfviewer(
                    file_pointer,
                    &config.general.pdf_viewer,
                    &config.general.pdf_dir,
                );
            }
            _ => {}
        }
    }

    // Disable raw mode so we return to normal terminal function
    match crossterm::terminal::disable_raw_mode() {
        Ok(_) => {}
        Err(err) => {
            log::error!("Error disabling raw mode: {err}");
            std::process::exit(1);
        }
    }
    // Leave alternate screen and return to original
    match crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen) {
        Ok(_) => {}
        Err(err) => {
            log::error!("Error leaving alternate terminal screen and returning to original: {err}");
            std::process::exit(1);
        }
    }
}
