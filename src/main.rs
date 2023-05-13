use std::io::stdout;
use std::{thread, time};
use std::process::exit;

use crossterm::{
    execute, Result,
    style::Print,
    cursor::{MoveTo, SavePosition, MoveDown, MoveToNextLine, RestorePosition, Hide, Show},
    terminal::{Clear,ClearType},
};

struct Position {
    x: u16,
    y: u16,
}

impl Position {
    fn new(x: u16, y: u16) -> Position {
        Position {x,y}
    }

    fn change(&mut self, x: i16, y: i16) {
        self.x = self.x.wrapping_add_signed(x);
        self.y = self.y.wrapping_add_signed(y);
    }
}

struct Dvd {
    pos: Position,
    logo: Vec<String>,
}

impl Dvd {
    pub fn new(pos: Position, logo: Vec<String>) -> Dvd {
        Dvd {pos,logo}
    }

    pub fn print(&self) -> Result<()> {
        execute!(
            stdout(),
            Clear(ClearType::All),
            Hide,
            MoveTo(self.pos.x,self.pos.y),
            SavePosition,
            )?;

        for line in &self.logo {
            execute!(
                stdout(),
                SavePosition,
                Print(line.to_string()),
                RestorePosition,
                MoveDown(0),
                SavePosition,
                )?;
        }
        Ok(())
    }

    pub fn change_position(&mut self, x_loops: u8) -> Result<()> {
        for _i in 0..x_loops {
            self.pos.change(1,1);
            self.print()?;
            wait_ms(500);
        }
        Ok(())
    }

    pub fn restore_cursor(&self) {
        execute!(
            stdout(),
            Show,
            MoveToNextLine(0),
            ).unwrap_or_else(|err| {
            println!("Problem restoring cursor: {err}");
            exit(1);
        });
    }
}

fn wait_ms(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}

fn main() {
    let dvd_logo: Vec<String> = vec![
        "⠀⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣶⣦⡀".to_string(),
        "⠀⢠⣿⣿⡿⠀⠀⠈⢹⣿⣿⡿⣿⣿⣇⠀⣠⣿⣿⠟⣽⣿⣿⠇⠀⠀⢹⣿⣿⣿".to_string(),
        "⠀⢸⣿⣿⡇⠀⢀⣠⣾⣿⡿⠃⢹⣿⣿⣶⣿⡿⠋⢰⣿⣿⡿⠀⠀⣠⣼⣿⣿⠏".to_string(),
        "⠀⣿⣿⣿⣿⣿⣿⠿⠟⠋⠁⠀⠀⢿⣿⣿⠏⠀⠀⢸⣿⣿⣿⣿⣿⡿⠟⠋⠁⠀".to_string(),
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣸⣟⣁⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀".to_string(),
        "⣠⣴⣶⣾⣿⣿⣻⡟⣻⣿⢻⣿⡟⣛⢻⣿⡟⣛⣿⡿⣛⣛⢻⣿⣿⣶⣦⣄⡀⠀".to_string(),
        "⠉⠛⠻⠿⠿⠿⠷⣼⣿⣿⣼⣿⣧⣭⣼⣿⣧⣭⣿⣿⣬⡭⠾⠿⠿⠿⠛⠉".to_string()
    ];
    let position = Position::new(0,0);

    let mut dvd = Dvd::new(position,dvd_logo);

    if let Err(e) = dvd.change_position(5) {
        println!("Application error: {e}");
        exit(1);
    }

    dvd.restore_cursor();
}
