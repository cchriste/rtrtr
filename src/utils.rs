// mod utils
// Color, Vec3, Vec4, Axis, Matrix, Range, Ray
// TODO:
//  [x] change Vector -> Vec3, ::init to ::new
//  [] create generic version of Vec<N> instead of all this cut n' pastin'
//  [] create str ops for Vec3 so they, and wrappers like Point and Color) are tolerable to print
//  [x] create chainable matrix ops (M.translate(t).rotate(r,Axis::X).scale(s))
//  [x] remember how to properly transform normals (vectors) back into world space (M⁻¹)ᵀ*n
//   - transforming ray into jumble space is actually M⁻¹*v, and M⁻¹*p
//  [] use core::ops::Range instead of reinventing it
//   - (https://doc.rust-lang.org/core/ops/struct.Range.html)
//  [] add const to initializer funcs as in Vec3::zero()

pub struct Color([f32; 4]);
//instantiate using: `let c = vec![r,g,b,a];`

// pretty handy... not really, just prints what the object implements
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub enum Axis { X, Y, Z }

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub v: [f32; 3],
}

use std::ops::{Mul, Div, Sub, Add, Neg, AddAssign, SubAssign, MulAssign, DivAssign, Index, IndexMut};
use std::fmt;

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4}î, {:.4}ĵ, {:.4}k̂)", self.v[0], self.v[1], self.v[2])
    }
}

// operator[]
impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.v[idx]
    }
}

// assignable operator[]
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.v[idx]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 { v: [-self.v[0],
                   -self.v[1],
                   -self.v[2]] }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { v: [self.v[0] + other.v[0],
                   self.v[1] + other.v[1],
                   self.v[2] + other.v[2]] }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) -> () {
        *self = Vec3 { v: [self.v[0] + other.v[0],
                           self.v[1] + other.v[1],
                           self.v[2] + other.v[2]] }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { v: [self.v[0] - other.v[0],
                   self.v[1] - other.v[1],
                   self.v[2] - other.v[2]] }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) -> () {
        *self = Vec3 { v: [self.v[0] - other.v[0],
                           self.v[1] - other.v[1],
                           self.v[2] - other.v[2]] }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, k: f32) -> Self {
        Self { v: [self.v[0] * k,
                   self.v[1] * k,
                   self.v[2] * k] }
    }
}

// yay! we can do k*Vec3
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 { v: [vec.v[0] * self,
                   vec.v[1] * self,
                   vec.v[2] * self] }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, k: f32) -> () {
        *self = Self { v: [self.v[0] * k,
                           self.v[1] * k,
                           self.v[2] * k] }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, k: f32) -> Self {
        Self { v: [self.v[0] / k,
                   self.v[1] / k,
                   self.v[2] / k] }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, k: f32) -> () {
        *self = Self { v: [self.v[0] / k,
                           self.v[1] / k,
                           self.v[2] / k] }
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.dot(v2)
}

// generate more evenly distributed random values
use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Uniform};

impl Vec3 {
    pub const fn zero() -> Self {
        Self { v: [0.0, 0.0, 0.0] }
    }

    pub fn new(v: [f32; 3]) -> Self {
        Self { v }
    }

    // vector with values in range [0,1)
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Self { v: [rng.gen(), rng.gen(), rng.gen()] }
    }

    // return vector of n purportedly well-distributed random Vec3s
    pub fn rand_arr(n: u32) -> Vec<Self> {
        let mut rng = thread_rng();
        let unitx = Uniform::new(0.0, 1.0); // maybe more uniform than otherwise
        let unity = Uniform::new(0.0, 1.0); // maybe more uniform than otherwise
        let unitz = Uniform::new(0.0, 1.0); // maybe more uniform than otherwise
        let mut ret = Vec::<Self>::new();
        for _ in 0..n {
            //ret.push(Self { v: [rng.sample(unitx), rng.sample(unity), rng.sample(unitz)] });
            ret.push(Self { v: [unitx.sample(&mut rng), unity.sample(&mut rng), unitz.sample(&mut rng)] });
        }
        ret
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

    pub fn dot(&self, other: Vec3) -> f32 {
        self.v[0]*other.v[0] + self.v[1]*other.v[1] + self.v[2]*other.v[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        //        |  î   ĵ   k̂ |
        // det of | a0  a1  a2 |
        //        | b0  b1  b2 |
        //
        // = (a1b2 - a2b1)î - (a2b0 - a0b2)ĵ + (a0b1 - a1b0)k̂
        //
        Vec3 { v: [self.v[1]*other.v[2] - self.v[2]*other.v[1],
                   self.v[2]*other.v[0] - self.v[0]*other.v[2],
                   self.v[0]*other.v[1] - self.v[1]*other.v[0]] }
    }

    pub fn unit_vector(&v: &Vec3) -> Vec3 {
        v.normalize()
    }

    pub fn normalize(&self) -> Vec3 {
        let magnitude = self.len();
        Vec3 { v: [self.v[0] / magnitude,
                   self.v[1] / magnitude,
                   self.v[2] / magnitude] }
    }
}


#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "♐ o{} v{}", self.origin, self.dir)
    }
}

impl Ray {

    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray {
            origin,
            //dir: dir.normalize()  // do we need to normalize this here? (in particular, FIXME: coordsys in a jumble will need non-normalized ray directions)
            dir
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        let v = self.dir.mul(t);
        Vec3 { v: [self.origin.x() + v.x(),
                   self.origin.y() + v.y(),
                   self.origin.z() + v.z()] }
    }

    pub fn transform(&self, csys: &Matrix) -> Ray {
        let o = csys.apply_to_point(self.origin);
        let v = csys.apply_to_vector(self.dir);
        Ray {
            origin: Vec3::new([o.x(), o.y(), o.z()]),
            dir: Vec3::new([v.x(), v.y(), v.z()]),
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
        Range::new(0.001, f32::INFINITY)  // don't hit things too close or you'll get shadow acne
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

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x{}\ny{}\nz{}\nw{}",
               self.rows[0],self.rows[1],self.rows[2],self.rows[3])
    }
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

    pub fn apply_to_point(&self, vec: Vec3) -> Vec3 {
        let vec = Vec4::new([vec[0], vec[1], vec[2], 1.0]);
        let vec = *self * vec;
        Vec3::new([vec[0], vec[1], vec[2]])
    }

    pub fn apply_to_vector(&self, vec: Vec3) -> Vec3 {
        let vec = Vec4::new([vec[0], vec[1], vec[2], 0.0]);
        let vec = *self * vec;
        Vec3::new([vec[0], vec[1], vec[2]])
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

    pub fn scale(v: Vec3) -> Self {
        let mut mat = Self::identity();
        mat.rows[0][0] *= v[0];
        mat.rows[1][1] *= v[1];
        mat.rows[2][2] *= v[2];
        mat
    }

    // TODO: skew
    // pub fn skew(...) -> Self {
    //     ...
    // }

    pub fn translate(&mut self, t: Vec3) -> &mut Self {
        self.rows[0][3] += t.x();
        self.rows[1][3] += t.y();
        self.rows[2][3] += t.z();
        self
    }

    pub fn rotation(rad: f32, axis: Axis) -> Self {
        let mut mat = Self::identity();
        match axis {
            Axis::X => {
                mat.rows[1][1] += rad.cos();
                mat.rows[1][2] += rad.sin();
                mat.rows[2][1] += -rad.sin();
                mat.rows[2][2] += rad.cos();
            },
            Axis::Y => {
                mat.rows[0][0] += rad.cos();
                mat.rows[0][2] += -rad.sin();
                mat.rows[2][0] += rad.sin();
                mat.rows[2][2] += rad.cos();
            },
            Axis::Z => {
                mat.rows[0][0] += rad.cos();
                mat.rows[0][1] += rad.sin();
                mat.rows[1][0] += -rad.sin();
                mat.rows[1][1] += rad.cos();
            }
        }
        mat
    }

    pub fn rotation_deg(deg: f32, axis: Axis) -> Self {
        Matrix::rotation(deg.to_radians(), axis)
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
        Self { rows: [ Vec4::new([self.rows[0].dot(other.col(0)), self.rows[0].dot(other.col(1)), self.rows[0].dot(other.col(2)), self.rows[0].dot(other.col(3))]),
                       Vec4::new([self.rows[1].dot(other.col(0)), self.rows[1].dot(other.col(1)), self.rows[1].dot(other.col(2)), self.rows[1].dot(other.col(3))]),
                       Vec4::new([self.rows[2].dot(other.col(0)), self.rows[2].dot(other.col(1)), self.rows[2].dot(other.col(2)), self.rows[2].dot(other.col(3))]),
                       Vec4::new([self.rows[3].dot(other.col(0)), self.rows[3].dot(other.col(1)), self.rows[3].dot(other.col(2)), self.rows[3].dot(other.col(3))]) ]
        }
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, other: Self) {
        self.rows[0] = Vec4::new([self.rows[0].dot(other.col(0)), self.rows[0].dot(other.col(1)), self.rows[0].dot(other.col(2)), self.rows[0].dot(other.col(3))]);
        self.rows[1] = Vec4::new([self.rows[1].dot(other.col(0)), self.rows[1].dot(other.col(1)), self.rows[1].dot(other.col(2)), self.rows[1].dot(other.col(3))]);
        self.rows[2] = Vec4::new([self.rows[2].dot(other.col(0)), self.rows[2].dot(other.col(1)), self.rows[2].dot(other.col(2)), self.rows[2].dot(other.col(3))]);
        self.rows[3] = Vec4::new([self.rows[3].dot(other.col(0)), self.rows[3].dot(other.col(1)), self.rows[3].dot(other.col(2)), self.rows[3].dot(other.col(3))]);
        println!("mul_assign:\n{}",self);
    }
}


// ================================================================================
// Vec4
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Vec4 {
    pub v: [f32; 4],
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4}î, {:.4}ĵ, {:.4}k̂, {:.4}l̂)", self.v[0], self.v[1], self.v[2], self.v[3])
    }
}

// operator[]
impl Index<usize> for Vec4 {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.v[idx]
    }
}

// assignable operator[]
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
        Vec4 { v: [self.rows[0].dot(vec),
                   self.rows[1].dot(vec),
                   self.rows[2].dot(vec),
                   self.rows[3].dot(vec)] }
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

    pub fn dot(&self, other: Vec4) -> f32 {
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

// ================================================================================
// Vec2
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Vec2 {
    pub v: [f32; 2],
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4}î, {:.4}ĵ)", self.v[0], self.v[1])
    }
}

// operator[]
impl Index<usize> for Vec2 {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.v[idx]
    }
}

// assignable operator[]
impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.v[idx]
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 { v: [-self.v[0],
                   -self.v[1]] }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { v: [self.v[0] + other.v[0],
                   self.v[1] + other.v[1]] }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) -> () {
        *self = Vec2 { v: [self.v[0] + other.v[0],
                           self.v[1] + other.v[1]] }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 { v: [self.v[0] - other.v[0],
                   self.v[1] - other.v[1]] }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) -> () {
        *self = Vec2 { v: [self.v[0] - other.v[0],
                           self.v[1] - other.v[1]] }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, k: f32) -> Self {
        Self { v: [self.v[0] * k,
                   self.v[1] * k] }
    }
}

// yay! we can do k*Vec2
impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, vec: Vec2) -> Vec2 {
        Vec2 { v: [vec.v[0] * self,
                   vec.v[1] * self] }
    }
}

// I don't think we want to be able to do v*M, only M*v
// impl Mul<Matrix> for Vec2 {
//     type Output = Vec2;
//     fn mul(self, vec: Vec2) -> Vec2 {
//         Vec2 { v: [dot(self.rows[0], vec),
//                    dot(self.rows[1], vec),
//                    dot(self.rows[2], vec),
//                    dot(self.rows[3], vec)] }
//     }
// }

// M*v
// impl Mul<Vec2> for Matrix {
//     type Output = Vec2;
//     fn mul(self, vec: Vec2) -> Vec2 {
//         Vec2 { v: [self.rows[0].dot(&vec),
//                    self.rows[1].dot(&vec),
//                    self.rows[2].dot(&vec),
//                    self.rows[3].dot(&vec)] }
//     }
// }

// impl MulAssign<f32> for Vec2 {
//     fn mul_assign(&mut self, k: f32) -> () {
//         *self = Self { v: [self.v[0] * k,
//                            self.v[1] * k,
//                            self.v[2] * k,
//                            self.v[3] * k] }
//     }
// }

// impl Div<f32> for Vec2 {
//     type Output = Self;

//     fn div(self, k: f32) -> Self {
//         Self { v: [self.v[0] / k,
//                    self.v[1] / k,
//                    self.v[2] / k,
//                    self.v[3] / k] }
//     }
// }

// impl DivAssign<f32> for Vec2 {
//     fn div_assign(&mut self, k: f32) -> () {
//         *self = Self { v: [self.v[0] / k,
//                            self.v[1] / k,
//                            self.v[2] / k,
//                            self.v[3] / k] }
//     }
// }

// pub fn dot(v1: &Vec2, v2: &Vec2) -> f32 {
//     v1.dot(v2)
// }

impl Vec2 {
    pub fn zero() -> Self {
        Self { v: [0.0, 0.0] }
    }

    // just implement stuff like this as it's needed (which it probably will be for sampling)
    // pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    //     Self { v: [x, y, z, w] }
    // }
    pub fn new(v: [f32; 2]) -> Self {
        Self { v }
    }

    // vector with values in range [0,1)
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        let unit = Uniform::new(0.0, 1.0); // maybe more uniform than otherwise
        Self { v: [rng.sample(unit), rng.sample(unit)] }
    }

    // return vector of n purportedly well-distributed random Vecs
    pub fn rand_arr(n: usize) -> Vec<Self> {
        let mut rng = thread_rng();
        let unitx = Uniform::new(0.0, 1.0); // maybe more uniform than otherwise
        let unity = Uniform::new(0.0, 1.0); // maybe more uniform than otherwise
        let mut ret = Vec::<Self>::new();
        for _ in 0..n {
            ret.push(Self { v: [rng.sample(unitx), rng.sample(unity)] });
        }
        ret
    }

    pub fn x(&self) -> f32 { self.v[0] }
    pub fn y(&self) -> f32 { self.v[1] }

    // pub fn len_squared(&self) -> f32 {
    //     self.dot(self)
    // }


    // pub fn len(&self) -> f32 {
    //     self.len_squared().sqrt()
    // }

    // pub fn dot(&self, other: &Vec2) -> f32 {
    //     self.v[0]*other.v[0] + self.v[1]*other.v[1] + self.v[2]*other.v[2] + self.v[3]*other.v[3]
    // }

    // pub fn cross(&self, other: &Vec2) -> Vec2 {
    //     //        |  î   ĵ   k̂ |
    //     // det of | a0  a1  a2 |
    //     //        | b0  b1  b2 |
    //     //
    //     // = (a1b2 - a2b1)î - (a2b0 - a0b2)ĵ + (a0b1 - a1b0)k̂
    //     //
    //     Vec2 { v: [self.v[1]*other.v[2] - self.v[2]*other.v[1],
    //                self.v[2]*other.v[0] - self.v[0]*other.v[2],
    //                self.v[0]*other.v[1] - self.v[1]*other.v[0],
    //                1.0] }
    // }

    // pub fn unit_vector(&v: &Vec2) -> Vec2 {
    //     v.normalize()
    // }

    // pub fn normalize(&self) -> Vec2 {
    //     let magnitude = self.len();
    //     Vec2 { v: [self.v[0] / magnitude,
    //                self.v[1] / magnitude,
    //                self.v[2] / magnitude] }
    // }
}

