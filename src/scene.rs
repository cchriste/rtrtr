//
// scene module
//
// scenes containing objects made of materials
//

use crate::*;
use crate::materials::*;

pub fn build_scene() -> Jumble {
    // materials
    let matgnd: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new([0.8, 0.8, 0.0])));
    // let matctr: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new([0.7, 0.3, 0.3])));
    let matleft: Rc<dyn Material> = Rc::new(Shiny::new(Color::new([0.8, 0.8, 0.8]), 0.7));
    let matctr: Rc<dyn Material> = Rc::new(Transparent::new(Color::new([0.7, 0.3, 0.3]), 0.0, 1.5));
    //let matleft: Rc<dyn Material> = Rc::new(Transparent::new(Color::new([0.8, 0.8, 0.8]), 0.0, 1.5));
    let matctrbook: Rc<dyn Material> = Rc::new(Transparent::new(Color::new([1.0, 1.0, 1.0]), 0.0, 1.5));
    //let matctrbook: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new([0.1, 0.2, 0.5])));
    //let matleftbook: Rc<dyn Material> = Rc::new(Transparent::new(Color::new([1.0, 1.0, 1.0]), 0.0, 1.5));
    let matright: Rc<dyn Material> = Rc::new(Shiny::new(Color::new([0.8, 0.6, 0.2]), 0.0));

    // instances of geometry
    let s1: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([0.0,0.0,-1.0]),
                                                        0.5, Rc::clone(&matctrbook)));
    let s2: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([0.0,-100.5,-1.0]),
                                                        100.0, Rc::clone(&matgnd)));
    let s3: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([-1.0,0.0,-1.0]),
                                                        0.5, Rc::clone(&matleft)));
    let s4: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([1.0,0.0,-1.0]),
                                                        0.5, Rc::clone(&matright)));

    // the main stage
    let mut scene = Jumble::new();
    scene.name = "main".to_string();


    let mut shiny_scene = Jumble::new();
    shiny_scene.name = "shiny".to_string();
    shiny_scene.add(Rc::clone(&s1)); // center
    shiny_scene.add(Rc::clone(&s2)); // ground
    shiny_scene.add(Rc::clone(&s3)); // left
    shiny_scene.add(Rc::clone(&s4)); // right
    scene.add(Rc::new(shiny_scene) as Rc<dyn Intersectable>);


    // test fov is correctly computed // TODO: move to unit tests when lib (see ch 12)
    // let mut fov_test_scene = Jumble::new();
    // fov_test_scene.name = "fov_test".to_string();
    // let radius = (std::f32::consts::PI / 4.0).cos();
    // NOTE: two ways to declare the same type (the book teaches the first)
    // let sl: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([-radius,0.0,-1.0]), radius));
    // let sr = Rc::new(Sphere::new(Vec3::new([radius,0.0,-1.0]), radius)) as Rc<dyn Intersectable>;
    // fov_test_scene.add(Rc::clone(&sl));
    // fov_test_scene.add(Rc::clone(&sr));
    //scene.add(Rc::new(fov_test_scene) as Rc<dyn Intersectable>);


    let mut sub_scene = Jumble::new();
    sub_scene.name = "sub".to_string();
    sub_scene.add(Rc::clone(&s1));
    sub_scene.add(Rc::clone(&s2));
    //scene.add(Rc::new(sub_scene) as Rc<dyn Intersectable>);


    let mut squishy_scene = Jumble::new();
    squishy_scene.name = "squishy".to_string();
    let mut csys = squishy_scene.csys();
    let scale = Matrix::scale(Vec3::new([1.5, 0.75, 1.0]));
    csys *= scale;
    csys.translate(Vec3::new([0.0,-0.5,0.0]));
    squishy_scene.set_csys(csys);
    squishy_scene.add(Rc::clone(&s1));
    squishy_scene.add(Rc::clone(&s2));
    //scene.add(Rc::new(squishy_scene) as Rc<dyn Intersectable>);


    let mut sq2 = Jumble::new();
    sq2.name = "sq2".to_string();
    let mut csys = sq2.csys();

    let rot = Matrix::rotation(-3.0*PI_4, Axis::Z);
    //let rot = Matrix::rotation(-PI_4, Axis::Y);
    //let rot = Matrix::rotation(-PI_4, Axis::X);
    if crate::DEBUG {
        //println!("rot:\n {}", rot);
    }
    csys *= rot;

    let scale = Matrix::scale(Vec3::new([0.5, 1.25, 1.0]));
    if crate::DEBUG {
        //println!("scale:\n {}", scale);
    }
    csys *= scale;

    csys.translate(Vec3::new([-1.25, 0.25, 0.0]));
    if crate::DEBUG {
        //println!("csys:\n{}", csys);
    }
    sq2.set_csys(csys);
    sq2.add(Rc::clone(&s1));
    //scene.add(Rc::new(sq2) as Rc<dyn Intersectable>);


    let mut sq3 = Jumble::new();
    sq3.name = "sq3".to_string();
    let mut csys = sq3.csys();

    let rot = Matrix::rotation(-3.0*PI_4, Axis::Z);
    if crate::DEBUG {
        //println!("rot:\n {}", rot);
    }
    csys *= rot;
    let rot = Matrix::rotation(-PI_2, Axis::X);
    if crate::DEBUG {
        //println!("rot:\n {}", rot);
    }
    csys *= rot;

    let scale = Matrix::scale(Vec3::new([0.5, 1.0, 1.1]));
    if crate::DEBUG {
        //println!("scale:\n {}", scale);
    }
    csys *= scale;

    csys.translate(Vec3::new([1.25,-0.333,-0.25]));
    if crate::DEBUG {
        //println!("csys:\n {}", csys);
    }
    sq3.set_csys(csys);
    sq3.add(Rc::clone(&s1));
    //scene.add(Rc::new(sq3) as Rc<dyn Intersectable>);


    scene
}

