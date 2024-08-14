use ratatui::{prelude::*, widgets::*};

pub fn test_ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello World!").block(Block::bordered().title("Greeting")),
        frame.size(),
    );
}
