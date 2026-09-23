use crate::body::{Body, Geometry};
use crate::math::Vec2;

#[repr(C)]
#[derive(Debug)]
pub struct CollisionInfo<'a> {
    pub a: &'a mut Body,
    pub b: &'a mut Body,
    pub start: Vec2,
    pub end: Vec2,
    pub normal: Vec2,
    pub depth: f32,
    pub proj_a: Vec2, // add to a pos to resolve collision
    pub proj_b: Vec2, // add to b pos to resolve collision
}

// TODO: Objects very close to each other?
pub fn is_colliding_circle_circle<'a>(
    a: &'a mut Body,
    b: &'a mut Body,
) -> Option<CollisionInfo<'a>> {
    if let (&Geometry::Circle { r: ra }, &Geometry::Circle { r: rb }) = (a.geom(), b.geom()) {
        let dist = b.pos().sub(a.pos());
        let rsum = ra + rb;

        let is_colliding = dist.mag_sq() <= rsum * rsum;

        if !is_colliding {
            return None;
        }

        let normal = dist.normalized();
        let start = b.pos().sub(normal.scaled(rb));
        let end = a.pos().add(normal.scaled(ra));
        let depth = end.sub(start).mag();

        let da = depth / (a.mass_inv() + b.mass_inv()) * a.mass_inv();
        let db = depth / (a.mass_inv() + b.mass_inv()) * b.mass_inv();

        let proj_a = normal.scaled(da).negated();
        let proj_b = normal.scaled(db);

        return Some(CollisionInfo {
            a,
            b,
            normal,
            start,
            end,
            depth,
            proj_a,
            proj_b,
        });
    }

    panic!("not circle circle");
}

// TODO: Fix pos first?
pub fn resolve(info: &mut CollisionInfo) {
    let e = info.a.restitution().min(info.b.restitution());
    let vrel = info.a.vel().sub(info.b.vel());

    let dir = info.normal;
    let mag = -(1.0 + e) * vrel.dot(info.normal) / (info.a.mass_inv() + info.b.mass_inv());
    let jn = Vec2::from_dir_mag(dir, mag);

    info.a.apply_impulse(jn);
    info.b.apply_impulse(jn.negated());
}
