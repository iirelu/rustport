use rng_double::{RngDouble, rng_double_next};

fn hsl_value(n1: f32, n2: f32, mut hue: f32) -> f32 {
    if hue > 6.0 {
        hue -= 6.0;
    } else if hue < 0.0 {
        hue += 6.0;
    }

    if hue < 1.0 {
        n1 + (n2 - n1) * hue
    } else if hue < 3.0 {
        n2
    } else if hue < 4.0 {
        n1 + (n2 - n1) * (4.0 - hue)
    } else {
        n1
    }
}

#[no_mangle]
pub unsafe extern fn hsl_to_rgb_float(
    h_: *mut f32,
    s_: *mut f32,
    l_: *mut f32)
{
    let mut h = *h_;
    let mut s = *s_;
    let mut l = *l_;

    h = h - h.floor();
    s = s.min(1.0).max(0.0);
    l = l.min(1.0).max(0.0);

    let (r, g, b) = if s == 0.0 {
        (l, l, l)
    } else {
        let m2 = if l <= 0.5 {
            l * (1.0 - s)
        } else {
            l + s - l * s
        };

        let m1 = l * 2.0 - m2;

        (hsl_value(m1, m2, h * 6.0 + 2.0),
        hsl_value(m1, m2, h * 6.0),
        hsl_value(m1, m2, h * 6.0 - 2.0))
    };

    *h_ = r;
    *s_ = g;
    *l_ = b;
}

#[no_mangle]
pub unsafe extern fn rgb_to_hsl_float(
    r_: *mut f32,
    g_: *mut f32,
    b_: *mut f32)
{
    let r = (*r_).min(1.0).max(0.0);
    let g = (*g_).min(1.0).max(0.0);
    let b = (*b_).min(1.0).max(0.0);

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);

    let (mut h, mut s, l) = (0.0, 0.0, (max-min)/2.0);

    if max != min {
        s = if l <= 0.5 {
            (max - min) / (max + min)
        } else {
            (max - min) / (2.0 - max - min)
        };

        let delta = if max - min == 0.0 {
            1.0
        } else {
            max - min
        };

        h = if r == max {
            (g - b) / delta
        } else if g == max {
            2.0 + (b - r) / delta
        } else if b == max {
            4.0 + (r - g) / delta
        } else {
            unreachable!()
        };

        h /= 6.0;

        if h < 0.0 {
            h += 1.0;
        }
    }
    *r_ = h;
    *g_ = s;
    *b_ = l;
}

#[no_mangle]
pub unsafe extern fn hsv_to_rgb_float(
    h_: *mut f32,
    s_: *mut f32,
    v_: *mut f32)
{
    let mut h = *h_;
    let mut s = *s_;
    let mut v = *v_;

    h = h - h.floor();
    s = s.max(0.0).min(1.0);
    v = v.max(0.0).min(1.0);

    let (r, g, b) = if s == 0.0 {
        (v, v, v)
    } else {
        let mut hue = h;
        if hue == 1.0 {
            hue = 0.0;
        }
        hue *= 6.0;

        let i = hue as u32;
        let f = hue - i as f32;
        let w = v * (1.0 - s);
        let q = v * (1.0 - (s * f));
        let t = v * (1.0 - (s * (1.0 - f)));

        match i {
            0 => (v, t, w),
            1 => (q, v, w),
            2 => (w, v, t),
            3 => (w, q, v),
            4 => (t, w, v),
            5 => (v, w, q),
            _ => unreachable!()
        }
    };

    *h_ = r;
    *s_ = g;
    *v_ = b;
}

#[no_mangle]
pub unsafe extern fn rgb_to_hsv_float(
    r_: *mut f32,
    g_: *mut f32,
    b_: *mut f32)
{
    let r = (*r_).max(0.0).min(1.0);
    let g = (*g_).max(0.0).min(1.0);
    let b = (*b_).max(0.0).min(1.0);

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);

    let mut h = 0.0;
    let mut s = 0.0;
    let v = max;

    let delta = max - min;

    if delta > 0.0001 {
        s = delta / max;
        if r == max {
            h = (g - b) / delta;
            if h < 0.0 {
                h += 6.0;
            }
        } else if g == max {
            h = 2.0 + (b - r) / delta;
        } else {
            h = 4.0 + (r - g) / delta;
        }
        h /= 6.0;
    }

    *r_ = h;
    *g_ = s;
    *b_ = v;
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
