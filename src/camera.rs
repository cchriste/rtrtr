use crate::*;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform; // generate more evenly distributed random values

#[derive(Debug)]
pub struct Camera {
    aperture: f32,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    right: Vec3,
    up: Vec3,
    botleft: Vec3,
    blur: Vec2,    // not sure if blur belongs in the camera, but it works
}

impl Camera {
    pub fn init(image_height: u32, aspect_ratio: f32, aperture: f32, sample_type: SampleType,
                vfov: f32, lf: Vec3, la: Vec3, vup: Vec3) -> Camera {
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
        // tan (1/2 fov) = 1/2 vp height    // assume focal len is 1
        //
        let viewport_height = 2.0 * (vfov.to_radians()/2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;
        let dist = (lf - la).len();
        let w = (lf - la).normalize();  // "backwards" so that w still points behind the camera
        let u = vup.normalize().cross(w).normalize(); // even though both unit must normalize bc not perp
        let v = w.cross(u);
        let right = u * viewport_width;//u * dist * viewport_width;
        let up = v * viewport_height;//v * dist * viewport_height;
        let botleft = lf - right/2.0 - up/2.0 - w;//*dist;
        let blur = Camera::get_blurriness(sample_type,
                                          aspect_ratio*image_height as f32, image_height as f32,
                                          viewport_width, viewport_height);
        println!("dist: {}",dist);
        println!("u: {}\nv: {}\nw: {}",u,v,w);
        println!("right: {}\nup: {}",right, up);
        Camera { aperture,
                 origin: lf,
                 u,v,w,
                 right,up,
                 botleft,
                 blur,
        }
    }

    pub fn gen_rays(&self, pct_x: f32, pct_y: f32, n: u32) -> Vec<Ray> {
        let mut rng = thread_rng();
        let unitx = Uniform::new(-0.5, 0.5); // maybe more uniform than otherwise
        let unity = Uniform::new(-0.5, 0.5); // maybe more uniform than otherwise
        // TODO: add jittering for more uniform coverage[]
        // for i in 0..self.jitters {
        //     for j in 0..self.jitters {
        //         let j: [f32; 2] = if DEBUG { [0.5, 0.5] } else { [jittersz*(i+rng.sample(unitx)), jittersz*(j+rng.sample(unity))] };
        // actually, use this: https://docs.rs/rand/0.5.0/rand/distributions/uniform/struct.Uniform.html
        let mut ret = Vec::<Ray>::new();
        for _ in 0..n {
            // let o: Vec3 = if DEBUG { self.origin } else { self.origin +
            //                                               self.u * self.aperture*rng.sample(unitx) +
            //                                               self.v * self.aperture*rng.sample(unity) };
            let o: Vec3 = self.origin; // debug
            let px: [f32; 2] = if DEBUG { [0.5, 0.5] } else { [rng.sample(unitx), rng.sample(unity)] };
            let dir =
                (self.botleft - o +
                 self.right*(pct_x + px[0]*self.blur[0]) +
                 self.up*(pct_y + px[1]*self.blur[1])).normalize();
            ret.push(Ray::new(o, dir));
        }
        ret
    }

    // TODO: it'd be fun to simulate zoomed pixels to see these distributions better
    // compute size of pixel in camera space
    fn get_blurriness(ref_type: SampleType,
                      image_width: f32, image_height: f32,
                      viewport_width: f32, viewport_height: f32) -> Vec2 {

        let blurriness = match ref_type {
            SampleType::PixelRatio => {
                // blurriness: [0.0050223214, 0.008928572] for IMAGE_HEIGHT = 200
                Vec2::new([1.0/image_width,
                           1.0/image_height]) // TODO: find the next issue here
            },
            SampleType::Blurry |
            SampleType::Blurrier => {
                // this is currently pixel dims, but it can get fancier
                // blurriness: [0.017867114, 0.018018018] for IMAGE_HEIGHT = 200
                Vec2::new([viewport_width/image_width,
                           viewport_height/image_height])
            },
        };
        println!("pixelsize: {:?}", blurriness);

        blurriness
    }
}

pub enum SampleType {
    PixelRatio,
    Blurry,
    Blurrier,
}

