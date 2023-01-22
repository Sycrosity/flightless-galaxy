use core::{fmt, ops::*};

use bevy::{math::Vec3, prelude::*};
use bevy_inspector_egui::{
    inspector_options::std_options::NumberDisplay, prelude::*, InspectorOptions,
};

use crate::prelude::*;

/// a 2d polar point (for [polar coordinate systems](https://en.wikipedia.org/wiki/Polar_coordinate_system)).
/// r is for the radius, theta for the angle, and z for screen order
#[derive(Clone, Copy, Default, PartialEq, Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Polar {
    /// The radius (distance) from the reference pole.
    #[inspector(min = 0., max = 1024., display = NumberDisplay::Slider)]
    pub r: f32,
    /// The polar angle from the reference direction, in radians.
    #[inspector(min = -PI, max = PI, display = NumberDisplay::Slider)]
    pub theta: f32,
    /// used for z-ordering elements: higher `z`-value will be in front of lower
    /// `z`-value.
    #[inspector(min = 0., max = 256.)]
    pub z: f32,
}

impl Polar {
    /// creates a new polar point.
    pub fn new(r: f32, theta: f32, z: f32) -> Self {
        Self { r, theta, z }
    }

    pub fn from_vec3(vec: Vec3) -> Self {
        Self {
            r: ((vec.x.powi(2)) + (vec.y.powi(2))).sqrt(),
            theta: vec.y.atan2(vec.x),
            z: vec.z,
        }
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self::from_vec3(Vec3::new(x, y, z))
    }

    pub fn to_vec3(self) -> Vec3 {
        Vec3::from_polar(self)
    }

    ///Rotation of the point in radians from its `theta` angle.
    pub fn rotation(self) -> f32 {
        self.theta - FRAC_PI_2
    }

    ///Rotation of the point as a [`Quat`] from its `theta` angle.
    pub fn rotation_quat(self) -> Quat {
        Quat::from_rotation_z(self.rotation())
    }
}

impl Add<Polar> for Polar {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r.add(rhs.r),
            theta: self.theta.add(rhs.theta),
            z: self.z.add(rhs.z),
        }
    }
}

impl AddAssign<Polar> for Polar {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.r.add_assign(rhs.r);
        self.theta.add_assign(rhs.theta);
        self.z.add_assign(rhs.z);
    }
}

impl Sub<Polar> for Polar {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r.sub(rhs.r),
            theta: self.theta.sub(rhs.theta),
            z: self.z.sub(rhs.z),
        }
    }
}

impl SubAssign<Polar> for Polar {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.r.sub_assign(rhs.r);
        self.theta.sub_assign(rhs.theta);
        self.z.sub_assign(rhs.z);
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for Polar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.r, self.theta, self.z)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for Polar {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(Polar))
            .field(&self.r)
            .field(&self.theta)
            .field(&self.z)
            .finish()
    }
}

pub trait Vec3Ext {
    fn from_rtz(r: f32, theta: f32, z: f32) -> Self;
    fn from_polar(polar: Polar) -> Self;
    fn to_polar(self) -> Polar;
}

impl Add<Polar> for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Polar) -> Self {
        Vec3::from_polar(Polar::from_vec3(self).add(rhs))
    }
}

impl AddAssign<Polar> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Polar) {
        // Polar::from_xvec3(self).add_assign(rhs);
        // self
        *self = Vec3::from_polar(Polar::from_vec3(*self).add(rhs))
    }
}

impl Sub<Polar> for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Polar) -> Self {
        Vec3::from_polar(Polar::from_vec3(self).sub(rhs))
    }
}

impl SubAssign<Polar> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Polar) {
        // Polar::from_xvec3(self).add_assign(rhs);
        // self
        *self = Vec3::from_polar(Polar::from_vec3(*self).sub(rhs))
    }
}

impl Vec3Ext for Vec3 {
    ///creates a new [`Vec3`] from `(r, theta, z)` in the [polar coordinate system](https://en.wikipedia.org/wiki/Polar_coordinate_system)
    fn from_rtz(r: f32, theta: f32, z: f32) -> Self {
        Self::from_polar(Polar::new(r, theta, z))
    }

    ///creates a new [`Vec3`] from `Polar` (a polar point in the [polar coordinate system](https://en.wikipedia.org/wiki/Polar_coordinate_system))
    fn from_polar(polar: Polar) -> Self {
        Self::new(
            polar.r * polar.theta.cos(),
            polar.r * polar.theta.sin(),
            polar.z,
        )
    }

    fn to_polar(self) -> Polar {
        Polar::from_vec3(self)
    }
}

pub trait TransformExt {
    fn from_polar_seperate(translation: Polar, rotation: Quat, scale: Vec3) -> Self;
    fn from_rtz(r: f32, theta: f32, z: f32) -> Self;
    fn from_polar(polar: Polar) -> Self;
}

impl TransformExt for Transform {
    /// Creates a new [`Transform`], with `(polar_translation, rotation, scale)`, not deriving the rotation from the [`Polar`].
    fn from_polar_seperate(polar_translation: Polar, rotation: Quat, scale: Vec3) -> Self {
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

    /// Creates a new [`Transform`], with `polar`. Rotation will be derived from the [`Polar`], with thescale 1 on
    /// all axes.
    fn from_polar(polar: Polar) -> Self {
        Self {
            translation: Vec3::from_polar(polar),
            rotation: polar.rotation_quat(),
            ..Self::IDENTITY
        }
    }
}
