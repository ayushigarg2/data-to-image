mod convert_to_image;

// Driver Code

fn main() {
    let  rows = 1080;
    let  columns = 1920;

    // Generate Sample Data
    let mut sample_data = vec![false; 2073600];

    for i in (0..2073600 as usize).step_by(123) {
        sample_data[i] = true
    }

    // Convert to image and save it as a png file
    let _ = convert_to_image::convert_to_image_util(&sample_data, columns, rows).save("sample.png");
}
