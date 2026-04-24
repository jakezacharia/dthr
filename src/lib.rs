mod algorithms;

pub use algorithms::bayer;
pub use algorithms::threshold;


#[derive(Debug)]
pub struct GrayImageBuffer {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize
}