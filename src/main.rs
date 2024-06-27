mod tui;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('Q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(70),
                Constraint::Percentage(20),
            ])
            .split(area);

        let title = Title::from(Line::from("Dungeon").bold());
        let msg_title = Title::from(Line::from("Messages").bold());
        let player_info_title = Title::from(Line::from("Player").bold());

        let instructions = Title::from(Line::from(vec!["Quit ".into(), "<Q>".yellow().bold()]));

        let msg_block = Block::default()
            .title(msg_title.alignment(Alignment::Left))
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let player_info_title_block = Block::default()
            .title(player_info_title.alignment(Alignment::Left))
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let dungeon_block = Block::default()
            .title(title.alignment(Alignment::Left))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        Paragraph::new("Welcome to RustHack!")
            .centered()
            .block(msg_block)
            .render(layout[0], buf);
        Paragraph::new("....@....")
            .centered()
            .block(dungeon_block)
            .render(layout[1], buf);

        Paragraph::new("Name:Default  PW:0  AC:0")
            .centered()
            .block(player_info_title_block)
            .render(layout[2], buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
