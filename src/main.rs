extern crate piston_window;

extern crate bt3;
extern crate bt3_render_gl;

use std::rc::Rc;
use std::path::Path;

use piston_window::*;

use bt3::imagefile;
use bt3::terrain::Terrain;
use bt3::render::Renderer;
use bt3_render_gl::GLRenderer;


fn load_terrain_into_renderer<R: Renderer>(renderer: &mut R, terrain: &Terrain) -> Result<(), String> {
    for region in terrain.regions.iter() {
        try!(renderer.load_region(&region));
    }

    Ok(())
}


fn main() {
    // Create window
    let window: PistonWindow =
        WindowSettings::new("BT3 example", [640, 480])
        .exit_on_esc(true)
        .samples(4)
        .into();

    // Create terrain
    let terrain = Rc::new(imagefile::load_terrain_from_image(Path::new("test.png")).unwrap());

    // Setup renderer
    let mut renderer = GLRenderer::new(&terrain, &mut window.factory.clone());
    load_terrain_into_renderer(&mut renderer, &terrain).unwrap();

    // Main loop
    for e in window {
        e.draw_3d(|stream| {
            for region in terrain.regions.iter() {
                renderer.draw_region(&region, stream).unwrap();
            }
        });
    }
}
