use std::path::PathBuf;

use image;
use image::Pixel;

use rand_distr::{Exp, Normal, Distribution};
use rand;

pub fn noise(file: &PathBuf) -> Result<(),()> {
    let mut img = match image::open(file){
        Ok(img) => img.into_rgb8(),
        Err(_e) => return Err(())
    };
    let mut rand_thr = rand::thread_rng();
    let mut exp1 = Exp::new(1.0).unwrap().sample_iter(&mut rand_thr);
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let ch = pixel.channels();
        let r = loop {
            match Normal::new(
                ch[0] as f32 + 2.0*exp1.next().unwrap(),
                30.0)
            .unwrap()
            .sample(&mut rand::thread_rng())
            .round() as i32 {
                n if (0..255).contains(&n) => break n as u8,
                _ => ()
            }
        };
        let g = loop {
            match Normal::new( ch[1] as f32, 30.0 )
            .unwrap()
            .sample(&mut rand::thread_rng())
            .round() as i32 {
                n if (0..255).contains(&n) => break n as u8,
                _ => ()
            }
        };
        let b = loop {
            match Normal::new(
                ch[2] as f32 + 2.0*exp1.next().unwrap(),
                30.0)
            .unwrap()
            .sample(&mut rand::thread_rng())
            .round() as i32 {
                n if (0..255).contains(&n) => break n as u8,
                _ => ()
            }
        };
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

