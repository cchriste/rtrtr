// Rusty the Ray Tracer
// September 2021
// ?'s: see #learning Rust note in Standard Notes)

// easy warmups
// [] color use vec4
// [] range use std::range

// TODO periodically disable these; it's just hard to develop with them
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
//#![allow(non_snake_case)]

// <config> /////////////////////////////
const DEBUG: bool = false;
const LITE: bool = false;
const BOOK: bool = true; // try to match Shirley's RTiaW configs

// Lambertian reflection equation
const REFL_TYPE: ReflectionType = if BOOK { ReflectionType::NormalPlusPointOnSphere } else { ReflectionType::NormalPlusPointInSphere };
//const REFL_TYPE: ReflectionType = ReflectionType::PointOnHemisphere;  // add this to the UI

// screen
const ASPECT: f32 = 16.0/9.0;  // width/height
const IMAGE_WIDTH: u32 = if BOOK { 400 } else { 200 };
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT) as u32;

// render
const SAMPLES_PER_PIXEL: u32 = if DEBUG {1} else if LITE {5} else if BOOK {100} else { 26 };
const MAX_DEPTH: i32 = if DEBUG {2} else if LITE {5} else if BOOK { 100 } else { 25 };

// camera
const FOCAL_LENGTH: f32 = 1.0;
const FOV: f32 = 90.0;
const SAMPLE_TYPE: camera::SampleType = SampleType::PixelRatio;
//const SAMPLE_TYPE: camera::SampleType = SampleType::Blurry;  // add this to the UI

// consts
const PI_4: f32 = PI / 4.0;
const PI_3: f32 = PI / 3.0;
const PI_2: f32 = PI / 2.0;

///////////////////////////// </config>

use ferris_says::say;
use png;
use std::io::{stdout, BufWriter};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::f32::consts::PI;

mod utils;
use crate::utils::*;

mod objects;
use crate::objects::{Sphere, Jumble, Shot, Intersectable, HitRecord};

mod camera; // FIXME: shouldn't this [be able to] go in camera.rs?
use crate::camera::*;

// color of ray(origin, dir)
fn ray_color(ray: &Ray, scene: &Jumble, depth: i32, indent_by: usize) -> Vec3 { // TODO: return color
    if depth <= 0 { return Vec3::zero(); } // you can only dive so deep...
    if crate::DEBUG { println!("{}...", crate::MAX_DEPTH-depth); }

    let mut hit = HitRecord::new();
    match scene.intersect(ray, &Range::default(), &mut hit, indent_by) {
        Shot::Hit => {
            if crate::DEBUG {
                println!("{}: {}", crate::MAX_DEPTH-depth, hit);
            }
            let target = random_reflection(REFL_TYPE, hit.point, hit.normal);
            return 0.5*ray_color(&Ray::new(hit.point, target - hit.point),
                                 scene, depth-1, indent_by);
        },
        Shot::Miss => {
            if crate::DEBUG {
                println!("{}: MISS", crate::MAX_DEPTH-depth);
            }
            let unit_dir = ray.dir.normalize();
            let t = 0.5*(unit_dir.y() + 1.0); // vertical percent along viewport
            let white = Vec3::new([1.0, 1.0, 1.0]);
            let bluey = Vec3::new([0.5, 0.7, 1.0]);
            return white*(1.0 - t) + bluey*t;
        }
    }
}

fn get_pixels_to_trace() -> Vec<[u32; 2]> {
    // indices of pixels to trace
    let mut pixels: Vec<[u32; 2]> = Vec::new();

    // handy for debugging just a couple of intersections
    let start_row = if DEBUG {IMAGE_HEIGHT/4-15} else {0};
    let end_row = IMAGE_HEIGHT;
    //let step_y: usize = if DEBUG { (IMAGE_HEIGHT).try_into().unwrap() } else { 1 };
    let step_y: usize = if DEBUG { (IMAGE_HEIGHT/4).try_into().unwrap() } else { 1 };

    let start_col = if DEBUG {IMAGE_WIDTH/2} else {0};
    let end_col = IMAGE_WIDTH;
    let step_x: usize = if DEBUG { (IMAGE_WIDTH).try_into().unwrap() } else { 1 };

    for j in (start_row..end_row).step_by(step_y) {
        for i in (start_col..end_col).step_by(step_x) {
            if DEBUG { println!("i,j: {},{}", i,j); }
            pixels.push([i, j]);
        }
    }
    pixels
}

static mut AVG_RANDOM_VEC: Vec3 = Vec3::zero();
static mut NUM_RANDOMS: u32 = 0;
static mut COLOR_RANGE: ([f32; 3], [f32; 3]) = ([1.0, 1.0, 1.0], [0.0, 0.0, 0.0]);

fn main() {

    // let tst = Vec3::new([1.0,2.0,3.0]);
    // println!("Test format v3: {} {:#?} {:?}",tst,tst,tst);
    // return;

    // allocate dst image (just set length, don't initialize)
    let mut img: Vec<f32> = Vec::with_capacity(usize::try_from(4*IMAGE_WIDTH*IMAGE_HEIGHT).unwrap());
    unsafe { img.set_len(img.capacity()); }

    let camera = Camera::init(ASPECT, FOCAL_LENGTH, FOV, IMAGE_HEIGHT, SAMPLE_TYPE);

    // build scene
    let scene = build_scene();

    let pixels = get_pixels_to_trace();
    for px in &pixels {
        let pct_x = px[0] as f32 / (IMAGE_WIDTH-1) as f32;
        let pct_y = px[1] as f32 / (IMAGE_HEIGHT-1) as f32;

        let nsamples = if !DEBUG {SAMPLES_PER_PIXEL} else {1};
        let mut color = Vec3::new([0.0,0.0,0.0]);
        let rays = camera.gen_rays(pct_x, pct_y, nsamples);
        for ray in rays {
            //let ray = camera.gen_ray(pct_x, pct_y);
            if DEBUG {
                println!("[pixel] ({}, {}):", px[0], px[1]);
                //println!("shooting {}",ray);
            }
            color += ray_color(&ray, &scene, MAX_DEPTH, 0/*indent*/);
        }
        color /= nsamples as f32;

        if DEBUG { //&& px[0] % IMAGE_WIDTH == 0 {
            println!("color: {}\n", color);
        }

        // update color minmax
        unsafe {
            for c in 0..3 {
                COLOR_RANGE.0[c] = COLOR_RANGE.0[c].min(color.v[c]);
                COLOR_RANGE.1[c] = COLOR_RANGE.1[c].max(color.v[c]);
            }
        }

        // 4 * (current height * image width + current width)
        let idx = usize::try_from(4*((IMAGE_HEIGHT-1 - px[1]) * IMAGE_WIDTH + px[0])).unwrap();
        img[idx + 0] = color.v[0];
        img[idx + 1] = color.v[1];
        img[idx + 2] = color.v[2];
        img[idx + 3] = 1.0;
    }

    //let num_samples: f32 = pixels.len() as f32 * SAMPLES_PER_PIXEL as f32;
    unsafe {
        println!("avg random vec ({} vecs): {}", NUM_RANDOMS, AVG_RANDOM_VEC / NUM_RANDOMS as f32);
        println!("color_range: {:?}", COLOR_RANGE);
    }

    write_img(r"/tmp/smoothcanvas.png", img, IMAGE_WIDTH, IMAGE_HEIGHT);
    conclude("Goodbye fellow Rustaceans!");
}

fn conclude(msg: &str) {
    let stdout = stdout();
    let message = String::from(msg);
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn write_img(filename: &str, img: Vec<f32>, width: u32, height: u32) {
    assert!(img.len() == (width * height * 4) as usize); // rgba

    // For reading and opening files
    use std::path::Path;
    use std::fs::File;

    let path = Path::new(filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8));
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);

    // convert floats to chars and apply gamma correction
    // γ (gamma) = 2.2, color saved = c^(1/γ), estimate γ as 2.0, so color = c^(1/2) = sqrt(c)
    let mut data: Vec<u8> = Vec::with_capacity(img.len());
    unsafe { data.set_len(data.capacity()); }
    for i in 0..img.len() {
        data[i] = ((256f32-f32::EPSILON) * f32::sqrt(img[i])) as u8; 
    }

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap(); // Save
}

fn build_scene() -> Jumble {
    // instances of geometry
    let s1 = Sphere::new(Vec3::new([0.0,0.0,-1.0]), 0.5);
    let s2 = Box::new(Sphere::new(Vec3::new([0.0,-100.5,-1.0]), 100.0));
    let s3 = Sphere::new(Vec3::new([0.0,0.0,-1.0]), 0.5);
    let s4 = Sphere::new(Vec3::new([0.0,0.0,-1.0]), 0.5);
    let s5 = Sphere::new(Vec3::new([0.0,0.0,-1.0]), 0.5);

    // the main stage
    let mut scene = Jumble::new();
    scene.name = "main".to_string();

    // test fov is correctly computed
    let mut fov_test_scene = Jumble::new();
    fov_test_scene.name = "fov_test".to_string();
    let radius = (std::f32::consts::PI / 4.0).cos();
    let sl = Sphere::new(Vec3::new([-radius,0.0,-1.0]), radius);
    let sr = Sphere::new(Vec3::new([radius,0.0,-1.0]), radius);
    fov_test_scene.add(Box::new(sl));
    fov_test_scene.add(Box::new(sr));
    //scene.add(Box::new(fov_test_scene));

    let mut sub_scene = Jumble::new();
    sub_scene.name = "sub".to_string();
    sub_scene.add(Box::new(s1));
    sub_scene.add(s2);
    scene.add(Box::new(sub_scene));

    let mut squishy_scene = Jumble::new();
    squishy_scene.name = "squishy".to_string();
    let mut csys = squishy_scene.csys();
    let scale = Matrix::scale(Vec3::new([1.5, 0.75, 1.0]));
    csys *= scale;
    csys.translate(Vec3::new([0.0,-0.5,0.0]));
    squishy_scene.set_csys(csys);
    squishy_scene.add(Box::new(s3));
    //squishy_scene.add(Box::new(s1));// FIXME: usa los objectos en mas espacios
    //squishy_scene.add(s2); // FIXME: usa los objectos en mas espacios
    scene.add(Box::new(squishy_scene));


    let mut sq2 = Jumble::new();
    sq2.name = "sq2".to_string();
    let mut csys = sq2.csys();

    let rot = Matrix::rotation(-3.0*PI_4, Axis::Z);
    //let rot = Matrix::rotation(-PI_4, Axis::Y);
    //let rot = Matrix::rotation(-PI_4, Axis::X);
    println!("rot:\n {}", rot);
    csys *= rot;

    let scale = Matrix::scale(Vec3::new([0.5, 1.25, 1.0]));
    println!("scale:\n {}", scale);
    csys *= scale;

    csys.translate(Vec3::new([-1.25, 0.25, 0.0]));
    println!("csys:\n {}", csys);
    sq2.set_csys(csys);
    sq2.add(Box::new(s4));
    scene.add(Box::new(sq2));


    let mut sq3 = Jumble::new();
    sq3.name = "sq3".to_string();
    let mut csys = sq3.csys();

    let rot = Matrix::rotation(-3.0*PI_4, Axis::Z);
    println!("rot:\n {}", rot);
    csys *= rot;
    let rot = Matrix::rotation(-PI_2, Axis::X);
    println!("rot:\n {}", rot);
    csys *= rot;

    let scale = Matrix::scale(Vec3::new([0.5, 1.0, 1.1]));
    println!("scale:\n {}", scale);
    csys *= scale;

    csys.translate(Vec3::new([1.25,-0.333,-0.25]));
    println!("csys:\n {}", csys);
    sq3.set_csys(csys);
    sq3.add(Box::new(s5));
    scene.add(Box::new(sq3));

    scene
}
