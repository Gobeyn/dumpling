use crate::configuration::config::Config;

use crate::file::loader::Loader;
use ratatui::prelude::*;

/// Render the tags block using the contents of the loaded paper
/// pointed at by `selected_idx`.
pub fn render(file_load: &Loader, config: &Config, selected_idx: usize) -> Vec<Line<'static>> {
    // Initialise rendered text
    let mut render_text: Vec<Line> = Vec::new();

    // Get author and year from selected paper
    let tags = match file_load.papers.get(selected_idx) {
        Some(p) => {
            let mut tag_txt = String::new();
            for tag in &p.tags {
                tag_txt.push_str(&tag.label);
                tag_txt.push_str(" | ");
            }
            tag_txt
        }
        None => "Error retrieving info".to_string(),
    };

    render_text.push(Line::from(Span::styled(
        tags,
        Style::default().fg(config.colors.tag_content),
    )));
    return render_text;
}
