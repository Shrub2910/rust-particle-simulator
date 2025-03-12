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

pub fn solve_collisions(particles: &mut Vec<Particle>){
    for i in 0..particles.len(){
        for j in 0..particles.len(){

            if i == j{
                continue;
            }

            let particle1_position: Vector2 = particles[i].get_position();
            let particle2_position: Vector2 = particles[j].get_position();

            let distance: Vector2 = particle1_position - particle2_position;
            let direction: Vector2 = distance.normalized();
            let distance_length = distance.length();

            let size_total = particles[i].get_radius() + particles[j].get_radius();

            if distance_length < size_total {
                let delta = size_total - distance_length;
                particles[i].set_position(particle1_position + direction * 0.5 * delta);
                particles[j].set_position(particle2_position - direction * 0.5 * delta);
            }
        }
    }
}

pub fn update_positions(particles: &mut Vec<Particle>, delta_time: f32) {
    for particle in particles{
        particle.update_position(delta_time);
    }
}

