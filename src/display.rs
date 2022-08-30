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

    pub fn clear_screen(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.set(x, y, false);
            }
        }
    }

    pub fn apply_sprite(&mut self, x: u8, y: u8, sprite: Vec<u8>) -> bool {
        let mut has_unset = false;
        for (i, row) in sprite.iter().enumerate() {
            let y_row = y + (i as u8);
            for n in 0..8 {
                let was_set = self.get(x+n, y_row);
                let is_set = (row & (0x80 >> n) > 0) ^ was_set;
                self.set(x+n, y_row, is_set);
                if !is_set && was_set {
                    has_unset = true;
                }
            }
        }
        has_unset
    }
    
    pub fn displayed_pixels(&self) -> Vec<(u8, u8)> {
        let mut active_coords = vec![];
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let is_active = self.get(x, y);
                if is_active {
                    active_coords.push((x, y));
                }
            }
        }
        active_coords
    }

    fn get(&self, x: u8, y: u8) -> bool {
        self.grid[(x % WIDTH) as usize][(y % HEIGHT) as usize]
    }

    fn set(&mut self, x: u8, y: u8, active: bool) {
        self.grid[(x % WIDTH) as usize][(y % HEIGHT) as usize] = active;
    }
}