extern crate gfx;
extern crate gfx_device_gl;
extern crate genmesh;

use std::rc::Rc;
use std::cell::RefCell;

use self::gfx::device::{Factory, BufferRole};
use self::gfx::extra::factory::FactoryExt;
use self::gfx::device::handle::Texture;
use self::gfx::device::tex::{ImageInfo, Format};
use self::gfx::traits::{ToIndexSlice, ToSlice};
use self::gfx::render::mesh::{Mesh, Slice};

use self::genmesh::{Vertices, Triangulate};
use self::genmesh::generators::{Plane, SharedVertex, IndexedPolygon};

use bt3::region::Region;
use bt3::terrain::Terrain;
use bt3::render::base::Renderer;


gfx_vertex!( Vertex {
    a_Pos@ pos: [f32; 3],
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
    let slice = factory.borrow_mut().create_buffer_static(&index_data, BufferRole::Index).to_slice(gfx::PrimitiveType::TriangleList);;

    (mesh, slice)
}


pub struct GFXRenderer {
    terrain: Rc<Terrain>,
    factory: Rc<RefCell<gfx_device_gl::Factory>>,

    plane_mesh: Mesh<gfx_device_gl::Resources>,
    plane_slice: Slice<gfx_device_gl::Resources>,

    pub heightmap: Texture<gfx_device_gl::Resources>,
}


impl GFXRenderer {
    pub fn new(terrain: &Rc<Terrain>, factory: &Rc<RefCell<gfx_device_gl::Factory>>) -> GFXRenderer {
        let (gridSizeX, gridSizeY) = terrain.get_grid_size();

        // Generate a 16x16 tile mesh
        let (plane_mesh, plane_slice) = gen_16x16_plane_mesh(&factory);

        GFXRenderer{
            terrain: terrain.clone(),
            factory: factory.clone(),
            plane_mesh: plane_mesh,
            plane_slice: plane_slice,
            heightmap: factory.borrow_mut().create_texture_rgba8(gridSizeX as u16 * 256, gridSizeY as u16 * 256).unwrap(),
        }
    }
}


impl Renderer for GFXRenderer {
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

    fn draw_region(&mut self, region: &Region) -> Result<(), String> {
        // Get the slot
        let (slotX, slotY) = match self.terrain.get_region_grid_slot(region) {
            Some(slot) => slot,
            None => return Err("Unable to draw region: region doesn't have a slot".to_string()),
        };

        // TODO: Draw the region
        Ok(())
    }
}
