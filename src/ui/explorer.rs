use crate::configuration::config::Config;
use crate::file::loader::Loader;
use ratatui::prelude::*;

/// Render the explorer side of the TUI. The title of all the loaded
/// papers is displayed. The title of the paper pointed at by
/// `selected_idx` is prepended by a selection icon as determined
/// by the configuration file.
pub fn render(
    file_load: &Loader,
    config: &Config,
    selected_idx: usize,
    explorer_area: Rect,
) -> Vec<Line<'static>> {
    // Initialise the text
    let mut render_text: Vec<Line> = Vec::new();

    // Enumerate through the papers
    for (i, paper) in file_load.papers.iter().enumerate() {
        let mut line = String::new();
        let mut style = Style::default()
            .fg(config.colors.explorer_unselected_fg)
            .bg(config.colors.explorer_unselected_bg);

        if i == selected_idx {
            let selection_icon = config.general.selection_icon.clone();
            line.push_str(&selection_icon);
            style = Style::default()
                .fg(config.colors.explorer_selected_fg)
                .bg(config.colors.explorer_selected_bg)
                .add_modifier(Modifier::BOLD);
        } else {
            let file_icon = config.general.file_icon.clone();
            line.push_str(&file_icon);
        }
        line.push_str(&paper.title);
        let line = truncate_string(line, explorer_area);
        render_text.push(Line::from(Span::styled(line, style)));
    }
    return render_text;
}

fn truncate_string(str: String, area: Rect) -> String {
    if str.chars().count() <= area.width as usize {
        return str;
    } else {
        let ellipsis = "…";
        let truncated_length = area.width as usize - ellipsis.len();
        let truncated: String = str.chars().take(truncated_length).collect();
        return format!("{}{}", truncated, ellipsis);
    }
}
