use::std::process;

use std::io::stdout;
use std::{thread, time};

use crossterm::{
    execute, Result,
    style::Print,
    cursor::{MoveTo, SavePosition, MoveDown, MoveToNextLine, RestorePosition, Hide, Show},
    terminal::{size,Clear,ClearType},
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

pub struct Terminal {
    size: (u16, u16),
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {size: (size().unwrap())}
    }
}

pub struct BoundingBox {
    top: u16,
    left: u16,
    bottom: u16,
    right: u16,
}



pub struct Graphic {
    graphic: Vec<String>,
    edges: BoundingBox,
}

impl Graphic {
    pub fn new(graphic: Vec<String>) -> Graphic {
        let mut longest_line_length = 0;

        for line in graphic.iter() {
            let length = line.chars().count() as u16;
            if length > longest_line_length {
                longest_line_length = length;
            }
        }
        let top = 0;
        let left = 0;
        let bottom = graphic.len() as u16;
        let right = longest_line_length;
        let edges = BoundingBox {top, left, bottom, right};

        Graphic {graphic, edges}
    }
}

pub struct Dvd {
    pos: Position,
    logo: Graphic,
}

impl Dvd {
    pub fn new(logo: Graphic) -> Dvd {
    let pos = Position::new(0,0);
        Dvd {pos, logo}
    }

    pub fn print(&self) -> Result<()> {
        execute!(
            stdout(),
            Clear(ClearType::All),
            Hide,
            MoveTo(self.pos.x,self.pos.y),
            SavePosition,
            )?;

        for line in &self.logo.graphic {
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

    pub fn move_and_print(&mut self, x_loops: u8) -> Result<()> {
        for _i in 0..x_loops {
            self.pos.change(1,1);
            self.print()?;
            wait_ms(500);
        }

        self.restore_cursor();
        Ok(())
    }

    fn restore_cursor(&self) {
        execute!(
            stdout(),
            Show,
            MoveToNextLine(0),
            ).unwrap_or_else(|err| {
            println!("Problem restoring cursor: {err}");
            process::exit(1);
        });
    }
}

pub fn wait_ms(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}
