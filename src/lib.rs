mod algorithms;

pub use algorithms::bayer;
pub use algorithms::threshold;


#[derive(Debug)]
pub struct GrayImageBuffer {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32
}