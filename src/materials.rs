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
    fn scatter(&self, ray: Ray, hit: &HitRecord, indent_by: usize) -> LightScatter;
    fn log(&self) -> String;
}

use core::fmt::Debug;
impl Debug for dyn Material {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.log())
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
    fn log(&self) -> String {
        format!("⊕ Lambertian c: {}", self.albedo)
    }

    // Lambertians always scatter, never absorb
    fn scatter(&self, ray: Ray, hit: &HitRecord, indent_by: usize) -> LightScatter {
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
                   Ray::new(hit.point, if dir.near_zero() { hit.normal } else { dir.normalize() }))
    }
}

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
    fn log(&self) -> String {
        format!("⊕ Shiny c: {} fuzz:{}",
                self.albedo, self.fuzz)
    }

    // Shinies always reflect, never absorb
    fn scatter(&self, ray: Ray, hit: &HitRecord, indent_by: usize) -> LightScatter {
        let indent = vec![' '; indent_by];
        let indent: String = indent.iter().cloned().collect();
        if DEBUG {
            println!("{} ⊕ Shiny.scatter: c:{} fuzz:{}", indent_by, self.albedo, self.fuzz);
            println!("{}ray: {:?}", indent, ray);
            println!("{}hit: {:?}", indent, hit);
        }

        let dir = ray.dir.reflect(&hit.normal) + self.fuzz*random_point_in_unit_sphere();
        if DEBUG {
            println!("{}dir: {:?}", indent, dir);
            println!("{}dir.dot(hit.normal): {}", indent, dir.dot(hit.normal));
        }
        if dir.dot(hit.normal) > 0.0 {
            if DEBUG {
                println!("{} reflected ray dir: {}", indent, dir);
            }
            return Attenuated(self.albedo, Ray::new(hit.point, dir.normalize()));
        }
        if DEBUG {
            println!("{} absorbed? must've been an abnormal day", indent);
        }
        Absorbed
    }
}

pub struct Transparent {
    pub albedo: Color,
    pub fuzz: f32,
    pub eta: f32,
}

impl Transparent {
    pub fn new(c: Color, fuzziness: f32, eta: f32) -> Self {
        Self { albedo: c,
               fuzz: if fuzziness > 1.0 { 1.0 } else { fuzziness },
               eta: if eta < 1.0 { 1.0 } else { eta },
        }
    }

    // Use Schlick's approximation for reflectance
    fn reflectance(&self, cos_theta: f32, src_eta: f32, dst_eta: f32) -> f32 {
        let mut r0 = (src_eta - dst_eta) / (src_eta + dst_eta);
        r0 = r0*r0;
        r0 + (1.0-r0) * (1.0-cos_theta).powi(5)
    }
}

impl Material for Transparent {
    fn log(&self) -> String{
        format!("⊕ Transparent c: {} η:{} fuzz:{}",
                self.albedo, self.eta, self.fuzz)
    }

    // reflect or refract, just pick one
    fn scatter(&self, ray: Ray, hit: &HitRecord, indent_by: usize) -> LightScatter {
        let indent = vec![' '; indent_by];
        let indent: String = indent.iter().cloned().collect();
        if DEBUG {
            println!("{} ⊕ Transparent.scatter of ray: {} at hit: {} using my c:{} η:{} fuzz:{}",
                     indent, ray, hit, self.albedo, self.eta, self.fuzz);
        }

        // FIXME: add previous material's eta to hit record (assume 1.0 for now)
        let src_eta = if hit.front_face { 1.0 } else { self.eta };
        let dst_eta = if hit.front_face { self.eta } else { 1.0 };
        let refraction_ratio = src_eta / dst_eta;
        let cos_theta = (-1.0*hit.normal.dot(ray.dir)).min(1.0); // *-1.0 so both in same direction
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let reflect = refraction_ratio * sin_theta > 1.0
            || self.reflectance(cos_theta, src_eta, dst_eta) > rand::thread_rng().gen();

        if reflect {
            let dir = ray.dir.reflect(&hit.normal) + self.fuzz*random_point_in_unit_sphere();
            if DEBUG {
                println!("{} reflected. ray dir: {}", indent, dir);
            }
            return Attenuated(self.albedo, Ray::new(hit.point, dir.normalize()));
        }
        else {
            if DEBUG {
                println!("\trefract this vector: {}", ray.dir);
                println!("\tfrom material with etai: {} to etat: {}", src_eta, dst_eta);
                println!("\thit normal: {}", hit.normal);
                let theta = cos_theta.acos();
                println!("\tcos_theta: {}", cos_theta);
                println!("\t∴ theta_i: {} deg ({} rad)", rad_to_deg(theta), theta);
            }

            let dir = ray.dir.refract(hit.normal, refraction_ratio, cos_theta)
                + self.fuzz*random_point_in_unit_sphere(); // TODO: give fuzzy refraction diff fuzz than reflections
            if DEBUG {
                println!("{} refracted. ray dir: {}", indent, dir.normalize());
            }
            // FIXME: add some albedo for how long (distance) it spent in the previous material
            // FIXME: set hit.eta when it goes through rather than assuming 1.0
            return Attenuated(self.albedo, Ray::new(hit.point, dir));
        }
    }
}
