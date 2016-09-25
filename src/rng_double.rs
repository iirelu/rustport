use std::mem;

pub struct RngDouble {
    state: u64,
    inc: u64,
}

#[no_mangle]
pub unsafe extern fn rng_double_new(seed: u64) -> *mut RngDouble {
    Box::into_raw(Box::new(RngDouble {
        // dirty and horrible, but its good enough
        state: seed,
        inc: seed,
    }))
}

#[no_mangle]
pub unsafe extern fn rng_double_free(rng_double: *mut RngDouble) {
    Box::from_raw(rng_double);
}

#[no_mangle]
pub unsafe extern fn rng_double_next(self_: *mut RngDouble) -> f64 {
    let oldstate = (*self_).state;
    (*self_).state = oldstate * 6364136223846793005 + ((*self_).inc|1);
    let xorshifted = ((oldstate >> 18) ^ oldstate) >> 27;
    let rot = oldstate >> 59;
    let rand = (xorshifted >> rot) | (xorshifted << (!rot & 31));

    let result: f32 = mem::transmute(0x3F800000 | (rand as u32 & 0x7FFFFF));
    result as f64 - 1.0
}

#[no_mangle]
pub unsafe extern fn rand_gauss(rng: *mut RngDouble) -> f32 {
    let mut sum = 0.0f64;
    sum += rng_double_next(rng);
    sum += rng_double_next(rng);
    sum += rng_double_next(rng);
    sum += rng_double_next(rng);
    (sum * 1.73205080757 - 3.46410161514) as f32
}
