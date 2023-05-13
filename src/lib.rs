use std::process;

use std::io::stdout;
use std::{thread, time};

use crossterm::{
    execute, Result,
    style::Print,
    cursor::{MoveTo, SavePosition, MoveDown, MoveToNextLine, RestorePosition, Hide, Show},
    terminal::{size,Clear,ClearType},
};

struct Terminal {
    size: (i32, i32),
}

impl Terminal {
    pub fn new() -> Terminal {
        let size = size().unwrap();
        let size = (size.0 as i32, size.1 as i32);
        Terminal {size}
    }
}

#[derive(PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        if x < 0 || y < 0 {
            panic!("Negative values for position!");
        }
        Position {x,y}
    }



    pub fn set(&mut self, pos: (i32, i32)) {
        self.x = pos.0;
        self.y = pos.1;
    }
}

struct BoundingBox {
    top: i32,
    left: i32,
    bottom: i32,
    right: i32,
}

#[derive(PartialEq)]
struct Direction {
    x: i32,
    y: i32,
}

pub struct Graphic {
    graphic: Vec<String>,
    edges: BoundingBox,
    pos: Position,
    direction: Direction,
    terminal: Terminal,
}

impl Graphic {
    pub fn new(graphic: Vec<String>) -> Graphic {
        let mut longest_line_length = 0;

        for line in graphic.iter() {
            let length = line.chars().count() as i32;
            if length > longest_line_length {
                longest_line_length = length;
            }
        }
        let top = 0;
        let left = 0;
        let bottom = graphic.len() as i32;
        let right = longest_line_length;
        let edges = BoundingBox {top, left, bottom, right};

        let pos = Position::new(0,0);
        
        let direction = Direction {x: 1, y: 1};
        let terminal = Terminal::new();
        Graphic {graphic, pos, edges, direction, terminal}
    }

    pub fn print(&self) -> Result<()> {
        execute!(
            stdout(),
            Clear(ClearType::All),
            Hide,
            MoveTo(self.pos.x as u16,self.pos.y as u16),
            SavePosition,
            )?;

        for line in &self.graphic {
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

    fn change(&mut self) {
        self.pos.x += self.direction.x as i32;
        self.pos.y += self.direction.y as i32; 
    }

    pub fn move_and_print(&mut self, x_loops: u8) -> Result<()> {
        for _i in 0..x_loops {
            self.check_bounce();
            self.change();
            self.print()?;
            wait_ms(50);
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

    fn check_bounce(&self) {
        if self.pos.x + self.edges.right >= self.terminal.size.0 {
            panic!("right edge hit border!");
        }

        if self.pos.x + self.direction.x <= 0 {
            panic!("left edge hit border!");
        }

        if self.pos.y + self.direction.y >= self.terminal.size.1 - self.edges.bottom {
            panic!("bottom edge hit border!");
        }

        if self.pos.y + self.direction.y <= 0 {
            panic!("top edge hit border!");
        }
    }
}

pub fn wait_ms(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}
