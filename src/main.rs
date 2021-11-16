use std::{fs::File, io::{Read, Write}, env};

const HEADER_SIZE: u8 = 14;
const DIB_SIZE: u8 = 40;

const OVERHEAD: u8 = HEADER_SIZE + DIB_SIZE;

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1]).unwrap();
    let mut data: Vec<u8> = Vec::new();

    file.read_to_end(&mut data).unwrap();

    let num_colors: u32 = (data[0x2e] as u32) | ((data[0x2f] as u32) << 8) | ((data[0x30] as u32) << 16) | ((data[0x31] as u32) << 24);

    let start = data[0x0A];

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

        let width: u32 = data[0x12] as u32 | ((data[0x13] as u32) << 8) | ((data[0x14] as u32) << 16) | ((data[0x15] as u32) << 24);
        let height: u32 = data[0x16] as u32 | ((data[0x17] as u32) << 8) | ((data[0x18] as u32) << 16) | ((data[0x19] as u32) << 24);

        let mut padding: u32 = 0;

        while ((width * 3) + padding) % 4 != 0 {
            padding += 1;
        }

        let row_size = padding + (width * 3);

        for row in 0..height {
            for pix in (start as u32 + ((row_size * row))..(start as u32 + (row_size * row) + row_size)).step_by(3) {
                let b = data[pix as usize+ 0];
                let g = data[pix as usize+ 1];
                let r = data[pix as usize+ 2];

                let grey = greyscale(b, g, r);

                data[pix as usize + 0] = grey;
                data[pix as usize + 1] = grey;
                data[pix as usize + 2] = grey;
                
            }
        }

    }

    let mut new_file = File::create("greyscale_bmp.bmp").unwrap();

    new_file.write(&data).expect("failed to write into new file");
}

fn greyscale(b: u8, g: u8, r: u8) -> u8 {
    ((0.299 * r as f32) + (0.587 * g as f32) + (0.114 * b as f32)) as u8
}
