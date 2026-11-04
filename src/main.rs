use std::io;

use crossterm::event::KeyModifiers;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct App {
    path: String,
    exit: bool,
}

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(frame.area());

        frame.render_widget(
            Paragraph::new("Left").block(Block::new().borders(Borders::ALL)),
            layout[0],
        );
        frame.render_widget(
            Paragraph::new("Right").block(Block::new().borders(Borders::ALL)),
            layout[1],
        );

        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('c') | KeyCode::Char('C'), KeyModifiers::CONTROL) => {
                        self.exit = true
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" r-expert ".bold() + " | ".into() + self.path.to_string().yellow());
        let instructions = Line::from(vec![" Quit ".into(), "<Ctrl+C> ".blue().bold()]);
        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        Paragraph::new("").block(block).render(area, buf);
    }
}
