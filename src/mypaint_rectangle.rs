#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MyPaintRectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl MyPaintRectangle {
    fn expand_to_include(&mut self, x: i32, y: i32) {
        if self.width == 0 || self.height == 0 {
            self.width = 1;
            self.height = 1;
            self.x = x;
            self.y = y;
        } else {
            if x < self.x {
                self.width += self.x - x;
                self.x = x;
            } else if x >= self.x + self.width {
                self.width = x - self.x + 1;
            }

            if y < self.y {
                self.height += self.y - y;
                self.y = y;
            } else if y >= self.y + self.height {
                self.height = y - self.y + 1;
            }
        }
    }
}


#[no_mangle]
pub unsafe extern fn mypaint_rectangle_expand_to_include_point(
    self_: *mut MyPaintRectangle, x: i32, y: i32)
{
    assert!(!self_.is_null());
    let r = &mut *self_;
    r.expand_to_include(x, y);
}

// almost certain this is never needed, but just in case

// #[no_mangle]
// pub unsafe extern fn mypaint_rectangle_copy(
//     self_: *mut MyPaintRectangle)
// {
//     println!("mypaint_rectangle_copy called");
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_RECT: MyPaintRectangle = MyPaintRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };

    fn r(x: i32, y: i32, w: i32, h: i32) -> MyPaintRectangle {
        MyPaintRectangle {
            x: x,
            y: y,
            width: w,
            height: h,
        }
    }

    #[test]
    fn test_mypaint_rectangle() {
        let mut rect = EMPTY_RECT;
        rect.expand_to_include(5, 0);
        assert_eq!(rect, r(5, 0, 1, 1));
        rect.expand_to_include(0, 0);
        assert_eq!(rect, r(0, 0, 6, 1));
        rect.expand_to_include(100, 100);
        assert_eq!(rect, r(0, 0, 101, 101));
        rect.expand_to_include(-100, -100);
        assert_eq!(rect, r(-100, -100, 201, 201));
    }

    #[test]
    fn test_mypaint_rectangle_ffi() {
        unsafe {
            let rect = Box::into_raw(Box::new(r(5, 2, 6, 6)));
            mypaint_rectangle_expand_to_include_point(rect, 100, 0);
            assert_eq!(*Box::from_raw(rect), r(5, 0, 96, 8));
        }
    }
}
