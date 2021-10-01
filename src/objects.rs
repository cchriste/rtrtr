//
// Intersectable objects and groups thereof.
//

use std::fmt::Debug;

// The reason we have to create this "dual-trait" is because objects in Jumble
// are `Box<dyn Intersectable>`, which can't be presumed Debug.
// TODO: try creating mod.Intersectable so this can be Intersectable: mod.Intersectable + Debug
//pub trait Intersectable: Intersectable + Debug {}

use crate::utils::{Ray, Vector, dot, print_type_of, Range, OutsideRange, Matrix};  // Vector-y!

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

// result of Ray intersection with some Intersectable
pub enum Shot {
    Hit,
    Miss,
}

pub trait Intersectable {
    // intersect ray with this object or collection
    // - Range is global allowed distance along ray
    // - HitRecord is updated when there is an intersection
    // - indent is used to print debugging output
    fn intersect(&self, ray: &Ray, rng: &Range, hit: &mut HitRecord, indent_by: usize) -> Shot;
}

// buncha stuff that can be intersected, including itself
//#[derive(Debug)]
pub struct Jumble {
    arr: Vec<Box<dyn Intersectable>>,
    pub csys: Matrix,
    //bbox: BoundingBox, //TODO
}

impl Jumble {
    pub fn new() -> Jumble {
        Jumble {
            arr: Vec::new(),
            csys: Matrix::identity(),
        }
    }

    pub fn add(&mut self, obj: Box<dyn Intersectable>) {
        self.arr.push(obj)
    }

    //fn update_bbox() {... // TODO
}

impl Intersectable for Jumble {
    fn intersect(&self, ray: &Ray, rng: &Range, hit: &mut HitRecord, indent_by: usize) -> Shot {
        // ugh: this is two big lines just to indent by a few spaces; TODO: macro me?
        let indent = vec![' '; indent_by];
        let indent: String = indent.iter().cloned().collect();

        // transform ray into this Jumble's coordinate system
        // TODO: ...and use it 
        let new_ray = ray.transform(&self.csys);
        if crate::DEBUG {
            println!("transformed ray: {:?}", new_ray);
        }

        let mut hit_something = false;
        if crate::DEBUG {
            println!("{}Jumble::intersect, ray: {:?}", indent, ray);
        }
        for obj in self.arr.iter() {  // NOTE: we'll leave parallelization for another day
            if crate::DEBUG {
                //print_type_of(obj); // prints interfaces obj implements (i.e., not useful)
                //println!("obj: {:?}", obj); // can just be too much (e.g., array of objects)
                println!("{}rng: {:?}", indent, rng);
            }
            match obj.intersect(&ray, rng, hit, indent_by+2) {
                Shot::Hit => { // NOTE: a long-winded way to say `hit_something |= intersect()
                    hit_something = true;
                    if crate::DEBUG {
                        println!("{}new_hit obj! t: {:?} p: {:?} n: {:?}", indent,
                                 hit.t, hit.point, hit.normal);
                    }
                    // if new_hit.t.outside(rng) {
                    //     if crate::DEBUG {
                    //         println!("{}hit, but not closest",indent);
                    //     }
                    // }
                    // else {
                    //     if crate::DEBUG {
                    //         println!("{}closest so far",indent);
                    //     }
                    //     //rng.min = new_hit.t;
                    //     hit = new_hit;
                    // }
                },
                Shot::Miss => (),
            }
        }
        if hit_something { // TODO: I think this can be simpler; no need to keep track of hit_something.
            // TODO: transform hit point and its normal out of csys
            // remember, normal is trickier

            if crate::DEBUG {
                println!("{}returning {:?}", indent, hit);
            }

            return Shot::Hit;
        }
        if crate::DEBUG { println!("{}Missed Jumble altogether! Air rayyyyy!", indent);}
        Shot::Miss
    }
}


#[derive(Debug)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray, rng: &Range, hit: &mut HitRecord, indent_by: usize) -> Shot {
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
        if discriminant < 0.0 { return Shot::Miss; }
        let disqrt = discriminant.sqrt();
        let t0 = (-half_b - disqrt) / a;
        let t1 = (-half_b + disqrt) / a;
        // Check both:
        //  - if both < rng.min: Shot::Miss
        //  - if both > rng.min: take the closer
        //  - if at least one > rng.min: use the larger
        //  - check selected is inside range
        let t = if t0 > rng.min && t1 > rng.min { t0.min(t1) } else { t0.max(t1) };
        if t.outside(&rng) || t > hit.t { return Shot::Miss; }

        let point = ray.at(t);

        // set normal to oppose ray direction and indicate whether it's a
        // hit against front face or back face of geometry
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
        hit.t = t;
        hit.point = point;
        hit.normal = if front_face {normal} else {-normal};
        hit.front_face = front_face;
        Shot::Hit
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
