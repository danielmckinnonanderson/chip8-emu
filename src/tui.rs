use std::{
    error::Error,
    io::{stdout, Stdout},
    time::Duration,
    ops::ControlFlow,
};
use ratatui::{prelude::*, widgets::{*, block::Title}};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode}
};

use crate::emu_display::{Chip8Display, self};


pub fn make_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

pub fn draw_frame(
    frame: &mut Frame,
    emu_display: &Chip8Display
) -> () {
    let (title_area, layout) = calculate_layout(frame.size());

    render_title(frame, title_area);
    render_display_buffer(frame, emu_display, *layout.first().unwrap().first().unwrap());
}

/// Calculate the layout of the UI elements.
///
/// Returns a tuple of the title area and the main areas.
fn calculate_layout(area: Rect) -> (Rect, Vec<Vec<Rect>>) {
    let main_layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);

    let block_layout = Layout::vertical([Constraint::Max(4); 10]);

    let [title_area, main_area] = main_layout.areas(area);
    let main_areas = block_layout
        .split(main_area)
        .iter()
        .map(|&area| {
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area)
                .to_vec()
        })
        .collect();
    (title_area, main_areas)
}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Chip-8 Emulator")
            .light_blue()
            .alignment(Alignment::Center),
        area,
    );
}

fn render_display_buffer(frame: &mut Frame, emu_display: &Chip8Display, area: Rect) -> () {
    // TODO - Could probably reuse this between frames
    let block = Block::new()
        .borders(Borders::ALL)
        .title("Display");

    let content = Paragraph::new(format!("{}", emu_display))
     .block(Block::new().title("Paragraph").borders(Borders::ALL))
     .style(Style::new().white().on_black())
     .alignment(Alignment::Center)
     .wrap(Wrap { trim: true });

    frame.render_widget(content.block(block), area);
}

fn render_borders(paragraph: &Paragraph, border: Borders, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(border)
        .title(format!("Borders::{border:#?}", border = border));
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_border_type(
    paragraph: &Paragraph,
    border_type: BorderType,
    frame: &mut Frame,
    area: Rect,
) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(border_type)
        .title(format!("BorderType::{border_type:#?}"));
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_styled_borders(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_style(Style::new().blue().on_white().bold().italic())
        .title("Styled borders");
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_styled_block(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .style(Style::new().blue().on_white().bold().italic())
        .title("Styled block");
    frame.render_widget(paragraph.clone().block(block), area);
}

// Note: this currently renders incorrectly, see https://github.com/ratatui-org/ratatui/issues/349
fn render_styled_title(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .title("Styled title")
        .title_style(Style::new().blue().on_white().bold().italic());
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_styled_title_content(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let title = Line::from(vec![
        "Styled ".blue().on_white().bold().italic(),
        "title content".red().on_white().bold().italic(),
    ]);
    let block = Block::new().borders(Borders::ALL).title(title);
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_multiple_titles(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .title("Multiple".blue().on_white().bold().italic())
        .title("Titles".red().on_white().bold().italic());
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_multiple_title_positions(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .title(
            Title::from("top left")
                .position(block::Position::Top)
                .alignment(Alignment::Left),
        )
        .title(
            Title::from("top center")
                .position(block::Position::Top)
                .alignment(Alignment::Center),
        )
        .title(
            Title::from("top right")
                .position(block::Position::Top)
                .alignment(Alignment::Right),
        )
        .title(
            Title::from("bottom left")
                .position(block::Position::Bottom)
                .alignment(Alignment::Left),
        )
        .title(
            Title::from("bottom center")
                .position(block::Position::Bottom)
                .alignment(Alignment::Center),
        )
        .title(
            Title::from("bottom right")
                .position(block::Position::Bottom)
                .alignment(Alignment::Right),
        );
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_padding(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .title("Padding")
        .padding(Padding::new(5, 10, 1, 2));
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_nested_blocks(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let outer_block = Block::new().borders(Borders::ALL).title("Outer block");
    let inner_block = Block::new().borders(Borders::ALL).title("Inner block");
    let inner = outer_block.inner(area);
    frame.render_widget(outer_block, area);
    frame.render_widget(paragraph.clone().block(inner_block), inner);
}
