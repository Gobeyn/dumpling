pub enum KeyEvents {
    NoEvent,
    Quit,
}

pub fn get_key_event() -> KeyEvents {
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
                    if key.code == crossterm::event::KeyCode::Char('q') {
                        return KeyEvents::Quit;
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
