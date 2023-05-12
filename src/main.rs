use std::io::stdout;
use std::{thread, time};

use crossterm::{
    execute, Result,
    style::Print,
    cursor::{MoveTo, SavePosition, MoveToNextLine},
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
}

struct Dvd {
    pos: Position,
    logo: [String; 7],
}

impl Dvd {
    pub fn new(pos: Position, logo: [String; 7]) -> Dvd {
        Dvd {pos,logo}
    }

    pub fn printer(&self) -> Result<()> {
        execute!(
            stdout(),
            Clear(ClearType::All),
            MoveTo(self.pos.x,self.pos.y),
            )?;

        for line in &self.logo {
            execute!(
                stdout(),
                SavePosition,
                Print(line.to_string()),
                MoveToNextLine(0),
                )?;
        }
        Ok(())
    }

    pub fn change_position(&mut self, x_loops: u8) -> Result<()> {
        for x_change in 0..x_loops {
            let new_position = Position {x: self.pos.x + x_change as u16, y: self.pos.y};
            self.printer()?;
            self.pos.x = new_position.x;
            wait_ms(500);
        }
        Ok(())
    }
}

fn wait_ms(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}
     
 fn main() -> Result<()> {
     let dvd_logo: [String; 7] = [
             "⠀⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣶⣦⡀".to_string(),
             "⠀⢠⣿⣿⡿⠀⠀⠈⢹⣿⣿⡿⣿⣿⣇⠀⣠⣿⣿⠟⣽⣿⣿⠇⠀⠀⢹⣿⣿⣿".to_string(),
             "⠀⢸⣿⣿⡇⠀⢀⣠⣾⣿⡿⠃⢹⣿⣿⣶⣿⡿⠋⢰⣿⣿⡿⠀⠀⣠⣼⣿⣿⠏".to_string(),
             "⠀⣿⣿⣿⣿⣿⣿⠿⠟⠋⠁⠀⠀⢿⣿⣿⠏⠀⠀⢸⣿⣿⣿⣿⣿⡿⠟⠋⠁⠀".to_string(),
             "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣸⣟⣁⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀".to_string(),
             "⣠⣴⣶⣾⣿⣿⣻⡟⣻⣿⢻⣿⡟⣛⢻⣿⡟⣛⣿⡿⣛⣛⢻⣿⣿⣶⣦⣄⡀⠀".to_string(),
             "⠉⠛⠻⠿⠿⠿⠷⣼⣿⣿⣼⣿⣧⣭⣼⣿⣧⣭⣿⣿⣬⡭⠾⠿⠿⠿⠛⠉".to_string()
         ];
     
     let position = Position {x: 0, y: 0};

     let dvd = Dvd::new(position,dvd_logo);
 
     dvd.printer().unwrap();

    // change_position(10, &mut position, &dvd_logo)?;

     Ok(())
 }
