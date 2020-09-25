/*
recebe uma cena de entrada (formato do arquivo TBD)
parseia numa struct Scene
executar Raytracing
Gravar arquivo final (png?)


material: como calcular a cor,
cor difusa, intensidade difusa
cor especular intensidade especular
cor base, intensidade base


*/
#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

#[derive(PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}

#[derive(PartialEq, Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64
}

#[derive(PartialEq, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}

#[derive(PartialEq, Debug)]
struct Ray {
    origin: Point,
    direction: Vector
}

#[derive(PartialEq, Debug)]
struct Sphere {
    center: Point,
    radius: f64,
    color: Color
}


//start assuming center (0,0,0), direction (0,0,-1), (0,1,0)
#[derive(PartialEq, Debug)]
struct Camera {
    center: Point,
    direction: Vector,
    up: Vector,
    fov: f64
}

#[derive(Debug)]
struct Scene {
    //camera: Camera,
    objects: Vec<Sphere>
}

trait Hittable {
    fn hit(&self, ray: Ray) -> Option<&Point> {
        None
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray) -> Option<&Point> {
    /*
    vec3 oc = r.origin() - center;
    float a = dot(r.direction(), r.direction());
    float b = 2.0 * dot(oc, r.direction());
    float c = dot(oc,oc) - radius*radius;
    float discriminant = b*b - 4*a*c;
    if(discriminant < 0){
        return -1.0;
    }
    else{
        return (-b - sqrt(discriminant)) / (2.0*a);
    }
    */
        None
    }
}

fn main() {
    
    // let mut img = Image::new(100, 100);
    // for (x, y) in img.coordinates() {
    //     img.set_pixel(x, y, px!(x, y, 200));
    // }
    // let _ = img.save("img.bmp");
    let s = Sphere {
        center: Point { x: 0.0, y: 0.0, z: -100.0}, 
        radius: 50.0,
        color: Color {r: 100, g: 200, b: 255}
    };

    let mut objects = Vec::new();
    objects.push(s);
    let scene = Scene {
        objects: objects
    };
    println!("{:?}", scene)
}
