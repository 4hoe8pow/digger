use color_eyre::Result;
use ratatui::crossterm::event::KeyCode;
use ratatui::Frame;
pub trait TerminalInputPort {
    fn read_key(&self) -> Result<Option<KeyCode>>;
    fn mode_inquery(&mut self, frame: &mut Frame) -> Result<()>;
    fn mode_normal(&mut self, frame: &mut Frame) -> Result<()>;
    fn mode_amend(&mut self, frame: &mut Frame) -> Result<()>;
    fn mode_raise(&mut self, frame: &mut Frame) -> Result<()>;
    fn next_row(&mut self) -> Result<()>;
    fn previous_row(&mut self) -> Result<()>;

    fn handle_input(&mut self, key: KeyCode) -> Result<()>;
    fn submit(&mut self) -> Result<()>;
}
