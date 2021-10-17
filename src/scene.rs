//
// scene module
//
// scenes containing objects made of materials
//

//pub use std::rc::Rc;
use crate::*;

// use crate::utils::*;
// use crate::objects::{Sphere, Jumble, Shot, Intersectable, HitRecord};

//todo: move build_scene from main.rs into this file when I figure out how

pub fn build_scene() -> Jumble {
    // instances of geometry
    let s1: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([0.0,0.0,-1.0]), 0.5));
    let s2: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([0.0,-100.5,-1.0]), 100.0));
    let s3: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([0.0,0.0,-1.0]), 0.5));

    // the main stage
    let mut scene = Jumble::new();
    scene.name = "main".to_string();


    // test fov is correctly computed
    let mut fov_test_scene = Jumble::new();
    fov_test_scene.name = "fov_test".to_string();
    let radius = (std::f32::consts::PI / 4.0).cos();
    // NOTE: two ways to declare the same type (the book teaches the first)
    let sl: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([-radius,0.0,-1.0]), radius));
    let sr = Rc::new(Sphere::new(Vec3::new([radius,0.0,-1.0]), radius)) as Rc<dyn Intersectable>;
    fov_test_scene.add(Rc::clone(&sl));
    fov_test_scene.add(Rc::clone(&sr));
    //scene.add(Rc::new(fov_test_scene) as Rc<dyn Intersectable>);


    let mut sub_scene = Jumble::new();
    sub_scene.name = "sub".to_string();
    sub_scene.add(Rc::clone(&s1));
    sub_scene.add(Rc::clone(&s2));
    scene.add(Rc::new(sub_scene) as Rc<dyn Intersectable>);


    let mut squishy_scene = Jumble::new();
    squishy_scene.name = "squishy".to_string();
    let mut csys = squishy_scene.csys();
    let scale = Matrix::scale(Vec3::new([1.5, 0.75, 1.0]));
    csys *= scale;
    csys.translate(Vec3::new([0.0,-0.5,0.0]));
    squishy_scene.set_csys(csys);
    squishy_scene.add(Rc::clone(&s3));
    squishy_scene.add(Rc::clone(&s1));
    squishy_scene.add(Rc::clone(&s2));
    scene.add(Rc::new(squishy_scene) as Rc<dyn Intersectable>);


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
    println!("csys:\n{}", csys);
    sq2.set_csys(csys);
    sq2.add(Rc::clone(&s1));
    scene.add(Rc::new(sq2) as Rc<dyn Intersectable>);


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
    sq3.add(Rc::clone(&s1));
    scene.add(Rc::new(sq3) as Rc<dyn Intersectable>);


    scene
}

