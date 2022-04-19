#[derive(Clone, Copy)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(env: Environment, proj: &Projectile) -> Projectile {
    let new_pos = proj.position + proj.velocity;
    let new_vel = proj.velocity + env.gravity + env.wind;
    return Projectile {
        position: new_pos,
        velocity: new_vel
    };
}
fn main() {
    let env = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(0.01, 0.0, 0.0),
    };

    let mut proj = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: vector(1.0, 1.0, 0.0).normalize(),
    };

    for _ in 1..18 {
        println!("{:?}", proj);
        proj = tick(env, &proj);
    }
}

struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

#[test]
fn color_test() {
    let c = Color {red: -0.5, green: 0.4, blue: 1.7};
    assert_eq!(c.red, -0.5);
    assert_eq!(c.green, 0.4);
    assert_eq!(c.blue, 1.7);
}


fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    fn is_point(&self) -> bool {
        self.w == 1.0
    }
    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    fn dot(&self, other: &Tuple) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn cross(&self, other: &Tuple) -> Tuple {
        vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[test]
fn tuple_point() {
    let a = Tuple {
        x: 4.3,
        y: -4.2,
        z: 3.1,
        w: 1.0,
    };
    assert_eq!(4.3, a.x);
    assert_eq!(-4.2, a.y);
    assert_eq!(3.1, a.z);
    assert_eq!(1.0, a.w);
    assert!(a.is_point());
    assert!(!a.is_vector());
}

#[test]
fn tuple_vector() {
    let a = Tuple {
        x: 4.3,
        y: -4.2,
        z: 3.1,
        w: 0.0,
    };
    assert_eq!(4.3, a.x);
    assert_eq!(-4.2, a.y);
    assert_eq!(3.1, a.z);
    assert_eq!(0.0, a.w);
    assert!(!a.is_point());
    assert!(a.is_vector());
}

#[test]
fn point_maker() {
    let p = point(1.0, 2.0, 3.0);
    assert_eq!(
        p,
        Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0
        }
    );
}

#[test]
fn vector_maker() {
    let v = vector(1.0, 2.0, 3.0);
    assert_eq!(
        v,
        Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 0.0
        }
    );
}

// arithmetic

use std::ops::{Add, Div, Mul, Neg, Sub};


impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}
impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

#[test]
fn add_colors() {
    let c1 = Color{red: 0.9, green: 0.6, blue: 0.75 };
    let c2 = Color{red: 0.7, green: 0.1, blue: 0.25 };
    assert_eq!(Color {red: 1.6, green: 0.7, blue: 1.0}, c1 + c1);
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

#[test]
fn tuple_addition() {
    let a1 = Tuple {
        x: 3.0,
        y: -2.0,
        z: 5.0,
        w: 1.0,
    };
    let a2 = Tuple {
        x: -2.0,
        y: 3.0,
        z: 1.0,
        w: 0.0,
    };
    assert_eq!(
        a1 + a2,
        Tuple {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 1.0
        }
    );
}

#[test]
fn point_subtraction() {
    let p1 = point(3.0, 2.0, 1.0);
    let p2 = point(5.0, 6.0, 7.0);
    assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
}

#[test]
fn sub_vec_from_point() {
    let p = point(3.0, 2.0, 1.0);
    let v = vector(5.0, 6.0, 7.0);
    assert_eq!(p - v, point(-2.0, -4.0, -6.0));
}

#[test]
fn sub_vecs() {
    let v1 = vector(3.0, 2.0, 1.0);
    let v2 = vector(5.0, 6.0, 7.0);
    assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
}

#[test]
fn sub_vec_from_zero() {
    let zero = vector(0.0, 0.0, 0.0);
    let v = vector(1.0, -2.0, 3.0);
    assert_eq!(zero - v, vector(-1.0, 2.0, -3.0));
}

#[test]
fn negating() {
    let a = Tuple {
        x: 1.0,
        y: -2.0,
        z: 3.0,
        w: -4.0,
    };
    assert_eq!(
        -a,
        Tuple {
            x: -1.0,
            y: 2.0,
            z: -3.0,
            w: 4.0
        }
    );
}

#[test]
fn mul_tuple_by_scalar() {
    let a = Tuple {
        x: 1.0,
        y: -2.0,
        z: 3.0,
        w: -4.0,
    };
    assert_eq!(
        a * 3.5,
        Tuple {
            x: 3.5,
            y: -7.0,
            z: 10.5,
            w: -14.0
        }
    );
}

#[test]
fn mul_tuple_by_fraction() {
    let a = Tuple {
        x: 1.0,
        y: -2.0,
        z: 3.0,
        w: -4.0,
    };
    assert_eq!(
        a * 0.5,
        Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0
        }
    );
}

#[test]
fn div_tuple_by_scalar() {
    let a = Tuple {
        x: 1.0,
        y: -2.0,
        z: 3.0,
        w: -4.0,
    };
    assert_eq!(
        a / 2.0,
        Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0
        }
    );
}

#[test]
fn magnitude_unit_x() {
    assert_eq!(vector(1.0, 0.0, 0.0).magnitude(), 1.0)
}

#[test]
fn magnitude_unit_y() {
    assert_eq!(vector(0.0, 1.0, 0.0).magnitude(), 1.0)
}

#[test]
fn magnitude_unit_z() {
    assert_eq!(vector(0.0, 0.0, 1.0).magnitude(), 1.0)
}

#[test]
fn magnitude_pos() {
    assert_eq!(vector(1.0, 2.0, 3.0).magnitude(), 14.0_f32.sqrt())
}

#[test]
fn magnitude_neg() {
    assert_eq!(vector(-1.0, -2.0, -3.0).magnitude(), 14.0_f32.sqrt())
}

#[test]
fn normalize_unit() {
    assert_eq!(vector(4.0, 0.0, 0.0).normalize(), vector(1.0, 0.0, 0.0))
}

#[test]
fn normalize_normal() {
    assert_eq!(
        vector(1.0, 2.0, 3.0).normalize(),
        vector(
            1.0 / 14.0_f32.sqrt(),
            2.0 / 14.0_f32.sqrt(),
            3.0 / 14.0_f32.sqrt()
        )
    )
}

#[test]
fn dot_product() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);
    assert_eq!(a.dot(&b), 20.0)
}

fn cross_product() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);
    assert_eq!(a.cross(&b), vector(-1.0, 2.0, -1.0));
    assert_eq!(b.cross(&a), vector(1.0, -2.0, 1.0));
}
