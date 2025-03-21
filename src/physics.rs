use raylib::prelude::*;

use crate::{particle::Particle, COF, PERIMETER_BOUNCE};

pub fn apply_gravity(particles: &mut Vec<Particle>){
    for particle in particles{
        particle.apply_force(Vector2 { x: 0.0, y: 1000.0 * particle.get_mass() });
    }
}

pub fn apply_mouse_gravity(particles: &mut Vec<Particle>, mouse_position: Vector2) {
    for particle in particles{
        let direction: Vector2 = (mouse_position - particle.get_position()).normalized();
        particle.apply_force(direction * 5000.0);
    }
}

pub fn apply_constraint(particles: &mut Vec<Particle>, position: Vector2, radius: f32) {
    for particle in particles{
        let distance: Vector2 = particle.get_position() - position;

        if (distance).length() > radius - particle.get_radius() {
            let velocity: Vector2 = particle.get_velocity();
            let direction: Vector2 = distance.normalized();
            
            let new_position: Vector2 = position + direction * (radius - particle.get_radius());

            let new_velocity: Vector2 = velocity - (direction * (velocity.dot(direction)) * 2.0);

            particle.set_position(new_position);
            particle.set_previous_position(new_position - new_velocity * PERIMETER_BOUNCE);

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
            let distance_length: f32 = distance.length();

            let particle1_radius: f32 = particles[i].get_radius();
            let particle2_radius: f32 = particles[j].get_radius();

            let size_total: f32 = particle1_radius + particle2_radius;

            if distance_length < size_total {
                let delta: f32 = size_total - distance_length;


                let particle1_mass: f32 = particles[i].get_mass();
                let particle2_mass: f32 = particles[j].get_mass();

                let new_particle1_position: Vector2 = particle1_position + direction * (particle1_radius / size_total) * delta;
                let new_particle2_position: Vector2 = particle2_position - direction * (particle2_radius / size_total) * delta;

                let particle1_velocity: Vector2 = particles[i].get_velocity();
                let particle2_velocity: Vector2 = particles[j].get_velocity();

                // Compute relative velocity along the normal direction
                let relative_velocity: f32 = (particle2_velocity - particle1_velocity).dot(direction);

                // Calculate impulse based on restitution and mass
                let impulse: f32 = (1.0 + COF) * relative_velocity / (particle1_mass + particle2_mass);

                // Calculate the new velocities
                let new_particle1_velocity: Vector2 = particle1_velocity + direction * impulse * particle2_mass;
                let new_particle2_velocity: Vector2 = particle2_velocity - direction * impulse * particle1_mass;



                particles[i].set_previous_position(new_particle1_position - new_particle1_velocity);
                particles[j].set_previous_position(new_particle2_position - new_particle2_velocity);

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

pub fn get_total_kinetic_energy(particles: &mut Vec<Particle>) -> f32{
    let mut kinetic_energy: f32 = 0.0;
    for particle in particles {
        let particle_velocity: Vector2 = particle.get_velocity();
        kinetic_energy += (particle.get_mass() * 0.5) * particle_velocity.length() * particle_velocity.length();
    }

    kinetic_energy
}