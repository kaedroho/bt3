pub struct Region {
    pub x: u32,
    pub y: u32,
    pub heights: [u16; 256 * 256]
}


impl Region {
    pub fn new(x: u32, y: u32) -> Region {
        Region{
            x: x,
            y: y,
            heights: [0; 256 * 256]
        }
    }
}
