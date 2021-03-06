// Rusty the Ray Tracer
// September 2021
// ?'s: see #learning Rust note in Standard Notes)

//FIXME: what is this? color_range: [(R:0.5849 G:0.7110 B:0.9000 A:1.0000), (R:0.5849 G:0.7110 B:0.9000 A:2.0000)]

// NIKE™ tasks:
// [] use lib.rs
// [] Rust Programming Language ch 10
// [x] push to GitHub
// [~] add to GitHub.io home page (still needs some pictures)
// [] add jittering for more uniform coverage (for gen_rays... where else?)
//  - NOTE: lens is a circle, pixel is a square

// TODO periodically disable these; it's just hard to develop with them
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
//#![allow(non_snake_case)]

// <config> /////////////////////////////

const DEBUG: bool = false;
const LITE: bool = false;
const BOOK: bool = false; // try to match Shirley's RTiOW configs
const FINAL: bool = false; // match RTiOW final image

// Lambertian reflection equation
const REFL_TYPE: ReflectionType = ReflectionType::NormalPlusPointOnSphere; // add this to the [Vulkan] UI
//const REFL_TYPE: ReflectionType = ReflectionType::NormalPlusPointInSphere; // add this to the [Vulkan] UI

// screen
const ASPECT: f32 = if FINAL { 3.0/2.0 } else { 16.0/9.0 };  // width/height
const IMAGE_WIDTH: u32 = if FINAL && BOOK { 1200 } else if BOOK { 400 } else { 200 };
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT) as u32;

// render
const SAMPLES_PER_PIXEL: u32 = if DEBUG {1} else if LITE {5} else if FINAL && BOOK {500} else if BOOK {100} else {26};
const MAX_DEPTH: i32 = if DEBUG {4} else if LITE {100} else if FINAL && BOOK { 50 } else if BOOK { 100 } else { 25 };

// camera
fn setup_camera() -> Camera {
    //let aperture: f32 = 0.1;
    let aperture: f32 = 0.001; // a tiny aperture simulates a point camera
    let fov: f32 = 40.0;
    let sample_type: camera::SampleType = SampleType::PixelRatio;
    //let sample_type: camera::SampleType = SampleType::Blurry;  // add this to the UI
    //let look_from: Vec3 = Vec3::new([1.0, 2.0, -1.0]);
    let look_from: Vec3 = Vec3::new([3.0, 1.75, 1.25]);
    //let look_from: Vec3 = Vec3::new([-2.0, 2.0, 1.0]);
    //let look_from: Vec3 = Vec3::new([13.0, 2.0, 3.0]);
    //let look_from: Vec3 = Vec3::new([3.0, 3.0, 2.0]);
    //let look_at: Vec3 = Vec3::new([0.0, 0.0, 0.0]);
    let look_at: Vec3 = Vec3::new([1.1, 0.85, -0.75]);
    //let look_at: Vec3 = Vec3::new([1.0, 0.0, -1.0]); // TODO: split into look_dir and focal_dist
    //let vup: Vec3 = Vec3::new([1.0, 0.0, 0.0]);
    let vup: Vec3 = Vec3::new([0.0, 1.0, 0.0]);
    let dist_to_focus: f32 = if FINAL { 10.0 } else { (look_at - look_from).len() };

    Camera::init(IMAGE_HEIGHT, ASPECT, aperture, sample_type,
                 fov, look_from, look_at, vup, dist_to_focus)
}

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
fn ray_color(ray: Ray, scene: &Jumble, depth: i32, indent_by: usize) -> Color {
    let indent = vec![' '; indent_by];
    let indent: String = indent.iter().cloned().collect();

    if depth <= 0 { return Color::black(); } // you can only dive so deep...
    if crate::DEBUG { println!("{}{}: starting ray_color...", indent, crate::MAX_DEPTH-depth); }

    let mut hit = HitRecord::new();
    match scene.intersect(ray, &Range::default(), &mut hit, indent_by) {
        Shot::Hit => {
            // return 0.5*Color::new([hit.normal.x()+1.0,
            //                        hit.normal.y()+1.0,
            //                        hit.normal.z()+1.0]);

            if crate::DEBUG {
                println!("{}{}: hit! {}", indent, crate::MAX_DEPTH-depth, hit);
            }
            match hit.material.scatter(ray, &hit, indent_by) {
                Attenuated(color, ray) => {
                    return color*ray_color(ray, scene, depth-1, indent_by);
                },
                Absorbed => return Color::black(),
            }
        },
        Shot::Miss => {
            if crate::DEBUG {
                println!("{}{}: miss.", indent, crate::MAX_DEPTH-depth);
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
    //let start_row = if DEBUG {70} else {0};
    let start_row = if DEBUG {IMAGE_HEIGHT/2 +1} else {0};
    let end_row = IMAGE_HEIGHT;
    //let end_row = IMAGE_HEIGHT/2+1;
    let step_y: usize = if DEBUG { (IMAGE_HEIGHT).try_into().unwrap() } else { 1 };

    let start_col = if DEBUG {IMAGE_WIDTH/2 +1} else {0};
    //let start_col = if DEBUG {125} else {0};
    let end_col = IMAGE_WIDTH;
    let step_x: usize = if DEBUG { ((IMAGE_WIDTH+10)).try_into().unwrap() } else { 1 };

    for j in (start_row..end_row).step_by(step_y) {
        for i in (start_col..end_col).step_by(step_x) {
            if DEBUG { println!("i,j: {},{}", i,j); }
            pixels.push([i, j]);
        }
    }
    pixels
}

// FIXME: no need for this to be a static mut used with unsafe, but handy to demonstrate
static mut COLOR_RANGE: (Color, Color) = (Color::white(), Color::black());

fn main() {

    // add an outline for debugging
    let outline = if crate::DEBUG { 1 } else { 0 };

    // allocate dst image
    let mut img: Vec<f32> =
        if DEBUG {
            vec![1.0; usize::try_from(4*(IMAGE_WIDTH+outline*2)*(IMAGE_HEIGHT+outline*2)).unwrap()]
        } else {
            Vec::with_capacity(usize::try_from(4*(IMAGE_WIDTH)*(IMAGE_HEIGHT)).unwrap())
        };
    // unless debugging, just set length, don't initialize (aka unnecessary optimization :)
    if !DEBUG { unsafe { img.set_len(img.capacity()); } }

    let mut camera = setup_camera(); // FIXME? camera stores an rng that mutates when used

    // build scene
    let scene = if FINAL { scene::build_rtiow_final_scene() } else { scene::build_scene() };

    let pixels = get_pixels_to_trace();
    for px in &pixels {
        let pct_x = px[0] as f32 / (IMAGE_WIDTH-1) as f32;
        let pct_y = px[1] as f32 / (IMAGE_HEIGHT-1) as f32;

        let nsamples = SAMPLES_PER_PIXEL;
        let mut color = Color::black();
        let rays = camera.gen_rays(pct_x, pct_y, nsamples);
        for ray in rays {
            if DEBUG {
                println!("[pixel] ({}, {}):", px[0], px[1]);
                //println!("shooting {}",ray);
            }
            color += ray_color(ray, &scene, MAX_DEPTH, 0/*indent*/);
        }
        color /= nsamples as f32;

        if DEBUG {
            println!("color: {}\n", color);
        }

        // update color minmax
        unsafe {
            for c in 0..4 {
                COLOR_RANGE.0[c] = COLOR_RANGE.0[c].min(color[c]);
                COLOR_RANGE.1[c] = COLOR_RANGE.1[c].max(color[c]);
            }
        }

        // set pixel
        let idx = pixel_idx(px, outline);
        img[idx + 0] = color[0];
        img[idx + 1] = color[1];
        img[idx + 2] = color[2];
        img[idx + 3] = color[3];
    }

    unsafe {
        println!("color_range: [{}, {}]", COLOR_RANGE.0, COLOR_RANGE.1);
    }

    io::write_img(r"/tmp/smoothcanvas.png", img, IMAGE_WIDTH+outline*2, IMAGE_HEIGHT+outline*2);
    io::conclude("Goodbye fellow Rustaceans!");
}

// get pixel index from inner image xy
fn pixel_idx(px: &[u32; 2], outline: u32) -> usize {
    let width = if DEBUG && outline > 0 { IMAGE_WIDTH + outline*2 } else { IMAGE_WIDTH };
    let height = if DEBUG && outline > 0 { IMAGE_HEIGHT + outline*2 } else { IMAGE_HEIGHT };

    // idx = 4 * (current height * image width + current width)
    usize::try_from(4*((height-1 - px[1]) * width + px[0])).unwrap()
}
