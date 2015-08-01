extern crate image;

mod bt3;
mod imagefile;

use std::path::Path;

use bt3::terrain::Terrain;
use bt3::render::base::Renderer;
use bt3::render::piston::PistonRenderer;


fn load_terrain_into_renderer(renderer: &mut Renderer, terrain: &Terrain) {
    renderer.init_terrain(terrain);

    for region in terrain.regions.iter() {
        renderer.load_region(&terrain, &region);
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
    let mut renderer = PistonRenderer::new();
    load_terrain_into_renderer(&mut renderer, &terrain);

}
