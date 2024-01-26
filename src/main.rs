mod ray;
mod vector;
mod color;
mod object;
mod cast;

use crate::ray::Ray;
use crate::vector::Vector3;
use crate::color::Color;
use crate::object::Object;

use std::fs;
use std::env;

fn main() {
    const SCREEN_WIDTH: usize = 100;
    const SCREEN_HEIGHT: usize = 100;
    const SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;


    let mut rays: Vec<Ray>= vec![Ray::null(); SIZE];
    let mut framebuffer: Vec<Color> = vec![Color::black(); SIZE];
    let mut framebuffer2: Vec<Color> = vec![Color::black(); SIZE];

    let sphere = Object::sphere(Vector3::new(0.0, 0.0, -5.0), 1.0, Color::red());

    let index = |x: usize, y: usize| -> usize {
        return (y * SCREEN_WIDTH) + x;
    };

    //let sky = Color::blend(Color::blue(), Color::blend(Color::white(), Color::black()));
    let sky = Color::new(0.098, 0.741, 1.0);

    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            rays[index(x, y)].pos = Vector3::null();
            rays[index(x, y)].dir = Vector3::normalize(&Vector3 {
                x: ((x as f32 / SCREEN_WIDTH as f32) - 0.5),
                y: ((y as f32 / SCREEN_HEIGHT as f32) - 0.5),
                z: -1.0,
            });
            let cast = Object::intersect(&sphere, &rays[index(x, y)]);

            if cast.hit {
                let val = 1.0 - (f32::atan(cast.dist * 0.25) / std::f32::consts::PI * 2.0);
                framebuffer[index(x, y)] = Color::scale(cast.color, val);
            } else {
                framebuffer[index(x, y)] = sky;
            }
        }
    }

    let args: Vec<String> = env::args().collect();

    if args.len() > 3 {
        panic!("No filepath was specified");
    }

    let path = &args[1];

    for x in 1..SCREEN_WIDTH-1 {
        for y in 1..SCREEN_HEIGHT-1 {
            let l = framebuffer[index(x-1, y)];
            let t = framebuffer[index(x, y-1)];
            let r = framebuffer[index(x+1, y)];
            let b = framebuffer[index(x, y+1)];

            //let s = Color::add(l, Color::add(t, Color::add(r, b)));

            let compose = |a: f32, b: f32, c: f32, d: f32| -> f32 {
                return f32::sqrt((0.25 * a * a) + (0.25 * b * b) + (0.25 * c * c) + (0.25 * d * d));
            };

            let s = Color::new(compose(l.r, t.r, r.r, b.r), compose(l.g, t.g, r.g, b.g), compose(l.b, t.b, r.b, b.b));


            //let a = Color::scale(s, 0.5);
            if args.len() == 3 && &args[2] == "aa" {
                // ""antialiasing""
                framebuffer2[index(x, y)] = s;
            } else {
                framebuffer2[index(x, y)] = framebuffer[index(x, y)];
            }
        }
    }

    let mut contents = String::new();

    contents.push_str("P3\n");
    contents.push_str(&format!("{} {}\n", SCREEN_WIDTH, SCREEN_HEIGHT));
    contents.push_str(&format!("{}\n", 255));

    for y in 0..SCREEN_WIDTH {
        for x in 0..SCREEN_HEIGHT {
            let pixel = &framebuffer2[index(y, x)];
            contents.push_str(&format!("{} {} {}  ", (pixel.r * 255.0) as u8, (pixel.g * 255.0) as u8, (pixel.b * 255.0) as u8));
        }
        contents.push('\n');
    }

    fs::write(path,  contents).expect(&format!("Unable to write to file {}", path));
}
