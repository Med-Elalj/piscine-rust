use std::u8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        if first == second {
            return self;
        };
        match (first, second) {
            (a, b) if a == self.r && b == self.g => std::mem::swap(&mut self.r, &mut self.g),
            (a, b) if a == self.r && b == self.b => std::mem::swap(&mut self.r, &mut self.b),
            (a, b) if a == self.r && b == self.a => std::mem::swap(&mut self.r, &mut self.a),
            (a, b) if a == self.g && b == self.r => std::mem::swap(&mut self.g, &mut self.r),
            (a, b) if a == self.g && b == self.b => std::mem::swap(&mut self.g, &mut self.b),
            (a, b) if a == self.g && b == self.a => std::mem::swap(&mut self.g, &mut self.a),
            (a, b) if a == self.b && b == self.r => std::mem::swap(&mut self.b, &mut self.r),
            (a, b) if a == self.b && b == self.g => std::mem::swap(&mut self.b, &mut self.g),
            (a, b) if a == self.b && b == self.a => std::mem::swap(&mut self.b, &mut self.a),
            (a, b) if a == self.a && b == self.r => std::mem::swap(&mut self.a, &mut self.r),
            (a, b) if a == self.a && b == self.g => std::mem::swap(&mut self.a, &mut self.g),
            (a, b) if a == self.a && b == self.b => std::mem::swap(&mut self.a, &mut self.b),
            _ => panic!("Values do not match any two distinct color components"),
        };
        self
    }
}