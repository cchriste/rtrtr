//
// the shiny, the clear, and neutral grey... materials
//

use crate::*;
use LightScatter::{ Attenuated, Absorbed };
use rand::{thread_rng, Rng};
use std::fmt;

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
        write!(f, "Material (???)") //how do I add details? Maybe impl Debug for them?)")
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
        let indent = vec![' '; indent_by];
        let indent: String = indent.iter().cloned().collect();
        if DEBUG {
            println!("{} ⊕ Lambertian.scatter: c:{}", indent_by, self.albedo);
        }

        let dir = random_direction(REFL_TYPE, hit.normal);
        if DEBUG {
            println!("{} reflected ray dir: {})", indent, dir);
        }
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
        let indent = vec![' '; indent_by];
        let indent: String = indent.iter().cloned().collect();
        if DEBUG {
            println!("{} ⊕ Shiny.scatter: c:{} fuzz:{}", indent_by, self.albedo, self.fuzz);
        }

        let dir = ray.dir.reflect(&hit.normal) + self.fuzz*random_point_in_unit_sphere();
        if dir.dot(hit.normal) > 0.0 {
            if DEBUG {
                println!("{} reflected ray dir: {}", indent, dir);
            }
            return Attenuated(self.albedo, Ray::new(hit.point, dir));
        }
        if DEBUG {
            println!("{} absorbed? must've been an abnormal day", indent);
        }
        Absorbed
    }
}

//#[derive(Debug)]
pub struct Transparent {
    pub albedo: Color,
    pub fuzz: f32,
    pub eta: f32,
}

// none of these are working... too tired to figure it out right now
// impl fmt::Display for Transparent {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "⊕ Transparent c:{} η:{} fuzz:{}",
//                self.albedo, self.eta, self.fuzz)
//     }
// }
// impl Debug for Transparent {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "⊕ Transparent c:{} η:{} fuzz:{}",
//                self.albedo, self.eta, self.fuzz)
//     }
// }

impl Transparent {
    pub fn new(c: Color, fuzziness: f32, eta: f32) -> Self {
        Self { albedo: c,
               fuzz: if fuzziness > 1.0 { 1.0 } else { fuzziness },
               eta: if eta < 1.0 { 1.0 } else { eta },
        }
    }
}

impl Material for Transparent {
    // reflect or refract, just pick one
    fn scatter(&self, ray: &Ray, hit: &HitRecord, indent_by: usize) -> LightScatter {
        let indent = vec![' '; indent_by];
        let indent: String = indent.iter().cloned().collect();
        if DEBUG {
            println!("{} ⊕ Transparent.scatter of ray: {} at hit: {} using my c:{} η:{} fuzz:{}",
                     indent, ray, hit, self.albedo, self.eta, self.fuzz);
        }

        let reflect = false; //rand::thread_rng().gen_bool(0.5);
        if reflect {
            let dir = ray.dir.reflect(&hit.normal) + self.fuzz*random_point_in_unit_sphere();
            if DEBUG {
                println!("{} reflected. ray dir: {}", indent, dir);
            }
            return Attenuated(self.albedo, Ray::new(hit.point, dir));
        }
        else {
            // FIXME: add previous material's eta to hit record (assume 1.0 for now)
            let src_eta = if hit.front_face { 1.0 } else { self.eta };
            let dst_eta = if hit.front_face { self.eta } else { 1.0 };
            let dir = ray.dir.refract(hit.normal, src_eta, dst_eta ) + self.fuzz*random_point_in_unit_sphere(); // FIXME: fuzzy refraction seems okay, but same fuzziness as reflection?
            if DEBUG {
                println!("{} refracted. ray dir: {}", indent, dir);
            }
            // FIXME: add some albedo for how long it spent in the previous material
            // FIXME: set hit.eta when it goes through
            return Attenuated(self.albedo, Ray::new(hit.point, dir));
        }
    }
}
