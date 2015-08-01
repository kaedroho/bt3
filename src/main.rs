extern crate image;

mod bt3;
mod imagefile;

use std::path::Path;

use bt3::terrain::Terrain;
use bt3::render::base::Renderer;


fn load_terrain_into_renderer(renderer: &mut Renderer, terrain: &Terrain) {
    for region in terrain.regions.iter() {
        renderer.load_region(&region);
    }
}


fn main() {
    // Create terrain
    let terrain = match imagefile::load_terrain_from_image(Path::new("test.png")) {
        Ok(terrain) => {
            Some(terrain)
        },
        Err(err) => {
            None
        }
    }.unwrap();

    // Setup renderer
    //let mut renderer = Renderer::new();
    //load_terrain_into_renderer(&mut renderer, &terrain);
}
