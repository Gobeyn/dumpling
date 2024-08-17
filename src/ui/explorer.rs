use crate::configuration::config::Config;
use crate::file::loader::Loader;
use ratatui::prelude::*;

pub fn render(file_load: &Loader, config: &Config, selected_idx: usize) -> Vec<Line<'static>> {
    // Initialise the text
    let mut render_text: Vec<Line> = Vec::new();

    // Enumerate through the papers
    for (i, paper) in file_load.papers.iter().enumerate() {
        let mut line = String::new();
        let mut style = Style::default()
            .fg(config.colors.explorer_unselected_fg)
            .bg(config.colors.explorer_unselected_bg);

        if i == selected_idx {
            let selection_icon = config.general.selection_icon.clone() + "  ";
            line.push_str(&selection_icon);
            style = Style::default()
                .fg(config.colors.explorer_selected_fg)
                .bg(config.colors.explorer_selected_bg)
                .add_modifier(Modifier::ITALIC);
        }
        line.push_str(&paper.title);
        render_text.push(Line::from(Span::styled(line, style)));
    }
    return render_text;
}
