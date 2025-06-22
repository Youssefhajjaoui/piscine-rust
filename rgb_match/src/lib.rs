#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let mut reult = self;
        if self.r == first {
            reult.r = second;
        } else if self.r == second {
            reult.r = first;
        }

        if self.g == first {
            reult.g = second;
        } else if self.g == second {
            reult.g = first;
        }

        if self.b == first {
            reult.b = second;
        } else if self.b == second {
            reult.b = first;
        }

        if self.a == first {
            reult.a = second;
        } else if self.a == second {
            reult.a = first;
        }
        return reult;
    }
}
