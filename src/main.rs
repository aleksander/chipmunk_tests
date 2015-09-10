extern crate chipmunk;
use chipmunk::space::Space;
use chipmunk::body::Body;
use chipmunk::shape::Shape;
use chipmunk::util::*;

extern crate piston_window;
use piston_window::*;

fn main() {
    let gravity = (0.0, 1000.0);
    let floor_friction = 1.0;
    let ball_friction = 0.7;
    let ball_radius = 50.0;
    let ball_mass = 1.0;
    let ball_pos = (50.0, 50.0);
    let ball_moment = moment_of_circle(ball_mass, ball_radius, 0.0);
    let floor_start = (10.0, 590.0);
    let floor_end = (600.0, 600.0);
    let floor_radius = 0.0;
    let zero = (0.0, 0.0);
    let time_step = 1.0 / 600.0;

    let mut space = Space::new();
    space.set_gravity(gravity.0, gravity.1);

    let mut floor_body = Body::new_static();
    let mut floor_shape = Shape::new_segment(&mut floor_body, floor_start, floor_end, floor_radius);

    floor_shape.set_friction(floor_friction);
    space.add_body(&mut floor_body);
    space.add_shape(&mut floor_shape);

    let mut ball_body = Body::new(ball_mass, ball_moment);
    let mut ball_shape = Shape::new_circle(&mut ball_body, ball_radius, zero);

    ball_body.set_position(ball_pos.0, ball_pos.1);
    ball_shape.set_friction(ball_friction);

    space.add_body(&mut ball_body);
    space.add_shape(&mut ball_shape);

    let window: PistonWindow = WindowSettings::new("Chipmunk test", (650, 650))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut time = 0.0;
    let mut max_sim_steps_per_update = 0;

    for body in space.bodies() {
        println!("position: {:?}", body.position());
    }

    for e in window {
        e.draw_2d(|_c, g| {
            let (x,y) = ball_body.position();
            let (vx,vy) = ball_body.velocity();
            clear([0.1, 0.1, 0.1, 1.0], g);
            line([1.0, 1.0, 1.0, 1.0], 1.0, [floor_start.0, floor_start.1, floor_end.0, floor_end.1], _c.transform.trans(0.0, 0.0), g);
            ellipse([1.0, 1.0, 1.0, 1.0], [x-50.0, y-50.0, 100.0, 100.0], _c.transform.trans(0.0, 0.0), g);
            line([1.0, 0.1, 0.1, 1.0], 1.0, [x, y, x+vx, y+vy], _c.transform.trans(0.0, 0.0), g);
        });
        
        e.update(|args| {
            time += args.dt;
            let mut steps = 0;
            while time >= time_step {
                time -= time_step;
                steps += 1;
                space.step(time_step);
            }
            if steps > max_sim_steps_per_update {
                max_sim_steps_per_update = steps;
            }
        });
    }
    
    println!("max_sim_steps_per_update: {}", max_sim_steps_per_update);
}

