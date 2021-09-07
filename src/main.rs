use ferris_says::say;
use png;
use std::io::{stdout, BufWriter};
use std::convert::TryFrom;
use std::f32; // for .sqrt() method

// screen
const ASPECT: f32 = 16.0/9.0;  // width/height
const IMAGE_WIDTH: u32 = 200;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT) as u32;

#[derive(Debug)]
struct Vector {
    v: [f32; 3],
}

impl Vector {
    fn negate(&mut self) -> &Vector {
        self.v[0] = -self.v[0];
        self.v[1] = -self.v[1];
        self.v[2] = -self.v[2];
        self
    }
}

// todo class vec (here are some helper functions; is operator overloading possible? function overloading isn't... :(

// todo: read ch 4 on ownership after unsuccessfully trying all sorts of variations here
// fn dotself(&(mut vec): &[f32; 3]) -> &[f32; 3] {
//     vec[0] *= vec[0];
//     vec[1] *= vec[1];
//     vec[2] *= vec[2];
//     return &vec; // doesn't work (neither does just &vec or vec
// }
fn dotvec(v1: [f32; 3], v2: [f32; 3]) -> [f32; 3] {
    [v1[0] * v2[0],
     v1[1] * v2[1],
     v1[2] * v2[2]]
}
fn addvec(v1: [f32; 3], v2: [f32; 3]) -> [f32; 3] {
    [v1[0] + v2[0],
     v1[1] + v2[1],
     v1[2] + v2[2]]
}
fn mulvec(v1: [f32; 3], k: f32) -> [f32; 3] {
    [v1[0] * k,
     v1[1] * k,
     v1[2] * k]
}
fn normalize(v: [f32; 3]) -> [f32; 3] {
    let magnitude = (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt();
    [v[0] / magnitude,
     v[1] / magnitude,
     v[2] / magnitude]
}

// todo class ray: (origin: [f32; 3], dir: [f32; 3]);

fn ray_color(ray: ([f32; 3], [f32; 3])) -> [f32; 3] {
    let (_origin, dir) = ray;
    let dir = normalize(dir);
    let c: [f32; 3] = [0.5 * (dir[1] + 1.0),
                       0.25,
                       0.5 * (dir[0] + 1.0)];    //c.a = 1.0;
    return c;
}

fn main() {
    let str = "create a png";
    // TODO: how do I print the type of str?
    println!("{}", str); // note the ! after println cuz it's a macro
    init("bowwwah!");

    let mut v1 = [1.0, 2., 3.];  // does this need to be let mut v1 since it's modified by dotself?
    let v2 = [4., -1., 3.];

    println!("v1 dot v2 = `{:?}", dotvec(v1, v2));
    // println!("v1 dot v1 = `{:#?}", dotself(&mut v1));
    // println!("v1 after dotself: {:?}", v1);

    let mut vec = Vector { v: [0.0, 1.0, -1.25] };
    println!("vec: {:?}", vec);
    vec.negate();
    println!("vec after negation: {:?}", vec);

    // viewport
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT;
    const FOCAL_LENGTH: f32 = 1.0;

    let origin = [0.0, 0.0, 0.0];
    let right = [1.0, 0.0, 0.0];
    let up = [0.0, 1.0, 0.0];
    //let topleft = [origin - right/2.0, origin + up/2.0, origin[2] - FOCAL_LENGTH];
    let topleft = addvec(origin, addvec([0., 0., origin[2] - FOCAL_LENGTH],
                                        addvec(mulvec(right, -0.5), mulvec(up, 0.5))));

    // For this case, setting size without initializing is not buying much, but for reading files... 
    // I'm just glad to learn how to do it.
    //let mut data: Vec<u8> = vec![255; usize::try_from(4 * IMAGE_WIDTH * IMAGE_HEIGHT).unwrap()];
    let mut data: Vec<u8> = Vec::with_capacity(usize::try_from(4 * IMAGE_WIDTH * IMAGE_HEIGHT).unwrap());
    println!("data.len: {}", data.len());
    unsafe { data.set_len(data.capacity()); }
    println!("data.len: {}", data.len());

    let mut color_bounds: ([f32; 3], [f32; 3]) = ([1.0, 1.0, 1.0], [0.0, 0.0, 0.0]);

    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            // let ray: vec = (origin,
            //                topleft +
            //                i / IMAGE_WIDTH * VIEWPORT_WIDTH +
            //                j / IMAGE_HEIGHT * VIEWPORT_HEIGHT - origin);
            let ray: ([f32; 3], [f32; 3]) = (origin,
                                             addvec(
                                                 addvec(topleft,
                                                        addvec(
                                                            addvec([0.0, 0.0, 0.0],
                                                                   mulvec(right, i as f32 / IMAGE_WIDTH as f32 * VIEWPORT_WIDTH )),
                                                            addvec([0.0, 0.0, 0.0],
                                                                   mulvec(up, -1. * i as f32 / IMAGE_HEIGHT as f32 * VIEWPORT_HEIGHT)))),
                                                 mulvec(origin, -1.0)));

            let c = ray_color(ray);
            // if j % IMAGE_WIDTH == 0 {
            //     println!("color: {:?}", c);
            // }
            for rgb in 0..3 {
                color_bounds.0[rgb] = color_bounds.0[rgb].min(c[rgb]);
                color_bounds.1[rgb] = color_bounds.1[rgb].max(c[rgb]);
            }

            data[usize::try_from(4*(i * IMAGE_WIDTH + j) + 0).unwrap()] = (255.999 * c[0]) as u8;
            data[usize::try_from(4*(i * IMAGE_WIDTH + j) + 1).unwrap()] = (255.999 * c[1]) as u8;
            data[usize::try_from(4*(i * IMAGE_WIDTH + j) + 2).unwrap()] = (255.999 * c[2]) as u8;
            data[usize::try_from(4*(i * IMAGE_WIDTH + j) + 3).unwrap()] = 255;
        }
    }

    println!("color_bounds: {:#?}", color_bounds);

    write_img(r"/tmp/first_canvas.png", data);
    conclude("Goodbye fellow Rustaceans!");
}

fn init(arg: &str) -> u32{
    println!("welcome to fn!");
    println!("arg: {}", arg);

    for i in 0..3 {
        println!("{}", i);
    }
    return 42;
}

fn conclude(msg: &str) {
    let stdout = stdout();
    let message = String::from(msg);
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn write_img(filename: &str, data: Vec<u8>) {
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

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap(); // Save
}
