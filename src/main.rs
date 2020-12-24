mod vector;
mod shapes;
mod canvas;
mod light;
mod material;

use canvas::Canvas;
use shapes::Sphere;
use shapes::Ray;
use material::Material;
use vector::Vec3f;
use light::Light;

#[cfg(feature = "parallel")]
use rayon::prelude::*;


const WIDTH:  usize  = 1920;
const HEIGHT: usize  = 1080;
const WIDTH_F:  f64  = WIDTH as f64;
const HEIGHT_F: f64  = HEIGHT as f64;
const FOV: f64 = std::f64::consts::PI/3.;
const ORIGIN: Vec3f = vector::ORIGIN;

const IVORY: Material = Material::new(
        Vec3f::new(0.4, 0.4, 0.3),
        Vec3f::new(0.6, 0.3, 0.1),
        50.
);

// const MORE_REFLECTIVE_IVORY: Material = Material::new(
//         Vec3f::new(0.4, 0.4, 0.3),
//         Vec3f::new(0.6, 0.3, 0.1),
//         100.
// );

const RED_RUBBER: Material = Material::new(
        Vec3f::new(0.3, 0.1, 0.1),
        Vec3f::new(0.9, 0.1, 0.0),
        10.
);

const MIRROR: Material = Material::new (
        Vec3f::new(1.0, 1.0, 1.0),
        Vec3f::new(0.0, 10.0, 0.8),
        1425.
);

// The spheres used on the scene.
// Feel free to modify 
const SPHERES: [Sphere; 5] = [
                Sphere::new(Vec3f::new(-20., 5., -20.), 6., MIRROR),
                Sphere::new(Vec3f::new(-3. , 0., -16.), 2., IVORY),
                Sphere::new(Vec3f::new(-1., -1.5, -12.), 2., MIRROR),
                Sphere::new(Vec3f::new(1.5, -0.5, -18.), 3., RED_RUBBER),
                Sphere::new(Vec3f::new(7., 5., -18.), 4., MIRROR),
];


fn render (spheres: &mut Vec<Sphere>, lights: &mut Vec<Light>, filename: &str) {

    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let (iu, ju) = (i, j);
            let (i, j) = ((2*i) as f64, (2*j) as f64); 
            let x =  (i + 1.) - WIDTH_F;
            let y = -(j + 1.) + HEIGHT_F;
            let dir = Vec3f::new
                (x, 
                 y, 
                 -HEIGHT_F/(f64::tan(FOV/2.0)))
                .normalize();
            let ray = Ray::new(ORIGIN, dir);
            canvas.set(iu, ju, ray.cast(spheres, lights, 0));
        }
    }

    canvas.save_to_image(&format!("{}.ppm", filename));
 
}

fn main() {


    #[cfg(not(feature = "parallel"))]
    // There'd be a closure problem when using this variable within the parallelized code
    let mut spheres = SPHERES.to_vec();

    #[cfg(feature = "parallel")]
    {
        // If using Rayon, we'll make a small animation of 600 frames
        // This could take long
        (0..600).into_par_iter().for_each(|i|
        {
            let i = i as f64;
            let j = if i > 300. {
                600. - i
            } else {
                i
            };

            let mut spheres = SPHERES.to_vec();
            spheres.push(Sphere::new(Vec3f::new(10. - j * 0.075, 10., -20.), 2., MIRROR));


            let r = 20.;
        
            let mut lights = vec![
                Light::new(Vec3f::new(
                    -20. + r*f64::cos(i/20.), 
                     20. + r*f64::sin(i/20.), 
                     20.), 
                1.5),
                Light::new(Vec3f::new(
                    40.+5. * r*f64::cos(i/25.), 
                    20., 
                    30. + 5. *r*f64::sin(i/25.)),
                1.7),
                Light::new(Vec3f::new(
                    260.-i, 
                    160. * r*f64::cos(i/20.), 
                    130. * r*f64::sin(i/20.)), 
                1.7)
            ];        
            let filename = format!("output_{}", i);
            render(&mut spheres, &mut lights, &filename);
            println!("{}.ppm saved!", filename);
        }
        ); 
    }

    #[cfg(not(feature = "parallel"))]
    {
        // If not using Rayon, render a simple scene
        let mut lights = Vec::with_capacity(3);

        lights.push(Light::new(Vec3f::new(-20., 20., 20.), 1.5));
        lights.push(Light::new(Vec3f::new(30., 50., -25.), 1.8));
        lights.push(Light::new(Vec3f::new(30., 20., 30.), 1.7));

        render(&mut spheres, &mut lights, "output");
    }    
}
