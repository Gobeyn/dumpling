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
    crossterm::terminal::enable_raw_mode().expect("Error enabling raw mode.");
    // Create alternate screen on top of current terminal session
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)
        .expect("Error changing to alternate screen.");
    // Define ratatui terminal interface with crossterm
    let mut terminal =
        ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout()))
            .expect("Error creating Ratatui terminal interface with crossterm");

    // Define UI drawing loop
    let mut run = true;
    let mut file_pointer: usize = 0;
    while run {
        let ui = ui_wrapper::ui_pre_args(file_load, config, file_pointer);
        terminal.draw(ui).expect("Error when rendering the TUI");
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
                file_load.open_file_in_editor(file_pointer);
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
    crossterm::terminal::disable_raw_mode().expect("Error disabling raw mode");
    // Leave alternate screen and return to original
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)
        .expect("Error leaving alternate screen");
}
