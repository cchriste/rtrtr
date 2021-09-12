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
//   maybe traits? here we declare the Foo trait with a bar method
// trait Foo {
//     fn bar(&self);
// }
// we now declare a function which takes an object implementing the Foo trait
// fn some_func<T: Foo>(foo: T) {
//     foo.bar();
// }
// - what is * notation for? (& is borrowing, etc)
// - what is try_from, as in usize::try_from(4 * IMAGE_WIDTH * IMAGE_HEIGHT).unwrap()
// - global variables? (`let foo = 42;` doesn't work)
//   let mut rng = rand::thread_rng(); // doesn't work
// - multiple files, please!
// - ternary expression guaranteeing assignment: let a = if x > 5 { 10 } else { 7 };

#[derive(Debug)]  // enables it to be printed. What else?
#[derive(Copy, Clone)]
struct Vector {
    v: [f32; 3],  // make it 4 elements to add w/a
}

// Thursday, September 9, 2021 - this is growing on me... maybe functions?
// ...that returns refs? so v.x() = 42.0;
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
    fn x(&self) -> f32 { self.v[0] }
    fn y(&self) -> f32 { self.v[1] }
    fn z(&self) -> f32 { self.v[2] }
    fn len(&self) -> f32 {
        (self.v[0]*self.v[0] +
         self.v[1]*self.v[1] +
         self.v[2]*self.v[2]).sqrt()
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
    fn sub(&self, other: &Vector) -> Vector {
        Vector { v: [self.v[0] - other.v[0],
                     self.v[1] - other.v[1],
                     self.v[2] - other.v[2]] }
    }
    fn mul(&self, k: f32) -> Vector {
        Vector { v: [self.v[0] * k,
                     self.v[1] * k,
                     self.v[2] * k] }
    }
    fn div(&self, k: f32) -> Vector {
        Vector { v: [self.v[0] / k,
                     self.v[1] / k,
                     self.v[2] / k] }
    }
    fn cross(&self, other: &Vector) -> Vector {
        //        |  î   ĵ   k̂ |
        // det of | a0  a1  a2 |
        //        | b0  b1  b2 |
        //
        // = (a1b2 - a2b1)î - (a2b0 - a0b2)ĵ + (a0b1 - a1b0)k̂
        //
        Vector { v: [self.v[1]*other.v[2] - self.v[2]*other.v[1],
                     self.v[2]*other.v[0] - self.v[0]*other.v[2],
                     self.v[0]*other.v[1] - self.v[1]*other.v[0]] }
    }
    // <ctc> is this really necessary? I don't think so.
    fn negate(&mut self) -> &Vector {
        self.v[0] = -self.v[0];
        self.v[1] = -self.v[1];
        self.v[2] = -self.v[2];
        self
    }
    fn normalize(&mut self) -> &Vector {
        let magnitude = self.len();
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
// still curious since it's a function that modifies an input -> ** just don't return **
// fn dotself(&(mut vec): &[f32; 3]) -> &[f32; 3] {
//     vec[0] *= vec[0];
//     vec[1] *= vec[1];
//     vec[2] *= vec[2];
//     return &vec; // doesn't work (neither does just &vec or vec
// }

// todo class ray: (origin: [f32; 3], dir: [f32; 3]);

// color of ray(origin, dir)
fn ray_color(ray: &(Vector, Vector)) -> Vector {
    let unit_dir = Vector::normalized(ray.1);
    if (1.0 - unit_dir.len()).abs() > f32::EPSILON {
        println!("unit dir len isn't correct! {:?}, {:?}", unit_dir, ray.1);
    }
    let t = 0.5*(unit_dir.y() + 1.0); // vertical percent along viewport
    let white = Vector::init(1.0, 1.0, 1.0);
    let bluey = Vector::init(0.5, 0.7, 1.0);
    //white.mul(1.0 - t).add(&bluey.mul(t))
    bluey.mul(t)
}

// try to reduce annoyingly verbose array indices from u32s
//fn idx(i: u32) -> usize { return usize::try_from(idx).unwrap(); }
// Damn: E0277 missing trait again

fn main() {

/*
    // experimental learning stuff (but annoying output)

    let str = "create a png";
    // TODO: how do I print the type of str?
    println!("{} {}", str, foo); // note the ! after println cuz it's a macro

    let v1 = Vector { v: [1.0, 2., 3.] };
    let v2 = Vector { v: [4., -1., 3.] };

    println!("v1 dot v2 = `{:?}", v1.dot(&v2));
    // println!("v1 dot v1 = `{:#?}", dotself(&mut v1));
    // println!("v1 after dotself: {:?}", v1);

    let mut vec = Vector { v: [0.0, 1.0, -1.25] };
    println!("vec: {:?}", vec);
    vec.negate();
    println!("vec after negation: {:?}", vec);
*/

    // ////////////////////////////////////////// //


    // viewport
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT;
    const FOCAL_LENGTH: f32 = 0.25;//1.0;

    let origin = Vector::init(0.0, 0.0, 0.0);
    let right = Vector::init(VIEWPORT_WIDTH, 0.0, 0.0);
    let up = Vector::init(0.0, VIEWPORT_HEIGHT, 0.0);
    let z = right.cross(&up);
    let look = z.mul(-1.0);

    //let botleft = origin - right/2.0 - up/2.0 + look*FOCAL_LENGTH];
    let botleft = origin.sub(&right.mul(0.5)).sub(&up.mul(0.5)).add(&look.mul(FOCAL_LENGTH));
    println!("botleft: {:?}", botleft);

    // allocate image (just set length, don't initialize)
    let mut img: Vec<f32> = Vec::with_capacity(usize::try_from(4 * IMAGE_WIDTH * IMAGE_HEIGHT).unwrap());
    unsafe { img.set_len(img.capacity()); }

    // keep track of min/max color values
    let mut color_bounds: ([f32; 3], [f32; 3]) = ([1.0, 1.0, 1.0], [0.0, 0.0, 0.0]);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pct_x = i as f32 / (IMAGE_WIDTH-1) as f32;
            let pct_y = j as f32 / (IMAGE_HEIGHT-1) as f32;

            // ray: origin = O, direction = point moving across image from botleft - origin
            let p1 = botleft.add(&right.mul(pct_x)).add(&up.mul(pct_y));
            let p0 = origin;
            let ray = (origin, p1.sub(&p0));

            let color = ray_color(&ray);
            // if j % IMAGE_WIDTH == 0 {
            //     println!("color: {:?}", color);
            // }

            // update color minmax
            for c in 0..3 {
                color_bounds.0[c] = color_bounds.0[c].min(color.v[c]);
                color_bounds.1[c] = color_bounds.1[c].max(color.v[c]);
            }

            img[usize::try_from(4*(j * IMAGE_WIDTH + i) + 0).unwrap()] = color.v[0];
            img[usize::try_from(4*(j * IMAGE_WIDTH + i) + 1).unwrap()] = color.v[1];
            img[usize::try_from(4*(j * IMAGE_WIDTH + i) + 2).unwrap()] = color.v[2];
            img[usize::try_from(4*(j * IMAGE_WIDTH + i) + 3).unwrap()] = 1.0;
        }
    }

    // compute color range
    // TODO: x-x = NaN or inf; fixme
    let rrng = 1.0/(color_bounds.1[0] - color_bounds.0[0]);
    let grng = 1.0/(color_bounds.1[1] - color_bounds.0[1]);
    let brng = 1.0/(color_bounds.1[2] - color_bounds.0[2]);
    let c_rng = Vector::init(if rrng.is_normal() { rrng } else { 1.0 },
                             if grng.is_normal() { grng } else { 1.0 },
                             if brng.is_normal() { brng } else { 1.0 });
    println!("color_bounds: {:#?}", color_bounds);
    println!("color_range_scalar: {:#?}", c_rng);

    //let c_rng = Vector::init(1.0, 1.0, 1.0);  // remove scaling (don't really want it actually)

    // scale color to [0..1]
    for i in 0..IMAGE_HEIGHT*IMAGE_WIDTH {
        let r = img[usize::try_from(4*i + 0).unwrap()];
        let g = img[usize::try_from(4*i + 1).unwrap()];
        let b = img[usize::try_from(4*i + 2).unwrap()];
        let a = img[usize::try_from(4*i + 3).unwrap()];

        img[usize::try_from(4*i + 0).unwrap()] = (r - color_bounds.0[0]) * c_rng.v[0] + color_bounds.0[0];
        img[usize::try_from(4*i + 1).unwrap()] = (g - color_bounds.0[1]) * c_rng.v[1] + color_bounds.0[1];
        img[usize::try_from(4*i + 2).unwrap()] = (b - color_bounds.0[2]) * c_rng.v[2] + color_bounds.0[2];
        // img[usize::try_from(4*i + 0).unwrap()] = r;
        // img[usize::try_from(4*i + 1).unwrap()] = g;
        // img[usize::try_from(4*i + 2).unwrap()] = b;
        img[usize::try_from(4*i + 3).unwrap()] = a;
    }

    write_img(r"/tmp/canvas_color.png", img);
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
