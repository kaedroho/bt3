extern crate piston_window;
extern crate cgmath;

extern crate gfx;

extern crate bt3;
extern crate bt3_render_gl;

use std::rc::Rc;
use std::path::Path;

use piston_window::*;

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

    // Create MVP matrix
    let model = Matrix4::identity();

    let view: AffineMatrix3<f32> = Transform::look_at(
        &Point3::new(-100.0, -100.0, 80.0),
        &Point3::new(0.0, 0.0, 40.0),
        &Vector3::unit_z(),
    );

    let proj = cgmath::perspective(
        cgmath::deg(60.0f32),
        window.stream.borrow().get_aspect_ratio(),
        0.1, 1000.0
    );

    let mvp = proj * view.mat * model;

    // Main loop
    for e in window {
        e.draw_3d(|stream| {
            for region in terrain.regions.iter() {
                renderer.draw_region(&region, stream, &mvp.into_fixed()).unwrap();
            }
        });
    }
}
