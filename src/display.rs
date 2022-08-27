pub const HEIGHT: u8 = 32;
pub const WIDTH: u8 = 64;

pub struct Display {
    grid: [[bool; HEIGHT as usize]; WIDTH as usize]
}

impl Display {
    pub fn new() -> Display {
        Display {
            grid: [[false; HEIGHT as usize]; WIDTH as usize]
        }
    }

    pub fn apply_sprite(&mut self, x: u8, y: u8, sprite: Vec<u8>) {
        let x = usize::from(x);
        let y = usize::from(y);
        for (i, row) in sprite.iter().enumerate() {
            let y_row = y + i;
            for n in 0..8 {
                self.grid[x+n][y_row] = (row & (1 << n) > 0) ^ self.grid[x+n][y_row];
            }
        }
    }

    pub fn set(&mut self, x: u8, y: u8, active: bool) {
        let x = usize::from(x);
        let y = usize::from(y);
        self.grid[x][y] = active;
    }

    pub fn is_active(&mut self, x: u8, y: u8) -> bool {
        let x = usize::from(x);
        let y = usize::from(y);
        self.grid[x][y]
    }
}