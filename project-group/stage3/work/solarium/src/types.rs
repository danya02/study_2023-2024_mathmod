use std::{
    ops::{Add, AddAssign, Div, Mul, Sub},
    vec,
};

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2))
    }
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct Particle {
    pub id: u64,
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f64,
    pub radius: f64,
}

impl Particle {
    pub fn make_random(radius_max: f64, angular_speed: f64, id: u64) -> Self {
        let radius = f64::sqrt(rand::random()) * radius_max;
        let angle: f64 = rand::random();
        let angle = angle * std::f64::consts::PI * 2.0;
        let (s, c) = f64::sin_cos(angle);
        let (x, y) = (angle * c, angle * s);
        let vx = -y * angular_speed * (radius_max / radius).powf(1.5);
        let vy = x * angular_speed * (radius_max / radius).powf(1.5);
        Self {
            id,
            position: Vec2::new(x, y),
            velocity: Vec2::new(vx, vy),
            mass: rand::random(),
            radius: rand::random(),
        }
    }

    pub fn is_zeroed(&self) -> bool {
        self.mass == 0.0 || self.radius == 0.0
    }

    pub fn is_finite(&self) -> bool {
        self.position.is_finite()
            && self.velocity.is_finite()
            && self.mass.is_finite()
            && self.radius.is_finite()
    }

    pub fn glue_to_other(&mut self, other: &mut Self) {
        self.position = ((self.position * self.mass) + (other.position * other.mass))
            / (self.mass + other.mass);
        self.velocity = ((self.velocity * self.mass) + (other.velocity * other.mass))
            / (self.mass + other.mass);
        self.mass += other.mass;
        self.radius = f64::cbrt(self.radius.powi(3) + other.radius.powi(3));
        other.velocity = Vec2::new(0.0, 0.0);
        other.radius = 0.0;
        other.mass = 0.0;
    }

    pub fn maybe_glue_to_other(&mut self, other: &mut Self) -> bool {
        if self.is_zeroed() || other.is_zeroed() {
            return false;
        }
        if (self.position - other.position).magnitude() < self.radius + other.radius {
            self.glue_to_other(other);
            return true;
        }
        false
    }
}

#[no_mangle]
pub extern "C" fn make_random_particle(radius_max: f64, angular_speed: f64) -> Particle {
    Particle::make_random(radius_max, angular_speed, rand::random())
}

#[no_mangle]
pub extern "C" fn make_random_particle_list(
    radius_max: f64,
    angular_speed: f64,
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
