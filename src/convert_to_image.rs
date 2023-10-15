extern crate image;

use std::cmp::min;
use image::{ImageBuffer, Luma, GrayImage, open};

// We will Create an image, 1080x1920
// Each pixel is set to either black or white
// Black Represents 0 and white represents 1

pub fn convert_to_image_util(data:&Vec<bool>, columns:u32, rows:u32) -> ImageBuffer<Luma<u8>, Vec<u8>>{

    // Create an empty image of columns×rows size
    let mut image = GrayImage::new(columns, rows);


    // Pick at max columns * rows length of data
    for i in 0..min(data.len(), (columns as usize) * (rows as usize)) {

        // If bit is 1, set color to white
        if data[i] == true {
            image.get_pixel_mut(i as u32%columns, i as u32/columns).0 = [255];
        }

        // Set it to 0 otherwise
        else {
            image.get_pixel_mut(i as u32%columns, i as u32/columns).0 = [0];
        }
    }

    return image
}


// This Function will convert the png image back to the boolean or binary data
// Input  : String, representing the path of the input image.
// Output : Boolean vector, where 0 is false and 1 is true

pub fn image_to_data(img_path: String ) -> Vec<bool>{

    // Open the image
    let image_result = open(img_path);


    // Exception handling
    let image_input = match image_result {

        // If image is loaded correctly, return it
        Ok(img) => img,

        // If there is an error, print it and return empty vector
        Err(_err) => {
            println!("Sorry, image file not found {}", _err.to_string());
            return Vec::new();
        }
    };

    // Create new Vector
    let mut bit_data = Vec::new();

    // Traverse the image, and if value is greater than 127, add true
    // Else, add false.

    for i in image_input.as_bytes() {
        if *i > 127 { bit_data.push(true); }
        else { bit_data.push(false); }
    }

    // Return the boolean vector
    return bit_data;
}
