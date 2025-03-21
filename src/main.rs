

use particle::Particle;
use physics::{apply_constraint, apply_gravity, apply_mouse_gravity, get_total_kinetic_energy, solve_collisions, update_positions};
use draw::{draw_constraint, draw_particles};
use raylib::prelude::*;
mod particle;
mod physics;
mod draw;

const COF: f32 = 0.9;
const PERIMETER_BOUNCE: f32 = 0.9;
const FRICTION: f32 = 0.999;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Particle simulator")
        .build();

    rl.set_target_fps(60);

    let mut particles: Vec<Particle> = Vec::new();

    let mut ball_create_mode: bool = true;
    let mut mouse_gravity: bool = false;

    let mut gravity_enabled: bool = false;
    let mut collisions_enabled: bool = true;

    while !rl.window_should_close() {
        let mouse_position: Vector2 = rl.get_mouse_position();


        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && ball_create_mode {
            let random_number:f32 = rl.get_random_value::<i32>(5..20) as f32;
            let new_particle: Particle = Particle::new(mouse_position, random_number, Color::color_from_hsv(rl.get_random_value::<i32>(0..255) as f32, 1.0, 1.0), random_number * random_number/100.0);
            
            particles.push(new_particle);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) && ball_create_mode {
            if particles.len() != 0{
                particles.pop();
            }
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

        if rl.is_key_pressed(KeyboardKey::KEY_G) {
            gravity_enabled = !gravity_enabled;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            collisions_enabled = !collisions_enabled;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        for _ in 0..8{
            if !ball_create_mode && mouse_gravity{
                apply_mouse_gravity(&mut particles, mouse_position);
            } else{
                if gravity_enabled {
                    apply_gravity(&mut particles);
                }
            }

            apply_constraint(&mut particles, Vector2 { x: 400.0, y: 300.0 }, 300.0);

            if collisions_enabled {
                solve_collisions(&mut particles);
            }

            update_positions(&mut particles, 0.0167 / 8.0);
        }

        let text: String = format!("Number of particles: {}", particles.len());
        let kinetic_energy_text: String = format!("Total kinetic energy: {}", get_total_kinetic_energy(&mut particles));

        d.draw_text(&text, 0, 0, 16, Color::BLACK);
        d.draw_text(&kinetic_energy_text, 0, 20, 16, Color::BLACK);

        draw_constraint(Vector2 { x: 400.0, y: 300.0 }, 300.0, Color::RED, &mut d);
        draw_particles(&mut particles, &mut d);

    } 
}
