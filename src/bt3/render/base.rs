use bt3::region::Region;
use bt3::terrain::Terrain;


pub trait Renderer {
    fn load_region(&mut self, region: &Region) -> Result<(), String>;
    fn unload_region(&mut self, region: &Region) -> Result<(), String>;
    fn draw_region(&mut self, region: &Region) -> Result<(), String>;
}
