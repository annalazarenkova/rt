mod ray;
mod vec3;
mod hittable;
mod hittable_list;
mod sphere;

use ray::Ray;
use vec3::Vec3;
use hittable::{HitRecord, Hittable};
use hittable_list::*;
use sphere::Sphere;


pub fn main(){
  
    let width: i32 = 200;
    let height: i32 = 100;
    let max_value: i32 = 255;
    //Construct a ppm file for image data, e.g
    //P3
    //3 2
    //255
    //255 0   0     0  255 0      0 0 255
    //255 225 0   255 255 255     0 0 0
    println!("P3\n{} {}\n{}", width, height, max_value);

    let lower_left_corner : Vec3 = Vec3::new(-2., -1., -1.);
    let horizontal: Vec3 = Vec3::new(4., 0., 0.);
    let vertical: Vec3 = Vec3::new(0., 2., 0.);
    let origin: Vec3 = Vec3::new(0., 0., 0.);
    
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.)));
    let world = HittableList::new(list);

    for j in (0..height).rev() {
        for i in 0..width {
            let u: f32 = i as f32 / width as f32;
            let v: f32 = j as f32 / height as f32;

            let r :Ray = Ray::ray(origin, lower_left_corner + horizontal*u + vertical*v );
            
            let col = color(&r, &world);

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
        return Vec3::new(rec.normal().x()+1.0, rec.normal().y()+1.0, rec.normal().z()+1.0) * 0.5;
    } else{
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t:f32 = 0.5*(unit_direction.y()+1.0);
    
        Vec3::new(1., 1., 1.) * (1.0 -t) + Vec3::new(0.5, 0.7, 1.)*t
    }
}
