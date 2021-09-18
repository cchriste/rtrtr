// intersectable objects

// expects it to be in /objects/utils.rs :(
// mod utils;
// use utils::Vector;

use crate::utils::{Ray, Vector};  // Vector-y!

pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector, radius: f32) -> Sphere {
        Sphere {
            // some Rust trick to just initialize them...
            center,
            radius  // do I need a comma here? omitting to check
        }
    }

    pub fn intersect(&self, ray: &Ray) -> f32 {
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
        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-half_b - discriminant.sqrt() ) / a;
        }
    }
}
