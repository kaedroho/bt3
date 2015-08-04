extern crate image;

use std::path::Path;
use std::fs::File;

use self::image::GenericImage;

use terrain::Terrain;


pub fn load_terrain_from_image(path: &Path) -> Result<Terrain, image::ImageError> {
    let mut img = try!(image::open(path));

    let mut terrain = Terrain::new(
        img.width() / 256,
        img.height() / 256
    );

    for region in terrain.regions.iter_mut() {
        let mut sub_img = img.sub_image(region.x * 256, region.y * 256, 256, 256);

        for (x, y, pixel) in sub_img.pixels() {
            let height: u16 = pixel.data[0] as u16 * 256 + pixel.data[1] as u16;
            region.heights[x as usize + y as usize * 256] = height;
        }
    }

    Ok(terrain)
}
