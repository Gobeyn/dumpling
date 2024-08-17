use crate::configuration::config::Config;
use crate::file::loader::Loader;
use ratatui::prelude::*;

pub fn render(file_load: &Loader, config: &Config, selected_idx: usize) -> Vec<Line<'static>> {
    // Initialise rendered text
    let mut render_text: Vec<Line> = Vec::new();

    // Get author and year from selected paper
    let (author, year) = match file_load.papers.get(selected_idx) {
        Some(p) => {
            let mut auth_txt = String::new();
            for auth in &p.authors {
                auth_txt.push_str(&auth.name);
            }
            let year_txt = format!("Published: {}", p.year);
            (auth_txt, year_txt)
        }
        None => (
            "Error retrieving info".to_string(),
            "Error retrieving info".to_string(),
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

    return render_text;
}
