use crate::configuration::config::Config;

/// Contains all the possible events that can happen while the TUI is
/// open.
pub enum KeyEvents {
    NoEvent,
    Next,
    Previous,
    Bibtex,
    Edit,
    Delete,
    Open,
    Quit,
}

/// Reads the user input, and if it matches with the key binds as
/// dictated by the configuration file, or by the Default implementation
/// of KeybindsFromFile.
pub fn get_key_event(config: &Config) -> KeyEvents {
    match crossterm::event::poll(std::time::Duration::from_millis(50)) {
        Ok(_) => {
            let event_read = match crossterm::event::read() {
                Ok(event) => event,
                Err(err) => {
                    log::warn!("Error reading events, `NoEvent` is returned: {err}");
                    return KeyEvents::NoEvent;
                }
            };

            if let crossterm::event::Event::Key(key) = event_read {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    if key.code == crossterm::event::KeyCode::Char(config.keybinds.quit) {
                        return KeyEvents::Quit;
                    } else if key.code == crossterm::event::KeyCode::Char(config.keybinds.next) {
                        return KeyEvents::Next;
                    } else if key.code == crossterm::event::KeyCode::Char(config.keybinds.previous)
                    {
                        return KeyEvents::Previous;
                    } else if key.code
                        == crossterm::event::KeyCode::Char(config.keybinds.bibtex_to_clipboard)
                    {
                        return KeyEvents::Bibtex;
                    } else if key.code == crossterm::event::KeyCode::Char(config.keybinds.edit) {
                        return KeyEvents::Edit;
                    } else if key.code
                        == crossterm::event::KeyCode::Char(config.keybinds.open_in_pdfviewer)
                    {
                        return KeyEvents::Open;
                    } else if key.code == crossterm::event::KeyCode::Char(config.keybinds.delete) {
                        return KeyEvents::Delete;
                    } else {
                        return KeyEvents::NoEvent;
                    }
                } else {
                    return KeyEvents::NoEvent;
                }
            } else {
                return KeyEvents::NoEvent;
            }
        }
        Err(err) => {
            log::warn!("Error polling event, `NoEvent` is returned: {err}");
            return KeyEvents::NoEvent;
        }
    };
}
