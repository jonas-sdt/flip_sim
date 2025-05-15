use core::{
    ops::{Add, Div, Mul},
    time::Duration,
};

pub type Density = f64;

pub trait HasPosition {
    fn get_pos(&self) -> Position;
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

impl Mul<Duration> for Velocity {
    type Output = Position;

    fn mul(self, rhs: Duration) -> Self::Output {
        Position {
            x: self.x * rhs.as_secs_f64(),
            y: self.y * rhs.as_secs_f64(),
        }
    }
}

impl Div<f64> for Velocity {
    type Output = Velocity;

    fn div(self, rhs: f64) -> Self::Output {
        Velocity {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<f64> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: f64) -> Self::Output {
        Velocity {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add for Velocity {
    type Output = Velocity;

    fn add(self, rhs: Self) -> Self::Output {
        Velocity {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Acceleration {
    pub x: f64,
    pub y: f64,
}

impl Mul<Duration> for Acceleration {
    type Output = Velocity;

    fn mul(self, rhs: Duration) -> Self::Output {
        return Velocity {
            x: self.x * rhs.as_secs_f64(),
            y: self.y * rhs.as_secs_f64(),
        };
    }
}
