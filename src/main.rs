use std::{env};
use std::path::Path;

use dthr::GrayImageBuffer;
use dthr::bayer::dither_bayer;

use image::{DynamicImage, GrayImage, ImageBuffer, ImageFormat, ImageReader, Luma, buffer, load};


fn load_image_to_buffer<P: AsRef<Path>>(path: P) -> Result<GrayImageBuffer, Box<dyn std::error::Error>> {
    
    let img: DynamicImage = ImageReader::open(path)?.decode()?;
    
    let gray: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();
    gray.save("grayscale.png")?;

    let height: u32 = gray.height();
    let width: u32 = gray.width();
    let buffer: Vec<u8> = gray.into_raw();

    Ok(GrayImageBuffer { data: buffer, width, height })
}

fn main() -> Result<(), Box<dyn std::error::Error>>{

    // $ ./dithr-cli 8 my_image.png 
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide path - usage: {} <image_path>", args[0]);
        std::process::exit(1);
    }

    let image_path: &String = &args[1];

    let mut gray_image_buffer: GrayImageBuffer = load_image_to_buffer(image_path)?;

    // can check buffer values (each pixel) BEFORE calling dithering here...
    // TODO - this is where user selection implementation will occur, for now we are just calling a fixed function
    dither_bayer(&mut gray_image_buffer.data, gray_image_buffer.height, gray_image_buffer.width, 2)?;

    let img = GrayImage::from_raw(
        gray_image_buffer.width as u32,
        gray_image_buffer.height as u32,
        gray_image_buffer.data,
    ).ok_or("Failed to make the image!")?;

    img.save("output.png")?;


    // checking AFTER - use this for debugging! 
    // println!("{gray_image_buffer:?}");

    // call bayer dithering

    Ok(())
}
    