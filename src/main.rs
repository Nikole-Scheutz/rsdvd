use std::io::stdout;

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
 
 
 fn dvd_printer(pos: Position, logo: [String; 7]) -> Result<()> {
     execute!(
         stdout(),
         Clear(ClearType::All),
         MoveTo(pos.x,pos.y),
         )?;

     for line in logo {
         execute!(
             stdout(),
             SavePosition,
             Print(line.to_string()),
             MoveToNextLine(0),
             )?;
     }
 
     Ok(())
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
 
     dvd_printer(position, dvd_logo).unwrap();
 
     Ok(())
 }
