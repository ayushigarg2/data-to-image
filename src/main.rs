extern crate image;
use image::{RgbImage, Rgb};

extern crate rand;
use rand::Rng;


fn main() {

    let  rows = 1080;
    let  columns = 1920;

    let mut img = RgbImage::new(columns, rows);

    let mut bitstream:Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();

    let capacity = rows*columns*3;


    for i in 0..capacity as usize {
        bitstream.push(rng.gen::<u8>());
        println!("{}", bitstream[i]);
    }


    let _ = img.save("hello.png");

}
