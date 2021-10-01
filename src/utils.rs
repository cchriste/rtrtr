// mod utils
// Color, Vec3, Vec4, Axis, Matrix, Range, Ray
// TODO:
//  [] trade Range for std version
//  [] change Vector -> Vec3, ::init to ::new
//  [] create str ops for Vec3 so they, and wrappers like Point and Color) are tolerable to print
//  [] create chainable matrix ops (M.translate(t).rotate(r,Axis::X).scale(s))
//  [] remember how to properly transform normals back into world space (M⁻¹)ᵀ*n
//   - transforming ray into jumble space is actually M⁻¹*v, and M⁻¹*p
//  [] use core::ops::Range instead of reinventing it
//   - (https://doc.rust-lang.org/core/ops/struct.Range.html)
//   - at least make a struct with that and the handy function.

pub struct Color([f32; 4]);
//instantiate using: `let c = vec![r,g,b,a];`

// pretty handy... not really, just prints what the object implements
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub enum Axis { X, Y, Z }

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Vector {
    pub v: [f32; 3],
}

use std::ops::{Mul, Div, Sub, Add, Neg, AddAssign, SubAssign, MulAssign, DivAssign, Index, IndexMut};

// TODO: there might be some template way to enable slice indices
// - https://doc.rust-lang.org/std/ops/trait.IndexMut.html
// such as: impl Index<T> for Vector { ...
//  output = ?? (how do I say "range of a vec"?)
//    ... just call v's index method
impl Index<usize> for Vector {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.v[idx]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.v[idx]
    }
}

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

    pub fn x(&self) -> f32 { self.v[0] }
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

    pub fn transform(&self, csys: &Matrix) -> Ray {
        let o = Vec4::new([self.origin.x(),
                           self.origin.y(),
                           self.origin.z(),
                           1.0]);
        let v = Vec4::new([self.dir.x(),
                           self.dir.y(),
                           self.dir.z(),
                           0.0]);
        let o = *csys * o;  // ?: derefernce argument or just pass copy?
        let v = *csys * v;
        Ray {
            origin: Vector::init(o.x(), o.y(), o.z()),
            dir: Vector::init(v.x(), v.y(), v.z()),
        }
    }
}

// [] just `!core::ops::Range::contains(val)
pub trait OutsideRange {
    fn outside(self, rng: &Range) -> bool;
}

// [] just core::ops::Range
#[derive(Debug)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}


impl OutsideRange for f32 {
    fn outside(self: f32, rng: &Range) -> bool {
        rng.outside(self)
    }
}

// a half-open range [tmin, tmax)
impl Range {
    pub fn default() -> Self {
        Range::new(0.00001, f32::INFINITY)
    }

    pub fn new(min: f32, max: f32) -> Self {
        Range { min, max }
    }

    pub fn inside(&self, val: f32) -> bool {
        !self.outside(val)
    }

    pub fn outside(&self, val: f32) -> bool {
        val < self.min || self.max < val
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Matrix {
    pub rows: [Vec4; 4],
}

impl Matrix {
    pub fn identity() -> Matrix {
        Matrix { rows: [ Vec4::new([1.0, 0.0, 0.0, 0.0]),
                         Vec4::new([0.0, 1.0, 0.0, 0.0]),
                         Vec4::new([0.0, 0.0, 1.0, 0.0]),
                         Vec4::new([0.0, 0.0, 0.0, 1.0]) ] }
    }

    pub fn new(r0: [f32; 4], r1: [f32; 4], r2: [f32; 4], r3: [f32; 4]) -> Matrix {
        Matrix { rows: [ Vec4::new(r0),
                         Vec4::new(r1),
                         Vec4::new(r2),
                         Vec4::new(r3) ] }
    }

    pub fn transpose(&self) -> Matrix {
        Matrix { rows: [ Vec4::new([self.rows[0][0], self.rows[1][0], self.rows[2][0], self.rows[3][0]]), 
                         Vec4::new([self.rows[0][1], self.rows[1][1], self.rows[2][1], self.rows[3][1]]), 
                         Vec4::new([self.rows[0][2], self.rows[1][2], self.rows[2][2], self.rows[3][2]]), 
                         Vec4::new([self.rows[0][3], self.rows[1][3], self.rows[2][3], self.rows[3][3]]) ]
        }
    }

    // only same as transpose if they're orthogonal
    //
    // adjugate matrix method:
    // A = Ã/|A|
    //
    pub fn inverse(&self) -> Matrix {
        // (thank you, https://semath.info/src/inverse-cofactor-ex4.html)

        // 入力したデータAB
        let m11 = self.rows[0][0];
        let m12 = self.rows[0][1];
        let m13 = self.rows[0][2];
        let m14 = self.rows[0][3];
        let m21 = self.rows[1][0];
        let m22 = self.rows[1][1];
        let m23 = self.rows[1][2];
        let m24 = self.rows[1][3];
        let m31 = self.rows[2][0];
        let m32 = self.rows[2][1];
        let m33 = self.rows[2][2];
        let m34 = self.rows[2][3];
        let m41 = self.rows[3][0];
        let m42 = self.rows[3][1];
        let m43 = self.rows[3][2];
        let m44 = self.rows[3][3];

        // 計算式
        let det = (m11 * m22 * m33 * m44 ) + (m11 * m23 * m34 * m42 ) + (m11 * m24 * m32 * m43 )
            - (m11 * m24 * m33 * m42 ) - (m11 * m23 * m32 * m44 ) - (m11 * m22 * m34 * m43 )
            - (m12 * m21 * m33 * m44 ) - (m13 * m21 * m34 * m42 ) - (m14 * m21 * m32 * m43 )
            + (m14 * m21 * m33 * m42 ) + (m13 * m21 * m32 * m44 ) + (m12 * m21 * m34 * m43 )
            + (m12 * m23 * m31 * m44 ) + (m13 * m24 * m31 * m42 ) + (m14 * m22 * m31 * m43 )
            - (m14 * m23 * m31 * m42 ) - (m13 * m22 * m31 * m44 ) - (m12 * m24 * m31 * m43 )
            - (m12 * m23 * m34 * m41 ) - (m13 * m24 * m32 * m41 ) - (m14 * m22 * m33 * m41 )
            + (m14 * m23 * m32 * m41 ) + (m13 * m22 * m34 * m41 ) + (m12 * m24 * m33 * m41 );

        Matrix {
            rows: [ Vec4::new([(m22*m33*m44 + m23*m34*m42 + m24*m32*m43 - m24*m33*m42 - m23*m32*m44 - m22*m34*m43)/det,
                               (-m12*m33*m44 - m13*m34*m42 - m14*m32*m43 + m14*m33*m42 + m13*m32*m44 + m12*m34*m43)/det,
                               (m12*m23*m44 + m13*m24*m42 + m14*m22*m43 - m14*m23*m42 - m13*m22*m44 - m12*m24*m43)/det,
                               (-m12*m23*m34 - m13*m24*m32 - m14*m22*m33 + m14*m23*m32 + m13*m22*m34 + m12*m24*m33)/det]),
                    Vec4::new([(-m21*m33*m44 - m23*m34*m41 - m24*m31*m43 + m24*m33*m41 + m23*m31*m44 + m21*m34*m43)/det,
                               (m11*m33*m44 + m13*m34*m41 + m14*m31*m43 - m14*m33*m41 - m13*m31*m44 - m11*m34*m43)/det,
                               (-m11*m23*m44 - m13*m24*m41 - m14*m21*m43 + m14*m23*m41 + m13*m21*m44 + m11*m24*m43)/det,
                               (m11*m23*m34 + m13*m24*m31 + m14*m21*m33 - m14*m23*m31 - m13*m21*m34 - m11*m24*m33)/det]),
                    Vec4::new([(m21*m32*m44 + m22*m34*m41 + m24*m31*m42 - m24*m32*m41 - m22*m31*m44 - m21*m34*m42)/det,
                               (-m11*m32*m44 - m12*m34*m41 - m14*m31*m42 + m14*m32*m41 + m12*m31*m44 + m11*m34*m42)/det,
                               (m11*m22*m44 + m12*m24*m41 + m14*m21*m42 - m14*m22*m41 - m12*m21*m44 - m11*m24*m42)/det,
                               (-m11*m22*m34 - m12*m24*m31 - m14*m21*m32 + m14*m22*m31 + m12*m21*m34 + m11*m24*m32)/det]),
                    Vec4::new([(-m21*m32*m43 - m22*m33*m41 - m23*m31*m42 + m23*m32*m41 + m22*m31*m43 + m21*m33*m42)/det,
                               (m11*m32*m43 + m12*m33*m41 + m13*m31*m42 - m13*m32*m41 - m12*m31*m43 - m11*m33*m42)/det,
                               (-m11*m22*m43 - m12*m23*m41 - m13*m21*m42 + m13*m22*m41 + m12*m21*m43 + m11*m23*m42)/det,
                               (m11*m22*m33 + m12*m23*m31 + m13*m21*m32 - m13*m22*m31 - m12*m21*m33 - m11*m23*m32)/det]) ]
        }
    }

    pub fn col(&self, i: usize) -> Vec4 {
        Vec4::new([self.rows[0][i], self.rows[1][i], self.rows[2][i], self.rows[3][i]])
    }

    pub fn row(&self, i: usize) -> Vec4 {
        return self.rows[i];
    }

    // TODO: return modifiable version of self (&mut Matrix, or &Matrix?)
    pub fn scale(&mut self, k: f32) -> () {
        self.rows[0][0] *= k;
        self.rows[1][1] *= k;
        self.rows[2][2] *= k;
        self.rows[3][3] *= k;
    }

    pub fn translate(&mut self, t: Vector) -> () {
        self.rows[0][3] += t.x();
        self.rows[1][3] += t.y();
        self.rows[2][3] += t.z();
    }

    pub fn rotate(&mut self, rad: f32, axis: Axis) -> () {
        println!("TODO: add Matrix::rotate");
        match axis {
            Axis::X => {
                self.rows[1][1] += rad.cos();
                self.rows[1][2] += rad.sin();
                self.rows[2][1] += -rad.sin();
                self.rows[2][2] += rad.cos();
            },
            Axis::Y => {
                self.rows[0][0] += rad.cos();
                self.rows[0][2] += -rad.sin();
                self.rows[2][0] += rad.sin();
                self.rows[2][2] += rad.cos();
            },
            Axis::Z => {
                self.rows[0][0] += rad.cos();
                self.rows[0][1] += rad.sin();
                self.rows[1][0] += -rad.sin();
                self.rows[1][1] += rad.cos();
            }
        }
    }

    pub fn rotate_deg(&mut self, deg: f32, axis: Axis) -> () {
        self.rotate(deg.to_radians(), axis);
    }
}

// impl Add for Matrix {
//     type Output = self;

//     fn add(self, other: self) -> self {
//         self { v: [self.rows[0] + other.rows[0],
//                    self.rows[1] + other.rows[1],
//                    self.rows[2] + other.rows[2],
//                    self.rows[3] + other.rows[3]] }
//     }
// }

// impl Sub for Matrix {
//     type Output = self;

//     fn sub(self, other: self) -> self {
//         self { v: [self.rows[0] - other.rows[0],
//                    self.rows[1] - other.rows[1],
//                    self.rows[2] - other.rows[2],
//                    self.rows[3] - other.rows[3]] }
//     }
// }

// should this include homogeneous coord?
// sure, though in most cases it's just 1*1
impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { rows: [ Vec4::new([self.rows[0].dot(&self.col(0)), self.rows[0].dot(&self.col(1)), self.rows[0].dot(&self.col(2)), self.rows[0].dot(&self.col(3))]),
                       Vec4::new([self.rows[1].dot(&self.col(0)), self.rows[1].dot(&self.col(1)), self.rows[1].dot(&self.col(2)), self.rows[1].dot(&self.col(3))]),
                       Vec4::new([self.rows[2].dot(&self.col(0)), self.rows[2].dot(&self.col(1)), self.rows[2].dot(&self.col(2)), self.rows[2].dot(&self.col(3))]),
                       Vec4::new([self.rows[3].dot(&self.col(0)), self.rows[3].dot(&self.col(1)), self.rows[3].dot(&self.col(2)), self.rows[3].dot(&self.col(3))]) ]
        }
    }
}


// ================================================================================
// Vec4 (do I really need a Vec3? I can't remember. I think so since w get in
// the way. For example with computing len.)
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Vec4 {
    pub v: [f32; 4],
}

impl Index<usize> for Vec4 {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.v[idx]
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.v[idx]
    }
}

impl Neg for Vec4 {
    type Output = Vec4;
    fn neg(self) -> Vec4 {
        Vec4 { v: [-self.v[0],
                   -self.v[1],
                   -self.v[2],
                   -self.v[3]] }
    }
}

// impl Add for Vec4 {
//     type Output = Vec4;

//     fn add(self, other: Vec4) -> Vec4 {
//         Vec4 { v: [self.v[0] + other.v[0],
//                    self.v[1] + other.v[1],
//                    self.v[2] + other.v[2],
//                    self.v[3] + other.v[3]] }
//     }
// }

// impl AddAssign for Vec4 {
//     fn add_assign(&mut self, other: Vec4) -> () {
//         *self = Vec4 { v: [self.v[0] + other.v[0],
//                            self.v[1] + other.v[1],
//                            self.v[2] + other.v[2],
//                            self.v[3] + other.v[3]] }
//     }
// }

// impl Sub for Vec4 {
//     type Output = Vec4;

//     fn sub(self, other: Vec4) -> Vec4 {
//         Vec4 { v: [self.v[0] - other.v[0],
//                    self.v[1] - other.v[1],
//                    self.v[2] - other.v[2],
//                    self.v[3] - other.v[3]] }
//     }
// }

// impl SubAssign for Vec4 {
//     fn sub_assign(&mut self, other: Vec4) -> () {
//         *self = Vec4 { v: [self.v[0] - other.v[0],
//                            self.v[1] - other.v[1],
//                            self.v[2] - other.v[2],
//                            self.v[3] - other.v[3]] }
//     }
// }

// impl Mul<f32> for Vec4 {
//     type Output = Self;

//     fn mul(self, k: f32) -> Self {
//         Self { v: [self.v[0] * k,
//                    self.v[1] * k,
//                    self.v[2] * k,
//                    self.v[3] * k] }
//     }
// }

// // yay! we can do k*Vec4
// impl Mul<Vec4> for f32 {
//     type Output = Vec4;

//     fn mul(self, vec: Vec4) -> Vec4 {
//         Vec4 { v: [vec.v[0] * self,
//                    vec.v[1] * self,
//                    vec.v[2] * self,
//                    vec.v[3] * self] }
//     }
// }

// I don't think we want to be able to do v*M, only M*v
// impl Mul<Matrix> for Vec4 {
//     type Output = Vec4;
//     fn mul(self, vec: Vec4) -> Vec4 {
//         Vec4 { v: [dot(self.rows[0], vec),
//                    dot(self.rows[1], vec),
//                    dot(self.rows[2], vec),
//                    dot(self.rows[3], vec)] }
//     }
// }

// M*v
impl Mul<Vec4> for Matrix {
    type Output = Vec4;
    fn mul(self, vec: Vec4) -> Vec4 {
        Vec4 { v: [self.rows[0].dot(&vec),
                   self.rows[1].dot(&vec),
                   self.rows[2].dot(&vec),
                   self.rows[3].dot(&vec)] }
    }
}

// impl MulAssign<f32> for Vec4 {
//     fn mul_assign(&mut self, k: f32) -> () {
//         *self = Self { v: [self.v[0] * k,
//                            self.v[1] * k,
//                            self.v[2] * k,
//                            self.v[3] * k] }
//     }
// }

// impl Div<f32> for Vec4 {
//     type Output = Self;

//     fn div(self, k: f32) -> Self {
//         Self { v: [self.v[0] / k,
//                    self.v[1] / k,
//                    self.v[2] / k,
//                    self.v[3] / k] }
//     }
// }

// impl DivAssign<f32> for Vec4 {
//     fn div_assign(&mut self, k: f32) -> () {
//         *self = Self { v: [self.v[0] / k,
//                            self.v[1] / k,
//                            self.v[2] / k,
//                            self.v[3] / k] }
//     }
// }

// pub fn dot(v1: &Vec4, v2: &Vec4) -> f32 {
//     v1.dot(v2)
// }

impl Vec4 {
    pub fn zero() -> Self {
        Self { v: [0.0, 0.0, 0.0, 0.0] }
    }

    // pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    //     Self { v: [x, y, z, w] }
    // }
    pub fn new(v: [f32; 4]) -> Self {
        Self { v }
    }

    pub fn x(&self) -> f32 { self.v[0] }
    pub fn y(&self) -> f32 { self.v[1] }
    pub fn z(&self) -> f32 { self.v[2] }
    pub fn w(&self) -> f32 { self.v[3] }

    // pub fn len_squared(&self) -> f32 {
    //     self.dot(self)
    // }

    // pub fn len(&self) -> f32 {
    //     self.len_squared().sqrt()
    // }

    pub fn dot(&self, other: &Vec4) -> f32 {
        self.v[0]*other.v[0] + self.v[1]*other.v[1] + self.v[2]*other.v[2] + self.v[3]*other.v[3]
    }

    // pub fn cross(&self, other: &Vec4) -> Vec4 {
    //     //        |  î   ĵ   k̂ |
    //     // det of | a0  a1  a2 |
    //     //        | b0  b1  b2 |
    //     //
    //     // = (a1b2 - a2b1)î - (a2b0 - a0b2)ĵ + (a0b1 - a1b0)k̂
    //     //
    //     Vec4 { v: [self.v[1]*other.v[2] - self.v[2]*other.v[1],
    //                self.v[2]*other.v[0] - self.v[0]*other.v[2],
    //                self.v[0]*other.v[1] - self.v[1]*other.v[0],
    //                1.0] }
    // }

    // pub fn unit_vector(&v: &Vec4) -> Vec4 {
    //     v.normalize()
    // }

    // pub fn normalize(&self) -> Vec4 {
    //     let magnitude = self.len();
    //     Vec4 { v: [self.v[0] / magnitude,
    //                self.v[1] / magnitude,
    //                self.v[2] / magnitude] }
    // }
}

