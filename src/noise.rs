use std::path::PathBuf;

use image;
use image::Pixel;

use rand_distr::{Exp, Normal, Distribution};
use rand;

struct Narg {
    pixel: u8,
    exp_rand: f32,
    std_dev: f32,
    exp_mult: f32,
}

impl Default for Narg {
    fn default() -> Self {
        Narg {
            pixel:0,
            exp_rand: 1.0,
            std_dev: 30.0,
            exp_mult: 2.0
        }
    }
}

fn loop_until_u8<F: Fn(&Narg)->f32>(nar: &Narg, f: F) -> u8 {
    loop {
        match f(&nar).round() as i32 {
            n @ 0..=255 => break n as u8,
            _ => ()
        }
    }
}

fn normal1(nar: &Narg) -> f32 {
    Normal::new(
        nar.pixel as f32 + nar.exp_mult * nar.exp_rand,
        nar.std_dev
    )
    .unwrap()
    .sample(&mut rand::thread_rng())
}

fn normal2 (nar: &Narg) -> f32 {
    Normal::new(
        nar.pixel as f32,
        nar.std_dev
    )
    .unwrap()
    .sample(&mut rand::thread_rng())
}

pub fn noise(file: &PathBuf) -> Result<(),()> {
    let mut img = match image::open(file){
        Ok(img) => img.into_rgb8(),
        Err(_e) => return Err(())
    };
    let mut rand_thr = rand::thread_rng();
    let mut exp1 = Exp::new(1.0).unwrap().sample_iter(&mut rand_thr);
    
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let mut nar = Narg {
            exp_rand: exp1.next().unwrap(),
            ..Narg::default()};
        let ch = pixel.channels();
        nar.pixel = ch[0];
        let r: u8 = loop_until_u8(&nar, normal1);
        nar.pixel = ch[1];
        let g = loop_until_u8(&nar, normal2);
        nar.pixel = ch[2];
        nar.exp_rand = exp1.next().unwrap();
        let b = loop_until_u8(&nar, normal1);
        *pixel = image::Rgb([r, g, b]);
    }
    match img.save_with_format("test.png", image::ImageFormat::Png) {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}


/* 
from sys import builtin_module_names
from PIL import Image
import random

XX = 1920
YY = 1080

im = Image.open('yagoda.jpg','r')

print(im.mode)

XX, YY = im.width, im.height

for x in range(XX):
    for y in range(YY):
        orig_red, orig_green, orig_blue = im.getpixel((x,y))
        while True:
            red = int(round(random.gauss(orig_red+2*random.expovariate(1), 30.0)))
            if red in range(0,255):
                break
        while True:
            green = int(round(random.gauss(orig_green, 30.0)))
            if green in range(0,255):
                break
        # green = orig_green
        while True:
            blue = int(round(random.gauss(orig_blue+2*random.expovariate(1), 30.0)))
            if blue in range(0,255):
                break
        tup = (red, green, blue)
        im.putpixel((x,y), tup)

im.save('test.png','PNG')
im.show()

*/

