// intersectable objects

use crate::utils::{Ray, Vector};  // Vector-y!

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
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord { t: f32::INFINITY, point: Vector::zero(), normal: Vector::zero() }
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
//#[derive(Intersectable)] // TODO: start with Sphere
pub struct Jumble<T> {
    arr: Vec<T>,
}

impl<T: Intersectable + std::fmt::Debug> Jumble<T> {
    pub fn new() -> Jumble<T> {
        Jumble::<T> { arr: Vec::new() }
    }

    pub fn add(&mut self, obj: T) {
        self.arr.push(obj)
    }

    // TODO: move this to impl Intersectable for Jumble {...
    pub fn intersect(&self, ray: &Ray, rng: &mut Range) -> Result {
        let mut hit_something = false;
        let mut record = HitRecord::new();
        //println!("Jumble::intersect");
        for obj in &self.arr {
            if crate::DEBUG {
                println!("obj: {:?}", obj);
                println!("rng: {:?}", rng);
            }
            match obj.intersect(&ray, rng) { // TODO: try modifying this in Sphere::intersect to see if it's passing a ref... after I fix the bug if it not working anymore
                Result::Hit(hit) => {
                    // println!("hit! t: {:?} p: {:?} n: {:?}",
                    //          hit.t, hit.point, hit.normal);
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

impl Intersectable for Sphere {
    // (just ignore rng for the object and let Jumble sort it out)
    fn intersect(&self, ray: &Ray, _rng: &mut Range) -> Result {
        let oc = ray.origin - self.center;
        //println!("{:?}",oc);
        let a = ray.dir.len_squared();
        //println!("{:?}",a);
        let half_b = oc.dot(&ray.dir);
        //println!("{:?}",b);
        let c = oc.len_squared() - self.radius*self.radius;
        //println!("{:?}",c);
        let discriminant = half_b*half_b - a*c;
        //println!("{:?}",discriminant);
        if discriminant >= 0.0 {
            let t = (-half_b - discriminant.sqrt() ) / a;
            let point = ray.at(t);
            let normal = (ray.at(t) - self.center).normalize();
            return Result::Hit(HitRecord { t, point, normal });
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
