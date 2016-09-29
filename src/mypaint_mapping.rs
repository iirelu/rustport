#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ControlPoints {
    points: [(f32, f32); 8],
    n: usize,
}

impl ControlPoints {
    pub fn new() -> Self {
        ControlPoints {
            points: [(0.0, 0.0); 8],
            n: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MypaintMapping {
    base_value: f32,
    inputs: usize,
    points_list: Vec<ControlPoints>,
    inputs_used: usize,
}

impl MypaintMapping {
    pub fn new(inputs: usize) -> Self {
        let vec = vec![ControlPoints::new(); inputs];
        MypaintMapping {
            base_value: 0.0,
            inputs: inputs,
            points_list: vec,
            inputs_used: 0,
        }
    }

    pub fn get_base_value(&self) -> f32 {
        self.base_value
    }
    pub fn set_base_value(&mut self, value: f32) {
        self.base_value = value;
    }

    pub fn get_n(&self, input: usize) -> Option<usize> {
        self.points_list.get(input).map(|x| x.n)
    }
    pub fn set_n(&mut self, input: usize, n: usize) {
        assert!(input < self.inputs);
        assert!(n <= 8);
        assert!(n != 1); // ?
        let p = &mut self.points_list[input];

        if n != 0 && p.n == 0 {
            self.inputs_used += 1;
        } else if n == 0 && p.n != 0 {
            self.inputs_used -= 1;
        }

        assert!(self.inputs_used <= self.inputs);
        p.n = n;
    }

    pub fn get_point(&self, input: usize, index: usize) -> (f32, f32) {
        assert!(input < self.inputs);
        assert!(index < 8);
        let p = &self.points_list[input];
        assert!(index < p.n);

        p.points[index]
    }
    pub fn set_point(&mut self, input: usize, index: usize, point: (f32, f32)) {
        assert!(input < self.inputs);
        assert!(index < 8);
        let p = &mut self.points_list[input];
        assert!(index < p.n);

        if index > 0 {
            assert!(point.0 >= p.points[index - 1].0);
        }
        p.points[index] = point;
    }

    pub fn is_constant(&self) -> bool {
        self.inputs_used == 0
    }

    pub fn get_inputs_used(&self) -> usize {
        self.inputs_used
    }

    pub fn calculate(&self, data: &[f32]) -> f32 {
        let mut result = self.base_value;
        if self.inputs_used == 0 {
            return result;
        }

        for j in 0..self.inputs {
            let p = &self.points_list[j];
            if p.n == 0 {
                continue;
            }

            let x = data[j];

            let mut p0 = p.points[0];
            let mut p1 = p.points[1];

            for i in 2..p.n {
                if x <= p1.0 {
                    break;
                }
                p0 = p1;
                p1 = p.points[i];
            }

            let y = if x == p1.0 {
                p0.1
            } else {
                let (x0, y0) = p0;
                let (x1, y1) = p1;
                (y1*(x - x0) + y0*(x1 - x)) / (x1 - x0)
            };
            result += y;
        }
        result
    }
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_new(inputs_: i32) -> *mut MypaintMapping {
    Box::into_raw(Box::new(MypaintMapping::new(inputs_ as usize)))
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
    (*self_).get_base_value()
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_set_base_value(
    self_: *mut MypaintMapping, value: f32)
{
    assert!(!self_.is_null());
    (*self_).set_base_value(value);
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_set_n(
    self_: *mut MypaintMapping, input: i32, n: i32)
{
    assert!(!self_.is_null());
    (*self_).set_n(input as usize, n as usize);
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_get_n(
    self_: *mut MypaintMapping, input: i32)
    -> i32
{
    assert!(!self_.is_null());
    (*self_).get_n(input as usize).unwrap() as i32
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_set_point(
    self_: *mut MypaintMapping, input: i32, index: i32, x: f32, y: f32)
{
    assert!(!self_.is_null());
    (*self_).set_point(input as usize, index as usize, (x, y));
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_get_point(
    self_: *mut MypaintMapping, input: i32, index: i32, x: *mut f32, y: *mut f32)
{
    assert!(!self_.is_null());
    let p = (*self_).get_point(input as usize, index as usize);
    *x = p.0;
    *y = p.1;
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_is_constant(
    self_: *mut MypaintMapping)
    -> bool
{
    assert!(!self_.is_null());
    (*self_).is_constant()
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_get_inputs_used_n(
    self_: *mut MypaintMapping)
    -> i32
{
    assert!(!self_.is_null());
    (*self_).get_inputs_used() as i32
}

#[no_mangle]
pub unsafe extern fn mypaint_mapping_calculate(
    self_: *mut MypaintMapping,
    data: *mut f32)
    -> f32
{
    assert!(!self_.is_null());
    use std::slice;
    let slice = slice::from_raw_parts(data, (*self_).inputs);
    (*self_).calculate(slice)
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
