mod draw;
mod game_state;

use crossterm::event::{read, Event, KeyCode, MouseButton, MouseEventKind};
use draw::{start_display, stop_display};
use game_state::{GameState, MoveCommand};
use std::process::exit;

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
