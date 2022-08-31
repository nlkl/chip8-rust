pub struct Display {
    pub width: u8,
    pub height: u8,
    framebuffer: Vec<bool>,
    wrap_sprites: bool,
}

impl Display {
    pub fn new(width: u8, height: u8, wrap_sprites: bool) -> Display {
        Display {
            width: width,
            height: height,
            framebuffer: vec![false; width as usize * height as usize],
            wrap_sprites: wrap_sprites,
        }
    }

    pub fn clear(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_visibility(x, y, false);
            }
        }
    }

    pub fn apply_sprite(&mut self, x_start: u8, y_start: u8, sprite: Vec<u8>) -> bool {
        let x_start = x_start % self.width;
        let y_start = y_start % self.height;
        let mut pixels_hidden = false;
        for (dy, mask) in sprite.iter().enumerate() {
            let y = y_start + (dy as u8);
            for dx in 0..8 {
                let x = x_start + dx;
                let was_displayed = self.is_visible(x, y);
                let is_displayed = (mask & (0x80 >> dx) > 0) ^ was_displayed;
                self.set_visibility(x, y, is_displayed);
                if !is_displayed && was_displayed {
                    pixels_hidden = true;
                }
            }
        }
        pixels_hidden
    }
    
    pub fn visible_pixels(&self) -> Vec<(u8, u8)> {
        let mut visible_pixels = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                if self.is_visible(x, y) {
                    visible_pixels.push((x, y));
                }
            }
        }
        visible_pixels
    }

    fn is_visible(&self, x: u8, y: u8) -> bool {
        let (x, y) = self.normalize_coords(x, y);
        if x < self.width && y < self.height {
            return self.framebuffer[(x as usize) + (y as usize) * (self.width as usize)];
        }
        false
    }

    fn set_visibility(&mut self, x: u8, y: u8, visible: bool) {
        let (x, y) = self.normalize_coords(x, y);
        if x < self.width && y < self.height {
            self.framebuffer[(x as usize) + (y as usize) * (self.width as usize)] = visible;
        }
    }

    fn normalize_coords(&self, x: u8, y: u8) -> (u8, u8) {
        if self.wrap_sprites {
            return (x % self.width, y % self.height)
        }
        (x, y)
    }
}

struct FrameBuffer {
    width: u8,
    height: u8,
    wrap: bool,
    memory: Vec<bool>,
}

impl FrameBuffer {
    fn new(width: u8, height: u8, wrap: bool) -> FrameBuffer {
        FrameBuffer {
            width: width,
            height: height,
            wrap: wrap,
            memory: vec![false; width as usize * height as usize],
        }
    }

    fn get(&self, x: u8, y: u8) -> bool {
        let (x, y) = self.normalize_coords(x, y);
        if x < self.width && y < self.height {
            return self.memory[(x as usize) + (y as usize) * (self.width as usize)];
        }
        return false;
    }

    fn set(&mut self, x: u8, y: u8, value: bool) {
        let (x, y) = self.normalize_coords(x, y);
        if x < self.width && y < self.height {
            self.memory[(x as usize) + (y as usize) * (self.width as usize)] = value;
        }
    }

    fn normalize_coords(&self, x: u8, y: u8) -> (u8, u8) {
        if self.wrap {
            return (x % self.width, y % self.height)
        }
        (x, y)
    }
}