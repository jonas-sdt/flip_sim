#![no_std]

mod hash_grid;
mod particle;
pub mod units;

#[cfg(test)]
mod test;

use core::{
    // collections::HashMap,
    ops::{Add, Div, Mul},
};
use hash_grid::HashGrid;
use units::*;

const NUM_PARTICLES: usize = 1_000_000;
const HEIGHT: usize = 64;
const WIDTH: usize = 64;

trait Fluid {
    fn get_density() -> Density;
    fn set_density(density: Density);
    fn from_particles(particles: heapless::Vec<particle::Particle, NUM_PARTICLES>);
}

fn from_grid<T>(spacing: f64, pos_in_cell: Position, corner_values: [T; 4]) -> T
where
    T: Mul<f64>,
    T::Output: Add<Output = T::Output>
        + Mul<f64, Output = T::Output>
        + Div<f64, Output = T::Output>
        + Copy,
{
    let h = spacing;
    // create from corner values
    let dx = pos_in_cell.x;
    let dy = pos_in_cell.y;
    let w_1 = (1 as f64 - dx / h) * (1 as f64 - dy / h);
    let w_2 = dx / h * (1 as f64 - dy / h);
    let w_3 = dx / h * dy / h;
    let w_4 = (1 as f64 - dx / h) * dy / h;

    let mut iter = corner_values.into_iter();
    let q_1 = iter.next().unwrap();
    let q_2 = iter.next().unwrap();
    let q_3 = iter.next().unwrap();
    let q_4 = iter.next().unwrap();
    let q_p = (q_1 * w_1 + q_2 * w_2 + q_3 * w_3 + q_4 * w_4) / (w_1 + w_2 + w_3 + w_4);

    todo!()
}

fn to_grid<T>(spacing: f64, pos_in_cell: Position, corner_values: [T; 4]) -> T
where
    T: Mul<f64>,
    T::Output: Add<Output = T::Output>
        + Mul<f64, Output = T::Output>
        + Div<f64, Output = T::Output>
        + Copy,
{
    let h = spacing;
    // create from corner values
    let dx = pos_in_cell.x;
    let dy = pos_in_cell.y;
    let w_1 = (1 as f64 - dx / h) * (1 as f64 - dy / h);
    let w_2 = dx / h * (1 as f64 - dy / h);
    let w_3 = dx / h * dy / h;
    let w_4 = (1 as f64 - dx / h) * dy / h;

    let mut iter = corner_values.into_iter();
    let q_1 = iter.next().unwrap();
    let q_2 = iter.next().unwrap();
    let q_3 = iter.next().unwrap();
    let q_4 = iter.next().unwrap();
    let q_p = (q_1 * w_1 + q_2 * w_2 + q_3 * w_3 + q_4 * w_4) / (w_1 + w_2 + w_3 + w_4);

    todo!()
}

trait CellMaterial {
    fn get_s(&self) -> f64;
    fn get_vel(&self) -> Velocity;
    fn set_vel(&mut self, vel: Velocity);
    fn get_pos(&self) -> Position;
}

struct AirCell {
    vel: Velocity,
    pos: Position,
}

impl CellMaterial for AirCell {
    fn get_s(&self) -> f64 {
        1.0
    }

    fn get_vel(&self) -> Velocity {
        self.vel
    }

    fn get_pos(&self) -> Position {
        self.pos
    }

    fn set_vel(&mut self, vel: Velocity) {
        self.vel = vel;
    }
}

struct WaterCell {
    vel: Velocity,
    pos: Position,
}

impl WaterCell {
    fn from_particles() -> Self {
        todo!()
    }
}

impl CellMaterial for WaterCell {
    fn get_s(&self) -> f64 {
        1.0
    }

    fn get_vel(&self) -> Velocity {
        self.vel
    }

    fn get_pos(&self) -> Position {
        self.pos
    }
    fn set_vel(&mut self, vel: Velocity) {
        self.vel = vel;
    }
}

struct SolidCell {
    pos: Position,
}

impl CellMaterial for SolidCell {
    fn get_s(&self) -> f64 {
        0.0
    }

    fn get_vel(&self) -> Velocity {
        Velocity { x: 0.0, y: 0.0 }
    }

    fn get_pos(&self) -> Position {
        self.pos
    }
    fn set_vel(&mut self, _vel: Velocity) {}
}

enum Cell {
    AirCell(AirCell),
    WaterCell(WaterCell),
    SolidCell(SolidCell),
}

fn force_incompressibility<C: CellMaterial>(
    this: &mut C,
    left: &C,
    right: &mut C,
    up: &mut C,
    down: C,
) {
    // d must become exactly 0 to have incompressibility
    let d = right.get_vel().x - this.get_vel().x + up.get_vel().y - this.get_vel().y;
    let s = right.get_s() + left.get_s() + up.get_s() + down.get_s();

    // adjust velocities accordingly
    this.set_vel(Velocity {
        x: this.get_vel().x + d * left.get_s() / s,
        y: this.get_vel().y + d * down.get_s() / s,
    });

    right.set_vel(Velocity {
        x: right.get_vel().x + d * right.get_s() / s,
        y: right.get_vel().y,
    });

    up.set_vel(Velocity {
        x: up.get_vel().x,
        y: up.get_vel().y + d * up.get_s() / s,
    });
}

struct Grid<'a> {
    grid: HashGrid<'a, particle::Particle, NUM_PARTICLES>,
    spacing: f64,
    // u_field: HashMap<(usize, usize), f64>,
    // v_field: HashMap<(usize, usize), f64>,
}

// impl Grid {
//     fn get_cell_coordinates(&self, pos: &Position) -> (usize, usize) {
//         let x_cell = (pos.x / self.spacing) as usize;
//         let y_cell = (pos.x / self.spacing) as usize;
//         return (x_cell, y_cell);
//     }

//     fn get_cell(&self, pos: &Position) -> &Cell {
//         let (x_cell, y_cell) = self.get_cell_coordinates(pos);
//         &self.grid[&(x_cell, y_cell)]
//     }

//     fn pos_in_cell(&self, cell_coordinates: (usize, usize), pos: &Position) -> Position {
//         let h = self.spacing;
//         Position {
//             x: pos.x - cell_coordinates.0 as f64 * h,
//             y: pos.y - cell_coordinates.1 as f64 * h,
//         }
//     }
// }

// // fn main() {
// //     println!("Hello, world!");
// // }
