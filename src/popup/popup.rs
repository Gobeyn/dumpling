use crate::ui::window::AppState;
use crossterm::event::KeyCode;

/// Define the internal information of a pop-up window.
pub struct PopupCore {
    pub input: String,
    pub char_index: usize,
    pub entered_message: String,
}

impl Default for PopupCore {
    /// Implement default values for [`PopupCore`], everything is either empty or zero.
    fn default() -> Self {
        Self {
            input: String::new(),
            char_index: 0,
            entered_message: String::new(),
        }
    }
}

impl PopupCore {
    /// Create a new [`PopupCore`] instance.
    pub fn new(input: String, char_index: usize, entered_message: String) -> Self {
        Self {
            input,
            char_index,
            entered_message,
        }
    }
    /// Clamp the cursor position between the start of the text and the last typed character.
    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        // Clamp cursor to the start of the text and the last typed thing.
        new_cursor_pos.clamp(0, self.input.chars().count())
    }
    /// Set the cursor back to the beginning of the text.
    pub fn reset_cursor(&mut self) {
        self.char_index = 0;
    }
    /// Move the cursor to the left.
    pub fn move_cursor_left(&mut self) {
        // Make sure we don't overflow with saturated sub
        let cursor_moved_left = self.char_index.saturating_sub(1);
        // Make sure we can't move the cursor farther than the beginning
        self.char_index = self.clamp_cursor(cursor_moved_left);
    }
    /// Move the cursor to the right.
    pub fn move_cursor_right(&mut self) {
        // Make sure we don't overflow with saturated add
        let cursor_moved_right = self.char_index.saturating_add(1);
        // Make sure we can't move cursor farther than the last typed thing.
        self.char_index = self.clamp_cursor(cursor_moved_right);
    }
    /// Get the byte index for appending a new character. This requires some work because Rust
    /// Strings.
    pub fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.char_index)
            .unwrap_or(self.input.len())
    }
    /// Insert a new character into the [`PopupCore`] input field.
    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }
    /// Remove the last character of the [`PopupCore`] input field.
    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.char_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.char_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    /// Set the [`entered_message`] field of the [`PopupCore`] structure to the current [`input`].
    pub fn submit_message(&mut self) {
        self.entered_message = self.input.clone();
        self.input.clear();
        self.reset_cursor();
    }
}

/// Handle key events when the pop-up window is open, such as moving the cursor, typing and
/// deleting text, entering the text or closing the pop-up.
pub fn handle_key_events(app_state: &mut AppState) {
    match crossterm::event::poll(std::time::Duration::from_millis(50)) {
        Ok(_) => {
            let event_read = match crossterm::event::read() {
                Ok(event) => event,
                Err(err) => {
                    log::warn!("Error reading events, `NoEvent` is returned: {err}");
                    return;
                }
            };

            if let crossterm::event::Event::Key(key) = event_read {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Enter => app_state.popup_core.submit_message(),
                        KeyCode::Char(to_insert) => app_state.popup_core.enter_char(to_insert),
                        KeyCode::Backspace => app_state.popup_core.delete_char(),
                        KeyCode::Left => app_state.popup_core.move_cursor_left(),
                        KeyCode::Right => app_state.popup_core.move_cursor_right(),
                        KeyCode::Esc => app_state.set_default(),
                        _ => {}
                    }
                }
            }
        }
        Err(err) => {
            log::warn!("Error polling key events: {err}");
            return;
        }
    }
}
