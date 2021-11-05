//
// scene module
//
// scenes containing objects made of materials
//

use crate::*;
use crate::materials::*;
use rand::{Rng, thread_rng};

pub fn build_scene() -> Jumble {
    // the main stage
    let mut scene = Jumble::new();
    scene.name = "main".to_string();


    // materials //
    let matgnd: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new([0.8, 0.8, 0.0])));

    // let matctr: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new([0.7, 0.3, 0.3])));
    //let matctr: Rc<dyn Material> = Rc::new(Transparent::new(Color::new([0.7, 0.3, 0.3]), 0.0, 1.5));
    let matctrbook: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new([0.1, 0.2, 0.5])));
    //let matctrbook: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new([1.0,1.0,1.0])));
    //let matctrbook: Rc<dyn Material> = Rc::new(Transparent::new(Color::new([1.0, 1.0, 1.0]), 0.0, 1.5));

    //let matleft: Rc<dyn Material> = Rc::new(Transparent::new(Color::new([0.8, 0.8, 0.8]), 0.0, 1.5));
    let matleft: Rc<dyn Material> = Rc::new(Shiny::new(Color::new([0.9, 0.9, 0.9]), 0.0));
    let matleftbook: Rc<dyn Material> = Rc::new(Transparent::new(Color::new([1.0, 1.0, 1.0]), 0.0, 1.5));

    let matright: Rc<dyn Material> = Rc::new(Shiny::new(Color::new([0.8, 0.6, 0.2]), 0.0));

/*
    // verify vfov working (one of the best things of the book are its tests)
    let radius = PI_4.cos();
    let matleft: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new([0.0, 0.0, 0.1])));
    let matright: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new([1.0, 0.0, 0.0])));
    let left: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([-radius,0.0,-1.0]), radius,
                                                          Rc::clone(&matleft)));
    let right: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([radius,0.0,-1.0]), radius,
                                                           Rc::clone(&matright)));
    scene.add(Rc::clone(&left));
    scene.add(Rc::clone(&right));
    return scene;
     */

    // instances of geometry
    let gnd: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([0.0,-100.5,-1.0]), 100.0,
                                                         Rc::clone(&matgnd)));
    let ctr: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([0.0,0.0,-1.0]), 0.5,
                                                         Rc::clone(&matctrbook)));
    let sctr: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([0.0,0.0,0.0]), 0.5,
                                                         Rc::clone(&matleft)));
    let lout: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([-1.0,0.0,-1.0]), 0.5,
                                                        Rc::clone(&matleftbook)));
    let lin: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([-1.0,0.0,-1.0]), -0.45,
                                                        Rc::clone(&matleftbook)));
    let l: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([-1.0,0.5,-1.0]), 0.5,
                                                       Rc::clone(&matleft)));
    let lbig: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([-100.5,0.5,-1.0]), 100.0,
                                                       Rc::clone(&matleft)));
    let r: Rc<dyn Intersectable> = Rc::new(Sphere::new(Vec3::new([1.0,0.0,-1.0]), 0.5,
                                                       Rc::clone(&matright)));

    let mut shiny_scene = Jumble::new();
    shiny_scene.name = "shiny".to_string();
    //shiny_scene.add(Rc::clone(&ctr)); // center
    shiny_scene.add(Rc::clone(&gnd)); // ground
    //shiny_scene.add(Rc::clone(&lout)); // left outer
    //shiny_scene.add(Rc::clone(&lin)); // left inner
    shiny_scene.add(Rc::clone(&lbig)); // left mirror
    //shiny_scene.add(Rc::clone(&l)); // left shiny
    //shiny_scene.add(Rc::clone(&r)); // right
    scene.add(Rc::new(shiny_scene) as Rc<dyn Intersectable>);

    let mut squishy_scene = Jumble::new();
    squishy_scene.name = "squishy".to_string();
    // one way to get uvw is to create the matrix then steal them -> a very poor way
    let mut mat = Matrix::identity();
    let rotz = Matrix::rotation_deg(45.0, Axis::Z);
    mat *= rotz;
    let rotx = Matrix::rotation_deg(-15.0, Axis::X);
    //mat *= rotx;
    //mat = mat.transpose();  // "passive" since we're rotating axes, not points and vectors
    //println!("uvw mat: {}", mat);

    let r0 = Matrix::rotation_deg(90.0, Axis::Z);
    let r1 = Matrix::rotation_deg(45.0, Axis::Y);
    let r2 = Matrix::rotation_deg(90.0, Axis::X);
    //let rotate = r0 * r1 * r2;
    let rotate = r0;
    //let rotate = Matrix::identity();
    //let scale = Matrix::scale(Vec3::new([1.0, 0.25, 0.5]));
    //let translate = Matrix::translation(Vec3::new([0.0, 0.5, -1.0]));
    // let basis = translate * rotate * scale; *
    // let basis = scale * rotate * translate; *
    //let basis = translate * scale * rotate;
    //let basis = rotate * scale * translate;
    //let basis = rotate * translate * scale;
    //let basis = scale * translate * rotate;
    //println!("basis:\n{}", basis);
    // let csys = CoordSys::new(Vec3::zero(), Vec3::new([1.0, 1.0, 1.0]),
    //                          basis.u(), basis.v(), basis.w());
    //let csys = CoordSys::from_matrix(basis.generic_inverse()); // works for geometry
    //let csys = CoordSys::from_matrix(basis.transpose()); // horribly fails with rot*scale*trans
    mat = rotate;
    println!("mat:\n{}", mat);
    let csys = CoordSys::new(
        Vec3::new([1.0, 0.5, -1.0]),  // origin
        //Vec3::new([0.0, 0.0, 0.0]),  // origin
        //Vec3::new([0.45, 1.5, 0.75]),  // scale
        //Vec3::new([0.5, 0.5, 0.5]),  // half scale
        Vec3::new([2.0, 0.15, 0.5]),  // saucer scale
        //Vec3::new([1.0, 1.0, 1.0]),  // scale
        // Vec3::new([0.0, -1.0, 0.0]),  // u
        // Vec3::new([1.0, 0.0, 0.0]),  // v
        // Vec3::new([0.0, 0.0, 1.0])); // w
        mat.u(), mat.v(), mat.w());
    squishy_scene.set_csys(csys);
    squishy_scene.add(Rc::clone(&sctr));
    //squishy_scene.add(Rc::clone(&r));
    scene.add(Rc::new(squishy_scene) as Rc<dyn Intersectable>);


    let mut sq2 = Jumble::new();
    sq2.name = "sq2".to_string();

    let rotate = Matrix::rotation(-3.0*PI_4, Axis::Z);
    //let rotate = Matrix::rotation(-PI_4, Axis::Y);
    //let rotate = Matrix::rotation(-PI_4, Axis::X);
    if crate::DEBUG {
        //println!("rotate:\n {}", rotate);
    }

    let scale = Matrix::scale(Vec3::new([0.5, 1.25, 1.0]));
    if crate::DEBUG {
        //println!("scale:\n {}", scale);
    }

    let translate = Matrix::translation(Vec3::new([-1.25, 0.25, 0.0]));
    if crate::DEBUG {
        //println!("csys:\n{}", csys);
    }
    let basis = scale * rotate * translate;
    let csys = CoordSys::new(Vec3::zero(), Vec3::new([1.0, 1.0, 1.0]),
                             basis.u(), basis.v(), basis.w());
    sq2.set_csys(csys);
    sq2.add(Rc::clone(&ctr));
    //scene.add(Rc::new(sq2) as Rc<dyn Intersectable>);


    let mut sq3 = Jumble::new();
    sq3.name = "sq3".to_string();
    let csys = Matrix::identity();

    let rotate = Matrix::rotation(-3.0*PI_4, Axis::Z) * Matrix::rotation(-PI_2, Axis::X);
    if crate::DEBUG {
        //println!("rotate:\n {}", rotate);
    }

    let scale = Matrix::scale(Vec3::new([0.5, 1.0, 1.1]));
    if crate::DEBUG {
        //println!("scale:\n {}", scale);
    }

    let translate = Matrix::translation(Vec3::new([1.25,-0.333,-0.25]));
    if crate::DEBUG {
        //println!("csys:\n {}", csys);
    }
    let basis = scale * rotate * translate;
    let csys = CoordSys::new(Vec3::zero(), Vec3::new([1.0, 1.0, 1.0]),
                             basis.u(), basis.v(), basis.w());
    sq3.set_csys(csys);
    sq3.add(Rc::clone(&ctr));
    //scene.add(Rc::new(sq3) as Rc<dyn Intersectable>);




////////////////////////////////////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////////////////////////////////////
    // let mut sub_scene = Jumble::new();
    // sub_scene.name = "sub".to_string();
    // sub_scene.add(Rc::clone(&ctr));
    // sub_scene.add(Rc::clone(&gnd));
    //scene.add(Rc::new(sub_scene) as Rc<dyn Intersectable>);
////////////////////////////////////////////////////////////////////////////////////////////


    scene
}

pub fn build_rtiow_final_scene() -> Jumble {
    // the main stage
    let mut scene = Jumble::new();
    scene.name = "main".to_string();

    // rng
    let mut rng = rand::thread_rng();

    // glass
    let glass: Rc<dyn Material> = Rc::new(Transparent::new(Color::new([1.0, 1.0, 1.0]), 0.0, 1.5));

    // ground
    let matgnd: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new([0.5, 0.5, 0.5])));
    scene.add(Rc::new(Sphere::new(Vec3::new([0.0,-1000.0,0.0]), 1000.0, Rc::clone(&matgnd))));

    // marbles
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Vec3::new([a as f32 + 0.9*rng.gen::<f32>(),
                                    0.2,
                                    b as f32 + 0.9*rng.gen::<f32>()]);

            if (center - Vec3::new([4.0, 0.2, 0.0])).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::rand() * Color::rand();
                    let mat: Rc<dyn Material> = Rc::new(Lambertian::new(albedo));
                    scene.add(Rc::new(Sphere::new(center, 0.2, Rc::clone(&mat))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::rand() / 2.0 + Color::new([0.5, 0.5, 0.5]);
                    let fuzz = rng.gen_range(0.0..=0.5);
                    let mat: Rc<dyn Material> = Rc::new(Shiny::new(albedo, fuzz));
                    scene.add(Rc::new(Sphere::new(center, 0.2, Rc::clone(&mat))));
                } else {
                    // glass
                    scene.add(Rc::new(Sphere::new(center, 0.2, Rc::clone(&glass))));
                }
            }
        }
    }

    // boulders
    scene.add(Rc::new(Sphere::new(Vec3::new([0.0, 1.0, 0.0]), 1.0, Rc::clone(&glass))));

    let mat: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new([0.4, 0.2, 0.1])));
    scene.add(Rc::new(Sphere::new(Vec3::new([-4.0, 1.0, 0.0]), 1.0, Rc::clone(&mat))));

    let mat: Rc<dyn Material> = Rc::new(Shiny::new(Color::new([0.4, 0.2, 0.1]), 0.0));
    scene.add(Rc::new(Sphere::new(Vec3::new([4.0, 1.0, 0.0]), 1.0, Rc::clone(&mat))));

    return scene;
}
