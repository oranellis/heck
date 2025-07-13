use std::cmp::min;

use rand::Rng;

#[derive(Debug, Clone)]
pub(crate) struct GameState {
    pub(crate) game_board: Vec<Vec<bool>>,
    pub(crate) cursor_x: usize,
    pub(crate) cursor_y: usize,
    pub(crate) board_width: usize,
    pub(crate) board_height: usize,
}

pub(crate) enum MoveCommand {
    Right,
    Down,
    Left,
    Up,
}

impl GameState {
    pub(crate) fn new(width: usize, height: usize) -> GameState {
        GameState {
            // [row[column]]
            game_board: vec![vec![false; width]; height],
            board_height: height,
            board_width: width,
            cursor_x: width / 2,
            cursor_y: height / 2,
        }
    }

    pub(crate) fn move_cursor(&mut self, move_command: MoveCommand) {
        match move_command {
            MoveCommand::Up => self.cursor_y = self.cursor_y.saturating_sub(1),
            MoveCommand::Down => self.cursor_y = min(self.board_height - 1, self.cursor_y + 1),
            MoveCommand::Left => self.cursor_x = self.cursor_x.saturating_sub(1),
            MoveCommand::Right => self.cursor_x = min(self.board_height - 1, self.cursor_x + 1),
        }
    }

    pub(crate) fn flip_at_position(&mut self, x_pos: usize, y_pos: usize) {
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

    pub(crate) fn random_flips(&mut self, count: usize) {
        let mut count_index = count;
        while count_index > 0 {
            let random_y = rand::rng().random_range(0..self.board_height);
            let random_x = rand::rng().random_range(0..self.board_width);
            self.flip_at_position(random_x, random_y);
            count_index -= 1;
        }
    }
}
