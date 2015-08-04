extern crate piston_window;
extern crate image;
#[macro_use]
extern crate gfx;

extern crate bt3;
extern crate bt3_render_gl;

use std::rc::Rc;
use std::path::Path;

use piston_window::{WindowSettings, PistonWindow, AdvancedWindow,clear};
use piston_window::image as draw_image;
use gfx::traits::{Factory, Stream, FactoryExt};

use bt3::imagefile;
use bt3::terrain::Terrain;
use bt3::render::Renderer;
use bt3_render_gl::GLRenderer;


gfx_vertex!(Vertex{
    a_Pos@ pos: [f32; 2],
    a_Uv@ uv: [f32; 2],
});


impl Vertex {
    fn new(p: [f32; 2], u: [f32; 2]) -> Vertex {
        Vertex {
            pos: p,
            uv: u,
        }
    }
}


gfx_parameters!(Params{
    t_Heightmap@ heightmap: gfx::shade::TextureParam<R>,
});


fn load_terrain_into_renderer<R: Renderer>(renderer: &mut R, terrain: &Terrain) {
    for region in terrain.regions.iter() {
        renderer.load_region(&region);
    }
}


fn main() {
    // Create window
    let mut window: PistonWindow =
        WindowSettings::new("piston: cube", [640, 480])
        .exit_on_esc(true)
        .samples(4)
        .into();
    window.set_capture_cursor(true);

    // Create terrain
    let terrain = Rc::new(imagefile::load_terrain_from_image(Path::new("test.png")).unwrap());

    // Setup renderer
    let mut renderer = GLRenderer::new(&terrain, &mut window.factory.clone());
    load_terrain_into_renderer(&mut renderer, &terrain);

    // Main loop
    for e in window {
        e.draw_3d(|stream| {
            for region in terrain.regions.iter() {
                renderer.draw_region(&region, stream);
            }
        });
    }
}
