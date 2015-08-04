use region::Region;


pub trait Renderer {
    fn load_region(&mut self, region: &Region) -> Result<(), String>;
    fn unload_region(&mut self, region: &Region) -> Result<(), String>;
}
