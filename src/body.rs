use crate::PIXELS_PER_METER;
use crate::math::{Vec2, wrap_angle_rad};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Body {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    rot: f32,
    ang_vel: f32,
    ang_acc: f32,
    mass: f32,
    mass_inv: f32,
    inert: f32,
    inert_inv: f32,
    force: Vec2,
    torque: f32,
    geom: Geometry,
}

impl Body {
    pub const fn new(geom: Geometry, pos: Vec2, mass: f32) -> Self {
        debug_assert!(mass.is_normal() && mass > 0.0);

        let mass_inv = 1.0 / mass;

        let inert = geom.inert(mass);
        let inert_inv = 1.0 / inert;
        debug_assert!(inert.is_finite() && inert > 0.0);
        debug_assert!(inert_inv.is_finite() && inert_inv > 0.0);

        Self {
            pos,
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            rot: 0.0,
            ang_vel: 0.0,
            ang_acc: 0.0,
            mass,
            mass_inv,
            inert,
            inert_inv,
            force: Vec2::ZERO,
            torque: 0.0,
            geom,
        }
    }

    pub const fn from_raw_coords(geom: Geometry, x: f32, y: f32, mass: f32) -> Self {
        debug_assert!(mass.is_normal() && mass > 0.0);

        let mass_inv = 1.0 / mass;

        let inert = geom.inert(mass);
        let inert_inv = 1.0 / inert;
        debug_assert!(inert.is_finite() && inert > 0.0);
        debug_assert!(inert_inv.is_finite() && inert_inv > 0.0);

        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            rot: 0.0,
            ang_vel: 0.0,
            ang_acc: 0.0,
            mass,
            mass_inv,
            inert,
            inert_inv,
            force: Vec2::ZERO,
            torque: 0.0,
            geom,
        }
    }

    pub const fn x(&self) -> f32 {
        self.pos.x()
    }

    pub const fn y(&self) -> f32 {
        self.pos.y()
    }

    pub const fn vel(&self) -> Vec2 {
        self.vel
    }

    pub const fn rot(&self) -> f32 {
        self.rot
    }

    pub const fn mass(&self) -> f32 {
        self.mass
    }

    pub const fn geom(&self) -> &Geometry {
        &self.geom
    }

    pub const fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub const fn set_pos_xy(&mut self, x: f32, y: f32) {
        self.pos = Vec2::new(x, y);
    }

    pub const fn set_vel(&mut self, vel: Vec2) {
        self.vel = vel;
    }

    pub const fn apply_force(&mut self, force: Vec2) {
        self.force.addi(force.scaled(PIXELS_PER_METER as f32));
    }

    pub const fn apply_torque(&mut self, torque: f32) {
        self.torque += torque;

        debug_assert!(self.torque.is_normal());
    }

    pub fn update(&mut self, dt: f32) {
        self.acc = self.force.scaled(self.mass_inv);
        self.vel.addi(self.acc.scaled(dt));
        self.pos.addi(self.vel.scaled(dt));
        self.force = Vec2::ZERO;

        self.ang_acc = self.torque * self.inert_inv;
        self.ang_vel += self.ang_acc * dt;
        self.rot = wrap_angle_rad(self.rot + self.ang_vel * dt);
        self.torque = 0.0;
        debug_assert!(self.ang_acc.is_finite());
        debug_assert!(self.ang_vel.is_finite());
        debug_assert!(self.rot.is_finite());
    }

    pub fn draw(&self, framebuffer: &mut crate::Framebuffer, c: crate::Rgba) {
        use crate::draw::draw_rect;

        draw_rect(
            framebuffer,
            self.pos.x() as usize,
            self.pos.y() as usize,
            3,
            3,
            c,
        );
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub enum Geometry {
    Circle { r: f32 },
    Box { w: f32, h: f32 },
}

impl Geometry {
    pub const fn circle(r: f32) -> Self {
        debug_assert!(r.is_normal() && r > 0.0);

        Self::Circle { r }
    }

    pub const fn box_(w: f32, h: f32) -> Self {
        debug_assert!(w.is_normal() && w > 0.0);
        debug_assert!(h.is_normal() && h > 0.0);

        Self::Box { w, h }
    }

    pub const fn inert(&self, mass: f32) -> f32 {
        match *self {
            Geometry::Circle { r } => 0.5 * r * r * mass,
            Geometry::Box { w, h } => 0.083333 * (w * w + h * h) * mass,
        }
    }
}
