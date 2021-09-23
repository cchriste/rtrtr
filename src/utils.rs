// my first module... called utils, but it's `[pub] mod utils` is actually it's
// filename, not a declaration. Confusing at first, but I think I got it.

pub struct Color([f32; 4]);
//instantiate using: `let c = vec![r,g,b,a];`

// pretty handy... not really, just prints what the object implements
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


#[derive(Debug)]  // enables it to be printed. What else?
#[derive(Copy, Clone)]
pub struct Vector {
    pub v: [f32; 3],  // make it 4 elements to add w/a
}

// Thursday, September 9, 2021 - this is growing on me... maybe functions?
// ...that returns refs? so v.x() = 42.0;
// alternative universe (maybe possible with...? maybe *self.v[0] = ?)
// struct Vector {
//     x: f32,
//     y: f32,
//     z: f32,
//     w: f32,
// }

use std::ops::{Mul, Div, Sub, Add, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector { v: [-self.v[0],
                     -self.v[1],
                     -self.v[2]] }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector { v: [self.v[0] + other.v[0],
                     self.v[1] + other.v[1],
                     self.v[2] + other.v[2]] }
    }
}

impl AddAssign for Vector {
    //type Output = Vector;

    fn add_assign(&mut self, other: Vector) -> () {
        *self = Vector { v: [self.v[0] + other.v[0],
                             self.v[1] + other.v[1],
                             self.v[2] + other.v[2]] }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector { v: [self.v[0] - other.v[0],
                     self.v[1] - other.v[1],
                     self.v[2] - other.v[2]] }
    }
}

impl SubAssign for Vector {
    //type Output = Vector;

    fn sub_assign(&mut self, other: Vector) -> () {
        *self = Vector { v: [self.v[0] - other.v[0],
                             self.v[1] - other.v[1],
                             self.v[2] - other.v[2]] }
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, k: f32) -> Self {
        Self { v: [self.v[0] * k,
                   self.v[1] * k,
                   self.v[2] * k] }
    }
}

// yay! we can do k*Vector
impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, vec: Vector) -> Vector {
        Vector { v: [vec.v[0] * self,
                     vec.v[1] * self,
                     vec.v[2] * self] }
    }
}

impl MulAssign<f32> for Vector {
    //type Output = Self;

    fn mul_assign(&mut self, k: f32) -> () {
        *self = Self { v: [self.v[0] * k,
                           self.v[1] * k,
                           self.v[2] * k] }
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, k: f32) -> Self {
        Self { v: [self.v[0] / k,
                   self.v[1] / k,
                   self.v[2] / k] }
    }
}

impl DivAssign<f32> for Vector {
    //type Output = Self;

    fn div_assign(&mut self, k: f32) -> () {
        *self = Self { v: [self.v[0] / k,
                           self.v[1] / k,
                           self.v[2] / k] }
    }
}

pub fn dot(v1: &Vector, v2: &Vector) -> f32 {
    v1.dot(v2)
}

impl Vector {
    pub fn zero() -> Self {
        Self { v: [0.0, 0.0, 0.0] }
    }

    pub fn init(x: f32, y: f32, z: f32) -> Self {
        Self { v: [x, y, z] }
    }
    pub fn x(&self) -> &f32 { &self.v[0] }
    pub fn y(&self) -> f32 { self.v[1] }
    pub fn z(&self) -> f32 { self.v[2] }

    pub fn len_squared(&self) -> f32 {
        self.v[0]*self.v[0] + self.v[1]*self.v[1] + self.v[2]*self.v[2]
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        self.v[0]*other.v[0] + self.v[1]*other.v[1] + self.v[2]*other.v[2]
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        //        |  î   ĵ   k̂ |
        // det of | a0  a1  a2 |
        //        | b0  b1  b2 |
        //
        // = (a1b2 - a2b1)î - (a2b0 - a0b2)ĵ + (a0b1 - a1b0)k̂
        //
        Vector { v: [self.v[1]*other.v[2] - self.v[2]*other.v[1],
                     self.v[2]*other.v[0] - self.v[0]*other.v[2],
                     self.v[0]*other.v[1] - self.v[1]*other.v[0]] }
    }

    pub fn unit_vector(&v: &Vector) -> Vector {
        v.normalize()
    }

    pub fn normalize(&self) -> Vector {
        let magnitude = self.len();
        Vector { v: [self.v[0] / magnitude,
                     self.v[1] / magnitude,
                     self.v[2] / magnitude] }
    }
}


#[derive(Debug)]
pub struct Ray {
    pub origin: Vector,
    pub dir: Vector
}

impl Ray {

    pub fn new(origin: Vector, dir: Vector) -> Ray {
        Ray {
            origin,
            dir: dir.normalize()  // do we need to normalize this here?
        }
    }

    pub fn at(&self, t: f32) -> Vector {
        let v = self.dir.mul(t);
        Vector { v: [self.origin.x() + v.x(),
                     self.origin.y() + v.y(),
                     self.origin.z() + v.z()] }
    }
}

