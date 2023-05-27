#![deny(unsafe_code)]
#![warn(
  clippy::all,
  clippy::await_holding_lock,
  clippy::char_lit_as_u8,
  clippy::checked_conversions,
  clippy::dbg_macro,
  clippy::debug_assert_with_mut_call,
  clippy::doc_markdown,
  clippy::empty_enum,
  clippy::enum_glob_use,
  clippy::exit,
  clippy::expl_impl_clone_on_copy,
  clippy::explicit_deref_methods,
  clippy::explicit_into_iter_loop,
  clippy::fallible_impl_from,
  clippy::filter_map_next,
  clippy::float_cmp_const,
  clippy::fn_params_excessive_bools,
  clippy::if_let_mutex,
  clippy::implicit_clone,
  clippy::imprecise_flops,
  clippy::inefficient_to_string,
  clippy::invalid_upcast_comparisons,
  clippy::large_types_passed_by_value,
  clippy::let_unit_value,
  clippy::linkedlist,
  clippy::lossy_float_literal,
  clippy::macro_use_imports,
  clippy::manual_ok_or,
  clippy::map_err_ignore,
  clippy::map_flatten,
  clippy::map_unwrap_or,
  clippy::match_on_vec_items,
  clippy::match_same_arms,
  clippy::match_wildcard_for_single_variants,
  clippy::mem_forget,
  clippy::mismatched_target_os,
  clippy::mut_mut,
  clippy::mutex_integer,
  clippy::needless_borrow,
  clippy::needless_continue,
  clippy::option_option,
  clippy::path_buf_push_overwrite,
  clippy::ptr_as_ptr,
  clippy::ref_option_ref,
  clippy::rest_pat_in_fully_bound_structs,
  clippy::same_functions_in_if_condition,
  clippy::semicolon_if_nothing_returned,
  clippy::string_add_assign,
  clippy::string_add,
  clippy::string_lit_as_bytes,
  clippy::string_to_string,
  clippy::todo,
  clippy::trait_duplication_in_bounds,
  clippy::unimplemented,
  clippy::unnested_or_patterns,
  clippy::unused_self,
  clippy::useless_transmute,
  clippy::verbose_file_reads,
  clippy::zero_sized_map_values,
  future_incompatible,
  nonstandard_style,
  rust_2018_idioms,
  rust_2021_compatibility
)]

mod grid;
mod newtypes;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use grid::cell::{draw_hexagon, HexagonCell};
#[cfg(feature = "debug")]
use grid::cell::{Column, Coordinate, HexagonFill, HexagonLines, Row};
#[cfg(feature = "debug")]
use newtypes::{pixel::Pixel, side_length::SideLength};

fn camera_setup(mut commands: Commands<'_, '_>) {
  commands.spawn(Camera2dBundle::default());
}

fn create_initial_hexagon(mut commands: Commands<'_, '_>) {
  commands.spawn(HexagonCell::default());
}

fn main() {
  let mut app = App::new();
  app
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Hexs".to_string(),
        resolution: (800., 800.).into(),
        ..Default::default()
      }),
      ..Default::default()
    }))
    .add_plugin(ShapePlugin)
    .add_startup_system(camera_setup.in_base_set(StartupSet::PreStartup))
    .add_startup_system(create_initial_hexagon.in_base_set(StartupSet::PreStartup))
    .add_startup_system(draw_hexagon);
  #[cfg(feature = "debug")]
  app
    .add_plugin(WorldInspectorPlugin::new())
    .register_type::<Column>()
    .register_type::<Row>()
    .register_type::<Coordinate>()
    .register_type::<HexagonCell>()
    .register_type::<HexagonFill>()
    .register_type::<HexagonLines>()
    .register_type::<Pixel>()
    .register_type::<SideLength>();
  app.run();
}
