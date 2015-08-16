extern crate cgmath;
extern crate piston_window;
extern crate graphics;
extern crate bt3;

use piston_window::*;

use bt3::terrain::Terrain;


fn draw_plan_debug<G: graphics::Graphics>(x: u32, y: u32, plan: &bt3::planner::Plan, transform: graphics::math::Matrix2d, draw_state: &DrawState, g: &mut G) {
    struct Level {
        color: [f32; 4],
        chunk_size: u32,
    }

    let levels: [Level; 5] = [
        Level{
            color: [1.0, 1.0, 1.0, 1.0],
            chunk_size: 16,
        },
        Level{
            color: [0.8, 0.8, 0.8, 1.0],
            chunk_size: 32,
        },
        Level{
            color: [0.6, 0.6, 0.6, 1.0],
            chunk_size: 64,
        },
        Level{
            color: [0.4, 0.4, 0.4, 1.0],
            chunk_size: 128,
        },
        Level{
            color: [0.2, 0.2, 0.2, 1.0],
            chunk_size: 256,
        },
    ];

    for command in plan.draw_commands.iter() {
        let ref level = levels[command.chunk_level as usize];

        let cx = x + command.chunk_x as u32 * level.chunk_size;
        let cy = y+ command.chunk_y as u32 * level.chunk_size;
        let w = level.chunk_size;
        let h = level.chunk_size;

        // Draw rectangle
        Rectangle::new(level.color).draw([cx as f64, cy as f64, w as f64, h as f64], &draw_state, transform, g);
    }
}


fn main() {
    // Create terrain
    let terrain = Terrain::new(4, 3);

    // Create planner
    let mut planner = bt3::planner::Planner::new();
    planner.lod_distances[0] = 100.0;
    planner.lod_distances[1] = 200.0;
    planner.lod_distances[2] = 400.0;
    planner.lod_distances[3] = 1000.0;

    // Create window
    let window: PistonWindow =
        WindowSettings::new("BT3 planner test", (1024, 768))
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .build()
        .unwrap();

    // Event loop
    for event in window {
        event.draw_2d(|c, g| {
            clear([1.0; 4], g);

            for region in terrain.regions.iter() {
                let plan = planner.plan_region(&region);
                draw_plan_debug(region.x * 256, region.y * 256, &plan, c.transform, &c.draw_state, g);
            }
        });

        if let Some(pos) = event.mouse_cursor_args() {
            planner.eye_position.x = pos[0];
            planner.eye_position.y = pos[1];
        };
    }
}
