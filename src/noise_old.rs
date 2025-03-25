use std::path::PathBuf;

use image;
use image::Pixel;

use rand_distr::{Exp, Normal, Distribution};
use rand;
use std::thread;
use std::sync::mpsc;

enum PixelMessage {
    Pixel(u8),
    Stop,
}

pub fn noise(file: &PathBuf) -> Result<(),()> {
    let mut img = match image::open(file){
        Ok(img) => img.into_rgb8(),
        Err(_e) => return Err(())
    };
    
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();
    let (tx1res, rx1res) = mpsc::channel();
    let (tx2res, rx2res) = mpsc::channel();
    let (tx3res, rx3res) = mpsc::channel();
    let handle1 = thread::spawn(move || {
        let mut rand_thr = rand::thread_rng();
        let mut exp1 = Exp::new(1.0).unwrap().sample_iter(&mut rand_thr);
        loop {
            let ch = match rx1.recv().unwrap() {
                PixelMessage::Pixel(p) => p,
                PixelMessage::Stop => break
            };
            let r = loop {
                match Normal::new(
                    ch as f32 + 2.0 * exp1.next().unwrap(),
                    30.0)
                .unwrap()
                .sample(&mut rand::thread_rng())
                .round() as i32 {
                    n @ 0..=255 => break n as u8,
                    _ => ()
                }
            };
            tx1res.send(r).unwrap();
        }
    });
    let handle2 = thread::spawn(move|| {
        let mut rand_thr = rand::thread_rng();
        let mut exp1 = Exp::new(1.0).unwrap().sample_iter(&mut rand_thr);
        loop {
            let ch = match rx2.recv().unwrap() {
                PixelMessage::Pixel(p) => p,
                PixelMessage::Stop => break
            };
            let g = loop {
                match Normal::new( ch as f32, 30.0 )
                .unwrap()
                .sample(&mut rand::thread_rng())
                .round() as i32 {
                    n @ 0..=255 => break n as u8,
                    _ => ()
                }
            };
            tx2res.send(g).unwrap();
        }
    });
    let handle3 = thread::spawn(move|| {
        let mut rand_thr = rand::thread_rng();
        let mut exp1 = Exp::new(1.0).unwrap().sample_iter(&mut rand_thr);
        loop {
            let ch = match rx3.recv().unwrap() {
                PixelMessage::Pixel(p) => p,
                PixelMessage::Stop => break
            };
            let b = loop {
                match Normal::new(
                    ch as f32 + 2.0 * exp1.next().unwrap(),
                    30.0)
                .unwrap()
                .sample(&mut rand::thread_rng())
                .round() as i32 {
                    n @ 0..=255 => break n as u8,
                    _ => ()
                }
            };
            tx3res.send(b).unwrap();
        }
    });
    let mut c = 0;
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        println!("{c}");
        c += 1;
        let ch = pixel.channels();
        tx1.send(PixelMessage::Pixel(ch[0])).unwrap();
        tx2.send(PixelMessage::Pixel(ch[1])).unwrap();
        tx3.send(PixelMessage::Pixel(ch[2])).unwrap();
        let r = rx1res.recv().unwrap();
        let g = rx2res.recv().unwrap();
        let b = rx3res.recv().unwrap();
        *pixel = image::Rgb([r, g, b]);
    }
    tx1.send(PixelMessage::Stop).unwrap();
    tx2.send(PixelMessage::Stop).unwrap();
    tx3.send(PixelMessage::Stop).unwrap();
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

