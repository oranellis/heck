use std::io::{stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::EnableMouseCapture,
    style::{Print, PrintStyledContent, ResetColor, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
    ExecutableCommand, QueueableCommand,
};

use crate::game_state::GameState;

pub(crate) fn start_display() -> std::io::Result<()> {
    stdout()
        .queue(SetTitle("Heck"))?
        .queue(EnterAlternateScreen)?
        .queue(ResetColor)?
        .queue(EnableMouseCapture)?
        .queue(Clear(crossterm::terminal::ClearType::All))?
        .queue(Hide)?
        .flush()?;
    enable_raw_mode()?;
    Ok(())
}

pub(crate) fn stop_display() -> std::io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}

impl GameState {
    pub(crate) fn draw(&mut self) -> std::io::Result<()> {
        stdout().queue(MoveTo(0, 0))?; //.queue(Clear(ClearType::All))?;

        for (y_pos, row) in self.game_board.iter().enumerate() {
            for (x_pos, element) in row.iter().enumerate() {
                let character = if x_pos == self.cursor_x && y_pos == self.cursor_y {
                    '░'
                } else {
                    '█'
                };

                if *element {
                    stdout().queue(PrintStyledContent(character.yellow().bold()))?;
                } else {
                    stdout().queue(PrintStyledContent(character.dark_grey()))?;
                }
            }
            stdout().queue(Print("\n\r"))?;
        }
        stdout().flush()?;
        Ok(())
    }
}
