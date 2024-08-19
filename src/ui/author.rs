use crate::configuration::config::Config;
use crate::file::loader::Loader;
use ratatui::prelude::*;

/// Render the author block using the contents of the loaded paper
/// pointed at by `selected_idx`.
pub fn render(file_load: &Loader, config: &Config, selected_idx: usize) -> Vec<Line<'static>> {
    // Initialise rendered text
    let mut render_text: Vec<Line> = Vec::new();

    // Get author and year from selected paper
    let (author, year, journal) = match file_load.papers.get(selected_idx) {
        Some(p) => {
            let mut auth_txt = String::new();
            for auth in &p.authors {
                auth_txt.push_str(&auth.name);
                auth_txt.push_str(" | ");
            }
            let year_txt = format!("Published year: {}", p.year);
            let journal_txt = format!("Published journal: {}", p.journal);
            (auth_txt, year_txt, journal_txt)
        }
        None => (
            "Error retrieving authors".to_string(),
            "Error retrieving year".to_string(),
            "Error retrieving journal".to_string(),
        ),
    };

    render_text.push(Line::from(Span::styled(
        author,
        Style::default().fg(config.colors.author_content),
    )));
    render_text.push(Line::from(Span::styled(
        year,
        Style::default().fg(config.colors.author_content),
    )));
    render_text.push(Line::from(Span::styled(
        journal,
        Style::default().fg(config.colors.author_content),
    )));

    return render_text;
}
