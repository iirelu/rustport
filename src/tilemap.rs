use libc::c_void;
use std::ptr;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TileIndex {
    x: i32,
    y: i32,
}

#[repr(C)]
pub struct TileMap {
    map: *mut *mut c_void,
    pub size: i32,
    item_size: usize,
    item_free_func: unsafe extern fn(*mut c_void),
}

#[no_mangle]
pub unsafe extern fn tile_map_new(
    size: i32, item_size: usize, item_free_func: unsafe extern fn(*mut c_void))
    -> *mut TileMap
{
    let mut vec = vec![ptr::null_mut(); (size*2).pow(2) as usize];
    let map = vec.as_mut_ptr();
    mem::forget(vec);
    Box::into_raw(Box::new(TileMap {
        size: size,
        item_size: item_size,
        item_free_func: item_free_func,
        map: map,
    }))
}

#[no_mangle]
pub unsafe extern fn tile_map_free(
    self_: *mut TileMap, free_items: bool)
{
    let tilemap = Box::from_raw(self_);
    let map_size = (2*tilemap.size).pow(2);
    if free_items {
        for i in 0..map_size {
            (tilemap.item_free_func)(*tilemap.map.offset(i as isize));
        }
    }
    Vec::from_raw_parts(tilemap.map, map_size as usize, map_size as usize);
}

#[no_mangle]
pub unsafe extern fn tile_map_contains(
    self_: *mut TileMap, index: TileIndex)
    -> bool
{
    let size = (*self_).size;
    index.x >= -size && index.x < size
        && index.y >= -size && index.y < size
}

#[no_mangle]
pub unsafe extern fn tile_map_get(
    self_: *mut TileMap, index: TileIndex)
    -> *mut *mut c_void
{
    let size = (*self_).size;
    assert!(tile_map_contains(self_, index));
    let rowstride = size*2;
    let offset = (size + index.y) * rowstride + size + index.x;
    (*self_).map.offset(offset as isize)
}

#[no_mangle]
pub unsafe extern fn tile_map_copy_to(
    self_: *mut TileMap, other: *mut TileMap)
{
    assert!((*other).size >= (*self_).size);

    let size = (*self_).size;
    for y in -size..size {
        for x in -size..size {
            let index = TileIndex { x: x, y: y };
            *tile_map_get(other, index) = *tile_map_get(self_, index);
        }
    }
}
