use cgmath::Vector3;
use std::f64;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use {HEIGHT, WIDTH};

pub struct UV {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Col {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Col {
    pub fn new(r: f64, g: f64, b: f64) -> Col {
        Col { r: r, g: g, b: b }
    }
}

impl Add<f64> for Col {
    type Output = Col;

    fn add(self, val: f64) -> Col {
        Col {
            r: self.r + val,
            g: self.g + val,
            b: self.b + val,
        }
    }
}

impl Add<Col> for Col {
    type Output = Col;

    fn add(self, col2: Col) -> Col {
        Col {
            r: self.r + col2.r,
            g: self.g + col2.g,
            b: self.b + col2.b,
        }
    }
}

impl Sub<f64> for Col {
    type Output = Col;

    fn sub(self, val: f64) -> Col {
        Col {
            r: self.r + val,
            g: self.g + val,
            b: self.b + val,
        }
    }
}

impl Sub<Col> for Col {
    type Output = Col;

    fn sub(self, col2: Col) -> Col {
        Col {
            r: self.r - col2.r,
            g: self.g - col2.g,
            b: self.b - col2.b,
        }
    }
}

impl Mul<f64> for Col {
    type Output = Col;

    fn mul(self, val: f64) -> Col {
        Col {
            r: self.r * val,
            g: self.g * val,
            b: self.b * val,
        }
    }
}

impl Div<f64> for Col {
    type Output = Col;

    fn div(self, val: f64) -> Col {
        Col {
            r: self.r / val,
            g: self.g / val,
            b: self.b / val,
        }
    }
}

pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

pub fn hsv(h: f64, s: f64, v: f64) -> Col {
    let r = if (h % 1.0) < 0.5 {
        (clamp(-6.0 * (h % 1.0) + 2.0, 0.0, 1.0) * s + 1.0 - s) * v
    } else {
        (clamp(6.0 * (h % 1.0) - 4.0, 0.0, 1.0) * s + 1.0 - s) * v
    };

    let g = if (h % 1.0) < 1.0 / 3.0 {
        (clamp(6.0 * (h % 1.0), 0.0, 1.0) * s + 1.0 - s) * v
    } else {
        (clamp(-6.0 * (h % 1.0) + 4.0, 0.0, 1.0) * s + 1.0 - s) * v
    };

    let b = if (h % 1.0) < 2.0 / 3.0 {
        (clamp(6.0 * (h % 1.0) - 2.0, 0.0, 1.0) * s + 1.0 - s) * v
    } else {
        (clamp(-6.0 * (h % 1.0) + 6.0, 0.0, 1.0) * s + 1.0 - s) * v
    };

    Col::new(r, g, b)
}

pub fn mix_col(col1: Col, col2: Col, mix: f64) -> Col {
    col1 * mix + col2 * (1.0 - mix)
}

pub fn color_ramp(mix: f64) -> Col {
    let col1 = Col::new(0.0, 0.2, 1.0);
    let col2 = Col::new(1.0, 1.0, 1.0);
    let col3 = Col::new(1.0, 0.6, 0.1);
    if mix < 1.0 / 3.0 {
        mix_col(col2, col1, mix * 3.0)
    } else if mix < 2.0 / 3.0 {
        mix_col(col3, col2, (mix - 1.0 / 3.0) * 3.0)
    } else {
        mix_col(col1, col3, (mix - 2.0 / 3.0) * 3.0)
    }
}

pub fn col_to_rgb_u32(rgb: Col) -> u32 {
    rgb_u32(
        (rgb.r * 255.0) as u32,
        (rgb.g * 255.0) as u32,
        (rgb.b * 255.0) as u32,
    )
}

pub fn rgb_u32(r: u32, g: u32, b: u32) -> u32 {
    let rg = (r << 8) | g;
    ((rg << 8) | b)
}

pub fn uv(index: usize) -> UV {
    UV {
        x: (index % WIDTH as usize) as f64,
        y: (index as f64 / WIDTH as f64) as f64,
    }
}
