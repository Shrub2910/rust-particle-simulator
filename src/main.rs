use particle::Particle;
use physics::{apply_constraint, apply_gravity, apply_mouse_gravity, solve_collisions, update_positions};
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

    let mut ball_create_mode: bool = true;
    let mut mouse_gravity: bool = false;

    while !rl.window_should_close() {
        let delta_time: f32 = rl.get_frame_time();
        let mouse_position: Vector2 = rl.get_mouse_position();


        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && ball_create_mode {
            let new_particle: Particle = Particle::new(mouse_position, 10.0, Color::RED, 1.0);
            particles.push(new_particle);
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
            mouse_gravity = true;
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT){
            mouse_gravity = false;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            particles.clear();
        }

        if rl.is_key_pressed(KeyboardKey::KEY_LEFT_SHIFT) {
            ball_create_mode = !ball_create_mode;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        for _ in 0..8{
            if !ball_create_mode && mouse_gravity{
                apply_mouse_gravity(&mut particles, mouse_position);
            } else{
                apply_gravity(&mut particles);
            }

            apply_constraint(&mut particles, Vector2 { x: 400.0, y: 300.0 }, 300.0);
            solve_collisions(&mut particles);
            update_positions(&mut particles, delta_time / 8.0);
        }

        let text: String = format!("Number of particles: {}", particles.len());

        d.draw_text(&text, 0, 0, 32, Color::BLACK);

        draw_particles(&mut particles, &mut d);
        draw_constraint(Vector2 { x: 400.0, y: 300.0 }, 300.0, Color::RED, &mut d);
    } 
}
