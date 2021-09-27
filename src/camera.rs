use crate::utils::{Ray, Vector, dot, print_type_of, Range, OutsideRange};
use rand::Rng;

#[derive(Debug)]
pub struct Camera {
    viewport_height: f32,
    viewport_width: f32,
    aspect_ratio: f32,
    jitter: [f32; 2],
    focal_length: f32,
    vfov: f32, // in degrees
    origin: Vector,
    right: Vector,
    up: Vector,
    z: Vector,  // questionable to keep both z and -z (look)
    look: Vector,
    botleft: Vector,  // TODO: alias Point as Vector (will struct Point(Vector) be enough? I think I've already done this... maybe in hello_world? prints kinda wierd though... I think there is a print trait though.
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
        let origin = Vector::zero(); // prolly want this as a param, as well as look, then right and up get calculated here (even for orthogonal)
        let right = Vector::init(viewport_width, 0.0, 0.0);
        let up = Vector::init(0.0, viewport_height, 0.0);
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

    pub fn gen_ray(&self, pct_x: f32, pct_y: f32) -> Ray {
        let j: [f32; 2] = rand::thread_rng().gen();
        Ray::new(self.origin,
                 self.botleft - self.origin +
                 self.right*(pct_x + (j[0]-0.5)*self.jitter[0]) +
                 self.up*(pct_y + (j[1]-0.5)*self.jitter[1]))
    }
}
