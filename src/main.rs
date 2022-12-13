mod ray;
mod vec3;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;

use ray::Ray;
use vec3::Vec3;
use hittable::{HitRecord, Hittable};
use hittable_list::*;
use sphere::Sphere;
use camera::Camera;
use rand::prelude::*;

  //Construct a ppm file for image data, e.g
    //P3
    //3 2
    //255
    //255 0   0     0  255 0      0 0 255
    //255 225 0   255 255 255     0 0 0

pub fn main(){
  
    let width: i32 = 200;
    let height: i32 = 100;
    let samples = 100;
    let max_value: i32 = 255;
    println!("P3\n{} {}\n{}", width, height, max_value);
    
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.)));
    let world = HittableList::new(list);

    let cam = Camera::new();
    let mut rng = rand::thread_rng();

    for j in (0..height).rev() {
        for i in 0..width {
            let mut col = Vec3::default();

            for _ in 0..samples {
                //TODO
                let u: f32 = (i as f32 + rng.gen::<f32>()) / width as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>()) / height as f32;
    
                let r = &cam.get_ray(u, v);
                col = col + color(&r, &world);
    
            }

            col = col / samples as f32;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;
            println!("{}, {}, {}", ir, ig, ib);
        }
    }

    let v1: Vec3 = Vec3::new(1., 2., 6.);
    let v2: Vec3 = Vec3::new(2., 6., 8.);

    let v3 = v1 + v2;

    // println!("Added v1 and v2: {:?}", v3);
}

fn color(r: &Ray, world: &HittableList ) -> Vec3 {
    let mut rec = HitRecord::default();
    
    if world.hit(r, 0.0, std::f32::MAX, &mut rec) {
        let target = rec.p() + rec.normal()+random_in_unit_sphere();
        return color(&Ray::new(rec.p(), target-rec.p()), &world)* 0.5;
    } else{
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t:f32 = 0.5*(unit_direction.y()+1.0);
    
        Vec3::new(1., 1., 1.) * (1.0 -t) + Vec3::new(0.5, 0.7, 1.)*t
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::default();
    let mut rng = rand::thread_rng();

    loop {
        p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())*2.0 - Vec3::new(1.0, 1.0, 1.0);
        if p.squart_length() < 1.0 {
            return p;
        }
    }

}