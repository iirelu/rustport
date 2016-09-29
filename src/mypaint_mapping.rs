#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ControlPoints {
    points: [(f32, f32); 8],
    n: i32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MypaintMapping {
    base_value: f32,
    inputs: i32,
    points_list: Vec<ControlPoints>,
    inputs_used: i32,
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_new(inputs_: i32) -> *mut MypaintMapping {
    let vec = vec![ControlPoints {
        points: [(0.0, 0.0); 8],
        n: 0
    }; inputs_ as usize];

    Box::into_raw(Box::new(MypaintMapping {
        base_value: 0.0,
        inputs: inputs_,
        points_list: vec,
        inputs_used: 0,
    }))
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_free(self_: *mut MypaintMapping) {
    Box::from_raw(self_);
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_get_base_value(
    self_: *mut MypaintMapping)
    -> f32
{
    assert!(!self_.is_null());
    (*self_).base_value
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_set_base_value(
    self_: *mut MypaintMapping, value: f32)
{
    assert!(!self_.is_null());
    (*self_).base_value = value;
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_set_n(
    self_: *mut MypaintMapping, input: i32, n: i32)
{
    assert!(!self_.is_null());
    let self_ = &mut *self_;

    assert!(input >= 0 && input < self_.inputs);
    assert!(n >= 0 && n <= 8);
    assert!(n != 1);
    let p = &mut self_.points_list[input as usize];

    if n != 0 && p.n == 0 {
        self_.inputs_used += 1;
    }
    if n == 0 && p.n != 0 {
        self_.inputs_used -= 1;
    }
    assert!(self_.inputs_used >= 0);
    assert!(self_.inputs_used <= self_.inputs);
    p.n = n;
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_get_n(
    self_: *mut MypaintMapping, input: i32)
    -> i32
{
    assert!(!self_.is_null());
    assert!(input >= 0 && input < (*self_).inputs);
    (*self_).points_list[input as usize].n
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_set_point(
    self_: *mut MypaintMapping, input: i32, index: i32, x: f32, y: f32)
{
    assert!(!self_.is_null());
    let self_ = &mut *self_;
    assert!(input >= 0 && input < self_.inputs);
    assert!(index >= 0 && index < 8);
    let p = &mut self_.points_list[input as usize];
    assert!(index < p.n);

    let index = index as usize;

    if index > 0 {
        assert!(x >= p.points[index - 1].0);
    }

    p.points[index] = (x, y);
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_get_point(
    self_: *mut MypaintMapping, input: i32, index: i32, x: *mut f32, y: *mut f32)
{
    assert!(!self_.is_null());
    let self_ = &mut *self_;
    assert!(input >= 0 && input < self_.inputs);
    assert!(index >= 0 && index < 8);
    let p = &mut self_.points_list[input as usize];
    assert!(index < p.n);

    *x = p.points[index as usize].0;
    *y = p.points[index as usize].1;
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_is_constant(
    self_: *mut MypaintMapping)
    -> bool
{
    assert!(!self_.is_null());
    (*self_).inputs_used == 0
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_get_inputs_used_n(
    self_: *mut MypaintMapping)
    -> i32
{
    assert!(!self_.is_null());
    (*self_).inputs_used
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_calculate(
    self_: *mut MypaintMapping,
    data: *mut f32)
    -> f32
{
    assert!(!self_.is_null());
    let self_ = &mut *self_;

    let mut result = self_.base_value;
    if self_.inputs_used == 0 {
        return result;
    }

    for j in 0..self_.inputs {
        let p = &mut self_.points_list[j as usize];

        if p.n == 0 {
            continue;
        }

        let x = *data.offset(j as isize);

        let (mut x0, mut y0) = p.points[0];
        let (mut x1, mut y1) = p.points[1];

        for i in 2..p.n as usize {
            if x <= x1 {
                break;
            }
            x0 = x1;
            y0 = y1;
            x1 = p.points[i].0;
            y1 = p.points[i].1;
        }

        let y = if x0 == x1 {
            y0
        } else {
            (y1*(x - x0) + y0*(x1 - x)) / (x1 - x0)
        };

        result += y;
    }
    result
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_calculate_single_input(
    self_: *mut MypaintMapping,
    mut input: f32)
    -> f32
{
    assert!(!self_.is_null());
    assert!((*self_).inputs == 1);
    mypaint_mapping_calculate(self_, &mut input as *mut _)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mypaint_mapping_ffi() {
        unsafe {
            let mapping = mypaint_mapping_new(9);
            mypaint_mapping_set_base_value(mapping, 0.35);
            mypaint_mapping_set_n(mapping, 3, 2);
            mypaint_mapping_set_n(mapping, 4, 3);
            mypaint_mapping_set_point(mapping, 3, 1, 0.2, 0.4);
            assert_eq!(mypaint_mapping_get_base_value(mapping), 0.35);
            assert_eq!(mypaint_mapping_get_n(mapping, 3), 2);
            assert_eq!(mypaint_mapping_get_n(mapping, 4), 3);
            println!("{:?}", *mapping);
        }
    }
}
