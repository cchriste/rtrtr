// intersectable objects

use std::fmt::Debug;
pub trait IntersectableDebug: Intersectable + Debug {}
use crate::utils::{Ray, Vector, dot};  // Vector-y!

#[derive(Debug)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}

// a range, such as [tmin, tmax)
impl Range {
    pub fn default() -> Self {
        Range::new(f32::INFINITY, f32::INFINITY)
    }

    pub fn new(min: f32, max: f32) -> Self {
        Range { min, max }
    }
}

// hit record
pub struct HitRecord {
    pub point: Vector,
    pub normal: Vector,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord { t: f32::INFINITY, point: Vector::zero(), normal: Vector::zero(), front_face: true }
    }
}

// result of Ray intersection with some Jumble
pub enum Result {  // TODO: rename me to... ?
    Hit(HitRecord),
    Miss
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, rng: &mut Range) -> Result {
        return Result::Miss;
    }
}

// buncha stuff that can be intersected, including itself
#[derive(Debug)]
pub struct Jumble {
    arr: Vec<Box<dyn IntersectableDebug>>,
}

impl Jumble {
    pub fn new() -> Jumble {
        Jumble { arr: Vec::new() }
    }

    //pub fn add(&mut self, obj: Box<dyn Intersectable>) {
    pub fn add(&mut self, obj: Box<dyn IntersectableDebug>) {
        self.arr.push(obj)
    }
}

impl IntersectableDebug for Jumble {}

impl Intersectable for Jumble {
    fn intersect(&self, ray: &Ray, rng: &mut Range) -> Result {
        let mut hit_something = false;
        let mut record = HitRecord::new();
        //println!("Jumble::intersect");
        for obj in self.arr.iter() {
            if crate::DEBUG {
                println!("obj: {:?}", obj);
                println!("rng: {:?}", rng);
            }
            match obj.intersect(&ray, rng) { // TODO: try modifying this in Sphere::intersect to see if it's passing a ref... after I fix the bug if it not working anymore
                Result::Hit(hit) => {
                    if crate::DEBUG {
                        println!("hit! t: {:?} p: {:?} n: {:?}",
                                 hit.t, hit.point, hit.normal);
                    }
                    if hit.t < rng.max && hit.t < rng.min {
                        rng.min = hit.t;
                        record = hit;
                    }
                    hit_something = true;
                },
                Result::Miss => (),
            }
        }
        if hit_something { // TODO: I think this can be simpler, maybe no need to keep track of hit_something.
            return Result::Hit(record);
        }
        Result::Miss
    }
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
}

impl IntersectableDebug for Sphere {}

impl Intersectable for Sphere {
    // (just ignore rng for the object and let Jumble sort it out)
    fn intersect(&self, ray: &Ray, _rng: &mut Range) -> Result {
        let oc = ray.origin - self.center;
        let a = ray.dir.len_squared();
        let half_b = oc.dot(&ray.dir);
        let c = oc.len_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant >= 0.0 {
            let disqrt = discriminant.sqrt();
            let t =
                if (-half_b - disqrt) >= 0.0 {
                    (-half_b - disqrt) / a
                } else {
                    (-half_b + disqrt) / a
                };
            if t < 0.0 {
                return Result::Miss;
            }
            let point = ray.at(t);

            // set normal to oppose ray direction and indicate whether it's a
            // hit against front face or back face of geometry (TODO: move
            // to HitRecord itself when more geometry is added)
            let normal = (ray.at(t) - self.center).normalize();
            let front_face = if dot(&normal, &ray.dir) < 0.0 {true} else {false};

            if crate::DEBUG {
                println!("oc: {:?}",oc);
                println!("a: {:?}",a);
                println!("half_b: {:?}",half_b);
                println!("c: {:?}",c);
                println!("disc: {:?}",discriminant);
                println!("t: {:?}", t);
                println!("ff: {}, normal: {:?}", front_face, normal);
            }

            return Result::Hit(HitRecord { t, point, normal: if front_face {normal} else {-normal}, front_face });
        }
        Result::Miss
    }
}

impl Sphere {
    pub fn new(center: Vector, radius: f32) -> Sphere {
        Sphere {
            center,
            radius
        }
    }
}
