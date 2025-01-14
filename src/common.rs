//! Commmon type definitions for 2d and 3d physics simulation.
use std::num::FpCategory;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Extensions to the Bevy `Vec3` type
pub trait Vec3Ext {
    /// Returns the quaternion that describes the rotation from `self` to `other`.
    fn quat_between(&self, other: Vec3) -> Quat;
    /// Returns a projected copy of the current vector on other
    fn project(self, other : Vec3) -> Vec3;
    /// Returns a slided copy of the current vector on normal
    fn slide(self, normal : Vec3) -> Vec3;
}

impl Vec3Ext for Vec3 {
    fn quat_between(&self, other: Vec3) -> Quat {
        let dot = self.dot(other);
        if dot > 0.995 || dot < -0.995 {
            return Quat::IDENTITY;
        }

        let axis = self.cross(other);
        let angle = (self.length_squared() * other.length_squared()).sqrt() + dot;
        Quat::from_axis_angle(axis, angle)
    }
	fn project(self, n : Vec3) -> Vec3 {
		if n.is_normalized() {
			self.dot(n) * n
		}
		else {
			self // Just return the given a vector if n is not normalized
		}
	}
	fn slide(self, n : Vec3) -> Vec3 {
		if n.is_normalized() {
			self - self.project(n)
		}
		else {
			self
		}
	}
}
// Extentions to the Bevy 'Vec2' type
pub trait Vec2Ext {
    /// Returns a projected copy of the current vector on other
    fn project(self, other : Vec2) -> Vec2;
    /// Returns a slided copy of the current vector on normal
    fn slide(self, normal : Vec2) -> Vec2;
}
impl Vec2Ext for Vec2 {
	fn project(self, n : Vec2) -> Vec2 {
		if n.is_normalized() {
			self.dot(n) * n
		}
		else {
			self // Just return the given a vector if n is not normalized
		}
	}
	fn slide(self, n : Vec2) -> Vec2 {
		if n.is_normalized() {
			self - self.project(n)
		}
		else {
			self
		}
	}
}


/// The global friction that affects every `RigidBody`, both 2d and 3d.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalFriction(pub f32);

impl Default for GlobalFriction {
    fn default() -> Self {
        Self(0.95)
    }
}

/// The mass of the object or an infinite mass.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Mass {
    /// Infinite mass (useful for immobile static objects).
    Infinite,
    /// A real mass (useful for semikinematic and mobile static objects).
    Real(f32),
}

impl Mass {
    /// Get the inverse mass.
    pub fn inverse(self) -> f32 {
        match self {
            Self::Infinite => 0.0,
            Self::Real(mass) => mass.recip(),
        }
    }

    /// Get the scalar mass.
    pub fn scalar(self) -> f32 {
        match self {
            Self::Infinite => 0.0,
            Self::Real(mass) => mass,
        }
    }
}

impl From<f32> for Mass {
    fn from(mass: f32) -> Self {
        match mass.classify() {
            FpCategory::Normal | FpCategory::Subnormal => Mass::Real(mass),
            _ => Mass::Infinite,
        }
    }
}

/// The status of a `RigidBody`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum Status {
    /// Static bodies do not check for collisions.
    ///
    /// They are normally unaffected by forces.
    /// They CAN be mobile, however only if the user sets its force/velocity manually.
    Static,
    /// Semikinematic bodies check for collisions.
    ///
    /// They are affected by forces, like gravity.
    /// Semikinematic bodies behave like kinematic bodies in some cases,
    /// and like dynamic bodies in others.  They have full collision detection with
    /// static bodies and with semikinematic bodies.  Two semikinematic bodies can
    /// push on each other based on their velocity and their mass.  A semikinematic
    /// body may stand on top of another.  Semikinematic bodies can step onto small
    /// static coliders (depending on the global step value).
    Semikinematic,
}

impl Default for Status {
    fn default() -> Self {
        Status::Semikinematic
    }
}
