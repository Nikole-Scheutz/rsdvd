use std::io::stdout;
use std::{thread, time};

use crossterm::{
    execute, Result,
    style::{Print, Color, SetForegroundColor},
    cursor::{MoveTo, SavePosition, MoveDown, MoveToNextLine, RestorePosition, Hide, Show},
    terminal::{size, Clear, ClearType},
};

struct Terminal {
    size: (i32, i32),
}

impl Terminal {
    pub fn new() -> Terminal {
        let size = size().expect("Could not get size of terminal");
        let size = (size.0 as i32, size.1 as i32);
        Terminal {size}
    }

    pub fn update_size(&mut self) {
        let size = size().expect("Could not get size of terminal");
        let size = (size.0 as i32, size.1 as i32);
        self.size = size;
    }

}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        if x < 0 || y < 0 {
            panic!("Negative values for cursor position are not allowed!");
        }
        Position {x,y}
    }

    pub fn set(&mut self, pos: (i32, i32)) {
        if pos.0 < 0 || pos.1 < 0 {
            panic!("Negative values for cursor position are not allowed!");
        }
        self.x = pos.0;
        self.y = pos.1;
    }
}

struct BoundingBox {
    bottom: i32,
    right: i32,
}

struct Direction {
    x: i32,
    y: i32,
}

pub struct ColorPalette {
    color_palette: Vec<Color>,
    current_color: u8,
}

impl ColorPalette {
    pub fn new(input_colors: Vec<Color>) -> ColorPalette {
        ColorPalette {color_palette: input_colors, current_color: 0}
    }

    pub fn next_color(&mut self) -> Color {
        if self.current_color == self.color_palette.len() as u8 - 1 {
            self.current_color = 0;
        } else { self.current_color += 1 as u8; };

        self.color_palette[self.current_color as usize]
    }
}

pub struct Graphic {
    graphic: Vec<String>,
    edges: BoundingBox,
    pos: Position,
    color_palette: ColorPalette,
    direction: Direction,
    terminal: Terminal,
}

impl Graphic {
    pub fn new(graphic: Vec<String>, color_palette: ColorPalette) -> Graphic {
        let mut longest_line_length = 0;

        for line in graphic.iter() {
            let length = line.chars().count() as i32;
            if length > longest_line_length {
                longest_line_length = length;
            }
        }
        let bottom = graphic.len() as i32;
        let right = longest_line_length;
        let edges = BoundingBox {bottom, right};

        let pos = Position::new(0,0);

        let direction = Direction {x: 1, y: 1};
        let terminal = Terminal::new();
        Graphic {graphic, pos, edges, direction, terminal, color_palette}
    }

    pub fn print_looper(&mut self, iterations: i32) -> Result<()> {
        match iterations {
            0 => {
                self.color_cursor()?;
                loop {
                    self.terminal.update_size();
                    self.print_loopable()?;
                }
            },
            _ => {
                self.color_cursor()?;
                for _i in 0..iterations {
                    self.terminal.update_size();
                    self.print_loopable()?;
                }
                self.restore_cursor()?;
                Ok(())
            }
        }
    }

    fn print(&self) -> Result<()> {
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

    fn print_loopable(&mut self) -> Result<()> {
        self.check_bounce()?;
        self.move_cursor();
        self.print()?;
        wait_ms(100);

        Ok(())
    }

    fn move_cursor(&mut self) {
        self.pos.x += self.direction.x as i32;
        self.pos.y += self.direction.y as i32; 
    }

    fn color_cursor(&mut self) -> Result<()> {
        let color = self.color_palette.next_color();
        execute!(
            stdout(),
            SetForegroundColor(color),
            )?;
        Ok(())
    }

    fn restore_cursor(&self) -> Result<()> {
        execute!(
            stdout(),
            Clear(ClearType::All),
            Show,
            MoveToNextLine(0),
            )?;

        execute!(
            stdout(),
            MoveTo(0,0),
            SavePosition,
            Clear(ClearType::All),
            SavePosition,
            )?;
        Ok(())
    }

    fn check_bounce(&mut self) -> Result<()> {
        if self.pos.x + self.edges.right > self.terminal.size.0 - 2 {
            self.direction.x = -self.direction.x;
            self.color_cursor()?;
        }

        if self.pos.x + self.direction.x < 0 {
            self.direction.x = -self.direction.x;
            self.color_cursor()?;
        }

        if self.pos.y + self.direction.y > self.terminal.size.1 - self.edges.bottom {
            self.direction.y = -self.direction.y;
            self.color_cursor()?;
        }

        if self.pos.y + self.direction.y <= - 1 {
            self.direction.y = -self.direction.y;
            self.color_cursor()?;
        }

        Ok(())
    }
}

pub fn wait_ms(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}

