use crate::*;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform; // generate more evenly distributed random values

#[derive(Debug)]
pub struct Camera {
    lens_radius: f32,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    right: Vec3,
    up: Vec3,
    botleft: Vec3,
    blur: Vec2, // this is the pixel size in camera space
    dist_to_focus: f32,

    rng: rand::rngs::ThreadRng,
    unitx: Uniform<f32>,
    unity: Uniform<f32>,
    // actually, use this: https://docs.rs/rand/0.5.0/rand/distributions/uniform/struct.Uniform.html
}

impl Camera {
    pub fn init(image_height: u32, aspect_ratio: f32, aperture: f32, sample_type: SampleType,
                vfov: f32, lf: Vec3, la: Vec3, vup: Vec3, dist_to_focus: f32) -> Camera {
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
        let w = (lf - la).normalize();  // "backwards" so that w still points behind the camera
        let u = vup.normalize().cross(w).normalize(); // even though both unit must normalize bc not perp
        let v = w.cross(u);
        let right = u * dist_to_focus * viewport_width;
        let up = v * dist_to_focus * viewport_height;
        let botleft = lf - right/2.0 - up/2.0 - w*dist_to_focus;
        let blur = Camera::get_blurriness(sample_type,
                                          aspect_ratio*image_height as f32, image_height as f32,
                                          viewport_width, viewport_height);
        println!("u: {}\nv: {}\nw: {}",u,v,w);
        println!("right: {}\nup: {}",right, up);
        Camera { lens_radius: aperture/2.0,
                 origin: lf,
                 u,v,w,
                 right,up,
                 botleft,
                 blur,
                 dist_to_focus,

                 rng: thread_rng(),
                 unitx: Uniform::new(-1.0, 1.0),
                 unity: Uniform::new(-1.0, 1.0),
        }
    }

    fn random_point_in_unit_disc(&mut self) -> Vec3 {
        loop {
            let v = Vec2::new([self.rng.sample(self.unitx), self.rng.sample(self.unitx)]);
            if v.len_squared() < 1.0 {
                return Vec3::new([v[0], v[1], 0.0]);
            }
        }
    }

    pub fn gen_rays(&mut self, pct_x: f32, pct_y: f32, n: u32) -> Vec<Ray> {
        // TODO: add jittering for more uniform coverage[]
        // for i in 0..self.jitters {
        //     for j in 0..self.jitters {
        //         let j: [f32; 2] = if DEBUG { [0.5, 0.5] } else { [jittersz*(i+rng.sample(unitx)), jittersz*(j+rng.sample(unity))] };
        let mut ret = Vec::<Ray>::new();
        for _ in 0..n {
            let rand = self.random_point_in_unit_disc();
            let offset = self.u * self.lens_radius*rand.x() + self.v * self.lens_radius*rand.y();
            let o: Vec3 = if DEBUG { self.origin } else { self.origin + offset };
            let px = if DEBUG { Vec2::new([0.0, 0.0]) } else { Vec2::new([self.rng.sample(self.unitx),
                                                                          self.rng.sample(self.unitx)]) };
            let dir =
                (self.botleft - o +
                 self.right*(pct_x + px[0]*self.blur[0]) +
                 self.up*(pct_y + px[1]*self.blur[1])).normalize();
            ret.push(Ray::new(o, dir));
        }
        ret
    }

    // TODO: it'd be fun to simulate zoomed pixels to see these distributions better
    // compute size of pixel in camera space, return px_size / 2
    fn get_blurriness(ref_type: SampleType,
                      image_width: f32, image_height: f32,
                      viewport_width: f32, viewport_height: f32) -> Vec2 {

        let blurriness = match ref_type {
            SampleType::PixelRatio => {
                // blurriness: [0.0050223214, 0.008928572] for IMAGE_HEIGHT = 200
                Vec2::new([0.5/image_width,
                           0.5/image_height]) // TODO: find the next issue here
            },
            SampleType::Blurry |
            SampleType::Blurrier => {
                // this is currently pixel dims, but it can get fancier
                // blurriness: [0.017867114, 0.018018018] for IMAGE_HEIGHT = 200
                Vec2::new([viewport_width/image_width/2.0,
                           viewport_height/image_height/2.0])
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

