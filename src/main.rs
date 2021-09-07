use ferris_says::say;
use png;
use std::io::{stdout, BufWriter};
use std::convert::TryFrom;
use std::f32; // for .sqrt() method

// screen
const ASPECT: f32 = 16.0/9.0;  // width/height
const IMAGE_WIDTH: u32 = 200;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT) as u32;

// ?'s:
// - is operator overloading possible? function overloading isn't... :(
// - what is * notation for? (& is borrowing, etc)


#[derive(Debug)]  // enables it to be printed. What else?
#[derive(Copy, Clone)]
struct Vector {
    v: [f32; 3],  // make it 4 elements to add w/a
}

// alternative universe (maybe possible with...?)
// struct Vector {
//     x: f32,
//     y: f32,
//     z: f32,
//     w: f32,
// }

impl Vector {
    fn init(x: f32, y: f32, z: f32) -> Vector {
        Vector { v: [x, y, z] }
    }

    fn negate(&mut self) -> &Vector {
        self.v[0] = -self.v[0];
        self.v[1] = -self.v[1];
        self.v[2] = -self.v[2];
        self
    }
    fn dot(&self, other: &Vector) -> Vector {
        Vector { v: [self.v[0] * other.v[0],
                     self.v[1] * other.v[1],
                     self.v[2] * other.v[2]] }
    }
    fn add(&self, other: &Vector) -> Vector {
        Vector { v: [self.v[0] + other.v[0],
                     self.v[1] + other.v[1],
                     self.v[2] + other.v[2]] }
    }
    // todo: +=, etc
    fn mul(&self, k: f32) -> Vector {
        Vector { v: [self.v[0] * k,
                     self.v[1] * k,
                     self.v[2] * k] }
    }
    fn normalize(&mut self) -> &Vector {
        let magnitude = (self.v[0]*self.v[0] +
                         self.v[1]*self.v[1] +
                         self.v[2]*self.v[2]).sqrt();
        self.v[0] /= magnitude;
        self.v[1] /= magnitude;
        self.v[2] /= magnitude;
        self
    }

    fn normalized(mut vec: Vector) -> Vector {
        vec.normalize();
        vec
    }
}

// todo: read ch 4 on ownership after unsuccessfully trying all sorts of variations here
// fn dotself(&(mut vec): &[f32; 3]) -> &[f32; 3] {
//     vec[0] *= vec[0];
//     vec[1] *= vec[1];
//     vec[2] *= vec[2];
//     return &vec; // doesn't work (neither does just &vec or vec
// }
// todo class ray: (origin: [f32; 3], dir: [f32; 3]);

fn ray_color(ray: (Vector, Vector)) -> Vector {
    // let (_origin, dir) = ray;
    // let dir = dir.normalize(dir);// hmm... can I get a _copy_ of ray.dir?
    let dir = Vector::normalized(ray.1);
    Vector { v: [0.5 * (dir.v[1] + 1.0),
                 0.25,
                 0.5 * (dir.v[0] + 1.0)] }
}

fn main() {
    let str = "create a png";
    // TODO: how do I print the type of str?
    println!("{}", str); // note the ! after println cuz it's a macro
    init("bowwwah!");

    let v1 = Vector { v: [1.0, 2., 3.] };
    let v2 = Vector { v: [4., -1., 3.] };

    println!("v1 dot v2 = `{:?}", v1.dot(&v2));
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

    let origin = Vector::init(0.0, 0.0, 0.0);
    let right = Vector::init(1.0, 0.0, 0.0);
    let up = Vector::init(0.0, 1.0, 0.0);
    //let topleft = [origin - right/2.0, origin + up/2.0, origin[2] - FOCAL_LENGTH];
    let topleft = origin.add(&Vector::init(0., 0., origin.v[2] - FOCAL_LENGTH)
                             .add(&right.mul(-0.5)).add(&up.mul(0.5)));

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
            let ray: (Vector, Vector) = (origin,
                                         topleft.add(&right.mul(i as f32 / IMAGE_WIDTH as f32 * VIEWPORT_WIDTH ))
                                                .add(&up.mul(-1. * i as f32 / IMAGE_HEIGHT as f32 * VIEWPORT_HEIGHT))
                                                .add(&origin.mul(-1.0)));

            let c = ray_color(ray);
            // if j % IMAGE_WIDTH == 0 {
            //     println!("color: {:?}", c);
            // }
            for rgb in 0..3 {
                color_bounds.0[rgb] = color_bounds.0[rgb].min(c.v[rgb]);
                color_bounds.1[rgb] = color_bounds.1[rgb].max(c.v[rgb]);
            }

            data[usize::try_from(4*(i * IMAGE_WIDTH + j) + 0).unwrap()] = (255.999 * c.v[0]) as u8;
            data[usize::try_from(4*(i * IMAGE_WIDTH + j) + 1).unwrap()] = (255.999 * c.v[1]) as u8;
            data[usize::try_from(4*(i * IMAGE_WIDTH + j) + 2).unwrap()] = (255.999 * c.v[2]) as u8;
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
