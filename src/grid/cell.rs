use std::{fmt::Display, ops::Deref};

use anyhow::Error;
use bevy::{prelude::*, render::color::Color};
use bevy_prototype_lyon::{prelude::*, shapes::Line};
use num_derive::FromPrimitive;
use num_integer::div_rem;
use num_traits::FromPrimitive;
use strum::{Display as StrumDisplay, EnumCount, EnumIter, IntoEnumIterator};

use crate::newtypes::{pixel::Pixel, side_length::SideLength};

#[cfg_attr(feature = "debug", derive(Reflect, FromReflect))]
#[derive(
  Clone, Component, Copy, Debug, Default, EnumCount, EnumIter, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, StrumDisplay,
)]
pub enum Letters {
  #[default]
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
}

impl TryFrom<usize> for Letters {
  type Error = Error;

  fn try_from(value: usize) -> Result<Self, Self::Error> {
    if value < Letters::COUNT {
      error!("The integer {value} is larger than the number of available letters");
    }
    Ok(Letters::from_usize(value).expect("This usize must represent a valid letter"))
  }
}

pub trait AsLetters {
  fn as_letters(&self) -> Vec<Letters>;
  fn from_letters_to_string<AsLetters: AsRef<[Letters]>>(letters_ref: AsLetters) -> String;
}

impl AsLetters for usize {
  fn as_letters(&self) -> Vec<Letters> {
    let mut remaining_divisor = *self;
    let mut output = vec![];
    loop {
      let (divisor, remainder) = div_rem(remaining_divisor, Letters::COUNT);
      output.insert(0, remainder.try_into().expect("This usize cannot be invalid"));
      if divisor == 0 {
        break;
      } else {
        remaining_divisor = divisor - 1;
      }
    }
    output
  }

  fn from_letters_to_string<AsLetters: AsRef<[Letters]>>(letters_ref: AsLetters) -> String {
    let letters = letters_ref.as_ref();
    letters.iter().fold(String::new(), |mut accumulated_string, letter| {
      accumulated_string.push_str(letter.to_string().as_str());
      accumulated_string
    })
  }
}

#[cfg_attr(feature = "debug", derive(Reflect, FromReflect))]
#[derive(Clone, Component, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Column(usize);

impl Deref for Column {
  type Target = usize;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Display for Column {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<usize> for Column {
  fn from(value: usize) -> Self {
    Self(value)
  }
}

impl AsLetters for Column {
  fn as_letters(&self) -> Vec<Letters> {
    self.0.as_letters()
  }

  fn from_letters_to_string<AsLetters: AsRef<[Letters]>>(letters_ref: AsLetters) -> String {
    usize::from_letters_to_string(letters_ref)
  }
}

#[cfg_attr(feature = "debug", derive(Reflect, FromReflect))]
#[derive(Clone, Component, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row(usize);

impl From<usize> for Row {
  fn from(value: usize) -> Self {
    Self(value)
  }
}

impl Display for Row {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl AsLetters for Row {
  fn as_letters(&self) -> Vec<Letters> {
    self.0.as_letters()
  }

  fn from_letters_to_string<AsLetters: AsRef<[Letters]>>(letters_ref: AsLetters) -> String {
    usize::from_letters_to_string(letters_ref)
  }
}

#[cfg_attr(feature = "debug", derive(Reflect, FromReflect))]
#[derive(Clone, Component, Copy, Debug, Default, PartialEq, Eq)]
pub struct Coordinate(Row, Column);

impl Display for Coordinate {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}

#[derive(Debug)]
pub struct HexagonCellConstructor {
  pub coordinate: Coordinate,
  pub side_length: SideLength,
  pub line_color: Color,
  pub line_thickness: Pixel,
  pub fill_color: Color,
}

impl Default for HexagonCellConstructor {
  fn default() -> Self {
    Self {
      coordinate: Coordinate::default(),
      side_length: SideLength::default(),
      line_color: Color::BLACK,
      line_thickness: Pixel::default(),
      fill_color: Color::WHITE,
    }
  }
}

#[cfg_attr(feature = "debug", derive(Reflect, FromReflect))]
#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub struct HexagonCell {
  coordinate: Coordinate,
  side_length: SideLength,
  line_color: Color,
  line_thickness: Pixel,
  fill_color: Color,
}

impl Default for HexagonCell {
  fn default() -> Self {
    Self::new(HexagonCellConstructor::default())
  }
}

impl HexagonCell {
  pub fn new(hexagon_constructor: HexagonCellConstructor) -> Self {
    Self {
      coordinate: hexagon_constructor.coordinate,
      side_length: hexagon_constructor.side_length,
      line_color: hexagon_constructor.line_color,
      line_thickness: hexagon_constructor.line_thickness,
      fill_color: hexagon_constructor.fill_color,
    }
  }
  pub fn coordinate(&self) -> &Coordinate {
    &self.coordinate
  }
  pub fn fill_color(&self) -> &Color {
    &self.fill_color
  }
  pub fn hexagon_cell_name(&self) -> Name {
    Name::new(format!("Hexagon Cell: {}", self.coordinate()))
  }
  pub fn line_color(&self) -> &Color {
    &self.line_color
  }
  pub fn line_thickness(&self) -> Pixel {
    self.line_thickness
  }
  pub fn side_length(&self) -> SideLength {
    self.side_length
  }
}

#[cfg_attr(feature = "debug", derive(Reflect, FromReflect))]
#[derive(Clone, Component, Copy, Debug, EnumCount, EnumIter, PartialEq, Eq, StrumDisplay)]
#[allow(clippy::enum_variant_names)]
pub enum HexagonLines {
  /*
    These are listed from a hexagon aligned so the top and bottom are parallel to
    a theoretical x-axis, starting with the line formed from the leftmost point
    to the top left point, and then continuing around clockwise
  */
  TopAscendingLine,
  TopHorizontalLine,
  TopDescendingLine,
  BottomAscendingLine,
  BottomHorizontalLine,
  BottomDescendingLine,
}

#[cfg_attr(feature = "debug", derive(Reflect, FromReflect))]
#[derive(Clone, Component, Copy, Debug)]
pub struct HexagonFill;

pub fn draw_hexagon(mut commands: Commands<'_, '_>, hexagon_cells: Query<'_, '_, (Entity, &HexagonCell)>) {
  for (hexagon_entity, hexagon_cell) in hexagon_cells.iter() {
    commands.entity(hexagon_entity).despawn_recursive();
    commands
      .spawn(*hexagon_cell)
      .insert(hexagon_cell.hexagon_cell_name())
      .insert(*hexagon_cell.coordinate())
      .insert(SpatialBundle::default())
      .with_children(|hexagon_fill_builder| {
        let center_point = Transform::default().translation;
        let hexagon = RegularPolygon {
          sides: 6,
          feature: RegularPolygonFeature::SideLength(*hexagon_cell.side_length()),
          ..Default::default()
        };
        hexagon_fill_builder
          .spawn((
            ShapeBundle {
              path: GeometryBuilder::build_as(&hexagon),
              ..Default::default()
            },
            Fill::color(*hexagon_cell.fill_color()),
          ))
          .insert(Name::new(format!("{}'s hexagonal fill", hexagon_cell.coordinate())))
          .insert(SpatialBundle::default())
          .insert(HexagonFill)
          .with_children(|hexagon_line_builder| {
            /*
                    +   +    |   +    ++++
                   +++  |    |  +++  +    +
                  +   + +    | +   +    +
                    ---------|--------- ++ +
                   |           ----------------\
                   |         -/        |        -\
            ++++   |       -/          |          \
            |  +   |      /            |           -\
            |+++   |    -/         + + +             \
            |  +   |  -/           +   -              -\
            ++ +   | /             + +++                -
                   | - ---------------------------------/
                      \-               |              /-
                        \-             |             /
                          \            |           /-
                           \-          |          /
                             \-        |        /-
                               \-------------- -

                    C is the centerpoint, B is the half height, and a1 and a2 are the quarter widths
                    S is the side length
                    The points for a hexagon are as follows, starting with the middle left point and going clockwise
                    point0- (C.x - 2*a1, c.y)
                    point1- (C.x - a1, c.y + b)
                    point2- (C.x + a1, c.y + b)
                    point3- (C.x + 2*a1, c.y)
                    point4- (C.x + a1, c.y - b)
                    point5 (C.x - a1, c.y - b)
                    */
            let a_length = hexagon_cell.side_length() / 2.0.into();
            let b_length = (hexagon_cell.side_length() * 3.0_f32.sqrt().into()) / 2.0.into();
            let mut points = vec![];
            points.push(Vec2::new(center_point.x - 2. * *a_length, center_point.y));
            points.push(Vec2::new(center_point.x - *a_length, center_point.y + *b_length));
            points.push(Vec2::new(center_point.x + *a_length, center_point.y + *b_length));
            points.push(Vec2::new(center_point.x + 2. * *a_length, center_point.y));
            points.push(Vec2::new(center_point.x + *a_length, center_point.y - *b_length));
            points.push(Vec2::new(center_point.x - *a_length, center_point.y - *b_length));
            for (iteration_count, hexagon_line_type) in HexagonLines::iter().enumerate() {
              let line = if iteration_count == HexagonLines::COUNT - 1 {
                Line(*points.get(iteration_count).unwrap(), *points.get(0).unwrap())
              } else {
                Line(
                  *points.get(iteration_count).unwrap(),
                  *points.get(iteration_count + 1).unwrap(),
                )
              };
              hexagon_line_builder
                .spawn((
                  ShapeBundle {
                    path: GeometryBuilder::build_as(&line),
                    ..Default::default()
                  },
                  Stroke::new(*hexagon_cell.line_color(), (*hexagon_cell.line_thickness()) as f32),
                ))
                .insert(Name::new(format!("{}: {}", hexagon_cell.coordinate(), hexagon_line_type)))
                .insert(SpatialBundle::from_transform(Transform::from_xyz(0., 0., 1.)))
                .insert(hexagon_line_type);
            }
          });
      });
  }
}

#[cfg(test)]
mod tests {
  use crate::grid::cell::AsLetters;

  #[test]
  fn single_letter_works_correctly() {
    let letter_a = 0_usize.as_letters();
    assert_eq!("A".to_string(), usize::from_letters_to_string(letter_a));
    let letter_j = 9_usize.as_letters();
    assert_eq!("J".to_string(), usize::from_letters_to_string(letter_j));
    let letter_z = 25_usize.as_letters();
    assert_eq!("Z".to_string(), usize::from_letters_to_string(letter_z));
    let letters_aa = 26_usize.as_letters();
    assert_eq!("AA".to_string(), usize::from_letters_to_string(letters_aa));
    let letters_aj = 35_usize.as_letters();
    assert_eq!("AJ".to_string(), usize::from_letters_to_string(letters_aj));
    let letters_az = 51_usize.as_letters();
    assert_eq!("AZ".to_string(), usize::from_letters_to_string(letters_az));
    let letters_ba = 52_usize.as_letters();
    assert_eq!("BA".to_string(), usize::from_letters_to_string(letters_ba));
    let letters_zz = 701_usize.as_letters();
    assert_eq!("ZZ".to_string(), usize::from_letters_to_string(letters_zz));
    let letters_aaa = 702_usize.as_letters();
    assert_eq!("AAA".to_string(), usize::from_letters_to_string(letters_aaa));
    let letters_aaj = 711_usize.as_letters();
    assert_eq!("AAJ".to_string(), usize::from_letters_to_string(letters_aaj));
    let letters_aaz = 727_usize.as_letters();
    assert_eq!("AAZ".to_string(), usize::from_letters_to_string(letters_aaz));
    let letters_aba = 728_usize.as_letters();
    assert_eq!("ABA".to_string(), usize::from_letters_to_string(letters_aba));
  }
}
