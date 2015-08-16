use cgmath::{Point, Point3, Vector3};

use region::Region;


struct LODLevelInfo {
    pub chunk_size: u32,
    pub chunk_radius: f64,
}


const LOD_LEVEL_INFO: [LODLevelInfo; 5] = [
    LODLevelInfo{
        chunk_size: 16,
        chunk_radius: 22.627416998, // sqrt((16 * 16) + (16 * 16)),
    },
    LODLevelInfo{
        chunk_size: 32,
        chunk_radius: 45.2548339959, // sqrt((32 * 32) + (32 * 32))
    },
    LODLevelInfo{
        chunk_size: 64,
        chunk_radius: 90.5096679919, // sqrt((64 * 64) + (64 * 64))
    },
    LODLevelInfo{
        chunk_size: 128,
        chunk_radius: 181.019335984, // sqrt((128 * 128) + (128 * 128))
    },
    LODLevelInfo{
        chunk_size: 256,
        chunk_radius: 362.038671968, // sqrt((256 * 256) + (256 * 256))
    },
];


pub struct DrawCommand {
    pub chunk_level: u8,
    pub chunk_x: u8,
    pub chunk_y: u8,
}


pub struct Plan {
    pub draw_commands: Vec<DrawCommand>,
}


impl Plan {
    pub fn new() -> Plan {
        Plan {
            draw_commands: Vec::with_capacity(256)
        }
    }
}


pub struct Planner {
    pub eye_position: Point3<f64>,
    pub lod_distances: [f64; 4],
}


impl Planner {
    pub fn new() -> Planner {
        Planner {
            eye_position: Point3::new(0.0, 0.0, 0.0),
            lod_distances: [0.0; 4],
        }
    }

    fn plan_chunk(&self, plan: &mut Plan, region: &Region, eye_position: Vector3<f64>, chunk_level: u8, chunk_x: u8, chunk_y: u8) {
        // Draw chunk if it is bottom level
        if chunk_level == 0 {
            plan.draw_commands.push(DrawCommand{
                chunk_level: chunk_level,
                chunk_x: chunk_x,
                chunk_y: chunk_y,
            });

            return;
        }

        // If the centre point of this chunk is not within the bounds of the lower lod level, draw this chunk
        let chunk_size = LOD_LEVEL_INFO[chunk_level as usize].chunk_size as f64;
        let chunk_centre_x = chunk_x as f64 * chunk_size + chunk_size / 2.0;
        let chunk_centre_y = chunk_y as f64 * chunk_size + chunk_size / 2.0;

        let dist_x = eye_position.x - chunk_centre_x;
        let dist_y = eye_position.y - chunk_centre_y;

        let dist_sq = dist_x * dist_x + dist_y * dist_y;

        let lower_lod_chunk_radius = LOD_LEVEL_INFO[chunk_level as usize - 1].chunk_radius;
        let lower_lod_dist = self.lod_distances[chunk_level as usize - 1] + lower_lod_chunk_radius;

        if dist_sq > lower_lod_dist * lower_lod_dist {
            plan.draw_commands.push(DrawCommand{
                chunk_level: chunk_level,
                chunk_x: chunk_x,
                chunk_y: chunk_y,
            });

            return;
        }

        // Traverse into lower level
        self.plan_chunk(plan, &region, eye_position, chunk_level - 1, chunk_x * 2, chunk_y * 2);
        self.plan_chunk(plan, &region, eye_position, chunk_level - 1, chunk_x * 2 + 1, chunk_y * 2);
        self.plan_chunk(plan, &region, eye_position, chunk_level - 1, chunk_x * 2, chunk_y * 2 + 1);
        self.plan_chunk(plan, &region, eye_position, chunk_level - 1, chunk_x * 2 + 1, chunk_y * 2 + 1);
    }

    pub fn plan_region(&self, region: &Region) -> Plan {
        let mut plan = Plan::new();
        let eye_position = self.eye_position.sub_p(&Point3::new(
            region.x as f64 * 256.0,
            region.y as f64 * 256.0,
            0.0
        ));

        // Start planning from the top chunk
        self.plan_chunk(&mut plan, &region, eye_position, 4, 0, 0);

        plan
    }
}
