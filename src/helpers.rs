use palette::{Hsv, Rgb, Hsl};

#[no_mangle]
pub unsafe extern fn hsl_to_rgb_float(
    h_: *mut f32,
    s_: *mut f32,
    l_: *mut f32)
{
    let h = *h_ - (*h_).floor();
    let s = (*s_).min(1.0).max(0.0);
    let l = (*l_).min(1.0).max(0.0);

    let rgb: Rgb = Hsl::new((h * 360.0).into(), s, l).into();

    *h_ = rgb.red;
    *s_ = rgb.green;
    *l_ = rgb.blue;
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

    let hsl: Hsl = Rgb::new(r, g, b).into();

    *r_ = hsl.hue.to_positive_degrees() / 360.0;
    *g_ = hsl.saturation;
    *b_ = hsl.lightness;
}

#[no_mangle]
pub unsafe extern fn hsv_to_rgb_float(
    h_: *mut f32,
    s_: *mut f32,
    v_: *mut f32)
{
    let h = *h_ - (*h_).floor();
    let s = (*s_).max(0.0).min(1.0);
    let v = (*v_).max(0.0).min(1.0);

    let rgb: Rgb = Hsv::new((h * 360.0).into(), s, v).into();

    *h_ = rgb.red;
    *s_ = rgb.green;
    *v_ = rgb.blue;
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

    let hsv: Hsv = Rgb::new(r, g, b).into();

    *r_ = hsv.hue.to_positive_degrees() / 360.0;
    *g_ = hsv.saturation;
    *b_ = hsv.value;
}
