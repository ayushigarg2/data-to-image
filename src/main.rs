extern crate image;
use image::{RgbImage, Rgb, ImageBuffer, Luma};

extern crate rand;
use rand::Rng;

// We will Create an image, 1080x1920
// Each pixel is set to either black or white
// Black Represents 0 and white represents 1


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
