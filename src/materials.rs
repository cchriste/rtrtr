//
// the shiny, the clear, and neutral grey... materials
//

use crate::*;
use LightScatter::{ Attenuated, Absorbed };

pub enum LightScatter {
    Attenuated(Color, Ray),
    Absorbed,
}

// interaction of [a ray of] light with a material
pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, indent_by: usize) -> LightScatter;
}

use core::fmt::Debug;
impl Debug for dyn Material {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Material (how do I add details? Maybe impl Debug for them?)")
    }
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub const fn new(c: Color) -> Self {
        Self { albedo: c }
    }
}

impl Material for Lambertian {
    // Lambertians always scatter, never absorb
    fn scatter(&self, ray: &Ray, hit: &HitRecord, indent_by: usize) -> LightScatter {
        let dir = random_direction(REFL_TYPE, hit.normal);
        Attenuated(self.albedo,
                   Ray::new(hit.point, if dir.near_zero() { hit.normal } else { dir }))
    }
}

#[derive(Debug)]
pub struct Shiny {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Shiny {
    pub fn new(c: Color, fuzziness: f32) -> Self {
        Self { albedo: c,
               fuzz: if fuzziness > 1.0 { 1.0 } else { fuzziness },
        }
    }
}

impl Material for Shiny {
    // Shinies always reflect, never absorb
    fn scatter(&self, ray: &Ray, hit: &HitRecord, indent_by: usize) -> LightScatter {
        let dir = ray.dir.reflect(&hit.normal) + self.fuzz*random_point_in_unit_sphere();
        if dir.dot(hit.normal) > 0.0 {
            if DEBUG {
                println!("shiny hit! (reflected ray dir: {})", dir);
            }
            return Attenuated(self.albedo, Ray::new(hit.point, dir));
        }
        if DEBUG {
            println!("shiny absorbed? must've been an abnormal day");
        }
        Absorbed
    }
}
