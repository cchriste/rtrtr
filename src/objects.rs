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
    //     vec3 oc = r.origin() - center;
        let oc = ray.origin.sub(&self.center);
        //println!("{:?}",oc);
    //     auto a = dot(r.direction(), r.direction());
        let a = ray.dir.dot(&ray.dir);
        //println!("{:?}",a);
    //     auto b = 2.0 * dot(oc, r.direction());
        let b = 2.0 * oc.dot(&ray.dir);
        //println!("{:?}",b);
    //     auto c = dot(oc, oc) - radius*radius;
        let c = oc.dot(&oc) - self.radius*self.radius;
        //println!("{:?}",c);
    //     auto discriminant = b*b - 4*a*c;
        let discriminant = b*b - 4.0*a*c;
        //println!("{:?}",discriminant);
    //     return (discriminant > 0);
        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()) / (2.0*a);
        }
    }
}
