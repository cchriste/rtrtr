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
}

impl Shiny {
    pub const fn new(c: Color) -> Self {
        Self { albedo: c }
    }
}

impl Material for Shiny {
    // Shinies always reflect, never absorb
    fn scatter(&self, ray: &Ray, hit: &HitRecord, indent_by: usize) -> LightScatter {
        let dir = random_direction(REFL_TYPE, hit.normal);
        if dir.dot(hit.normal) > 0.0 {
            return Attenuated(self.albedo, Ray::new(hit.point, dir));
        }
        Absorbed
    }
}
