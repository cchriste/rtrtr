// Rusty the Ray Tracer
// September 2021
// ?'s: see #learning Rust note in Standard Notes)

// TODO periodically disable these; it's just hard to develop with them
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

use ferris_says::say;
use png;
use std::io::{stdout, BufWriter};
use std::convert::TryFrom;

mod utils;
use crate::utils::{Ray, Vector};

mod objects;
use crate::objects::{Sphere};

// screen
const ASPECT: f32 = 16.0/9.0;  // width/height
const IMAGE_WIDTH: u32 = 200;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT) as u32;


// color of ray(origin, dir)
fn ray_color(ray: &Ray) -> Vector {
    let unit_dir = ray.dir.normalize();

    let s = Sphere::new(Vector::init(0.0,0.0,-1.0), 0.5);
    let t = s.intersect(ray);
    if t > 0.0 {
        let N = (ray.at(t) - s.center).normalize();
        return 0.5*Vector::init(N.x()+1.0, N.y()+1.0, N.z()+1.0);
    }

    let t = 0.5*(unit_dir.y() + 1.0); // vertical percent along viewport
    let white = Vector::init(1.0, 1.0, 1.0);
    let bluey = Vector::init(0.5, 0.7, 1.0);
    white*(1.0 - t) + bluey*t
}


fn main() {

    // viewport
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT;
    const FOCAL_LENGTH: f32 = 1.0;

    let origin = Vector::init(0.0, 0.0, 0.0);
    let right = Vector::init(VIEWPORT_WIDTH, 0.0, 0.0);
    let up = Vector::init(0.0, VIEWPORT_HEIGHT, 0.0);
    let z = right.cross(&up).normalize();
    let look = z * -1.0;
    println!("right: {:?}",right);
    println!("up: {:?}",up);
    println!("z: {:?}",z);
    println!("look: {:?}",look); // not sure I want this, but it'll be the camera's frame soon

    let botleft = origin - right/2.0 - up/2.0 + look*FOCAL_LENGTH;
    println!("botleft: {:?}", botleft);

    // allocate image (just set length, don't initialize)
    let mut img: Vec<f32> = Vec::with_capacity(usize::try_from(4 * IMAGE_WIDTH * IMAGE_HEIGHT).unwrap());
    unsafe { img.set_len(img.capacity()); }

    // keep track of min/max color values
    let mut color_range: ([f32; 3], [f32; 3]) = ([1.0, 1.0, 1.0], [0.0, 0.0, 0.0]);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
    // for j in (IMAGE_HEIGHT/2..IMAGE_HEIGHT/2+2).rev() {
    //     for i in IMAGE_WIDTH/2..IMAGE_WIDTH/2+2 {
            let pct_x = i as f32 / (IMAGE_WIDTH-1) as f32;
            let pct_y = j as f32 / (IMAGE_HEIGHT-1) as f32;

            // ray: origin = O, direction = point moving across image from botleft - origin
            let p1 = botleft + right*pct_x + up*pct_y;
            let p0 = origin;
            let ray = Ray::new(origin, p1 - p0);

            let color = ray_color(&ray);
            // if j % IMAGE_WIDTH == 0 {
            //     println!("color: {:?}", color);
            // }

            // update color minmax
            for c in 0..3 {
                color_range.0[c] = color_range.0[c].min(color.v[c]);
                color_range.1[c] = color_range.1[c].max(color.v[c]);
            }

            // 4 * (current height * image width + current width)
            let idx = usize::try_from(4*((IMAGE_HEIGHT-1 - j) * IMAGE_WIDTH + i)).unwrap();
            img[idx + 0] = color.v[0];
            img[idx + 1] = color.v[1];
            img[idx + 2] = color.v[2];
            img[idx + 3] = 1.0;
        }
    }

    println!("color_range: {:?}", color_range);

    write_img(r"/tmp/canvas.png", img);
    conclude("Goodbye fellow Rustaceans!");
}

fn conclude(msg: &str) {
    let stdout = stdout();
    let message = String::from(msg);
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn write_img(filename: &str, img: Vec<f32>) {
    // For reading and opening files
    use std::path::Path;
    use std::fs::File;

    let path = Path::new(filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMAGE_WIDTH, IMAGE_HEIGHT);
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
        data[i] = (255.999 * img[i]) as u8;
    }

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap(); // Save
}
