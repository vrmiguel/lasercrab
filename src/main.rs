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

fn render (spheres: &mut Vec<Sphere>, lights: &mut Vec<Light>) {
    let width  = 1920;
    let height = 1080;
    let fov = std::f64::consts::PI/3.;
    let origin = Vec3f::new(0., 0., 0.);    
    
    let mut canvas = Canvas::new(width, height);

    for j in 0..height {
        for i in 0..width {
            // let idx = i + j * width;
            let (iu, ju) = (i, j);
            let (i, j, width, height) = (i as f64, j as f64, width as f64, height as f64); 
            let x =  (2.0*(i + 0.5)/width  - 1.0)*f64::tan(fov/2.)*width/height;
            let y = -(2.0*(j + 0.5)/height - 1.0)*f64::tan(fov/2.);
            let dir = Vec3f::new(x, y, -1.0);
            let dir = dir.normalize();
            let ray = Ray::new(origin, dir);
            canvas.set(iu, ju, ray.cast(spheres, lights, 0));
        }
    }

    canvas.save_to_image("output.ppm");
 
}

fn main() {
    // let sphere = Sphere::new(Vec3f::new(-3.0, 0.0, -16.0), 2.0);
    let ivory = Material::new(
        Vec3f::new(0.4, 0.4, 0.3),
        Vec3f::new(0.6, 0.3, 0.1),
        50.
    );
    let red_rubber = Material::new(
        Vec3f::new(0.3, 0.1, 0.1),
        Vec3f::new(0.9, 0.1, 0.0),
        10.
    );
    let mirror = Material::new (
        Vec3f::new(1.0, 1.0, 1.0),
        Vec3f::new(0.0, 10.0, 0.8),
        1425.
    );

    let mut spheres = Vec::with_capacity(5);

    spheres.push(Sphere::new(
        Vec3f::new(-20.,    5.,   -20.), 6., mirror)
    );

    spheres.push(Sphere::new(
        Vec3f::new(-3.,    0.,   -16.), 2., ivory)
    );

    spheres.push(Sphere::new(
        Vec3f::new(-1.,    -1.5,   -12.), 2., mirror)
    );


    spheres.push(Sphere::new(
        Vec3f::new(1.5,    -0.5,   -18.), 3., red_rubber)
    );

    spheres.push(Sphere::new(
        Vec3f::new(18.5,    -0.5,   -18.), 3., ivory)
    );

    spheres.push(Sphere::new(
        Vec3f::new(7.,    5.,   -18.), 4., mirror)
    );

    let mut lights = Vec::with_capacity(3);

    lights.push(Light::new(Vec3f::new(-20., 20.,  20.), 1.5));
    lights.push(Light::new(Vec3f::new(30., 50.,  -25.), 1.8));
    lights.push(Light::new(Vec3f::new(30., 20.,  30.), 1.7));

    render(&mut spheres, &mut lights);
}
