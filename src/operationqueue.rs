use std::ptr;
use std::mem;
use tilemap::*;
use fifo::fifo as Fifo;
use fifo::{fifo_new, fifo_free, fifo_peek_first, fifo_peek_last, fifo_push, fifo_pop};
use libc::{free, malloc, c_void};

#[repr(C)]
pub struct OperationDataDrawDab {
    x: f32,
    y: f32,
    radius: f32,
    color_r: u16,
    color_g: u16,
    color_b: u16,
    color_a: f32,
    opaque: f32,
    hardness: f32,
    aspect_ratio: f32,
    angle: f32,
    normal: f32,
    lock_alpha: f32,
    colorize: f32,
}

#[repr(C)]
pub struct OperationQueue {
    tile_map: *mut TileMap,
    dirty_tiles: *mut TileIndex,
    dirty_tiles_n: usize,
}

#[no_mangle]
pub unsafe extern fn operation_queue_new() -> *mut OperationQueue {
    let self_ = Box::into_raw(Box::new(OperationQueue {
        tile_map: ptr::null_mut(),
        dirty_tiles: ptr::null_mut(),
        dirty_tiles_n: 0
    }));

    operation_queue_resize(self_, 10);

    self_
}

unsafe extern fn operation_delete_func(user_data: *mut c_void) {
    if !user_data.is_null() {
        Box::from_raw(user_data);
    }
}

unsafe extern fn free_fifo(item: *mut c_void) {
    let op_queue = item as *mut Fifo;
    if !op_queue.is_null() {
        fifo_free(op_queue, operation_delete_func);
    }
}

#[no_mangle]
pub unsafe extern fn operation_queue_resize(
    self_: *mut OperationQueue, new_size: usize)
    -> bool
{
    if new_size == 0 {
        if !(*self_).tile_map.is_null() {
            assert!(!(*self_).dirty_tiles.is_null());

            tile_map_free((*self_).tile_map, true);
            (*self_).tile_map = ptr::null_mut();
            free((*self_).dirty_tiles as *mut c_void);
            (*self_).dirty_tiles = ptr::null_mut();
            (*self_).dirty_tiles_n = 0;
        }
        true
    } else {
        let new_tile_map = tile_map_new(
            new_size as i32, mem::size_of::<Fifo>(), free_fifo);
        let new_map_size = (new_size*2).pow(2);
        // let new_dirty_tiles = {
        //     let vec = vec![mem::uninitialized(); new_map_size];
        //     let address = vec.as_mut_ptr();
        //     mem::forget(address);
        //     address
        // };

        // let new_dirty_tiles = Box::into_raw(
        //     vec![mem::uninitialized(); new_map_size].into_boxed_slice());
        let new_dirty_tiles = malloc(
            new_map_size*mem::size_of::<TileIndex>()) as *mut TileIndex;

        if !(*self_).tile_map.is_null() {
            tile_map_copy_to((*self_).tile_map, new_tile_map);
            for i in 0..(*self_).dirty_tiles_n as isize {
                *new_dirty_tiles.offset(i) = *(*self_).dirty_tiles.offset(i);
            }

            tile_map_free((*self_).tile_map, false);
            free((*self_).dirty_tiles as *mut c_void);
        }

        (*self_).tile_map = new_tile_map;
        (*self_).dirty_tiles = new_dirty_tiles;

        false
    }
}

#[no_mangle]
pub unsafe extern fn operation_queue_free(self_: *mut OperationQueue) {
    operation_queue_resize(self_, 0);

    Box::from_raw(self_);
}

unsafe fn remove_duplicate_tiles(array: *mut TileIndex, length: usize) -> usize {
    if length < 2 {
        return length;
    }

    let mut new_length = 1;

    'outer: for i in 1..length {
        for j in 1..new_length {
            if *array.offset(j as isize) == *array.offset(i as isize) {
                continue 'outer;
            }
        }
        *array.offset(new_length as isize) = *array.offset(i as isize);
        new_length += 1;
    }
    new_length
}

#[no_mangle]
pub unsafe extern fn operation_queue_get_dirty_tiles(
    self_: *mut OperationQueue, tiles_out: *mut *mut TileIndex)
    -> usize
{
    (*self_).dirty_tiles_n = remove_duplicate_tiles(
        (*self_).dirty_tiles, (*self_).dirty_tiles_n);

    *tiles_out = (*self_).dirty_tiles;
    (*self_).dirty_tiles_n
}

#[no_mangle]
pub unsafe extern fn operation_queue_clear_dirty_tiles(
    self_: *mut OperationQueue)
{
    (*self_).dirty_tiles_n = 0;
}

#[no_mangle]
pub unsafe extern fn operation_queue_add(
    self_: *mut OperationQueue, index: TileIndex, op: *mut OperationDataDrawDab)
{
    while !tile_map_contains((*self_).tile_map, index) {
        operation_queue_resize(self_, ((*(*self_).tile_map).size*2) as usize);
    }

    let mut queue_pointer =
        tile_map_get((*self_).tile_map, index) as *mut *mut Fifo;
    let mut op_queue = *queue_pointer;

    if op_queue.is_null() {
        op_queue = fifo_new();
        *queue_pointer = op_queue;
    }

    if fifo_peek_first(op_queue).is_null() {
        let max = ((*(*self_).tile_map).size*2).pow(2) as usize;
        if (*self_).dirty_tiles_n >= max {
            (*self_).dirty_tiles_n = remove_duplicate_tiles(
                (*self_).dirty_tiles, (*self_).dirty_tiles_n);
        }
        assert!((*self_).dirty_tiles_n < max);
        *(*self_).dirty_tiles.offset((*self_).dirty_tiles_n as isize) = index;
        (*self_).dirty_tiles_n += 1;
    }
    fifo_push(op_queue, op as *mut c_void);
}

#[no_mangle]
pub unsafe extern fn operation_queue_pop(
    self_: *mut OperationQueue, index: TileIndex)
    -> *mut OperationDataDrawDab
{
    if !tile_map_contains((*self_).tile_map, index) {
        return ptr::null_mut();
    }

    let mut queue_pointer =
        tile_map_get((*self_).tile_map, index) as *mut *mut Fifo;
    let op_queue = *queue_pointer;

    if op_queue.is_null() {
        return ptr::null_mut();
    }
    let op = fifo_pop(op_queue) as *mut OperationDataDrawDab;
    if op.is_null() {
        fifo_free(op_queue, operation_delete_func);
        *queue_pointer = ptr::null_mut();
    }
    op
}

#[no_mangle]
pub unsafe extern fn operation_queue_peek_first(
    self_: *mut OperationQueue, index: TileIndex)
    -> *mut OperationDataDrawDab
{
    if !tile_map_contains((*self_).tile_map, index) {
        return ptr::null_mut();
    }

    let op_queue = *tile_map_get((*self_).tile_map, index) as *mut Fifo;
    if op_queue.is_null() {
        ptr::null_mut()
    } else {
        fifo_peek_first(op_queue) as *mut OperationDataDrawDab
    }
}

#[no_mangle]
pub unsafe extern fn operation_queue_peek_last(
    self_: *mut OperationQueue, index: TileIndex)
    -> *mut OperationDataDrawDab
{
    if !tile_map_contains((*self_).tile_map, index) {
        return ptr::null_mut();
    }

    let op_queue = *tile_map_get((*self_).tile_map, index) as *mut Fifo;
    if op_queue.is_null() {
        ptr::null_mut()
    } else {
        fifo_peek_last(op_queue) as *mut OperationDataDrawDab
    }
}
