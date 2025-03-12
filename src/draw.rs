use raylib::prelude::*;

use crate::particle::Particle;

pub fn draw_particles(particles: &mut Vec<Particle>, d: &mut RaylibDrawHandle){
    for particle in particles{
        particle.draw(d);
    }
}

pub fn draw_constraint(position: Vector2, radius: f32, color:Color, d: &mut RaylibDrawHandle) {
    d.draw_circle_lines(position.x.round() as i32, position.y.round() as i32, radius, color);
}
