// Rusty the Ray Tracer
// September 2021
// ?'s: see #learning Rust note in Standard Notes)

// TODO periodically disable these; it's just hard to develop with them
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
//#![allow(non_snake_case)]

const DEBUG: bool = false;
const LITE: bool = true;

use ferris_says::say;
use png;
use std::io::{stdout, BufWriter};
use std::convert::TryFrom;
use std::convert::TryInto;
use rand::Rng;

mod utils;
use crate::utils::{Ray, Vector, Color, Range};

mod objects;
use crate::objects::{Sphere, Jumble, Shot, Intersectable, HitRecord};

mod camera; // FIXME: shouldn't this [be able to] go in camera.rs?
use crate::camera::Camera;

// color of ray(origin, dir)
fn ray_color(ray: &Ray, scene: &Jumble) -> Vector { // TODO: return color
    let mut range = Range::default();
    // println!("range: {:?}", range);
    let mut hit = HitRecord::new();
    match scene.intersect(ray, &mut range, &mut hit, /*depth,*/ 0/*indent*/) {
        Shot::Hit => {
            if crate::DEBUG {
                println!("HIT! time: {}, point: {:?}, normal: {:?}",
                         hit.t, hit.point, hit.normal);
            }
            return 0.5*Vector::init(hit.normal.x()+1.0,
                                    hit.normal.y()+1.0,
                                    hit.normal.z()+1.0);
        },
        Shot::Miss => {
            let unit_dir = ray.dir.normalize();
            let t = 0.5*(unit_dir.y() + 1.0); // vertical percent along viewport
            let white = Vector::init(1.0, 1.0, 1.0);
            let bluey = Vector::init(0.5, 0.7, 1.0);
            return white*(1.0 - t) + bluey*t;
        }
    }
}

fn main() {
    // screen
    const ASPECT: f32 = 16.0/9.0;  // width/height
    const IMAGE_WIDTH: u32 = 200;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT) as u32;
    const SAMPLES_PER_PIXEL: u32 = if LITE {1} else {100};

    // camera
    let focal_length = 1.0;
    let fov: f32 = 90.0;
    let viewport_height = focal_length*2.0 * (fov.to_radians()/2.0).tan(); // I don't like manually computing this here since it's already done in the camera; maybe move blurriness to the camera, shake it up and jitter some samples

    // this is currently pixel dims, but it can get fancier
    let blurriness = [viewport_height*ASPECT/IMAGE_WIDTH as f32,
                      viewport_height/IMAGE_HEIGHT as f32];

    let camera = Camera::init(ASPECT, focal_length, fov, blurriness);

    // allocate image (just set length, don't initialize)
    let mut img: Vec<f32> = Vec::with_capacity(usize::try_from(4 * IMAGE_WIDTH * IMAGE_HEIGHT).unwrap());
    unsafe { img.set_len(img.capacity()); }

    // keep track of min/max color values (TODO: make this a Color)
    let mut color_range: ([f32; 3], [f32; 3]) = ([1.0, 1.0, 1.0], [0.0, 0.0, 0.0]);

    // build scene
    let scene = build_scene();

    // indices of pixels to trace
    let mut pixels: Vec<[u32; 2]> = Vec::new();

    // handy for debugging just a couple of intersections
    let start_row = if DEBUG {IMAGE_HEIGHT/2+10} else {0};
    let end_row = IMAGE_HEIGHT;
    let step_y: usize = if DEBUG { (IMAGE_HEIGHT/2).try_into().unwrap() } else { 1 };

    let start_col = if DEBUG {IMAGE_WIDTH/4} else {0};
    let end_col = IMAGE_WIDTH;
    let step_x: usize = if DEBUG { (IMAGE_WIDTH/4).try_into().unwrap() } else { 1 };

    for j in (start_row..end_row).step_by(step_y) { 
        for i in (start_col..end_col).step_by(step_x) {
            if DEBUG { println!("i,j: {},{}", i,j); }
            pixels.push([i, j]);
        }
    }

    for px in pixels {
        let pct_x = px[0] as f32 / (IMAGE_WIDTH-1) as f32;
        let pct_y = px[1] as f32 / (IMAGE_HEIGHT-1) as f32;

        let nsamples = if !DEBUG {SAMPLES_PER_PIXEL} else {1};
        let mut color = Vector::init(0.0,0.0,0.0);
        for s in 0..nsamples {
            let ray = camera.gen_ray(pct_x, pct_y);
            if DEBUG { println!("ray: {:?}",ray); }
            color += ray_color(&ray, &scene);
        }
        color /= nsamples as f32;

        if DEBUG { //&& px[0] % IMAGE_WIDTH == 0 {
            println!("color: {:?}\n", color);
        }

        // update color minmax
        for c in 0..3 {
            color_range.0[c] = color_range.0[c].min(color.v[c]);
            color_range.1[c] = color_range.1[c].max(color.v[c]);
        }

        // 4 * (current height * image width + current width)
        let idx = usize::try_from(4*((IMAGE_HEIGHT-1 - px[1]) * IMAGE_WIDTH + px[0])).unwrap();
        img[idx + 0] = color.v[0];
        img[idx + 1] = color.v[1];
        img[idx + 2] = color.v[2];
        img[idx + 3] = 1.0;
    }

    println!("color_range: {:?}", color_range);

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

    // convert floats to chars
    let mut data: Vec<u8> = Vec::with_capacity(img.len());
    unsafe { data.set_len(data.capacity()); }
    for i in 0..img.len() {
        data[i] = ((256f32-f32::EPSILON) * img[i]) as u8;
    }

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap(); // Save
}

fn build_scene() -> Jumble {
    // instances of geometry
    let s1 = Sphere::new(Vector::init(0.0,0.0,-1.0), 0.5);
    let s2 = Sphere::new(Vector::init(0.0,-100.5,-1.0), 100.0);
    let s3 = Sphere::new(Vector::init(0.0,0.0,-1.0), 0.5);

    // the main stage
    let mut scene = Jumble::new();

    // test fov is correctly computed
    let mut fov_test_scene = Jumble::new();
    let radius = (std::f32::consts::PI / 4.0).cos();
    let sl = Sphere::new(Vector::init(-radius,0.0,-1.0), radius);
    let sr = Sphere::new(Vector::init(radius,0.0,-1.0), radius);
    fov_test_scene.add(Box::new(sl));
    fov_test_scene.add(Box::new(sr));
    //scene.add(Box::new(fov_test_scene));

    let mut sub_scene = Jumble::new();
    sub_scene.add(Box::new(s1));
    sub_scene.add(Box::new(s2));
    scene.add(Box::new(sub_scene));

    let mut squishy_scene = Jumble::new();
    squishy_scene.csys.translate(Vector { v: [-0.25, 0.25, 0.0] });
    squishy_scene.add(Box::new(s3)); // TODO: add same geometry to diff scenes (one thing at a time)
    //scene.add(Box::new(squishy_scene));

    scene
}
