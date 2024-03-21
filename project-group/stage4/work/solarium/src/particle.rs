use crate::{vec2::Vec2, Num};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Particle {
    pub id: u64,
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: Num,
    pub radius: Num,
    pub zeroed: bool,
}

impl Particle {
    const MASS: (Num, Num) = (0.0, 0.7);
    const DENSITY: (Num, Num) = (0.1, 1.3);

    pub fn make_random(radius_max: Num, angular_speed: Num, id: u64) -> Self {
        let radius = Num::sqrt(rand::random()) * radius_max;
        let angle: Num = rand::random();
        let angle = angle * (std::f64::consts::PI as Num) * 2.0;

        let (s, c) = f64::sin_cos(angle as f64);

        let (x, y) = (radius * c as Num, radius * s as Num);

        let vx = -y * angular_speed * (radius_max / radius).powf(1.5);
        let vy = x * angular_speed * (radius_max / radius).powf(1.5);

        let mass = rand::random::<Num>() / (Self::MASS.1 - Self::MASS.0) + Self::MASS.0;
        let my_density =
            rand::random::<Num>() / (Self::DENSITY.1 - Self::DENSITY.0) + Self::DENSITY.0;
        let my_volume = mass / my_density;
        let radius = Num::cbrt(0.75 * (std::f64::consts::PI as Num) * my_volume);
        Self {
            id,
            position: Vec2::new(x, y),
            velocity: Vec2::new(vx, vy),
            mass,
            radius,
            zeroed: false,
        }
    }

    #[inline]
    pub fn is_zeroed(&self) -> bool {
        self.zeroed
        // self.mass == 0.0 || self.radius == 0.0
    }

    pub fn is_finite(&self) -> bool {
        self.position.is_finite()
            && self.velocity.is_finite()
            && self.mass.is_finite()
            && self.radius.is_finite()
    }

    #[inline]
    pub fn glue_to_other(&mut self, other: &mut Self) {
        self.position = ((self.position * self.mass) + (other.position * other.mass))
            / (self.mass + other.mass);
        self.velocity = ((self.velocity * self.mass) + (other.velocity * other.mass))
            / (self.mass + other.mass);
        self.mass += other.mass;
        self.radius = Num::cbrt(self.radius.powi(3) + other.radius.powi(3));
        other.velocity = Vec2::new(0.0, 0.0);
        other.radius = 0.0;
        other.mass = 0.0;
        other.zeroed = true;
    }

    /// Zeroes the passed in particle if glued (and returns true)
    pub fn maybe_glue_to_other(&mut self, other: &mut Self) -> bool {
        if self.is_zeroed() || other.is_zeroed() {
            return false;
        }

        let radius_sum = self.radius + other.radius;

        // Fast check for distance: if the two particles's individual coordinates differ too much,
        // then they can definitely not be colliding.
        let displacement = self.position - other.position;
        if displacement.x.abs() > radius_sum || displacement.y.abs() > radius_sum {
            return false;
        }

        if displacement.magnitude() < radius_sum {
            self.glue_to_other(other);
            return true;
        }
        false
    }
}

#[no_mangle]
pub extern "C" fn make_random_particle(radius_max: Num, angular_speed: Num) -> Particle {
    Particle::make_random(radius_max, angular_speed, rand::random())
}

#[no_mangle]
pub extern "C" fn make_random_particle_list(
    radius_max: Num,
    angular_speed: Num,
    count: usize,
) -> *mut Particle {
    let mut dest = vec![];
    dest.reserve_exact(count);
    for _ in 0..count {
        dest.push(make_random_particle(radius_max, angular_speed));
    }
    dest.shrink_to_fit();

    let ptr = dest.as_mut_ptr();
    std::mem::forget(dest);
    ptr
}
