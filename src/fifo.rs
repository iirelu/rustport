use libc::c_void;
use std::ptr;

struct fifo_item {
    next: *mut fifo_item,
    payload: *mut c_void,
}

pub struct fifo {
    first: *mut fifo_item,
    last: *mut fifo_item,
}

#[no_mangle]
pub unsafe extern fn fifo_new() -> *mut fifo {
    Box::into_raw(Box::new(fifo {
        first: ptr::null_mut(),
        last: ptr::null_mut(),
    }))
}

#[no_mangle]
pub unsafe extern fn fifo_free(
    queue: *mut fifo, user_free: unsafe extern fn(*mut c_void))
{
    loop {
        let item = (*queue).first;
        if item == ptr::null_mut() {
            break;
        }
        (*queue).first = (*item).next;
        user_free((*item).payload);
    }
    Box::from_raw(queue);
}

#[no_mangle]
pub unsafe extern fn fifo_push(queue: *mut fifo, data: *mut c_void) {
    let item = Box::into_raw(Box::new(fifo_item {
        next: ptr::null_mut(),
        payload: data
    }));
    if (*queue).last.is_null() {
        (*queue).first = item;
    } else {
        (*(*queue).last).next = item;
    }
    (*queue).last = item;
}

#[no_mangle]
pub unsafe extern fn fifo_pop(queue: *mut fifo) -> *mut c_void {
    let item = (*queue).first;
    if item.is_null() {
        return ptr::null_mut();
    }

    (*queue).first = (*item).next;
    if (*queue).first.is_null() {
        (*queue).last = ptr::null_mut();
    }

    let data = (*item).payload;
    Box::from_raw(item);
    data
}

#[no_mangle]
pub unsafe extern fn fifo_peek_first(queue: *mut fifo) -> *mut c_void {
    if (*queue).first.is_null() {
        return ptr::null_mut();
    }
    (*(*queue).first).payload
}

#[no_mangle]
pub unsafe extern fn fifo_peek_last(queue: *mut fifo) -> *mut c_void {
    if (*queue).last.is_null() {
        return ptr::null_mut();
    }
    (*(*queue).last).payload
}
