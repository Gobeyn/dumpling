use crate::configuration::config::Config;
use crate::file::loader::Loader;
use ratatui::prelude::*;

pub fn render(file_load: &Loader, config: &Config, selected_idx: usize) -> Vec<Line<'static>> {
    // Initialise rendered text
    let mut render_text: Vec<Line> = Vec::new();

    // Get title from selected file
    let title = match file_load.papers.get(selected_idx) {
        Some(p) => p.title.clone(),
        None => "Error retrieving info".to_string(),
    };

    // Add the title to the render
    render_text.push(Line::from(Span::styled(
        title,
        Style::default().fg(config.colors.title_content),
    )));

    return render_text;
}
