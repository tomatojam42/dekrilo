use std::path::PathBuf;

use image;
use image::Pixel;

use rand_distr::{Exp, Normal, Distribution};
use rand;

use rayon::prelude::*;

use crate::DError;

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
fn round_f32_u8(pixel: f32) -> u8 {
    match pixel.round() as i32 {
        n @ 0..=255 => n as u8,
        256.. => 255,
        ..0 => 0,
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

pub fn noise(file: &PathBuf) -> Result<(),DError> {
    let mut img = match image::open(file){
        Ok(img) => img.into_rgb8(),
        Err(_e) => return Err(DError::CantReadFile)
    };
    
    img.par_enumerate_pixels_mut().map(|(_x, _y, pixel)|{
        let mut rand_thr = rand::thread_rng();
        let mut exp1 = Exp::new(1.0).unwrap().sample_iter(&mut rand_thr);
        let mut nar = Narg {
            exp_rand: exp1.next().unwrap(),
            ..Narg::default()};
        let ch = pixel.channels();
        nar.pixel = ch[0];
        let r: u8 = round_f32_u8(normal1(&nar));
        nar.pixel = ch[1];
        let g = round_f32_u8(normal2(&nar));
        nar.pixel = ch[2];
        nar.exp_rand = exp1.next().unwrap();
        let b = round_f32_u8(normal1(&nar));
        *pixel = image::Rgb([r, g, b]);
    }).count();

    match img.save_with_format("test.png", image::ImageFormat::Png) {
        Ok(_) => Ok(()),
        Err(_) => Err(DError::CantWriteFile)
    }
}
