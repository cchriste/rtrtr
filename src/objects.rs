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
        Range::new(0.0, f32::INFINITY)
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
    fn intersect(&self, ray: &Ray, mut rng: &Range) -> Result {
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

    pub fn intersect(&self, ray: &Ray, mut rng: &Range) -> Result {
        let mut record = HitRecord::new();
        for obj in &self.arr {
            //println!("wtf is obj: {:?}", obj);
            match obj.intersect(&ray, &rng) {
                Result::Hit(hit) => {
                    // println!("hit! {:?} {:?} {:?}",
                    //          hit.t, hit.point, hit.normal);
                    if hit.t < record.t { record = hit; } // TODO: also make sure it's within rng (yeah, easy, but I'm tired)
                },
                Result::Miss => (),
            }
        }
        if record.t > rng.min && record.t < rng.max {
            // FIXME this isn't what min and max are for: they're supposed to be used to throw out sample that aren't in range
            // if record.t < rng.min { rng.min = record.t };
            // if record.t > rng.max { rng.max = record.t };
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
    fn intersect(&self, ray: &Ray, mut rng: &Range) -> Result {
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
            // some Rust trick to just initialize them...
            center,
            radius  // do I need a comma here? omitting to check
        }
    }
}
