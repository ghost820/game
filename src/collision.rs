use crate::body::{Body, Geometry};
use crate::math::Vec2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CollisionInfo<'a> {
    pub a: &'a Body,
    pub b: &'a Body,
    pub start: Vec2,
    pub end: Vec2,
    pub normal: Vec2,
    pub depth: f32,
    pub proj_a: Vec2, // add to a pos to resolve collision
    pub proj_b: Vec2, // add to b pos to resolve collision
}

// TODO: Objects very close to each other?
pub fn is_colliding_circle_circle<'a>(a: &'a Body, b: &'a Body) -> Option<CollisionInfo<'a>> {
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
