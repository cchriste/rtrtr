// Rusty the Ray Tracer
// September 2021
// ?'s: see #learning Rust note in Standard Notes)

// easy warmups
// [x] color use vec4
// [-] range use std::ops::range is actually just for Idx, things like (2..5) for 2,3,4

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

use png;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::f32::consts::PI;
use std::rc::Rc;

mod utils;  // TODO: figure out how to move these to lib.rs where it belongs?
mod objects;
mod camera;
mod scene;
mod io;
mod materials;

use crate::utils::*;
use crate::objects::*;
use crate::camera::*;
use materials::LightScatter::{ Attenuated, Absorbed };

// color of ray(origin, dir)
fn ray_color(ray: &Ray, scene: &Jumble, depth: i32, indent_by: usize) -> Color {
    if depth <= 0 { return Color::black(); } // you can only dive so deep...
    if crate::DEBUG { println!("{}...", crate::MAX_DEPTH-depth); }

    let mut hit = HitRecord::new();
    match scene.intersect(ray, &Range::default(), &mut hit, indent_by) {
        Shot::Hit => {
            if crate::DEBUG {
                println!("{}: {}", crate::MAX_DEPTH-depth, hit);
            }
            match hit.material.scatter(ray, &hit, indent_by) {
                Attenuated(color, ray) => {
                    return color*ray_color(&ray, scene, depth-1, indent_by);
                },
                Absorbed => return Color::black(),
            }
        },
        Shot::Miss => {
            if crate::DEBUG {
                println!("{}: MISS", crate::MAX_DEPTH-depth);
            }
            let unit_dir = ray.dir.normalize();
            let t = 0.5*(unit_dir.y() + 1.0); // vertical percent along viewport
            let bluey = Color::new([0.5, 0.7, 1.0]);
            return Color::white()*(1.0 - t) + bluey*t;
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

// Added to track random vectors since shadows seem to be on the right just a little more than the left
static mut AVG_RANDOM_VEC: Vec3 = Vec3::zero(); // maybe remove these two as they have validated random
static mut NUM_RANDOMS: u32 = 0;   // but... avg random vec (208079 vecs): (0.5003î, 0.5002ĵ, 0.5001k̂)

static mut COLOR_RANGE: (Color, Color) = (Color::white(), Color::black());

fn main() {

    // let tst = Vec3::new([1.0,2.0,3.0]);
    // println!("Test format v3: {} {:#?} {:?}",tst,tst,tst);
    // return;

    // allocate dst image (just set length, don't initialize)
    let mut img: Vec<f32> = Vec::with_capacity(usize::try_from(4*IMAGE_WIDTH*IMAGE_HEIGHT).unwrap());
    unsafe { img.set_len(img.capacity()); }

    let camera = Camera::init(ASPECT, FOCAL_LENGTH, FOV, IMAGE_HEIGHT, SAMPLE_TYPE);

    // build scene
    let scene = scene::build_scene();

    let pixels = get_pixels_to_trace();
    for px in &pixels {
        let pct_x = px[0] as f32 / (IMAGE_WIDTH-1) as f32;
        let pct_y = px[1] as f32 / (IMAGE_HEIGHT-1) as f32;

        let nsamples = if !DEBUG {SAMPLES_PER_PIXEL} else {1};
        let mut color = Color::black();
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
            for c in 0..4 {
                COLOR_RANGE.0[c] = COLOR_RANGE.0[c].min(color[c]);
                COLOR_RANGE.1[c] = COLOR_RANGE.1[c].max(color[c]);
            }
        }

        // 4 * (current height * image width + current width)
        let idx = usize::try_from(4*((IMAGE_HEIGHT-1 - px[1]) * IMAGE_WIDTH + px[0])).unwrap();
        img[idx + 0] = color[0];
        img[idx + 1] = color[1];
        img[idx + 2] = color[2];
        img[idx + 3] = color[3];
    }

    //let num_samples: f32 = pixels.len() as f32 * SAMPLES_PER_PIXEL as f32;
    unsafe {
        println!("avg random vec ({} vecs): {}", NUM_RANDOMS, AVG_RANDOM_VEC / NUM_RANDOMS as f32);
        println!("color_range: [{}, {}]", COLOR_RANGE.0, COLOR_RANGE.1);
    }

    io::write_img(r"/tmp/smoothcanvas.png", img, IMAGE_WIDTH, IMAGE_HEIGHT);
    io::conclude("Goodbye fellow Rustaceans!");
}

