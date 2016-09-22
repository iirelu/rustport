use std::cmp;

#[no_mangle]
pub unsafe extern fn draw_dab_pixels_BlendMode_Normal(
    mut mask: *mut u16,
    mut rgba: *mut u16,
    color_r: u16,
    color_g: u16,
    color_b: u16,
    opacity: u16)
{
    loop {
        while *mask != 0 {
            let opa_a = *mask as u32 * opacity as u32 >> 15;
            let opa_b = (1<<15)-opa_a;
            *rgba.offset(3) = (opa_a + (opa_b * *rgba.offset(3) as u32 >> 15)) as u16;
            *rgba.offset(0) = ((opa_a * color_r as u32 + opa_b * *rgba.offset(0) as u32) >> 15) as u16;
            *rgba.offset(1) = ((opa_a * color_g as u32 + opa_b * *rgba.offset(1) as u32) >> 15) as u16;
            *rgba.offset(2) = ((opa_a * color_b as u32 + opa_b * *rgba.offset(2) as u32) >> 15) as u16;
            mask = mask.offset(1);
            rgba = rgba.offset(4);
        }
        if *mask.offset(1) == 0 {
            break;
        }
        rgba = rgba.offset(*mask.offset(1) as isize);
        mask = mask.offset(2);
    }
}

const LUMA_RED_COEFF: f32 = 0.3 * (1<<15) as f32;
const LUMA_GREEN_COEFF: f32 = 0.59 * (1<<15) as f32;
const LUMA_BLUE_COEFF: f32 = 0.11 * (1<<15) as f32;

fn luma(r: u32, g: u32, b: u32) -> u32 {
    r * LUMA_RED_COEFF as u32
        + g * LUMA_GREEN_COEFF as u32
        + b * LUMA_BLUE_COEFF as u32
}

unsafe fn set_rgb16_lum_from_rgb16(
    topr: u16,
    topg: u16,
    topb: u16,
    botr: *mut u16,
    botg: *mut u16,
    botb: *mut u16)
{
    let botlum = (luma(*botr as u32, *botg as u32, *botb as u32) >> 15) as u16;
    let toplum = (luma(topr as u32, topg as u32, topb as u32) >> 15) as u16;
    let diff = botlum as i16 - toplum as i16;
    let mut r = topr as i32 + diff as i32;
    let mut g = topg as i32 + diff as i32;
    let mut b = topb as i32 + diff as i32;

    let lum = (r * LUMA_RED_COEFF as i32
               + g * LUMA_GREEN_COEFF as i32
               + b * LUMA_BLUE_COEFF as i32) >> 15;
    let cmin = cmp::min(r, cmp::min(g, b));
    let cmax = cmp::max(r, cmp::max(g, b));
    if cmin < 0 {
        r = lum + ((r - lum) * lum) / (lum - cmin);
        g = lum + ((g - lum) * lum) / (lum - cmin);
        b = lum + ((b - lum) * lum) / (lum - cmin);
    }
    if cmax > (1<<15) {
        r = lum + ((r - lum) * ((1<<15) - lum)) / (cmax - lum);
        g = lum + ((g - lum) * ((1<<15) - lum)) / (cmax - lum);
        b = lum + ((b - lum) * ((1<<15) - lum)) / (cmax - lum);
    }
    debug_assert!((0 <= r) && (r <= (1 << 15)));
    debug_assert!((0 <= g) && (g <= (1 << 15)));
    debug_assert!((0 <= b) && (b <= (1 << 15)));

    *botr = r as u16;
    *botg = g as u16;
    *botb = b as u16;
}

#[no_mangle]
pub unsafe extern fn draw_dab_pixels_BlendMode_Color(
    mut mask: *mut u16,
    mut rgba: *mut u16,
    color_r: u16,
    color_g: u16,
    color_b: u16,
    opacity: u16)
{
    loop {
        while *mask != 0 {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let a = *rgba.offset(3);
            if a != 0 {
                r = (((*rgba.offset(0) as u32) << 15) / a as u32) as u16;
                g = (((*rgba.offset(1) as u32) << 15) / a as u32) as u16;
                b = (((*rgba.offset(2) as u32) << 15) / a as u32) as u16;
            }
            set_rgb16_lum_from_rgb16(
                color_r, color_g, color_b,
                &mut r as *mut u16, &mut g as *mut u16, &mut b as *mut u16);
            r = (r as u32 * a as u32 >> 15) as u16;
            g = (g as u32 * a as u32 >> 15) as u16;
            b = (b as u32 * a as u32 >> 15) as u16;

            let opa_a = *mask as u32 * opacity as u32 >> 15;
            let opa_b = (1<<15)-opa_a;
            *rgba.offset(0) = ((opa_a * r as u32 + opa_b * *rgba.offset(0) as u32) >> 15) as u16;
            *rgba.offset(1) = ((opa_a * g as u32 + opa_b * *rgba.offset(1) as u32) >> 15) as u16;
            *rgba.offset(2) = ((opa_a * b as u32 + opa_b * *rgba.offset(2) as u32) >> 15) as u16;

            mask = mask.offset(1);
            rgba = rgba.offset(4);
        }
        if *mask.offset(1) == 0 {
            break;
        }
        rgba = rgba.offset(*mask.offset(1) as isize);
        mask = mask.offset(2);
    }
}

#[no_mangle]
pub unsafe extern fn draw_dab_pixels_BlendMode_Normal_and_Eraser(
    mut mask: *mut u16,
    mut rgba: *mut u16,
    color_r: u16,
    color_g: u16,
    color_b: u16,
    color_a: u16,
    opacity: u16)
{
    loop {
        while *mask != 0 {
            let mut opa_a = *mask as u32 * opacity as u32 >> 15;
            let opa_b = (1<<15) - opa_a;
            opa_a = opa_a * color_a as u32 >> 15;
            *rgba.offset(3) = (opa_a + (opa_b * *rgba.offset(3) as u32 >> 15)) as u16;
            *rgba.offset(0) = ((opa_a * color_r as u32 + opa_b * *rgba.offset(0) as u32) >> 15) as u16;
            *rgba.offset(1) = ((opa_a * color_g as u32 + opa_b * *rgba.offset(1) as u32) >> 15) as u16;
            *rgba.offset(2) = ((opa_a * color_b as u32 + opa_b * *rgba.offset(2) as u32) >> 15) as u16;
            mask = mask.offset(1);
            rgba = rgba.offset(4);
        }
        if *mask.offset(1) == 0 {
            break;
        }
        rgba = rgba.offset(*mask.offset(1) as isize);
        mask = mask.offset(2);
    }
}

#[no_mangle]
pub unsafe extern fn draw_dab_pixels_BlendMode_LockAlpha(
    mut mask: *mut u16,
    mut rgba: *mut u16,
    color_r: u16,
    color_g: u16,
    color_b: u16,
    opacity: u16)
{
    loop {
        while *mask != 0 {
            let mut opa_a = *mask as u32 * opacity as u32 >> 15;
            // no idea if this is how this works or if c does this implicitly
            opa_a = ::std::cmp::min(opa_a, 1<<15);
            let opa_b = (1<<15) - opa_a;

            opa_a *= *rgba.offset(3) as u32;
            opa_a >>= 15;

            *rgba.offset(0) = ((opa_a*color_r as u32 + opa_b * *rgba.offset(0) as u32) >> 15) as u16;
            *rgba.offset(1) = ((opa_a*color_g as u32 + opa_b * *rgba.offset(1) as u32) >> 15) as u16;
            *rgba.offset(2) = ((opa_a*color_b as u32 + opa_b * *rgba.offset(2) as u32) >> 15) as u16;

            mask = mask.offset(1);
            rgba = rgba.offset(4);
        }
        if *mask.offset(1) == 0 {
            break;
        }
        rgba = rgba.offset(*mask.offset(1) as isize);
        mask = mask.offset(2);
    }
}

#[no_mangle]
pub unsafe extern fn get_color_pixels_accumulate(
    mut mask: *mut u16,
    mut rgba: *mut u16,
    sum_weight: *mut f32,
    sum_r: *mut f32,
    sum_g: *mut f32,
    sum_b: *mut f32,
    sum_a: *mut f32)
{
    let mut weight = 0;
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut a = 0;
    loop {
        while *mask != 0 {
            let opa = *mask as u32;
            weight += opa;
            r += opa * *rgba.offset(0) as u32 >> 15;
            g += opa * *rgba.offset(1) as u32 >> 15;
            b += opa * *rgba.offset(2) as u32 >> 15;
            a += opa * *rgba.offset(3) as u32 >> 15;
            mask = mask.offset(1);
            rgba = rgba.offset(4);
        }
        if *mask.offset(1) == 0 {
            break;
        }
        rgba = rgba.offset(*mask.offset(1) as isize);
        mask = mask.offset(2);
    }

    *sum_weight += weight as f32;
    *sum_r += r as f32;
    *sum_g += g as f32;
    *sum_b += b as f32;
    *sum_a += a as f32;
}
