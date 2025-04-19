const NUM_PARTICLES: usize = 1_000_000;
const HEIGHT: usize = 64;
const WIDTH: usize = 64;

type Density = f64;

trait Particle {
    fn simulate(&mut self, time_step: std::time::Duration, gravity: (f64, f64));
}

trait Fluid {
    fn get_density() -> Density;
    fn set_density(density: Density);
    fn from_particles(particles: heapless::Vec<Box<dyn Particle>, NUM_PARTICLES>);
}

struct WaterParticle {
    position: (i32, i32),
    velocity: (i32, i32),
}

struct AirCell {}

struct WaterCell {}

struct SolidCell;

enum Cell {
    AirCell(AirCell),
    WaterCell(WaterCell),
    SolidCell(SolidCell),
}

struct Grid {}

fn main() {
    println!("Hello, world!");
}
