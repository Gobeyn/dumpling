use super::ui_wrapper;
use crate::file::loader::Loader;
use crate::key::event;

pub fn create_window(file_load: &mut Loader, filedir: &std::path::PathBuf) {
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
        let ui = ui_wrapper::ui_pre_args(file_load, file_pointer);
        terminal.draw(ui).expect("Error when rendering the TUI");
        let key_event: event::KeyEvents = event::get_key_event();
        match key_event {
            event::KeyEvents::Quit => {
                run = false;
            }
            event::KeyEvents::Next => {
                file_load.load_next(filedir);
                if file_pointer >= file_load.papers.len() - 1 {
                    file_pointer = file_load.papers.len() - 1;
                } else {
                    file_pointer += 1;
                }
            }
            event::KeyEvents::Previous => {
                file_load.load_previous(filedir);
                if file_pointer <= 1 {
                    file_pointer = 0;
                } else {
                    file_pointer -= 1;
                }
            }
            event::KeyEvents::Bibtex => {
                file_load.bibtex_entry_to_clipboard(file_pointer);
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
