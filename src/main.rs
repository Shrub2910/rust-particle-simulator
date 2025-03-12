use particle::Particle;
use physics::{apply_constraint, apply_gravity, update_positions};
use draw::{draw_constraint, draw_particles};
use raylib::prelude::*;
mod particle;
mod physics;
mod draw;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Particle simulator")
        .build();

    rl.set_target_fps(60);

    let mut particles: Vec<Particle> = Vec::new();

    while !rl.window_should_close() {
        let delta_time: f32 = rl.get_frame_time();

        if rl.is_key_pressed(KeyboardKey::KEY_W) {
            let new_particle: Particle = Particle::new(Vector2 { x: 200.0, y: 200.0 }, 10.0, Color::RED, 1.0);
            particles.push(new_particle);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        apply_gravity(&mut particles);
        apply_constraint(&mut particles, Vector2 { x: 400.0, y: 300.0 }, 300.0);
        update_positions(&mut particles, delta_time);

        draw_particles(&mut particles, &mut d);
        draw_constraint(Vector2 { x: 400.0, y: 300.0 }, 300.0, Color::RED, &mut d);
        
    } 
}
