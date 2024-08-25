use super::author;
use super::description;
use super::explorer;
use super::tags;
use super::title;
use super::window::{AppState, PopupState};
use crate::configuration::config::Config;
use crate::file::loader::Loader;
use ratatui::{prelude::*, widgets::*};

/// Given all the needed program information e.g. the loaded papers,
/// the configuration file and the currently pointed at loaded paper
/// index, return a UI function that only takes in a `Frame` to be
/// used for rendering.
pub fn ui_pre_args<'a>(
    file_load: &'a Loader,
    config: &'a Config,
    app_state: &'a AppState,
    selected_idx: usize,
) -> Box<dyn Fn(&mut Frame) + 'a> {
    Box::new(move |frame: &mut Frame| {
        let master_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(30), Constraint::Percentage(70)],
        )
        .split(frame.size());
        let explorer_rect = master_layout[0].clone();

        // Paper explorer UI
        let explorer_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(85), Constraint::Percentage(15)],
        )
        .margin(2)
        .split(master_layout[0]);

        let explorer_master_block = Block::new()
            .title(" Paper Explorer ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(config.colors.master_block_title)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(config.colors.master_block_border));

        let explorer_block = Block::new()
            .title(" Titles ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(config.colors.content_block_title)
                    .add_modifier(Modifier::ITALIC),
            )
            .border_type(BorderType::Plain)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(config.colors.content_block_border));

        let explorer_render = explorer::render(file_load, config, selected_idx);
        let explorer_paragraph = Paragraph::new(explorer_render)
            .block(explorer_block)
            .alignment(Alignment::Left);

        let tag_block = Block::new()
            .title(" Tags ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(config.colors.content_block_title)
                    .add_modifier(Modifier::ITALIC),
            )
            .border_type(BorderType::Plain)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(config.colors.content_block_border));
        let tag_render = tags::render(file_load, config, selected_idx);
        let tag_paragraph = Paragraph::new(tag_render)
            .block(tag_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        // Paper content viewer UI
        let content_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(60),
            ],
        )
        .margin(2)
        .split(master_layout[1]);

        let content_block = Block::new()
            .title(" Content ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(config.colors.master_block_title)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(config.colors.master_block_border));

        let title_block = Block::new()
            .title(" Title ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(config.colors.content_block_title)
                    .add_modifier(Modifier::ITALIC),
            )
            .border_type(BorderType::Plain)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(config.colors.content_block_border));
        let title_render = title::render(&file_load, &config, selected_idx);
        let title_paragraph = Paragraph::new(title_render)
            .block(title_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let author_block = Block::new()
            .title(" Authors, year & journal ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(config.colors.content_block_title)
                    .add_modifier(Modifier::ITALIC),
            )
            .border_type(BorderType::Plain)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(config.colors.content_block_border));
        let author_render = author::render(&file_load, &config, selected_idx);
        let author_paragraph = Paragraph::new(author_render)
            .block(author_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        let desc_block = Block::new()
            .title(" Description ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(config.colors.content_block_title)
                    .add_modifier(Modifier::ITALIC),
            )
            .border_type(BorderType::Plain)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(config.colors.content_block_border));
        let desc_render = description::render(&file_load, &config, selected_idx);
        let desc_paragraph = Paragraph::new(desc_render)
            .block(desc_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(explorer_master_block, master_layout[0]);
        frame.render_widget(explorer_paragraph, explorer_layout[0]);
        frame.render_widget(tag_paragraph, explorer_layout[1]);
        frame.render_widget(content_block, master_layout[1]);
        frame.render_widget(title_paragraph, content_layout[0]);
        frame.render_widget(author_paragraph, content_layout[1]);
        frame.render_widget(desc_paragraph, content_layout[2]);

        if let PopupState::ConfirmDelete = app_state.popup_state {
            let block = Block::new()
                .title(" Confirm delete (y/n) ")
                .title_alignment(Alignment::Left)
                .title_style(Style::default().fg(config.colors.popup_block_title))
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(config.colors.popup_block_border));
            let popup_area = get_popup_rect(selected_idx, explorer_rect);
            let popup_text = pad_string_back(
                app_state.popup_core.input.clone(),
                popup_area.width as usize,
            );
            let popup_line = Line::from(Span::styled(
                popup_text,
                Style::default().fg(config.colors.popup_text),
            ));
            let popup_par = Paragraph::new(popup_line)
                .block(block)
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            frame.render_widget(popup_par, popup_area)
        }
    })
}

/// Define the location of the pop-up window.
pub fn get_popup_rect(selected_idx: usize, explorer_rect: Rect) -> Rect {
    // Use the width of the explorer window to set the x location of the rectangle.
    let x: u16 = explorer_rect.width;
    // Set the y location of the rectangle at the current selected file, for this we need
    // to pad with +2 due to margin, title, etc.
    let y: u16 = selected_idx as u16 + 2;
    // Set the width of the pop-up box equal to 75% the width of the explorer.
    let width: u16 = 3 * explorer_rect.width / 4;
    // Set the height to 3, which is the minimum we need for a simply y/n box.
    let height: u16 = 3;
    return Rect::new(x, y, width, height);
}
/// Pad the back of a string with non-breaking space character. This allows us
/// to remove the background from underlying widgets which are visible even when
/// padding with normal spaces.
pub fn pad_string_back(text: String, padding: usize) -> String {
    format!("{:\u{00A0}<padding$}", text)
}
