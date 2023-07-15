use std::{
  fmt::Display,
  ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use bevy::prelude::*;

#[cfg_attr(feature = "debug", derive(Reflect))]
#[derive(Clone, Component, Copy, Debug, PartialEq, PartialOrd)]
pub struct SideLength(f32);

impl Default for SideLength {
  fn default() -> Self {
    Self(50.)
  }
}

impl Deref for SideLength {
  type Target = f32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Display for SideLength {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<f32> for SideLength {
  fn from(value: f32) -> Self {
    Self(value)
  }
}

impl Add for SideLength {
  type Output = Self;

  fn add(self, right_hand_side: Self) -> Self::Output {
    Self(self.0 + right_hand_side.0)
  }
}

impl AddAssign for SideLength {
  fn add_assign(&mut self, right_hand_side: Self) {
    *self = *self + right_hand_side;
  }
}

impl Sub for SideLength {
  type Output = Self;

  fn sub(self, right_hand_side: Self) -> Self::Output {
    Self(self.0 - right_hand_side.0)
  }
}

impl SubAssign for SideLength {
  fn sub_assign(&mut self, right_hand_side: Self) {
    *self = *self - right_hand_side;
  }
}

impl Mul for SideLength {
  type Output = Self;

  fn mul(self, right_hand_side: Self) -> Self::Output {
    Self(self.0 * right_hand_side.0)
  }
}

impl MulAssign for SideLength {
  fn mul_assign(&mut self, right_hand_side: Self) {
    *self = *self * right_hand_side;
  }
}

impl Div for SideLength {
  type Output = Self;

  fn div(self, right_hand_side: Self) -> Self::Output {
    Self(self.0 / right_hand_side.0)
  }
}

impl DivAssign for SideLength {
  fn div_assign(&mut self, right_hand_side: Self) {
    *self = *self / right_hand_side;
  }
}
