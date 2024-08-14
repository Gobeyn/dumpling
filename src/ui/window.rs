use super::super::key::event;
use super::ui_wrapper;

pub fn create_window() {
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
    while run {
        terminal
            .draw(ui_wrapper::test_ui)
            .expect("Error when rendering the TUI");
        let key_event: event::KeyEvents = event::get_key_event();
        match key_event {
            event::KeyEvents::Quit => {
                run = false;
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
