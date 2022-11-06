use bevy::prelude::*;

/// a 2d polar point (for [polar coordinate systems](https://en.wikipedia.org/wiki/Polar_coordinate_system)).
/// r is for the radius, theta for the angle, and z for screen order
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Polar {
    /// The radius (distance) from the reference pole.
    r: f32,
    /// The polar angle from the reference direction.
    theta: f32,
    /// used for z-ordering elements: higher `z`-value will be in front of lower
    /// `z`-value.
    z: f32,
}

impl Polar {
    /// creates a new polar point.
    pub fn new(r: f32, theta: f32, z: f32) -> Self {
        Self { r, theta, z }
    }
}

pub trait Vec3Ext {
    fn from_rtz(r: f32, theta: f32, z: f32) -> Self;
    fn from_polar(polar: Polar) -> Self;
}

impl Vec3Ext for Vec3 {
    ///creates a new [`Vec3`] from `(r, theta, z)` in the [polar coordinate system](https://en.wikipedia.org/wiki/Polar_coordinate_system)
    fn from_rtz(r: f32, theta: f32, z: f32) -> Self {
        Self::new(
            r * theta.to_radians().cos(),
            r * theta.to_radians().sin(),
            z,
        )
    }

    ///creates a new [`Vec3`] from `Polar` (a polar point in the [polar coordinate system](https://en.wikipedia.org/wiki/Polar_coordinate_system))
    fn from_polar(polar: Polar) -> Self {
        Self::new(
            polar.r * polar.theta.to_radians().cos(),
            polar.r * polar.theta.to_radians().sin(),
            polar.z,
        )
    }
}

pub trait TransformExt {
    fn new_polar(translation: Polar, rotation: Quat, scale: Vec3) -> Self;
    fn from_rtz(r: f32, theta: f32, z: f32) -> Self;
    fn from_polar(polar: Polar) -> Self;
}

impl TransformExt for Transform {
    /// Creates a new [`Transform`], with `(polar_translation, rotation, scale)`.
    fn new_polar(polar_translation: Polar, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation: Vec3::from_polar(polar_translation),
            rotation,
            scale,
        }
    }

    ///create a new [`Transform`] at the 2d [polar coordinates](https://en.wikipedia.org/wiki/Polar_coordinate_system) `(r, theta)`. The z component is used for z-ordering elements: higher z-value will be in front of lower z-value.
    fn from_rtz(r: f32, theta: f32, z: f32) -> Self {
        Self::from_polar(Polar::new(r, theta, z))
    }

    /// Creates a new [`Transform`], with `polar`. Rotation will be 0 and scale 1 on
    /// all axes.
    fn from_polar(polar: Polar) -> Self {
        Self {
            translation: Vec3::from_polar(polar),
            ..Self::identity()
        }
    }
}
