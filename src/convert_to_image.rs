extern crate image;

use image::{ImageBuffer, Luma, GrayImage};

// We will Create an image, 1080x1920
// Each pixel is set to either black or white
// Black Represents 0 and white represents 1


pub fn convert_to_image_util(data:&Vec<bool>, columns:u32, rows:u32) -> ImageBuffer<Luma<u8>, Vec<u8>>{

    // Create an empty image of columns×rows size

    let mut image = GrayImage::new(columns, rows);

    for i in 0..data.len() {

        // If bit is 1, set color to white
        if data[i] == true {
            image.get_pixel_mut(i as u32/columns, i as u32%rows).0 = [255];
        }

        // Set it to 0 otherwise
        else {
            image.get_pixel_mut(i as u32/columns, i as u32%rows).0 = [0];
        }
    }

    return image
}