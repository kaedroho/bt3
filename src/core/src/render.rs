use region::Region;


pub trait Renderer {
    type Stream;

    fn load_region(&mut self, region: &Region) -> Result<(), String>;
    fn unload_region(&mut self, region: &Region) -> Result<(), String>;
    fn draw_region(&mut self, region: &Region, stream: &mut Self::Stream) -> Result<(), String>;
}
