use crate::configuration::config::Config;

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

pub fn get_key_event(config: &Config) -> KeyEvents {
    match crossterm::event::poll(std::time::Duration::from_millis(50)) {
        Ok(_) => {
            let event_read = match crossterm::event::read() {
                Ok(event) => event,
                Err(_) => {
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
        Err(_) => {
            return KeyEvents::NoEvent;
        }
    };
}
