use crate::file::loader::Loader;
use ratatui::prelude::*;

pub fn render(file_load: &Loader, selected_idx: usize) -> Vec<Line<'static>> {
    // Initialise the text
    let mut render_text: Vec<Line> = Vec::new();

    // Enumerate through the papers
    for (i, paper) in file_load.papers.iter().enumerate() {
        let mut line = String::new();
        let mut style = Style::default().fg(Color::Rgb(0, 0, 255));

        if i == selected_idx {
            line.push_str("    ");
            style = Style::default()
                .fg(Color::Rgb(0, 0, 255))
                .bg(Color::Rgb(48, 48, 48))
                .add_modifier(Modifier::ITALIC);
        }
        line.push_str(&paper.title);
        render_text.push(Line::from(Span::styled(line, style)));
    }
    return render_text;
}
