extern crate cgmath;

extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

extern crate bt3;
extern crate bt3_render_gl;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;

use cgmath::FixedArray;
use cgmath::{Matrix4, Point3, Vector3};
use cgmath::{Transform, AffineMatrix3};

use gfx::extra::stream::Stream;

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
    let (mut stream, mut device, mut factory) = gfx_window_glutin::init(
        glutin::Window::new().unwrap());
    let mut factory = Rc::new(RefCell::new(factory));

    stream.out.window.set_title("Terrain example");

    // Create terrain
    let terrain = Rc::new(imagefile::load_terrain_from_image(Path::new("test.png")).unwrap());

    // Setup renderer
    let mut renderer = GLRenderer::new(&terrain, &mut factory.clone());
    load_terrain_into_renderer(&mut renderer, &terrain).unwrap();

    // Create MVP matrix
    let model = Matrix4::identity();

    let view: AffineMatrix3<f32> = Transform::look_at(
        &Point3::new(-100.0, -100.0, 80.0),
        &Point3::new(0.0, 0.0, 40.0),
        &Vector3::unit_z(),
    );

    let proj = cgmath::perspective(
        cgmath::deg(60.0f32),
        stream.get_aspect_ratio(),
        0.1, 1000.0
    );

    let mvp = proj * view.mat * model;

    // Main loop
    'main: loop {
        // quit when Esc is pressed.
        for event in stream.out.window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) => break 'main,
                glutin::Event::Closed => break 'main,
                _ => {},
            }
        }

        stream.clear(gfx::ClearData {
            color: [0.3, 0.3, 0.3, 1.0],
            depth: 1.0,
            stencil: 0,
        });

        for region in terrain.regions.iter() {
            renderer.draw_region(&region, &mut stream, &mvp.into_fixed()).unwrap();
        }

        stream.present(&mut device);
    }
}
