use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
    pub a: f32,
}

impl Hsl {
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        Self { h, s, l, a: 1.0 }
    }

    pub fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    pub fn to_css(&self) -> String {
        if (self.a - 1.0).abs() < 0.001 {
            format!("hsl({:.0} {:.0}% {:.0}%)", self.h, self.s, self.l)
        } else {
            format!(
                "hsla({:.0} {:.0}% {:.0}% / {:.0}%)",
                self.h,
                self.s,
                self.l,
                self.a * 100.0
            )
        }
    }

    pub fn to_glass(&self) -> Self {
        Self {
            l: (self.l + 15.0).min(98.0),
            a: 0.1,
            ..*self
        }
    }

    pub fn to_surface(&self) -> Self {
        Self {
            l: (self.l - 5.0).max(2.0),
            a: 0.6,
            ..*self
        }
    }

    pub fn lighten(&self, amount: f32) -> Self {
        Self::new(self.h, self.s, (self.l + amount).clamp(0.0, 100.0))
    }

    pub fn darken(&self, amount: f32) -> Self {
        Self::new(self.h, self.s, (self.l - amount).clamp(0.0, 100.0))
    }

    pub fn saturate(&self, amount: f32) -> Self {
        Self::new(self.h, (self.s + amount).clamp(0.0, 100.0), self.l)
    }

    pub fn rotate(&self, degrees: f32) -> Self {
        Self::new((self.h + degrees).rem_euclid(360.0), self.s, self.l)
    }

    pub fn complement(&self) -> Self {
        self.rotate(180.0)
    }
}

impl fmt::Display for Hsl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css())
    }
}

impl Default for Hsl {
    fn default() -> Self {
        Self::new(263.0, 70.0, 55.0)
    }
}
