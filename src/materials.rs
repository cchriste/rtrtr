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
    fn reflectance(&self, cos_theta: f32, ref_ratio: f32) -> f32 {
        let mut r0 = (1.0-ref_ratio) / (1.0+ref_ratio);
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
    fn scatter(&self, ray: &Ray, hit: &HitRecord, indent_by: usize) -> LightScatter {
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
        // TODO: instead of dividing by length of ray, just take min(cos_theta,1.0) (in other places, too)
        let cos_theta = -1.0*hit.normal.dot(ray.dir) / ray.dir.len(); // *-1.0 so both are in same direction
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        // reflect if eta_ratio*sin(theta) > 1.0 (can't refract) or [more often if] reflectance is high
        let reflect = refraction_ratio * sin_theta > 1.0 ||
            self.reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen();
        if reflect {
            let dir = ray.dir.reflect(&hit.normal) + self.fuzz*random_point_in_unit_sphere();
            if DEBUG {
                println!("{} reflected. ray dir: {}", indent, dir);
            }
            return Attenuated(self.albedo, Ray::new(hit.point, dir));
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
                println!("{} refracted. ray dir: {}", indent, dir);
            }
            // FIXME: add some albedo for how long (distance) it spent in the previous material
            // FIXME: set hit.eta when it goes through rather than assuming 1.0
            return Attenuated(self.albedo, Ray::new(hit.point, dir));
        }
    }
}
