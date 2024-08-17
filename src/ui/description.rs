use crate::configuration::config::Config;
use crate::file::loader::Loader;
use ratatui::prelude::*;

/// Render the description block using the contents of the loaded paper
/// pointed at by `selected_idx`.
pub fn render(file_load: &Loader, config: &Config, selected_idx: usize) -> Vec<Line<'static>> {
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
        Style::default().fg(config.colors.description_content),
    )));

    return render_text;
}
