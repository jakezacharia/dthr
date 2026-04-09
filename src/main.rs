use std::{env};
use std::path::Path;
use dthr::bayer::dither_bayer;
use image::imageops::dither;
use image::{DynamicImage, GrayImage, ImageBuffer, ImageFormat, ImageReader, Luma, buffer};


fn load_image_to_buffer<P: AsRef<Path>>(path: P) -> Result<(Vec<u8>, usize, usize), Box<dyn std::error::Error>> {
    
    let img = ImageReader::open(path)?.decode()?;
    
    let gray: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();
    gray.save("grayscale.png")?;

    let height: usize = gray.height() as usize;
    let width: usize = gray.width() as usize;
    let buffer: Vec<u8> = gray.into_raw();

    Ok((buffer, width, height))
}


fn main() -> Result<(), Box<dyn std::error::Error>>{

    // $ ./dithr-cli 8 my_image.png 
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: {} <image_path>", args[0]);
        std::process::exit(1);
    }
    let image_path: &String = &args[1];
    let (buffer, width, height) = load_image_to_buffer(image_path)?;

    println!("{buffer:?}");

    println!("loaded Image: {}x{}", width, height);

    Ok(())
}
    
