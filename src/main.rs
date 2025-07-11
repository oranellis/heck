use std::{
    cmp::min,
    io::{stdout, Write},
    process::exit,
};

use rand::Rng;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind},
    style::{Print, PrintStyledContent, ResetColor, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
    ExecutableCommand, QueueableCommand,
};

#[derive(Debug, Clone)]
struct GameState {
    game_board: Vec<Vec<bool>>,
    cursor_x: usize,
    cursor_y: usize,
    board_width: usize,
    board_height: usize,
}

enum MoveCommand {
    Right,
    Down,
    Left,
    Up,
}

impl GameState {
    fn new(width: usize, height: usize) -> GameState {
        GameState {
            // [row[column]]
            game_board: vec![vec![false; width]; height],
            board_height: height,
            board_width: width,
            cursor_x: width / 2,
            cursor_y: height / 2,
        }
    }

    fn draw(&mut self) -> std::io::Result<()> {
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

    fn move_cursor(&mut self, move_command: MoveCommand) {
        match move_command {
            MoveCommand::Up => self.cursor_y = self.cursor_y.saturating_sub(1),
            MoveCommand::Down => self.cursor_y = min(self.board_height - 1, self.cursor_y + 1),
            MoveCommand::Left => self.cursor_x = self.cursor_x.saturating_sub(1),
            MoveCommand::Right => self.cursor_x = min(self.board_height - 1, self.cursor_x + 1),
        }
    }

    fn flip_at_position(&mut self, x_pos: usize, y_pos: usize) {
        let mut row: usize = y_pos.saturating_sub(1);
        while row <= min(self.board_height - 1, y_pos + 1) {
            let mut column: usize = x_pos.saturating_sub(1);
            while column <= min(self.board_width - 1, x_pos + 1) {
                self.game_board[row][column] = !self.game_board[row][column];
                column += 1;
            }
            row += 1;
        }
    }

    fn random_flips(&mut self, count: usize) {
        let mut count_index = count;
        while count_index > 0 {
            let random_y = rand::rng().random_range(0..self.board_height);
            let random_x = rand::rng().random_range(0..self.board_width);
            self.flip_at_position(random_x, random_y);
            count_index -= 1;
        }
    }
}

fn start_display() -> std::io::Result<()> {
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

fn stop_display() -> std::io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}

fn main() {
    // setup
    let width: usize = 10;
    let height: usize = 10;

    let mut game_state = GameState::new(width, height);

    start_display().unwrap();

    game_state.random_flips(10);

    // bobject to store game state

    loop {
        // main game loop

        // draw
        game_state.draw().unwrap();

        // event handler
        let event = read().unwrap();
        match event {
            Event::Key(key_event) => {
                if let KeyCode::Char(the_char) = key_event.code {
                    if the_char == 'q' {
                        stop_display().unwrap();
                        exit(0);
                    } else if the_char == 'h' {
                        game_state.move_cursor(MoveCommand::Left);
                    } else if the_char == 'j' {
                        game_state.move_cursor(MoveCommand::Down);
                    } else if the_char == 'k' {
                        game_state.move_cursor(MoveCommand::Up);
                    } else if the_char == 'l' {
                        game_state.move_cursor(MoveCommand::Right);
                    } else if the_char == ' ' {
                        game_state.flip_at_position(game_state.cursor_x, game_state.cursor_y);
                    }
                }
            }
            Event::Mouse(mouse_event) => {
                if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                    game_state.cursor_x = mouse_event.column as usize;
                    game_state.cursor_y = mouse_event.row as usize;
                    game_state.flip_at_position(game_state.cursor_x, game_state.cursor_y);
                }
            }
            _ => {}
        }

        let mut is_any_light_on: bool = false;
        'outer: for row in game_state.game_board.iter() {
            for bulb in row.iter() {
                if *bulb {
                    is_any_light_on = true;
                    break 'outer;
                }
            }
        }

        if !is_any_light_on {
            stop_display().unwrap();
            println!("Congratulations, you win!");
            exit(0);
        }
    }
}
