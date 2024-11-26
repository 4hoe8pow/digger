use color_eyre::{eyre::Ok, Result};
use dapplication::input_ports::terminal_input_port::TerminalInputPort;
use ddomain::value_objects::app_mode::AppMode;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    DefaultTerminal,
};

pub struct TerminalController<T: TerminalInputPort> {
    mode: AppMode,
    input_port: T,
}

impl<T: TerminalInputPort> TerminalController<T> {
    pub fn new(input_port: T) -> Self {
        TerminalController {
            mode: AppMode::Normal,
            input_port,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            let _ = terminal.draw(|frame| match self.mode {
                AppMode::Normal => {
                    let _ = self.input_port.mode_normal(frame);
                }
                AppMode::Inquery => {
                    let _ = self.input_port.mode_inquery(frame);
                }
                AppMode::Amend => {
                    let _ = self.input_port.mode_amend(frame);
                }
                AppMode::Raise => {
                    let _ = self.input_port.mode_raise(frame);
                }
            });

            if self.handle_event(event::read()?)? {
                break;
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<bool> {
        let Event::Key(key) = event else {
            return Ok(false);
        };
        if key.kind != KeyEventKind::Press {
            return Ok(false);
        }

        match (&self.mode, key.code) {
            (_, KeyCode::Char('q')) if !matches!(self.mode, AppMode::Amend | AppMode::Raise) => {
                return Ok(true)
            }

            // 通常時
            (AppMode::Normal, KeyCode::Char('l')) => self.mode = AppMode::Inquery,
            (AppMode::Normal, KeyCode::Char('+')) => self.mode = AppMode::Raise,
            (AppMode::Normal, KeyCode::Char('j') | KeyCode::Down) => self.input_port.next_row()?,
            (AppMode::Normal, KeyCode::Char('k') | KeyCode::Up) => {
                self.input_port.previous_row()?
            }

            // 照会
            (AppMode::Inquery, KeyCode::Char('l')) => self.mode = AppMode::Amend,
            (AppMode::Inquery, KeyCode::Char('h') | KeyCode::Left) => self.mode = AppMode::Normal,
            (AppMode::Inquery, KeyCode::Char('j') | KeyCode::Down) => self.input_port.next_row()?,
            (AppMode::Inquery, KeyCode::Char('k') | KeyCode::Up) => {
                self.input_port.previous_row()?
            }

            // 訂正
            (AppMode::Amend, KeyCode::Esc) => self.mode = AppMode::Inquery,
            (AppMode::Amend, _) => self.input_port.handle_input(key.code)?,

            // 起票
            (AppMode::Raise, KeyCode::Esc) => self.mode = AppMode::Normal,
            (AppMode::Raise, _) => self.input_port.handle_input(key.code)?,

            // その他のキー入力
            _ => {}
        }

        Ok(false)
    }
}
