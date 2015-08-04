#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate genmesh;
extern crate cgmath;

extern crate bt3;

use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;

use gfx::device::{Factory, BufferRole};
use gfx::extra::factory::FactoryExt;
use gfx::extra::stream::{Stream, OwnedStream};
use gfx::device::handle::{Texture, Program};
use gfx::device::tex::{ImageInfo, Format};
use gfx::traits::{ToIndexSlice, ToSlice};
use gfx::render::mesh::{Mesh, Slice};

use genmesh::{Vertices, Triangulate};
use genmesh::generators::{Plane, SharedVertex, IndexedPolygon};

use cgmath::FixedArray;
use cgmath::{Matrix4, Point3, Vector3};
use cgmath::{Transform, AffineMatrix3};

use bt3::region::Region;
use bt3::terrain::Terrain;
use bt3::render::base::Renderer;


gfx_vertex!( Vertex {
    a_Pos@ pos: [f32; 3],
});


gfx_parameters!( Params {
    u_Model@ model: [[f32; 4]; 4],
    u_View@ view: [[f32; 4]; 4],
    u_Proj@ proj: [[f32; 4]; 4],
    u_Offset@ offset: [f32; 2],
    t_Heightmap@ heightmap: gfx::shade::TextureParam<R>,
});


fn gen_16x16_plane_mesh(factory: &Rc<RefCell<gfx_device_gl::Factory>>) -> (Mesh<gfx_device_gl::Resources>, Slice<gfx_device_gl::Resources>) {
    let plane = Plane::subdivide(16, 16);

    let vertex_data: Vec<Vertex> = plane.shared_vertex_iter()
        .map(|(x, y)| {
            Vertex {
                pos: [x, y, 0f32],
            }
        })
        .collect();

    let index_data: Vec<u16> = plane.indexed_polygon_iter()
        .triangulate()
        .vertices()
        .map(|i| i as u16)
        .collect();

    let mesh = factory.borrow_mut().create_mesh(&vertex_data);
    let slice = factory.borrow_mut().create_buffer_static(&index_data, BufferRole::Index).to_slice(gfx::PrimitiveType::TriangleList);

    (mesh, slice)
}


fn gen_program(factory: &Rc<RefCell<gfx_device_gl::Factory>>) -> Program<gfx_device_gl::Resources> {
    let vs = gfx::ShaderSource {
        glsl_150: Some(include_bytes!("shaders/terrain_150.glslv")),
        .. gfx::ShaderSource::empty()
    };
    let fs = gfx::ShaderSource {
        glsl_150: Some(include_bytes!("shaders/terrain_150.glslf")),
        .. gfx::ShaderSource::empty()
    };
    factory.borrow_mut().link_program_source(vs, fs).unwrap()
}


pub struct GFXRenderer {
    terrain: Rc<Terrain>,
    factory: Rc<RefCell<gfx_device_gl::Factory>>,

    plane_mesh: Mesh<gfx_device_gl::Resources>,
    plane_slice: Slice<gfx_device_gl::Resources>,

    program: Program<gfx_device_gl::Resources>,

    pub heightmap: Texture<gfx_device_gl::Resources>,
}


impl GFXRenderer {
    pub fn new(terrain: &Rc<Terrain>, factory: &Rc<RefCell<gfx_device_gl::Factory>>) -> GFXRenderer {
        let (gridSizeX, gridSizeY) = terrain.get_grid_size();

        // Generate a 16x16 tile mesh
        let (plane_mesh, plane_slice) = gen_16x16_plane_mesh(&factory);

        // Compile shader program
        let program = gen_program(&factory);

        GFXRenderer{
            terrain: terrain.clone(),
            factory: factory.clone(),
            plane_mesh: plane_mesh,
            plane_slice: plane_slice,
            program: program,
            heightmap: factory.borrow_mut().create_texture_rgba8(gridSizeX as u16 * 256, gridSizeY as u16 * 256).unwrap(),
        }
    }
}


impl Renderer for GFXRenderer {
    type Stream = OwnedStream<gfx_device_gl::Device, gfx_device_gl::Output>;

    fn load_region(&mut self, region: &Region) -> Result<(), String> {
        // Get the slot
        let (slotX, slotY) = match self.terrain.get_region_grid_slot(region) {
            Some(slot) => slot,
            None => return Err("Unable to load region: region doesn't have a slot".to_string()),
        };

        // Make ImageInfo object describing slot location within texture
        let mut imginfo = ImageInfo::from(*self.heightmap.get_info());
        imginfo.xoffset = slotX as u16 * 256;
        imginfo.yoffset = slotY as u16 * 256;
        imginfo.width = 256;
        imginfo.height = 256;
        imginfo.format = Format::DEPTH16;

        // Copy data into texture
        let mut factory = self.factory.borrow_mut();
        factory.update_texture(&self.heightmap, &imginfo, &region.heights, None).unwrap();

        Ok(())
    }

    fn unload_region(&mut self, region: &Region) -> Result<(), String> {
        // Get the slot
        let (slotX, slotY) = match self.terrain.get_region_grid_slot(region) {
            Some(slot) => slot,
            None => return Err("Unable to unload region: region doesn't have a slot".to_string()),
        };

        // Nothing to do here
        Ok(())
    }

    fn draw_region(&mut self, region: &Region, stream: &mut Self::Stream) -> Result<(), String> {
        // Get the slot
        let (slotX, slotY) = match self.terrain.get_region_grid_slot(region) {
            Some(slot) => slot,
            None => return Err("Unable to draw region: region doesn't have a slot".to_string()),
        };

        let data = Params {
            model: Matrix4::identity().into_fixed(),
            view: Matrix4::identity().into_fixed(),
            proj: cgmath::perspective(cgmath::deg(60.0f32),
                                      stream.get_aspect_ratio(),
                                      0.1, 1000.0
                                      ).into_fixed(),
            offset: [256.0 * slotX as f32, 256.0 * slotX as f32],
            heightmap: (self.heightmap.clone(), None),
            _r: PhantomData,
        };

        let mut batch = gfx::batch::Full::new(self.plane_mesh.clone(), self.program.clone(), data).unwrap();

        batch.slice = self.plane_slice.clone();
        batch.state = gfx::DrawState::new().depth(gfx::state::Comparison::LessEqual, true);

        let view: AffineMatrix3<f32> = Transform::look_at(
            &Point3::new(-100.0, -100.0, 80.0),
            &Point3::new(0.0, 0.0, 40.0),
            &Vector3::unit_z(),
        );
        batch.params.view = view.mat.into_fixed();

        for x in 0..16 {
            for y in 0..16 {
                batch.params.offset = [
                    256.0 * slotX as f32 + 16.0 * x as f32,
                    256.0 * slotY as f32 + 16.0 * y as f32
                ];

                stream.draw(&batch).unwrap();
            }
        }

        Ok(())
    }
}
