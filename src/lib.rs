#![allow(non_snake_case, non_camel_case_types)]
extern crate libc;
extern crate palette;

pub mod brushmodes;
pub mod helpers;
pub mod rng_double;
pub mod operationqueue;
pub mod mypaint_rectangle;
pub mod mypaint_surface;
pub mod mypaint_mapping;

// porting progress:
// loc  before  after
// c    3996    2392
// rust 0       1064
