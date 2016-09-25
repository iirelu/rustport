#![allow(non_snake_case, non_camel_case_types)]

extern crate libc;

pub mod brushmodes;
pub mod helpers;
pub mod rng_double;
pub mod operationqueue;

#[cfg(test)]
mod tests {
    use operationqueue::*;
    use std::ptr;
    use std::collections::HashSet;

    const DAB: OperationDataDrawDab = OperationDataDrawDab {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
        color_r: 0,
        color_g: 0,
        color_b: 0,
        color_a: 0.0,
        opaque: 0.0,
        hardness: 0.0,
        aspect_ratio: 1.0,
        angle: 0.0,
        normal: 0.0,
        lock_alpha: 0.0,
        colorize: 0.0,
    };

    fn i(x: i32, y: i32) -> TileIndex {
        TileIndex { x: x, y: y }
    }

    // helper function for asserting the output of get_dirty_tiles
    // explicitly ignores the order in which tiles come in, as it's undefined
    unsafe fn assert_dirty_tiles(op_queue: *mut OperationQueue, indices: &[TileIndex]) {
        let mut tiles_out: *mut TileIndex = ptr::null_mut();
        let len = operation_queue_get_dirty_tiles(op_queue, &mut tiles_out as *mut _);

        let mut hashset = indices.into_iter().collect::<HashSet<_>>();
        println!("{:?}", hashset);
        for i in 0..len as isize {
            let tile = *tiles_out.offset(i);
            assert!(hashset.remove(&*tiles_out.offset(i)), "{:?}", tile);
        }
        assert!(hashset.is_empty());
        operation_queue_clear_dirty_tiles(op_queue);
    }

    fn generic_dab() -> *mut OperationDataDrawDab {
        Box::into_raw(Box::new(DAB))
    }

    #[test]
    fn test_op_queue() {
        unsafe {
            let mut dab = DAB;
            let op_queue = operation_queue_new();
            operation_queue_add(op_queue, i(0,0), &mut dab as *mut _);
            assert_dirty_tiles(op_queue, &[i(0,0)]);
            assert_eq!(*operation_queue_pop(op_queue, i(0,0)), dab);
        }
    }

    #[test]
    fn test_op_queue_add() {
        unsafe {
            let mut dab = DAB;
            let op_queue = operation_queue_new();
            operation_queue_add(op_queue, i(1,1), &mut dab as *mut _);
        }
    }

    #[test]
    fn test_op_queue_empty_pop() {
        unsafe {
            let op_queue = operation_queue_new();
            assert_eq!(operation_queue_pop(op_queue, i(1,1)), ptr::null_mut());
            assert_eq!(operation_queue_pop(op_queue, i(1,1)), ptr::null_mut());
            assert_eq!(operation_queue_pop(op_queue, i(123,1)), ptr::null_mut());
            assert_eq!(operation_queue_pop(op_queue, i(0,0)), ptr::null_mut());
        }
    }

    #[test]
    fn test_op_queue_many_add() {
        unsafe {
            let op_queue = operation_queue_new();
            for x in 0..10000 {
                operation_queue_add(op_queue, i(0,x), &mut DAB as *mut _);
            }
        }
    }

    #[test]
    fn test_op_queue_many_same_add() {
        unsafe {
            let op_queue = operation_queue_new();
            for _ in 0..10000 {
                let mut dab = DAB;
                operation_queue_add(op_queue, i(0,0), &mut dab as *mut _);
            }
            for _ in 0..10 {
                let op = operation_queue_pop(op_queue, i(0,0));
                assert_eq!(*op, DAB);
            }
        }
    }
}
