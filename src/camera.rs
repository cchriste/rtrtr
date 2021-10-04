use crate::utils::{Ray, Vec2, Vec3, dot, print_type_of, Range, OutsideRange};
use rand::{Rng, thread_rng};
use rand::distributions::Uniform; // generate more evenly distributed random values

#[derive(Debug)]
pub struct Camera {
    viewport_height: f32,
    viewport_width: f32,
    aspect_ratio: f32,
    jitter: Vec2,
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
    pub fn init(aspect_ratio: f32, focal_length: f32, vfov: f32, image_height: u32, sample_type: SampleType) -> Camera {
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
        let right = Vec3::new([viewport_width, 0.0, 0.0]);
        let up = Vec3::new([0.0, viewport_height, 0.0]);
        let z = right.cross(&up).normalize();
        let look = z * -1.0;
        let botleft = origin - right/2.0 - up/2.0 + look*focal_length;
        let jitter = get_blurriness(sample_type);
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

    pub fn gen_rays(&self, pct_x: f32, pct_y: f32, n: u32) -> Vec<Ray> {
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

use crate::{ NUM_RANDOMS, AVG_RANDOM_VEC, FOCAL_LENGTH, FOV, ASPECT, IMAGE_WIDTH, IMAGE_HEIGHT };
pub fn random_point_in_unit_sphere() -> Vec3 {
    loop {
        let v = Vec3::rand();
        unsafe {
            NUM_RANDOMS += 1;
            AVG_RANDOM_VEC += v;
        }
        if v.len_squared() < 1.0 {
            return v - Vec3::new([0.5,0.5,0.5]);
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_point_in_unit_sphere().normalize()
}

pub enum ReflectionType {
    NormalPlusPointInSphere,
    NormalPlusPointOnSphere,
    PointOnHemisphere,
}

pub fn random_reflection(ref_type: ReflectionType, pt: Vec3, normal: Vec3) -> Vec3 {
    match ref_type {
        ReflectionType::NormalPlusPointInSphere => return pt + normal + random_point_in_unit_sphere(),
        ReflectionType::NormalPlusPointOnSphere => return pt + normal + random_unit_vector(),
        ReflectionType::PointOnHemisphere => {
            let vec = random_unit_vector();
            return if vec.dot(normal) > 0.0 { pt + vec } else { pt - vec };
        },
    }
}

pub enum SampleType {
    PixelRatio,
    Blurry,
    Blurrier,
}

// TODO: it'd be fun to simulate zoomed pixels to see these distributions better
fn get_blurriness(ref_type: SampleType) -> Vec2 {

    let blurriness = match ref_type {
        SampleType::PixelRatio => {
            // blurriness: [0.0050251256, 0.009009009] for IMAGE_HEIGHT = 200
            Vec2::new([1.0/(IMAGE_WIDTH-1) as f32,
                              1.0/(IMAGE_HEIGHT-1) as f32])
        },
        SampleType::Blurry |
        SampleType::Blurrier => {
            // Pass blurriness level to camera as size of pixel in camera space. I don't
            // like manually computing this here since it's already done in the camera;
            // maybe move blurriness to the camera, shake it up and jitter some samples
            let viewport_height = FOCAL_LENGTH*2.0 * (FOV.to_radians()/2.0).tan();

            // this is currently pixel dims, but it can get fancier
            // blurriness: [0.017867114, 0.018018018] for IMAGE_HEIGHT = 200
            Vec2::new([viewport_height*ASPECT/(IMAGE_WIDTH-1) as f32,
                       viewport_height/(IMAGE_HEIGHT-1) as f32])
        },
    };

    println!("pixelsize: {:?}", blurriness);
    blurriness
}

