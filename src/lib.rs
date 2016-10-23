#![allow(non_snake_case, non_camel_case_types)]
extern crate libc;
extern crate palette;
extern crate rand;

pub mod brushmodes;
pub mod helpers;
pub mod operationqueue;
pub mod mypaint_rectangle;
pub mod mypaint_surface;
pub mod mypaint_mapping;
pub mod mypaint_brush_settings;
pub mod mypaint_brush_settings_gen;
pub mod mypaint_brush;

// porting progress:
// loc  before  after
// c    3996    2392
// rust 0       1064

// as of oct 20 2016:
//
// porting progress:
// loc  before  after
// c    3996    1180
// rust 0       2220
