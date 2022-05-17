// What is this about for me? It is about computers and UI.
// What is line buffering

use std::io::{stdin, stdout, Stdout, Write};
use termion::{
    clear,
    cursor::Goto,
    cursor::{self, DetectCursorPos},
    event::{Event, Key},
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

type AppError = Box<dyn std::error::Error>;

fn main() -> Result<(), AppError> {
    Game::init()?.run()
}

#[derive(Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
    Empty,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let character = match self {
            Player::O => "◯",
            Player::X => "⨉",
            Player::Empty => " ",
        };
        f.write_str(character)
    }
}

struct Game {
    turn: Player,
    board: [Player; 9],
    stdout: RawTerminal<Stdout>,
}

const START_X: u16 = 2;
const START_Y: u16 = 5;

impl Game {
    fn init() -> Result<Self, AppError> {
        let mut stdout = stdout().into_raw_mode()?;
        write!(stdout, "{}", clear::All)?;
        stdout.write(cursor::BlinkingUnderline.to_string().as_bytes())?;

        Ok(Self {
            stdout,
            turn: Player::X,
            board: [Player::Empty; 9],
        })
    }

    fn run(&mut self) -> Result<(), AppError> {
        self.board[1] = Player::X;
        self.render()?;

        // numeric wrapping on iteration would be nice... .next() instead of increment and decrement

        while let Some(Ok(Event::Key(key))) = stdin().events().next() {
            let mut board_index = self.cursor_to_index()?;
            // Placing and skipping could be separate?
            // placing a turn should progress to the next empty spot automatically
            // the cursor could blink with the character that will be placed "under" it? and remove turn indicator
            // left side padding by one character
            match key {
                Key::Right => {
                    loop {
                        if board_index < self.board.len() - 1 {
                            let next_index = board_index + 1;
                            if self.board[next_index] == Player::Empty {
                                let (x, y) = Self::index_to_cursor(next_index)?;
                                write!(self.stdout, "{}", termion::cursor::Goto(x, y))?;
                                break;
                            } else {
                                board_index = board_index + 1;
                            }
                        } else {
                            let (x, y) = Self::index_to_cursor(0)?;
                            write!(self.stdout, "{}", termion::cursor::Goto(x, y))?;
                            break;
                        }
                    }
                }
                Key::Left if board_index > 0 => {
                    let (x, y) = Self::index_to_cursor(board_index - 1)?;
                    write!(self.stdout, "{}", termion::cursor::Goto(x, y))?;
                }
                // overflow
                Key::Left => {
                    let (x, y) = Self::index_to_cursor(self.board.len() - 1)?;
                    write!(self.stdout, "{}", termion::cursor::Goto(x, y))?;
                }
                Key::Char('\n') => {
                    // Map cursor position to board position
                    let board_index = self.cursor_to_index()?;
                    if self.board[board_index] == Player::Empty {
                        self.board[board_index] = self.turn;
                        // Write the turn char
                        write!(self.stdout, "{}", self.turn)?;
                        // Toggle the player
                        match self.turn {
                            Player::X => self.turn = Player::O,
                            Player::O => self.turn = Player::X,
                            _ => {}
                        }
                        let (x, y) = self.stdout.cursor_pos()?;
                        write!(
                            self.stdout,
                            "{}{}{}",
                            termion::cursor::Goto(x - 1, y),
                            self.turn,
                            termion::cursor::Goto(x - 1, y)
                        )?;
                    }
                }
                Key::Char('q') => {
                    break;
                }
                _ => {
                    continue;
                }
             }
            self.stdout.flush()?;
        }
        Ok(())
    }

    fn index_to_cursor(board_index: usize) -> Result<(u16, u16), AppError> {
        let cursor_position = match board_index {
            0 => (2, 5),
            1 => (6, 5),
            2 => (10, 5),
            3 => (2, 7),
            4 => (6, 7),
            5 => (10, 7),
            6 => (2, 9),
            7 => (6, 9),
            8 => (10, 9),
            _ => panic!("Invalid index"),
        };
        Ok(cursor_position)
    }

    fn cursor_to_index(&mut self) -> Result<usize, AppError> {
        let board_index = match self.stdout.cursor_pos()? {
            (2, 5) => 0,
            (6, 5) => 1,
            (10, 5) => 2,
            (2, 7) => 3,
            (6, 7) => 4,
            (10, 7) => 5,
            (2, 9) => 6,
            (6, 9) => 7,
            (10, 9) => 8,
            _ => panic!("Invalid cursor position"),
        };
        Ok(board_index)
    }

    fn render(&mut self) -> Result<(), AppError> {
        self.stdout.write(Goto(1, 1).to_string().as_bytes())?;
        self.render_board()?;
        self.stdout
            .write(Goto(START_X, START_Y).to_string().as_bytes())?;
        self.stdout.flush()?;
        Ok(())
    }

    fn render_board(&mut self) -> Result<(), AppError> {
        // todo: the turn line won't be needed
        let board = format!(
            "      ╭─╮\r
Turn: │{}│\r
      ╰─╯\r
            \r
 {} │ {} │ {}\r
───┼───┼───\r
 {} │ {} │ {}\r
───┼───┼───\r
 {} │ {} │ {}\r
\r
",
            self.turn,
            self.board[0],
            self.board[1],
            self.board[2],
            self.board[3],
            self.board[4],
            self.board[5],
            self.board[6],
            self.board[7],
            self.board[8],
        );
        self.stdout.write(board.as_bytes())?;
        Ok(())
    }
}

// Reset stdout
impl Drop for Game {
    fn drop(&mut self) {
        // write!(
        //     self.stdout,
        //     "{}{}{}{}",
        //     clear::All,
        //     style::Reset,
        //     cursor::Goto(1, 1),
        //     cursor::BlinkingBlock
        // )
        // .unwrap();
    }
}

#[test]
fn my_thing() {
    assert!("asdf" == "asdf");
}
