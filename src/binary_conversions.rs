
// This module converts the text file into a binary vector
// And Binary data back to text file

// Import fs for file operations
use std::fs;

// Function 1.
// Convert Text file to binary vector
// Input  : A string that represents the path of file.
// Output : A boolean vector, in which 0 is false and 1 is true


pub fn text_file_to_binary(file_path : String) -> Vec<bool>{

    let mut data : Vec<bool> = Vec::new();

    let text_data_result = fs::read_to_string(file_path);

    // Now, this will return Result<String> object.
    // It is Result::Ok( object:T ) if it is alright and data is fetched
    // It is Result::Err( E ) if file could not be read for some reason

    let text_data = match text_data_result {
        Ok(text) => text,
        Err(_error) => {
            println!("Sorry, could not open the file. \nCreating empty data");
            return data;
        }
    };

    for letter in text_data.chars() {
        let bit_string = format!("{:08b}", letter as u8);

        for bit in bit_string.chars() {
            match bit {
                '0' => data.push(false),
                '1' => data.push(true),
                _ => {}
            }
        }
    }

    return data;
}

// Function 2.
// Convert binary data to the text.
// Input  : A boolean vector that represents the binary data.
// Output : A String, representing the original data as the string.

pub fn binary_to_text(data : &Vec<bool>) -> String{

    let mut data_string = String::new();

    for i in (0..data.len()).step_by(8) {
        let mut bit_string = String::new();

        for bit in 0..8 {
            match data[i+bit] {
                false => bit_string.push('0'),
                true => bit_string.push('1')
            }
        }

        // Convert bit_string to char and push it to data
        data_string.push(u8::from_str_radix(&*bit_string, 2).unwrap() as char);
    }

    return data_string;
}
