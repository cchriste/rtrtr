use crate::utils::{Ray, Vec3, dot, print_type_of, Range, OutsideRange};
use rand::{Rng, thread_rng};
use rand::distributions::Uniform; // generate more evenly distributed random values

#[derive(Debug)]
pub struct Camera {
    viewport_height: f32,
    viewport_width: f32,
    aspect_ratio: f32,
    jitter: [f32; 2],
    focal_length: f32,
    vfov: f32, // in degrees
    origin: Vec3,
    right: Vec3,
    up: Vec3,
    z: Vec3,  // questionable to keep both z and -z (look)
    look: Vec3,
    botleft: Vec3,  // TODO: alias Point as Vec3 (will struct Point(Vec3) be enough? I think I've already done this... maybe in hello_world? prints kinda wierd though... I think there is a print trait though.
}

impl Camera {
    pub fn init(aspect_ratio: f32, focal_length: f32, vfov: f32, jitter: [f32; 2]) -> Camera {
        //
        // Compute viewport height based on vertical field of view.
        //
        //     /|\
        //    / | 1/2 vp_height
        //   /  |/
        //fov---|
        //   \  |
        //    \ |
        //     \|
        //
        // tan (1/2 fov) = 1/2 vp height / focal len
        //
        let viewport_height = focal_length*2.0 * (vfov.to_radians()/2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Vec3::zero(); // prolly want this as a param, as well as look, then right and up get calculated here (even for orthogonal)
        let right = Vec3::new(viewport_width, 0.0, 0.0);
        let up = Vec3::new(0.0, viewport_height, 0.0);
        let z = right.cross(&up).normalize();
        let look = z * -1.0;
        let botleft = origin - right/2.0 - up/2.0 + look*focal_length;
        Camera { aspect_ratio,
                 vfov,
                 focal_length,
                 viewport_height,
                 viewport_width,
                 origin,
                 right,
                 up,
                 z,
                 look,
                 botleft,
                 jitter,
        }
    }

    // pub fn gen_ray(&self, pct_x: f32, pct_y: f32) -> Ray {
    //     let mut rng = thread_rng();
    //     let unit = Uniform::new(0.0, 1.0); // maybe more uniform than otherwise
    //     let j: [f32; 2] = [rng.sample(unit), rng.sample(unit)];
    //     Ray::new(self.origin,
    //              self.botleft - self.origin +
    //              self.right*(pct_x + (j[0]-0.5)*self.jitter[0]) +
    //              self.up*(pct_y + (j[1]-0.5)*self.jitter[1]))
    // }

    pub fn gen_rays(&self, pct_x: f32, pct_y: f32, n: usize) -> Vec<Ray> {
        let mut rng = thread_rng();
        let unitx = Uniform::new(-0.5, 0.5); // maybe more uniform than otherwise
        let unity = Uniform::new(-0.5, 0.5); // maybe more uniform than otherwise
        let mut ret = Vec::<Ray>::new();
        for _ in 0..n {
            let j: [f32; 2] = [rng.sample(unitx), rng.sample(unity)];
            ret.push(Ray::new(self.origin,
                              self.botleft - self.origin +
                              self.right*(pct_x + j[0]*self.jitter[0]) +
                              self.up*(pct_y + j[1]*self.jitter[1])));
        }
        ret
    }
}
