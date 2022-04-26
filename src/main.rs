use std::io::{stdin, stdout, Stdout, Write};
use termion::{
    clear, cursor,
    cursor::Goto,
    event::{Event, Key},
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    style,
};

#[derive(Copy, Clone)]
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
    cursor: usize,
    board: [Player; 9],
    x: u16,
    y: u16,
}
impl Game {
    fn place_turn(&mut self) {
        self.board[self.cursor] = self.turn;
        self.toggle_player();
    }
    fn incr_x(&mut self) {
        self.x = self.x + 1;
    }
    fn decr_x(&mut self) {
        self.x = self.x - 1;
    }
    fn incr_y(&mut self) {
        self.y = self.y + 1;
    }
    fn decr_y(&mut self) {
        self.y = self.y - 1;
    }

    fn toggle_player(&mut self) {
        match self.turn {
            Player::X => self.turn = Player::O,
            Player::O => self.turn = Player::X,
            Player::Empty => {}
        }
    }
    fn render(&self, stdout: &mut RawTerminal<Stdout>) -> Result<(), Box<dyn std::error::Error>> {
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
        stdout.write(buf.as_bytes())?;
        stdout.flush()?;
        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            turn: Player::X,
            cursor: 0,
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
            x: 1,
            y: 1,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdin = stdin().events();
    let mut stdout = stdout().into_raw_mode()?;
    let mut game = Game::default();
    game.turn = Player::O;

    stdout.write(clear::All.to_string().as_bytes())?;
    stdout.flush()?;
    stdout.write(Goto(1, 1).to_string().as_bytes())?;
    stdout.flush()?;

    game.render(&mut stdout)?;

    // A map needs to be made between the canvas and the board cursor (0-8)
    // A true map is not needed if the is a mathmatical (constant) relation between them

    while let Some(Ok(ev)) = stdin.next() {
        match ev {
            Event::Key(Key::Down) => {}
            Event::Key(Key::Up) => {
                write!(stdout, "{}", termion::cursor::Goto(*x, *y))?;
            }
            Event::Key(Key::Right) => {
                write!(stdout, "{}", termion::cursor::Goto(*x, *y))?;
            }
            Event::Key(Key::Left) => {
                write!(stdout, "{}", termion::cursor::Goto(*x, *y))?;
            }
            Event::Key(Key::Char('\n')) => {
                game.place_turn();
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

fn write_position(game: &Game, stdout: &RawTerminal<Stdout>) -> Result<(), std::io::Error> {
    write!(stdout, "{}", termion::cursor::Goto(game.x, game.y))?;
    Ok(())
}
