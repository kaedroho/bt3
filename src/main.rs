extern crate piston_window;
extern crate image;
#[macro_use]
extern crate gfx;

mod bt3;
mod imagefile;

use std::rc::Rc;
use std::path::Path;

use piston_window::{WindowSettings, PistonWindow, AdvancedWindow,clear};
use piston_window::image as draw_image;
use gfx::traits::{Factory, Stream, FactoryExt};

use bt3::terrain::Terrain;
use bt3::render::base::Renderer;
use bt3::render::gfx::GFXRenderer;


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


fn load_terrain_into_renderer(renderer: &mut Renderer, terrain: &Terrain) {
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
    let terrain = Rc::new(match imagefile::load_terrain_from_image(Path::new("test.png")) {
        Ok(terrain) => {
            Some(terrain)
        },
        Err(err) => {
            None
        }
    }.unwrap());

    // Setup renderer
    let mut renderer = GFXRenderer::new(&terrain, &mut window.factory.clone());
    load_terrain_into_renderer(&mut renderer, &terrain);



    let ref mut factory = window.factory.borrow_mut().clone();

    // fullscreen quad
    let vertex_data = [
        Vertex::new([-1.0, -1.0], [0.0, 1.0]),
        Vertex::new([ 1.0, -1.0], [1.0, 1.0]),
        Vertex::new([ 1.0,  1.0], [1.0, 0.0]),

        Vertex::new([-1.0, -1.0], [0.0, 1.0]),
        Vertex::new([ 1.0,  1.0], [1.0, 0.0]),
        Vertex::new([-1.0,  1.0], [0.0, 0.0]),
    ];
    let mesh = factory.create_mesh(&vertex_data);

    let program = {
        let vs = gfx::ShaderSource {
            glsl_120: Some(include_bytes!("shader/simple_120.glslv")),
            glsl_150: Some(include_bytes!("shader/simple_150.glslv")),
            .. gfx::ShaderSource::empty()
        };
        let fs = gfx::ShaderSource {
            glsl_120: Some(include_bytes!("shader/simple_120.glslf")),
            glsl_150: Some(include_bytes!("shader/simple_150.glslf")),
            .. gfx::ShaderSource::empty()
        };
        factory.link_program_source(vs, fs).unwrap()
    };


    let uniforms = Params{
        heightmap: (renderer.heightmap, None),
        _r: std::marker::PhantomData,
    };
    let mut batch = gfx::batch::Full::new(mesh, program, uniforms).unwrap();

    // Main loop
    for e in window {
        e.draw_3d(|stream| {
            stream.draw(&batch).unwrap();
        });
    }
}
