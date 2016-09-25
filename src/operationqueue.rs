use std::ptr;

use std::collections::{HashMap, HashSet, VecDeque};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OperationDataDrawDab {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color_r: u16,
    pub color_g: u16,
    pub color_b: u16,
    pub color_a: f32,
    pub opaque: f32,
    pub hardness: f32,
    pub aspect_ratio: f32,
    pub angle: f32,
    pub normal: f32,
    pub lock_alpha: f32,
    pub colorize: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct TileIndex {
    pub x: i32,
    pub y: i32,
}

pub struct OperationQueue {
    tile_map: HashMap<TileIndex, VecDeque<*mut OperationDataDrawDab>>,
    dirty_tiles: HashSet<TileIndex>,
    // implementation detail, need to keep track of this in order to free what
    // operation_queue_get_dirty_tiles returns in operation_queue_clear_dirty_tiles
    _dt_vec: Option<Vec<TileIndex>>,
}

impl OperationQueue {
    fn new() -> Self {
        OperationQueue {
            tile_map: HashMap::new(),
            dirty_tiles: HashSet::new(),
            _dt_vec: None,
        }
    }

    // note: this should return a ref, but until the c code that uses this is
    // eliminated it returns a pointer and manages it
    fn dirty_tiles(&mut self) -> (*mut TileIndex, usize) {
        let mut vec = self.dirty_tiles.iter()
            .map(|&x| x)
            .collect::<Vec<_>>();
        let ptr = vec.as_mut_ptr();
        let len = vec.len();
        self._dt_vec = Some(vec);
        (ptr, len)
    }

    fn clear_dirty_tiles(&mut self) {
        self._dt_vec = None;
        self.dirty_tiles.clear();
    }

    fn push(&mut self, index: TileIndex, op: *mut OperationDataDrawDab) {
        self.tile_map.entry(index).or_insert(VecDeque::new())
            .push_back(op);
        self.dirty_tiles.insert(index);
    }

    fn pop(&mut self, index: TileIndex) -> Option<*mut OperationDataDrawDab> {
        let ret = match self.tile_map.get_mut(&index) {
            Some(mut op) => op.pop_front(),
            None => None,
        };
        if ret.is_none() {
            self.tile_map.remove(&index);
        }
        ret
    }
}


#[no_mangle]
pub unsafe extern fn operation_queue_new() -> *mut OperationQueue {
    Box::into_raw(Box::new(OperationQueue::new()))
}

#[no_mangle]
pub unsafe extern fn operation_queue_free(self_: *mut OperationQueue) {
    assert!(!self_.is_null());
    Box::from_raw(self_);
}

#[no_mangle]
pub unsafe extern fn operation_queue_get_dirty_tiles(
    self_: *mut OperationQueue, tiles_out: *mut *mut TileIndex)
    -> usize
{
    assert!(!self_.is_null());
    let (out, len) = (&mut *self_).dirty_tiles();
    *tiles_out = out;
    len
}

#[no_mangle]
pub unsafe extern fn operation_queue_clear_dirty_tiles(
    self_: *mut OperationQueue)
{
    assert!(!self_.is_null());
    (&mut *self_).clear_dirty_tiles();
}

#[no_mangle]
pub unsafe extern fn operation_queue_add(
    self_: *mut OperationQueue, index: TileIndex, op: *mut OperationDataDrawDab)
{
    assert!(!self_.is_null());
    assert!(!op.is_null());
    (&mut *self_).push(index, op);
}

#[no_mangle]
pub unsafe extern fn operation_queue_pop(
    self_: *mut OperationQueue, index: TileIndex)
    -> *mut OperationDataDrawDab
{
    assert!(!self_.is_null());
    match (&mut *self_).pop(index) {
        Some(pop) => pop,
        None => ptr::null_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
