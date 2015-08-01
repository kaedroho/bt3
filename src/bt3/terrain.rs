use bt3::region::Region;


pub struct Terrain {
    regionsX: u32,
    regionsY: u32,
    pub regions: Vec<Region>
}


impl Terrain {
    pub fn new(regionsX: u32, regionsY: u32) -> Terrain {
        let mut terrain = Terrain{
            regionsX: regionsX,
            regionsY: regionsY,
            regions: Vec::with_capacity((regionsX * regionsY) as usize)
        };

        for x in 0..regionsX {
            for y in 0..regionsY {
                terrain.regions.push(Region::new(x, y));
            }
        }

        terrain
    }

    pub fn get_grid_size(&self) -> (u32, u32) {
        (self.regionsX, self.regionsY)
    }

    pub fn get_region_grid_slot(&self, region: &Region) -> Option<(u32, u32)> {
        if region.x < self.regionsX && region.y < self.regionsY {
            Some((region.x, region.y))
        } else {
            None
        }
    }
}
