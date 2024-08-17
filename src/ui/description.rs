use crate::file::loader::Loader;
use ratatui::prelude::*;

pub fn render(file_load: &Loader, selected_idx: usize) -> Vec<Line<'static>> {
    // Initialise rendered text
    let mut render_text: Vec<Line> = Vec::new();

    // Get description from selected file
    let desc = match file_load.papers.get(selected_idx) {
        Some(p) => p.description.clone(),
        None => "Error retrieving info".to_string(),
    };

    // Add the title to the render
    render_text.push(Line::from(Span::styled(
        desc,
        Style::default().fg(Color::Rgb(255, 255, 255)),
    )));

    return render_text;
}
