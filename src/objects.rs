//
// Intersectable objects and groups thereof.
//

use std::fmt;

// The reason we have to create this "dual-trait" is because objects in Jumble
// are `Box<dyn Intersectable>`, which can't be presumed Debug.
// TODO: try creating mod.Intersectable so this can be Intersectable: mod.Intersectable + Debug
//pub trait Intersectable: Intersectable + Debug {}
// FIXME: the solution is to fmt::debug like for dyn Material (works, but useful?)

use crate::*;
use crate::materials::*;
pub use std::rc::Rc;  // FIXME: purportedly we don't have to `use` in every module, but not working

// hit record
#[derive(Debug)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl fmt::Display for HitRecord{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "✔{} p{} n{} ff:{} mat:{:?}",
               self.t, self.point, self.normal, self.front_face, self.material)
    }
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: f32::INFINITY,
            point: Vec3::zero(),
            normal: Vec3::zero(),
            front_face: true,
            material: Rc::new(Lambertian::new(Color::white())),
        }
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
    pub name: String,
    arr: Vec<Rc<dyn Intersectable>>,
    csys: Matrix,
    csys_inv: Matrix,
    csys_inv_xpose: Matrix,
    //bbox: AABoundingBox, //TODO
}

impl Jumble {
    pub fn new() -> Jumble {
        Jumble {
            name: String::from("anon"),
            arr: Vec::new(),
            csys: Matrix::identity(),
            csys_inv: Matrix::identity().inverse(),
            csys_inv_xpose: Matrix::identity().inverse().transpose(),
        }
    }

    pub fn csys(&self) -> Matrix {
        self.csys
    }

    pub fn add(&mut self, obj: Rc<dyn Intersectable>) {
        self.arr.push(obj)
    }

    pub fn set_csys(&mut self, csys: Matrix) {
        self.csys = csys;
        self.csys_inv = self.csys.inverse();
        self.csys_inv_xpose = self.csys_inv.transpose();
    }

    //fn update_bbox() {... // TODO
}

impl Intersectable for Jumble {
    fn intersect(&self, ray: &Ray, rng: &Range, hit: &mut HitRecord, indent_by: usize) -> Shot {
        // ugh: this is two big lines just to indent by a few spaces; TODO: macro me?
        let indent = vec![' '; indent_by];
        let indent: String = indent.iter().cloned().collect();
        if crate::DEBUG {
            println!("{}intersect {{{}}} with {}", indent, self.name, ray);

            // println!("{}csys: ",indent);
            // println!("{}",self.csys);
            // println!("{}csys_inv: ",indent);
            // println!("{}",self.csys_inv);
            // println!("{}csys_inv_xpose: ",indent);
            // println!("{}",self.csys_inv_xpose);
        }

        // transform ray into this Jumble's coordinate system
        let ray = ray.transform(&self.csys_inv);
        if crate::DEBUG {
            println!("{} - transformed {}", indent, ray);
        }

        let mut hit_something = false;
        for obj in self.arr.iter() {  // NOTE: we'll leave parallelization for another day
            if crate::DEBUG {
                //print_type_of(obj); // prints interfaces obj implements (i.e., not useful)
                //println!("obj: {:?}", obj); // can just be too much (e.g., array of objects)
                // println!("{}rng: {:?}", indent, rng);
            }
            match obj.intersect(&ray, rng, hit, indent_by+2) {
                Shot::Hit => { // NOTE: a long-winded way to say `hit_something |= intersect()
                    hit_something = true;
                    // if crate::DEBUG {
                    //     println!("{} - hit", indent);
                    // }
                },
                Shot::Miss => { 
                    // if crate::DEBUG {
                    //     println!("{} - miss",indent);
                    // }
                },
            }
        }
        if hit_something {
            // TODO: transform hit point and its normal out of csys
            // remember, normal is trickier
            if crate::DEBUG {
                //println!("{} - pre-xform: {}", indent, hit);
            }
            hit.point = self.csys.apply_to_point(hit.point);
            hit.normal = self.csys_inv_xpose.apply_to_vector(hit.normal);

            if crate::DEBUG {
                //println!("{} - pst-xform: {}", indent, hit);
            }
            return Shot::Hit;
        }
        //if crate::DEBUG { println!("{}air rayyyyy!", indent);}
        Shot::Miss
    }
}


#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "⦿ c{} rad:{:.2}", self.center, self.radius)
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray, rng: &Range, hit: &mut HitRecord, indent_by: usize) -> Shot {
        let indent = vec![' '; indent_by];
        let indent: String = indent.iter().cloned().collect();
        if crate::DEBUG {
            println!("{}{}", indent, self);
        }
        let oc = ray.origin - self.center;
        let a = ray.dir.len_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.len_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if DEBUG {
            println!("rng: {:?}", rng);
            println!("hit.t: {}", hit.t);
        }
        if discriminant < 0.0 {
            if crate::DEBUG {
                println!("{} - miss", indent);
            }
            return Shot::Miss;
        }
        let disqrt = discriminant.sqrt();
        let t0 = (-half_b - disqrt) / a;
        let t1 = (-half_b + disqrt) / a;
        // Check both:
        //  - if both < rng.min: Shot::Miss
        //  - if both > rng.min: take the closer
        //  - if at least one > rng.min: use the larger
        //  - check selected is inside range
        let t = if t0 > rng.min && t1 > rng.min { t0.min(t1) } else { t0.max(t1) };
        if DEBUG {
            println!("t: {}", t); // what if t is behind ray origin??
            println!("t0: {}, t1: {}", t0, t1);
        }
        if t.outside(&rng) || t > hit.t {
            if crate::DEBUG {
                println!("{} - miss", indent);
            }
            return Shot::Miss;
        }

        hit.point = ray.at(t);

        // set normal to oppose ray direction and indicate whether it's a
        // hit against front face or back face of geometry
        let normal = (hit.point - self.center) / self.radius; // ** negative radius inverts normal trick **
        hit.front_face = if dot(normal, ray.dir) < 0.0 {true} else {false};

        hit.t = t;
        hit.normal = if hit.front_face {normal} else {-normal};
        hit.material = Rc::clone(&self.material);

        if crate::DEBUG {
            // println!("oc: {}",oc);
            // println!("a: {}",a);
            // println!("half_b: {}",half_b);
            // println!("c: {}",c);
            // println!("disc: {}",discriminant);
            println!("{} - hit! {}",indent, hit);
        }

        Shot::Hit
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material: mat,
        }
    }
}
