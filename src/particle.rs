use raylib::prelude::*;

pub struct Particle {
    position_current: Vector2,
    position_old: Vector2,
    force: Vector2,
    radius: f32,
    color: Color,
    mass: f32,
}

impl Particle {
    pub fn new(position: Vector2, radius: f32, color: Color, mass: f32) -> Self{
        let force: Vector2 = Vector2::new(0.0,0.0);
        Self {
            position_current: position,
            position_old: position,
            force: force,
            radius: radius,
            color:color,
            mass: mass,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocity: Vector2 = self.position_current - self.position_old;

        self.position_old = self.position_current;

        let acceleration = self.force / self.mass;

        self.position_current = self.position_current + velocity + acceleration * dt * dt;

        self.force = Vector2::zero();
    }

    pub fn apply_force(&mut self, force: Vector2) {
        self.force += force;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.position_current.x.round() as i32,
        self.position_current.y.round() as i32,
            self.radius,
            self.color);
    }

    pub fn get_position(&self) -> Vector2 {
        self.position_current
    }

    pub fn set_position(&mut self, new_position: Vector2){
        self.position_current = new_position;
    }

    pub fn set_previous_position(&mut self, new_position: Vector2) {
        self.position_old = new_position;
    }

    pub fn get_velocity(&self) -> Vector2 {
        self.position_current - self.position_old
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_mass(&self) -> f32 {
        self.mass
    }

}