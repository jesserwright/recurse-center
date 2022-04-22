use std::io::{stdin, stdout, Write};
use termion::{
    clear, cursor,
    cursor::Goto,
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
    style,
};

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
        write!(f, "{}", character)
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::Empty
    }
}

struct Game {
    turn: Player,
    board: [Player; 9],
}

impl Default for Game {
    fn default() -> Self {
        Self {
            turn: Player::X,
            board: [
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ],
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdin = stdin().events();
    let mut stdout = stdout().into_raw_mode()?; //this raw mode is doing something different
    let mut game = Game::default();
    game.board[0] = Player::X;
    game.turn = Player::O;

    stdout.write(clear::All.to_string().as_bytes())?;
    stdout.flush()?;
    stdout.write(Goto(1, 1).to_string().as_bytes())?;
    stdout.flush()?;
    // find the indexes for the positions and wrap around them in sequence as a matrix
    let buf = format!(
        "      ╭─╮\r
Turn: │{}│\r
      ╰─╯\r
            \r
 {} │ {} │ {}\r
───┼───┼───\r
 {} │ {} │ {}\r
───┼───┼───\r
 {} │ {} │ {}\r
",
        game.turn,
        game.board[0],
        game.board[1],
        game.board[2],
        game.board[3],
        game.board[4],
        game.board[5],
        game.board[6],
        game.board[7],
        game.board[8],
    );
    stdout.write(buf.as_bytes())?;
    stdout.flush()?;
    let (mut x, mut y) = (2, 1);
    while let Some(Ok(ev)) = stdin.next() {
        match ev {
            Event::Key(Key::Down) => {
                y = y + 1;
                write!(stdout, "{}", termion::cursor::Goto(x, y))?;
            }
            Event::Key(Key::Up) => {
                y = y - 1;
                write!(stdout, "{}", termion::cursor::Goto(x, y))?;
            }
            Event::Key(Key::Right) => {
                x = x + 1;
                write!(stdout, "{}", termion::cursor::Goto(x, y))?;
            }
            Event::Key(Key::Left) => {
                x = x - 1;
                write!(stdout, "{}", termion::cursor::Goto(x, y))?;
            }
            Event::Key(Key::Ctrl('c')) => {
                write!(stdout, "\r\nctrl-c")?;
            }
            Event::Key(Key::Char('q')) => return Ok(()),
            _ => {
                continue;
            }
        }
        stdout.flush()?;
    }
    // listen for KBD events and look for arrow keys and save the cursor state and re-print on move
    // on enter key, place the move and re
    write!(
        stdout,
        "{}{}{}",
        clear::All,
        style::Reset,
        cursor::Goto(1, 1)
    )?;
    return Ok(());
}
