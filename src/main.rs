use std::{fs::File, io::{Read, Write}, env};

// header and dib size for 1 type of bmp format. doesnt work on any others.
const HEADER_SIZE: u8 = 14;
const DIB_SIZE: u8 = 40;

// bytes that dont have anything to do with the pixels or color palette
const OVERHEAD: u8 = HEADER_SIZE + DIB_SIZE;

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    if !file_path[file_path.len()-3..].eq("bmp") {
        panic!("File is not a bmp");
    }

    let mut file = File::open(file_path).unwrap();



    let mut data: Vec<u8> = Vec::new();

    file.read_to_end(&mut data).unwrap();

    // finds how many colors are in the file.
    let num_colors: u32 = (data[0x2e] as u32) | ((data[0x2f] as u32) << 8) | ((data[0x30] as u32) << 16) | ((data[0x31] as u32) << 24);

    let start = data[0x0A];

    // when a bmp is low in color count, edit palette data instead of pixel data
    if num_colors > 0 {
        
        for cols in 0..num_colors {
            let col_index: usize = OVERHEAD as usize + (4 * cols as usize);
            
            let b = data[col_index + 0];
            let g = data[col_index + 1];
            let r = data[col_index + 2];

            let grey = greyscale(b, g, r);

            data[col_index + 0] = grey;
            data[col_index + 1] = grey;
            data[col_index + 2] = grey;
        }

    } else {

        // gets width and height of image
        let width: u32 = data[0x12] as u32 | ((data[0x13] as u32) << 8) | ((data[0x14] as u32) << 16) | ((data[0x15] as u32) << 24);
        let height: u32 = data[0x16] as u32 | ((data[0x17] as u32) << 8) | ((data[0x18] as u32) << 16) | ((data[0x19] as u32) << 24);

        // gets the width of the images red greem and blue data per row
        let bmp_width = width * 3;

        // for some reason, rows of pixels must be a muliple of 4, bmp files will append bytes until the number of bytes per row is divisable by 4
        let mut padding: u32 = 0;
        while (bmp_width + padding) % 4 != 0 {
            padding += 1;
        }
        let row_size = padding + bmp_width;

        // iterates over every row of pixel data in the image
        for row in 0..height {
            // will find the greyscale value of every pixel in a row (minus the padding added to make it a multiple of 4)
            for pix in (start as u32 + ((row_size * row))..(start as u32 + (row_size * row) + bmp_width)).step_by(3) {

                // grabs BGR values of pixel
                let b = data[pix as usize+ 0];
                let g = data[pix as usize+ 1];
                let r = data[pix as usize+ 2];

                // gets greyscale value
                let grey = greyscale(b, g, r);

                // replaces BGR values with their greyscale
                data[pix as usize + 0] = grey;
                data[pix as usize + 1] = grey;
                data[pix as usize + 2] = grey;
                
            }
        }

    }

    // creates a new file at the projects root directory
    let mut new_file = File::create("greyscale_bmp.bmp").unwrap();

    // writes the greyscaled data into the new image
    new_file.write(&data).expect("failed to write into new file");
}

// formula to convert RGB values to greyscale
fn greyscale(b: u8, g: u8, r: u8) -> u8 {
    ((0.299 * r as f32) + (0.587 * g as f32) + (0.114 * b as f32)) as u8
}
