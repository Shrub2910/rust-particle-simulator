use raylib::prelude::*;

use crate::particle::Particle;

pub fn apply_gravity(particles: &mut Vec<Particle>){
    for particle in particles{
        particle.apply_force(Vector2 { x: 0.0, y: 1000.0 });
    }
}

pub fn apply_mouse_gravity(particles: &mut Vec<Particle>, mouse_position: Vector2) {
    for particle in particles{
        let direction: Vector2 = (mouse_position - particle.get_position()).normalized();
        particle.apply_force(direction * 1000.0);
    }
}

pub fn apply_constraint(particles: &mut Vec<Particle>, position: Vector2, radius: f32) {
    for particle in particles{
        let distance: Vector2 = particle.get_position() - position;

        if (distance).length() > radius - particle.get_radius() {
            let velocity: f32 = particle.get_velocity();
            let direction: Vector2 = distance.normalized();
            
            let new_position: Vector2 = position + direction * (radius - particle.get_radius());
            particle.set_position(new_position);
            particle.set_previous_position(new_position + direction * velocity);

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

                let new_particle1_position: Vector2 = particle1_position + direction * 0.5 * delta;
                let new_particle2_position: Vector2 = particle2_position - direction * 0.5 * delta;

                let particle1_velocity = particles[i].get_velocity();
                let particle2_velocity = particles[j].get_velocity();

                let new_velocity = (particle1_velocity + particle2_velocity) * 0.5;

                particles[i].set_previous_position(new_particle1_position - direction * new_velocity);
                particles[j].set_previous_position(new_particle2_position + direction * new_velocity);

                particles[i].set_position(new_particle1_position);
                particles[j].set_position(new_particle2_position);

            }
        }
    }
}

pub fn update_positions(particles: &mut Vec<Particle>, delta_time: f32) {
    for particle in particles{
        particle.update_position(delta_time);
    }
}

