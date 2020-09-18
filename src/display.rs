pub const HEIGHT: usize = 32;
pub const WIDTH: usize = 64;

pub struct Display {
    grid: [[bool; HEIGHT]; WIDTH]
}

pub fn new_display() -> Display {
    Display {
        grid: [[false; HEIGHT]; WIDTH]
    }
}

impl Display {
    pub fn apply_sprite(&mut self, x: usize, y: usize, sprite: Vec<u8>) {
        for (i, row) in sprite.iter().enumerate() {
            let y_row = y + i;
            for n in 0..8 {
                self.grid[x+n][y_row] = (row & (1 << n) > 0) ^ self.grid[x+n][y_row];
            }
        }
    }

    pub fn set(&mut self, x: usize, y: usize, active: bool) {
        self.grid[x][y] = active;
    }

    pub fn is_active(&mut self, x: usize, y: usize) -> bool {
        self.grid[x][y]
    }
}