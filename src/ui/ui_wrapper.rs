use super::author;
use super::description;
use super::explorer;
use super::title;
use crate::file::loader::Loader;
use ratatui::{prelude::*, widgets::*};

pub fn ui_pre_args<'a>(file_load: &'a Loader, selected_idx: usize) -> Box<dyn Fn(&mut Frame) + 'a> {
    Box::new(move |frame: &mut Frame| {
        let master_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(30), Constraint::Percentage(70)],
        )
        .split(frame.size());

        // Paper explorer UI
        let explorer_block = Block::new()
            .title(" Paper Explorer ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(Color::Rgb(255, 255, 255))
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 255, 255)));

        let explorer_render = explorer::render(file_load, selected_idx);
        let explorer_paragraph = Paragraph::new(explorer_render)
            .block(explorer_block)
            .alignment(Alignment::Left);

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
                    .fg(Color::Rgb(255, 255, 255))
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 255, 255)));

        let title_block = Block::new()
            .title(" Title ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(Color::Rgb(255, 255, 255))
                    .add_modifier(Modifier::ITALIC),
            )
            .border_type(BorderType::Plain)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 255, 255)));
        let title_render = title::render(&file_load, selected_idx);
        let title_paragraph = Paragraph::new(title_render)
            .block(title_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let author_block = Block::new()
            .title(" Authors ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(Color::Rgb(255, 255, 255))
                    .add_modifier(Modifier::ITALIC),
            )
            .border_type(BorderType::Plain)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 255, 255)));
        let author_render = author::render(&file_load, selected_idx);
        let author_paragraph = Paragraph::new(author_render)
            .block(author_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        let desc_block = Block::new()
            .title(" Description ")
            .title_alignment(Alignment::Center)
            .title_style(
                Style::default()
                    .fg(Color::Rgb(255, 255, 255))
                    .add_modifier(Modifier::ITALIC),
            )
            .border_type(BorderType::Plain)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 255, 255)));
        let desc_render = description::render(&file_load, selected_idx);
        let desc_paragraph = Paragraph::new(desc_render)
            .block(desc_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(explorer_paragraph, master_layout[0]);
        frame.render_widget(content_block, master_layout[1]);
        frame.render_widget(title_paragraph, content_layout[0]);
        frame.render_widget(author_paragraph, content_layout[1]);
        frame.render_widget(desc_paragraph, content_layout[2]);
    })
}
