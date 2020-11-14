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

const SPHERES: [Sphere; 5] = [
                Sphere::new(Vec3f::new(-20., 5., -20.), 6., MIRROR),
                Sphere::new(Vec3f::new(-3. , 0., -16.), 2., IVORY),
                Sphere::new(Vec3f::new(-1., -1.5, -12.), 2., MIRROR),
                Sphere::new(Vec3f::new(1.5, -0.5, -18.), 3., RED_RUBBER),
                Sphere::new(Vec3f::new(7., 5., -18.), 4., MIRROR),
];


fn render (spheres: &mut Vec<Sphere>, lights: &mut Vec<Light>, filename: &str) {
    let width  = 1920;
    let height = 1080;
    let fov = std::f64::consts::PI/3.;
    let origin = Vec3f::new(0., 0., 0.);
    let fovtan = f64::tan(fov/2.);
    
    let mut canvas = Canvas::new(width, height);

    for j in 0..height {
        for i in 0..width {
            let (iu, ju) = (i, j);
            let (i, j, width, height) = ((2*i) as f64, (2*j) as f64, width as f64, height as f64); 
            let x =  ((i + 1.)/width  - 1.0)*fovtan*width/height;
            let y = -((j + 1.)/height - 1.0)*fovtan;
            let dir = Vec3f::new(x, y, -1.0).normalize();
            let ray = Ray::new(origin, dir);
            canvas.set(iu, ju, ray.cast(spheres, lights, 0));
        }
    }

    canvas.save_to_image(&format!("{}.ppm", filename).to_string());
 
}

fn main() {


    #[cfg(not(feature = "parallel"))]
    // There'd be a closure problem when using this variable within the parallelized code
    let mut spheres = SPHERES.to_vec();

    #[cfg(feature = "parallel")]
    {
        (0..300).into_par_iter().for_each(|i|
        {
            let i = i as f64;

            let morphing_diffuse_color = if i > 150. {
                1.0
            } else {
                i * 0.006 + 0.1
            };

            let morphing_material = Material::new(
                Vec3f::new(morphing_diffuse_color, morphing_diffuse_color, morphing_diffuse_color),
                Vec3f::new(10.0 - 2.*i * 0.03333, 2.*i + 0.033333, 2.*i * 0.00266666),
                if i > 142. {
                    1425.
                } else {
                    10. + 15. * i 
                }
            );

            let mut spheres = SPHERES.to_vec();
            spheres.push(Sphere::new(Vec3f::new(10. - i * 0.075, 10., -20.), 2., morphing_material));

            let mut lights = vec![
                Light::new(Vec3f::new(-20.+1.5*i, 20. + 1.5-i, 20.), 1.5),
                Light::new(Vec3f::new(-40.+i, 50.+i, -25.+i/50.), 1.8 + i * 0.0005),            
                Light::new(Vec3f::new(40.+i, 10.+i, 30.-i/50.), 1.7),
                Light::new(Vec3f::new(260.-i, 160.-i, 130.-i/50.), 1.7)
            ];        
            let filename = format!("output_{}", i);
            render(&mut spheres, &mut lights, &filename);
            println!("{}.ppm saved!", filename);
        }
        ); 
    }

    #[cfg(not(feature = "parallel"))]
    {
        let mut lights = Vec::with_capacity(3);

        lights.push(Light::new(Vec3f::new(-20., 20., 20.), 1.5));
        lights.push(Light::new(Vec3f::new(30., 50., -25.), 1.8));
        lights.push(Light::new(Vec3f::new(30., 20., 30.), 1.7));

        render(&mut spheres, &mut lights, "output");
    }    
}
