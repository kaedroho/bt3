use region::Region;


pub struct Terrain {
    regions_x: u32,
    regions_y: u32,
    pub regions: Vec<Region>
}


impl Terrain {
    pub fn new(regions_x: u32, regions_y: u32) -> Terrain {
        let mut terrain = Terrain{
            regions_x: regions_x,
            regions_y: regions_y,
            regions: Vec::with_capacity((regions_x * regions_y) as usize)
        };

        for x in 0..regions_x {
            for y in 0..regions_y {
                terrain.regions.push(Region::new(x, y));
            }
        }

        terrain
    }

    pub fn get_grid_size(&self) -> (u32, u32) {
        (self.regions_x, self.regions_y)
    }

    pub fn get_region_grid_slot(&self, region: &Region) -> Option<(u32, u32)> {
        if region.x < self.regions_x && region.y < self.regions_y {
            Some((region.x, region.y))
        } else {
            None
        }
    }
}
