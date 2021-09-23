// intersectable objects

use std::fmt::Debug;
pub trait IntersectableDebug: Intersectable + Debug {}
use crate::utils::{Ray, Vector, dot, print_type_of, Range, OutsideRange};  // Vector-y!
//TODO: use core::ops::Range instead of this (https://doc.rust-lang.org/core/ops/struct.Range.html)

// hit record
#[derive(Debug)]
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
    fn intersect(&self, ray: &Ray, rng: &mut Range, indent_by: usize) -> Result {
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
    fn intersect(&self, ray: &Ray, rng: &mut Range, indent_by: usize) -> Result {
        let indent = vec![' '; indent_by];
        let indent: String = indent.iter().cloned().collect();
        let mut hit_something = false;
        let mut record = HitRecord::new();
        if crate::DEBUG {
            println!("{}Jumble::intersect, ray: {:?}", indent, ray);
        }
        for obj in self.arr.iter() {
            if crate::DEBUG {
                //print_type_of(obj); // prints interfaces obj implements (i.e., not useful)
                //println!("obj: {:?}", obj); // can just be too much (e.g., array of objects)
                println!("{}rng: {:?}", indent, rng);
            }
            match obj.intersect(&ray, rng, indent_by+2) {
                Result::Hit(hit) => {
                    hit_something = true;
                    if crate::DEBUG {
                        println!("{}hit obj! t: {:?} p: {:?} n: {:?}", indent,
                                 hit.t, hit.point, hit.normal);
                    }
                    if hit.t.outside(rng) {
                        if crate::DEBUG {
                            println!("{}hit, but not closest",indent);
                        }
                    }
                    else {
                        if crate::DEBUG {
                            println!("{}closest so far",indent);
                        }
                        rng.min = hit.t;
                        record = hit;
                    }
                },
                Result::Miss => (),
            }
        }
        if hit_something { // TODO: I think this can be simpler, maybe no need to keep track of hit_something.
            //println!("{}returning {:?}", indent, record);
            return Result::Hit(record);
        }
        if crate::DEBUG { println!("{}Missed Jumble altogether! Air rayyyyy!", indent);}
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
    fn intersect(&self, ray: &Ray, rng: &mut Range, indent_by: usize) -> Result {
        let indent = vec![' '; indent_by];
        let indent: String = indent.iter().cloned().collect();
        if crate::DEBUG {
            println!("{}Sphere {:?}, ray: {:?}, rng: {:?}", indent, self, ray, rng);
        }
        let oc = ray.origin - self.center;
        let a = ray.dir.len_squared();
        let half_b = oc.dot(&ray.dir);
        let c = oc.len_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 { return Result::Miss; }
        let disqrt = discriminant.sqrt();
        let t0 = (-half_b - disqrt) / a;
        let t1 = (-half_b + disqrt) / a; // half decent optimization won't compute this if unnecessary
        let t = if t0.outside(rng) { t1 } else { t0 };
        if t.outside(rng) { return Result::Miss; }
        //if rng.outside(t) { return Result::Miss; }
        let point = ray.at(t);

        // set normal to oppose ray direction and indicate whether it's a
        // hit against front face or back face of geometry (TODO: move
        // to HitRecord itself when more geometry is added)
        let normal = (ray.at(t) - self.center).normalize();
        let front_face = if dot(&normal, &ray.dir) < 0.0 {true} else {false};

        if crate::DEBUG {
            // println!("oc: {:?}",oc);
            // println!("a: {:?}",a);
            // println!("half_b: {:?}",half_b);
            // println!("c: {:?}",c);
            // println!("disc: {:?}",discriminant);
            println!("{}t: {:?}", indent, t);
            println!("{}ff: {}, normal: {:?}", indent, front_face, normal);
        }

        //println!("{}returning a hit",indent);
        return Result::Hit(HitRecord { t, point, normal: if front_face {normal} else {-normal}, front_face });
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
