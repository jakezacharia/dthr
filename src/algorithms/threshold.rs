pub fn threshold(buffer: &mut [u8], height: u32, width: u32, threshold: u8) {
    // iterate through each value in the vector
    for y in 0..height {
        for x in 0..width {
            let i = (y * width + x) as usize;
            buffer[i] = if buffer[i] > threshold { 255 } else { 0 };
        }
    }
}