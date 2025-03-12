use raylib::prelude::*;

use crate::particle::Particle;

pub fn apply_gravity(particles: &mut Vec<Particle>){
    for particle in particles{
        particle.apply_force(Vector2 { x: 0.0, y: 1000.0 });
    }
}

pub fn apply_constraint(particles: &mut Vec<Particle>, position: Vector2, radius: f32) {
    for particle in particles{
        let distance: Vector2 = particle.get_position() - position;

        if (distance).length() > radius - particle.get_radius() {
            let direction: Vector2 = distance.normalized();
            particle.set_position(position + direction * (radius - particle.get_radius()));
        }
    }
}

pub fn update_positions(particles: &mut Vec<Particle>, delta_time: f32) {
    for particle in particles{
        particle.update_position(delta_time);
    }
}

