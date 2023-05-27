use std::ops::Deref;

use bevy::prelude::*;

#[cfg_attr(feature = "debug", derive(Reflect, FromReflect))]
#[derive(Clone, Component, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pixel(usize);

impl Default for Pixel {
  fn default() -> Self {
    Self(5)
  }
}

impl Deref for Pixel {
  type Target = usize;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<usize> for Pixel {
  fn from(value: usize) -> Self {
    Self(value)
  }
}
