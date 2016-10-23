// everything is swamped with unused var warnings so this hides it for now
#![allow(unused_variables)]

use mypaint_brush_settings_gen::MyPaintBrushSetting;
use mypaint_brush_settings_gen::MyPaintBrushState;
use mypaint_brush_settings_gen::MyPaintBrushInput::*;
use mypaint_brush_settings::MyPaintBrushSettingInfo;
use mypaint_mapping::*;
use mypaint_surface::*;
use rng_double::*;
use helpers::*;

// note: in the c code these are enum variants from mypaint_brush_settings_gen
const MYPAINT_BRUSH_INPUTS_COUNT: usize = 9;
const MYPAINT_BRUSH_SETTINGS_COUNT: usize = 45;
const MYPAINT_BRUSH_STATES_COUNT: usize = 30;

const ACTUAL_RADIUS_MIN: f32 = 0.2;
const ACTUAL_RADIUS_MAX: f32 = 1000.0;

const TAU: f32 = 6.2831853071;

pub struct MyPaintBrush {
    print_inputs: bool,
    stroke_total_painting_time: f64,
    stroke_current_idling_time: f64,
    state: MyPaintBrushState<f32>,
    rng: *mut RngDouble,
    settings: MyPaintBrushSetting<MypaintMapping>,
    settings_value: MyPaintBrushSetting<f32>,

    speed_mapping_gamma: (f32, f32),
    speed_mapping_m: (f32, f32),
    speed_mapping_q: (f32, f32),

    reset_requested: bool,
    refcount: i32,
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_new() -> *mut MyPaintBrush {
    let brush = Box::into_raw(Box::new(MyPaintBrush {
        print_inputs: false,
        stroke_total_painting_time: 0.0,
        stroke_current_idling_time: 0.0,
        state: MyPaintBrushState::default(),
        rng: rng_double_new(1000),
        settings: MyPaintBrushSetting::default(),
        settings_value: MyPaintBrushSetting::default(),
        speed_mapping_gamma: (0.0, 0.0),
        speed_mapping_m: (0.0, 0.0),
        speed_mapping_q: (0.0, 0.0),
        reset_requested: true,
        refcount: 1,
    }));
    mypaint_brush_new_stroke(brush);
    settings_base_values_have_changed(brush);
    (*brush).reset_requested = true;
    brush
}

unsafe fn brush_free(
    self_: *mut MyPaintBrush)
{
    assert!(!self_.is_null());
    let self_ = Box::from_raw(self_);
    rng_double_free(self_.rng);
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_unref(
    self_: *mut MyPaintBrush)
{
    assert!(!self_.is_null());
    (*self_).refcount -= 1;
    if (*self_).refcount == 0 {
        brush_free(self_);
    }
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_ref(
    self_: *mut MyPaintBrush)
{
    assert!(!self_.is_null());
    (*self_).refcount += 1;
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_get_total_stroke_painting_time(
    self_: *mut MyPaintBrush)
    -> f64
{
    assert!(!self_.is_null());
    (*self_).stroke_total_painting_time
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_set_print_inputs(
    self_: *mut MyPaintBrush, enabled: bool)
{
    assert!(!self_.is_null());
    (*self_).print_inputs = enabled;
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_reset(
    self_: *mut MyPaintBrush)
{
    assert!(!self_.is_null());
    (*self_).reset_requested = true;
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_new_stroke(
    self_: *mut MyPaintBrush)
{
    assert!(!self_.is_null());
    (*self_).stroke_current_idling_time = 0.0;
    (*self_).stroke_total_painting_time = 0.0;
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_set_base_value(
    self_: *mut MyPaintBrush,
    id: u32,
    value: f32)
{
    assert!(!self_.is_null());
    assert!(id < MYPAINT_BRUSH_SETTINGS_COUNT as u32);
    (*self_).settings.int_to_state(id as usize).set_base_value(value);
    settings_base_values_have_changed(self_);
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_get_base_value(
    self_: *mut MyPaintBrush,
    id: u32)
    -> f32
{
    assert!(!self_.is_null());
    assert!(id < MYPAINT_BRUSH_SETTINGS_COUNT as u32);
    (*self_).settings.int_to_state(id as usize).get_base_value()
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_set_mapping_n(
    self_: *mut MyPaintBrush,
    id: u32,
    input: u32,
    n: i32)
{
    assert!(!self_.is_null());
    assert!(id < MYPAINT_BRUSH_SETTINGS_COUNT as u32);
    (*self_).settings.int_to_state(id as usize).set_n(input as usize, n as usize);
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_get_mapping_n(
    self_: *mut MyPaintBrush,
    id: u32,
    input: u32)
    -> i32
{
    assert!(!self_.is_null());
    assert!(id < MYPAINT_BRUSH_SETTINGS_COUNT as u32);
    (*self_).settings.int_to_state(id as usize).get_n(input as usize).unwrap() as i32
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_is_constant(
    self_: *mut MyPaintBrush,
    id: u32)
    -> bool
{
    assert!(!self_.is_null());
    assert!(id < MYPAINT_BRUSH_SETTINGS_COUNT as u32);
    (*self_).settings.int_to_state(id as usize).is_constant()
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_get_inputs_used_n(
    self_: *mut MyPaintBrush,
    id: u32)
    -> i32
{
    assert!(!self_.is_null());
    assert!(id < MYPAINT_BRUSH_SETTINGS_COUNT as u32);
    (*self_).settings.int_to_state(id as usize).get_inputs_used() as i32
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_set_mapping_point(
    self_: *mut MyPaintBrush,
    id: u32,
    input: u32,
    index: i32,
    x: f32,
    y: f32)
{
    assert!(!self_.is_null());
    assert!(id < MYPAINT_BRUSH_SETTINGS_COUNT as u32);
    (*self_).settings.int_to_state(id as usize).set_point(input as usize, index as usize, (x, y));
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_get_mapping_point(
    self_: *mut MyPaintBrush,
    id: u32,
    input: u32,
    index: i32,
    x: *mut f32,
    y: *mut f32)
{
    assert!(!self_.is_null());
    assert!(id < MYPAINT_BRUSH_SETTINGS_COUNT as u32);
    let p = (*self_).settings.int_to_state(id as usize).get_point(input as usize, index as usize);
    *x = p.0;
    *y = p.1;
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_get_state(
    self_: *mut MyPaintBrush,
    i: u32)
    -> f32
{
    assert!(!self_.is_null());
    assert!(i < MYPAINT_BRUSH_STATES_COUNT as u32);
    *(*self_).state.int_to_state(i as usize)
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_set_state(
    self_: *mut MyPaintBrush,
    i: u32,
    value: f32)
{
    assert!(!self_.is_null());
    assert!(i < MYPAINT_BRUSH_STATES_COUNT as u32);
    *(*self_).state.int_to_state(i as usize) = value;
}

fn smallest_angular_difference(a: f32, b: f32) -> f32 {
    let a = a % 360.0;
    let b = b % 360.0;
    let (d_cw, d_ccw) = if a > b {
        (a - b, b + 360.0 - a)
    } else {
        (a + 360.0 - b, b - a)
    };
    if d_cw < d_ccw {
        -d_cw
    } else {
        d_ccw
    }
}

fn exp_decay(T_const: f32, t: f32) -> f32 {
    if T_const <= 0.001 {
        0.0
    } else {
        (-t / T_const).exp()
    }
}

#[no_mangle]
pub unsafe extern fn settings_base_values_have_changed(
    self_: *mut MyPaintBrush)
{
    // horrible, but better than the loop that used to be here
    fn precalc_with_gamma(gamma: f32) -> (f32, f32, f32) {
        let c1 = (45.0 + gamma).ln();
        let m = 0.015 * (45.0 + gamma);
        let q = 0.5 - m * c1;
        (gamma, m, q)
    }
    assert!(!self_.is_null());
    let self_ = &mut *self_;

    let (gamma0, m0, q0) = precalc_with_gamma(
        self_.settings.speed1_gamma.get_base_value());

    let (gamma1, m1, q1) = precalc_with_gamma(
        self_.settings.speed2_gamma.get_base_value());

    self_.speed_mapping_gamma = (gamma0, gamma1);
    self_.speed_mapping_m = (m0, m1);
    self_.speed_mapping_q = (q0, q1);
}

#[no_mangle]
pub unsafe extern fn update_states_and_setting_values(
    self_: *mut MyPaintBrush,
    step_ddab: f32,
    step_dx: f32,
    step_dy: f32,
    step_dpressure: f32,
    step_declination: f32,
    step_ascension: f32,
    mut step_dtime: f32)
{
    assert!(!self_.is_null());
    let self_ = &mut *self_;
    if step_dtime < 0.0 {
        step_dtime = 0.001;
        println!("time is running backwards!");
    } else if step_dtime == 0.0 {
        step_dtime = 0.001;
    }

    self_.state.x += step_dx;
    self_.state.y += step_dy;
    self_.state.pressure += step_dpressure;

    self_.state.declination += step_declination;
    self_.state.ascension += step_ascension;

    let base_radius = self_.settings.radius_logarithmic.get_base_value()
        .exp();

    if self_.state.pressure <= 0.0 {
        self_.state.pressure = 0.0;
    }
    let pressure = self_.state.pressure;

    {
        let base_threshold = self_.settings.stroke_threshold.get_base_value();

        if self_.state.stroke_started == 0.0 {
            if pressure > base_threshold + 0.0001 {
                self_.state.stroke_started = 1.0;
                self_.state.stroke = 0.0;
            }
        } else {
            if pressure <= base_threshold * 0.9 + 0.0001 {
                self_.state.stroke_started = 0.0;
            }
        }
    }

    let norm_dx = step_dx / step_dtime / base_radius;
    let norm_dy = step_dy / step_dtime / base_radius;
    let norm_speed = (norm_dx*norm_dx + norm_dy*norm_dy).sqrt();
    let norm_dist = norm_speed * step_dtime;

    let inputs = [
        pressure * self_.settings.pressure_gain_log.get_base_value().exp(),
        (self_.speed_mapping_gamma.0 + self_.state.norm_speed1_slow).ln()
            * self_.speed_mapping_m.0 + self_.speed_mapping_q.1,
        (self_.speed_mapping_gamma.1 + self_.state.norm_speed2_slow).ln()
            * self_.speed_mapping_m.1 + self_.speed_mapping_q.1,
        rng_double_next(self_.rng) as f32,
        self_.state.stroke.min(1.0),
        {
            let dx = self_.state.direction_dx;
            let dy = self_.state.direction_dy;
            (dx.atan2(dy) / TAU * 360.0 + 180.0) % 180.0
        },
        self_.state.declination,
        ((self_.state.ascension + 180.0) % 360.0) - 180.0,
        self_.state.custom_input
    ];

    for i in 0..MYPAINT_BRUSH_SETTINGS_COUNT {
        *self_.settings_value.int_to_state(i) =
            self_.settings.int_to_state(i).calculate(&inputs);
    }

    {
        let fac = 1.0 - exp_decay(self_.settings_value.slow_tracking_per_dab, step_ddab);
        self_.state.actual_x += (self_.state.x - self_.state.actual_x) * fac;
        self_.state.actual_y += (self_.state.y - self_.state.actual_y) * fac;
    }

    {
        let fac = 1.0 - exp_decay(self_.settings_value.speed1_slowness, step_dtime);
        self_.state.norm_speed1_slow +=
            (norm_speed - self_.state.norm_speed1_slow) * fac;
        let fac = 1.0 - exp_decay(self_.settings_value.speed2_slowness, step_dtime);
        self_.state.norm_speed2_slow +=
            (norm_speed - self_.state.norm_speed2_slow) * fac;
    }

    {
        let mut time_constant = (
            self_.settings_value.offset_by_speed_slowness*0.01)
            .exp() - 1.0;
        time_constant = time_constant.max(0.002);
        let fac = 1.0 - exp_decay(time_constant, step_dtime);
        self_.state.norm_dx_slow +=
            (norm_dx - self_.state.norm_dx_slow) * fac;
        self_.state.norm_dy_slow +=
            (norm_dy - self_.state.norm_dy_slow) * fac;
    }

    {
        let mut dx = step_dx / base_radius;
        let mut dy = step_dy / base_radius;
        let step_in_dabtime = (dx*dx + dy*dy).sqrt();
        let fac = 1.0 - exp_decay(
            (self_.settings_value.direction_filter*0.5).exp() - 1.0,
            step_in_dabtime);

        let dx_old = self_.state.direction_dx;
        let dy_old = self_.state.direction_dy;
        if sq(dx_old-dx) + sq(dy_old-dy) > sq(dx_old+dx) + sq(dy_old+dy) {
            dx = -dx;
            dy = -dy;
        }
        self_.state.direction_dx += (dx - self_.state.direction_dx) * fac;
        self_.state.direction_dy += (dy - self_.state.direction_dy) * fac;
    }

    {
        let fac = 1.0 - exp_decay(self_.settings_value.custom_input_slowness, 0.1);
        self_.state.custom_input +=
            (self_.settings_value.custom_input
             - self_.state.custom_input)
            * fac;
    }

    {
        let frequency = (-self_.settings_value.stroke_duration_logarithmic).exp();
        self_.state.stroke +=
            norm_dist * frequency;
        self_.state.stroke =
            self_.state.stroke.max(0.0);
        let wrap = 1.0 + self_.settings_value.stroke_holdtime;

        if self_.state.stroke > wrap {
            self_.state.stroke = if wrap > 10.9 {
                1.0
            } else {
                (self_.state.stroke % wrap).max(0.0)
            }
        }
    }

    let radius_log = self_.settings_value.radius_logarithmic;
    self_.state.actual_radius =
        radius_log.exp().min(ACTUAL_RADIUS_MAX).max(ACTUAL_RADIUS_MIN);

    self_.state.actual_elliptical_dab_ratio =
        self_.settings_value.elliptical_dab_ratio;
    self_.state.actual_elliptical_dab_angle =
        self_.settings_value.elliptical_dab_angle;
}

fn sq(x: f32) -> f32 {
    x*x
}

#[no_mangle]
pub unsafe extern fn prepare_and_draw_dab(
    self_: *mut MyPaintBrush,
    surface: *mut MyPaintSurface)
    -> bool
{
    assert!(!self_.is_null());
    let self_ = &mut *self_;
    self_.settings_value.opaque =
        self_.settings_value.opaque.max(0.0);

    let mut opaque = self_.settings_value.opaque
        * self_.settings_value.opaque_multiply;
    opaque = opaque.min(1.0).max(0.0);

    if self_.settings_value.opaque_linearize != 0.0 {
        let mut dabs_per_pixel = (
            self_.settings.dabs_per_actual_radius.get_base_value()
            + self_.settings.dabs_per_basic_radius.get_base_value()
        ) * 2.0;

        dabs_per_pixel = dabs_per_pixel.max(1.0);
        dabs_per_pixel = 1.0 + self_.settings.opaque_linearize.get_base_value()
            * (dabs_per_pixel - 1.0);

        let alpha = opaque;
        let beta = 1.0 - alpha;
        let beta_dab = beta.powf(1.0 / dabs_per_pixel);
        let alpha_dab = 1.0 - beta_dab;
        opaque = alpha_dab;
    }
    let mut x = self_.state.actual_x;
    let mut y = self_.state.actual_y;

    let base_radius = self_.settings.radius_logarithmic.get_base_value().exp();

    if self_.settings_value.offset_by_speed != 0.0 {
        let mult = self_.settings_value.offset_by_speed * 0.1 * base_radius;
        x += self_.state.norm_dx_slow * mult;
        y += self_.state.norm_dy_slow * mult;
    }

    {
        let mut amp = self_.settings_value.offset_by_random;
        if amp != 0.0 {
            amp = amp.max(0.0);
            x += rand_gauss(self_.rng) * amp * base_radius;
            y += rand_gauss(self_.rng) * amp * base_radius;
        }
    }

    let mut radius = self_.state.actual_radius;

    if self_.settings_value.radius_by_random != 0.0 {
        let mut radius_log = self_.settings_value.radius_logarithmic;
        radius_log += rand_gauss(self_.rng)
            * self_.settings_value.radius_by_random;

        radius = radius_log.exp().min(ACTUAL_RADIUS_MAX).max(ACTUAL_RADIUS_MIN);

        let alpha_correction = sq(self_.state.actual_radius / radius);
        if alpha_correction <= 1.0 {
            opaque *= alpha_correction;
        }
    }

    if self_.settings_value.smudge_length < 1.0
        && (self_.settings_value.smudge != 0.0
            || !self_.settings.smudge.is_constant())
    {
        let mut fac = self_.settings_value.smudge_length
            .max(0.01);
        let px = x.round();
        let py = y.round();

        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        let mut a = 0.0;

        self_.state.last_getcolor_recentness *= fac;
        if self_.state.last_getcolor_recentness < 0.5*fac {
            if self_.state.last_getcolor_recentness == 0.0 {
                fac = 0.0;
            }
            self_.state.last_getcolor_recentness = 1.0;

            let mut smudge_radius =
                radius * self_.settings_value.smudge_radius_log.exp();
            smudge_radius = smudge_radius.min(1.0).max(0.0);
            mypaint_surface_get_color(surface, px, py, smudge_radius,
                &mut r as *mut _,
                &mut g as *mut _,
                &mut b as *mut _,
                &mut a as *mut _);
            self_.state.last_getcolor_r = r;
            self_.state.last_getcolor_g = g;
            self_.state.last_getcolor_b = b;
            self_.state.last_getcolor_a = a;
        } else {
            r = self_.state.last_getcolor_r;
            g = self_.state.last_getcolor_g;
            b = self_.state.last_getcolor_b;
            a = self_.state.last_getcolor_a;
        }

        self_.state.smudge_a =
            (fac*self_.state.smudge_a + (1.0-fac)*a)
            .min(1.0).max(0.0);
        self_.state.smudge_ra =
            fac*self_.state.smudge_ra + (1.0-fac)*r*a;
        self_.state.smudge_ga =
            fac*self_.state.smudge_ga + (1.0-fac)*g*a;
        self_.state.smudge_ba =
            fac*self_.state.smudge_ba + (1.0-fac)*b*a;
    }

    let mut color_h = self_.settings.color_h.get_base_value();
    let mut color_s = self_.settings.color_s.get_base_value();
    let mut color_v = self_.settings.color_v.get_base_value();
    let mut eraser_target_alpha = 1.0;

    if self_.settings_value.smudge > 0.0 {
        hsv_to_rgb_float(
            &mut color_h as *mut _,
            &mut color_s as *mut _,
            &mut color_v as *mut _);
        let fac = self_.settings_value.smudge
            .min(1.0);
        eraser_target_alpha = ((1.0-fac) + fac*self_.state.smudge_a)
            .min(1.0).max(0.0);
        if eraser_target_alpha > 0.0 {
            color_h = (fac*self_.state.smudge_ra + (1.0-fac)*color_h) / eraser_target_alpha;
            color_s = (fac*self_.state.smudge_ga + (1.0-fac)*color_s) / eraser_target_alpha;
            color_v = (fac*self_.state.smudge_ba + (1.0-fac)*color_v) / eraser_target_alpha;
        } else {
            color_h = 1.0;
            color_s = 0.0;
            color_v = 0.0;
        }
        rgb_to_hsv_float(
            &mut color_h as *mut _,
            &mut color_s as *mut _,
            &mut color_v as *mut _);
    }

    eraser_target_alpha *= 1.0 - self_.settings_value.eraser;

    color_h += self_.settings_value.change_color_h;
    color_s += self_.settings_value.change_color_hsv_s;
    color_v += self_.settings_value.change_color_v;

    if self_.settings_value.change_color_l != 0.0
        || self_.settings_value.change_color_hsl_s != 0.0
    {
        hsv_to_rgb_float(
            &mut color_h as *mut _,
            &mut color_s as *mut _,
            &mut color_v as *mut _);
        rgb_to_hsl_float(
            &mut color_h as *mut _,
            &mut color_s as *mut _,
            &mut color_v as *mut _);
        color_v += self_.settings_value.change_color_l;
        color_s += self_.settings_value.change_color_hsl_s;
        hsl_to_rgb_float(
            &mut color_h as *mut _,
            &mut color_s as *mut _,
            &mut color_v as *mut _);
        rgb_to_hsv_float(
            &mut color_h as *mut _,
            &mut color_s as *mut _,
            &mut color_v as *mut _);
    }

    let mut hardness = self_.settings_value.hardness
        .min(1.0).max(0.0);

    let current_fadeout_in_pixels = radius * (1.0 - hardness);
    let min_fadeout_in_pixels = self_.settings_value.anti_aliasing;

    if current_fadeout_in_pixels < min_fadeout_in_pixels {
        let current_optical_radius = radius - (1.0 - hardness)*radius/2.0;

        hardness = (current_optical_radius - (min_fadeout_in_pixels/2.0))
            / (current_optical_radius + (min_fadeout_in_pixels/2.0));
        radius = min_fadeout_in_pixels / (1.0 - hardness);
    }

    let snap_to_pixel = self_.settings_value.snap_to_pixel;
    if snap_to_pixel > 0.0 {
        let snapped_x = x.floor() + 0.5;
        let snapped_y = y.floor() + 0.5;
        x = x + (snapped_x - x) * snap_to_pixel;
        y = y + (snapped_y - y) * snap_to_pixel;

        let mut snapped_radius = (radius * 2.0).round() / 2.0;
        snapped_radius = snapped_radius.max(0.5);
        if snap_to_pixel > 0.9999 {
            snapped_radius -= 0.0001;
        }

        radius = radius + (snapped_radius - radius) * snap_to_pixel;
    }

    hsv_to_rgb_float(
        &mut color_h as *mut _,
        &mut color_s as *mut _,
        &mut color_v as *mut _);

    mypaint_surface_draw_dab(surface,
        x, y, radius,
        color_h, color_s, color_v,
        opaque, hardness,
        eraser_target_alpha,
        self_.state.actual_elliptical_dab_ratio,
        self_.state.actual_elliptical_dab_angle,
        self_.settings_value.lock_alpha,
        self_.settings_value.colorize) != 0
}

#[no_mangle]
pub unsafe extern fn count_dabs_to(
    self_: *mut MyPaintBrush,
    x: f32,
    y: f32,
    pressure: f32,
    dt: f32)
    -> f32
{
    let self_ = &mut *self_;

    {
        // just holding a ref to it cause we use it so much here
        let rad = &mut self_.state.actual_radius;
        if *rad == 0.0 {
            *rad = self_.settings.radius_logarithmic.get_base_value().exp();
        }
        *rad = rad.min(ACTUAL_RADIUS_MAX).max(ACTUAL_RADIUS_MIN);
    }

    let base_radius = self_.settings.radius_logarithmic.get_base_value()
        .exp().max(ACTUAL_RADIUS_MIN).min(ACTUAL_RADIUS_MAX);

    let xx = x - self_.state.x;
    let yy = y - self_.state.y;

    let dist = {
        let dab_ratio = &mut self_.state.actual_elliptical_dab_ratio;
        if *dab_ratio > 1.0 {
            let angle_rad = *dab_ratio / 360.0 * TAU;
            let (sn, cs) = angle_rad.sin_cos();
            ((yy*cs - xx*sn) * *dab_ratio).hypot(yy*sn + xx*cs)
        } else {
            xx.hypot(yy)
        }
    };

    let res1 = dist / self_.state.actual_radius *
        self_.settings.dabs_per_actual_radius.get_base_value();
    let res2 = dist / base_radius *
        self_.settings.dabs_per_basic_radius.get_base_value();
    let res3 = dt *
        self_.settings.dabs_per_second.get_base_value();
    res1 + res2 + res3
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_stroke_to(
    self_: *mut MyPaintBrush,
    surface: *mut MyPaintSurface,
    mut x: f32,
    mut y: f32,
    mut pressure: f32,
    mut xtilt: f32,
    mut ytilt: f32,
    mut dtime: f64)
    -> i64
{
    let self_ = &mut *self_;

    let mut tilt_ascension = 0.0;
    let mut tilt_declination = 90.0;
    if xtilt != 0.0 || ytilt != 0.0 {
        xtilt = xtilt.min(1.0).max(-1.0);
        ytilt = ytilt.min(1.0).max(-1.0);

        tilt_ascension = 360.0 * (-xtilt).atan2(ytilt) / TAU;
        let rad = xtilt.hypot(ytilt);
        tilt_declination = 90.0 - rad*60.0;

        assert!(tilt_ascension.is_finite() && tilt_declination.is_finite());
    }

    pressure = pressure.max(0.0);
    if !x.is_finite() || !y.is_finite()
        || x.abs() > 1e10 || y.abs() > 1e10
    {
        // workaround attempt for https://gna.org/bugs/?14372
        x = 0.0;
        y = 0.0;
        pressure = 0.0;
    }

    assert!(x.abs() < 1e8 && y.abs() < 1e8);

    // avoid div by zero
    if dtime <= 0.0 {
        dtime = 0.0001;
    }

    if dtime > 0.1 && pressure != 0.0 && self_.state.pressure == 0.0 {
        // workaround for tablets that don't report motion without pressure
        mypaint_brush_stroke_to(self_ as *mut _, surface, x, y, 0.0, 9.0, 0.0, dtime - 0.0001);
        dtime = 0.0001;
    }

    {
        let tracking_noise = self_.settings.tracking_noise.get_base_value();
        if tracking_noise != 0.0 {
            let base_radius = self_.settings.radius_logarithmic.get_base_value()
                .exp();
            x += rand_gauss(self_.rng) * tracking_noise * base_radius;
            y += rand_gauss(self_.rng) * tracking_noise * base_radius;
        }
        let fac = 1.0 - exp_decay(
            self_.settings.slow_tracking.get_base_value(),
            100.0 * dtime as f32);
        let sx = self_.state.x;
        let sy = self_.state.y;
        x = sx + (x - sx)*fac;
        y = sy + (y - sy)*fac;
    }

    let mut dabs_moved = self_.state.partial_dabs;
    let mut dabs_todo = count_dabs_to(self_ as *mut _, x, y, pressure, dtime as f32);

    if dtime > 5.0 || self_.reset_requested {
        self_.reset_requested = false;

        self_.state = MyPaintBrushState::default();

        self_.state.x = x;
        self_.state.y = y;
        self_.state.pressure = pressure;

        self_.state.actual_x = x;
        self_.state.actual_y = y;
        self_.state.stroke = 1.0;

        return 1;
    }

    // enum { UNKNOWN, YES, NO } painted = UNKNOWN;
    const UNKNOWN: u8 = 0;
    const YES: u8 = 1;
    const NO: u8 = 2;
    let mut painted = UNKNOWN;

    let mut dtime_left = dtime;

    let mut step_ddab;
    let mut step_dx;
    let mut step_dy;
    let mut step_dpressure;
    let mut step_dtime;
    let mut step_declination;
    let mut step_ascension;
    while dabs_moved + dabs_todo >= 1.0 {
        if dabs_moved > 0.0 {
            step_ddab = 1.0 - dabs_moved;
            dabs_moved = 0.0;
        } else {
            step_ddab = 1.0;
        }
        let frac = step_ddab / dabs_todo;
        step_dx = frac * (x - self_.state.x);
        step_dy = frac * (y - self_.state.y);
        step_dpressure = frac * (pressure - self_.state.pressure);
        step_dtime = frac as f64 * dtime_left;

        step_declination = frac * (tilt_declination - self_.state.declination);
        step_ascension = frac * smallest_angular_difference(self_.state.ascension, tilt_ascension);

        update_states_and_setting_values(self_ as *mut _,
            step_ddab, step_dx, step_dy, step_dpressure,
            step_declination, step_ascension, step_dtime as f32);
        let painted_now = prepare_and_draw_dab(self_ as *mut _, surface);
        if painted_now {
            painted = YES;
        } else {
            painted = NO;
        }

        dtime_left -= step_dtime;
        dabs_todo = count_dabs_to(self_ as *mut _, x, y, pressure, dtime_left as f32);
    }

    step_ddab = dabs_todo;
    step_dx = x - self_.state.x;
    step_dy = y - self_.state.y;
    step_dpressure = pressure - self_.state.pressure;
    step_declination = tilt_declination - self_.state.declination;
    step_ascension = smallest_angular_difference(self_.state.ascension, tilt_ascension);
    step_dtime = dtime_left;

    update_states_and_setting_values(self_ as *mut _,
        step_ddab, step_dx, step_dy, step_dpressure,
        step_declination, step_ascension, step_dtime as f32);

    self_.state.partial_dabs = dabs_moved + dabs_todo;

    if painted == UNKNOWN {
        painted = if self_.stroke_current_idling_time > 0.0 || self_.stroke_total_painting_time == 0.0 {
            NO
        } else {
            YES
        }
    }
    if painted == YES {
        self_.stroke_total_painting_time += dtime;
        self_.stroke_current_idling_time = 0.0;

        if self_.stroke_total_painting_time > (4.0 + 3.0 * pressure) as f64 {
            if step_dpressure >= 0.0 {
                return 1;
            }
        }
    } else {
        self_.stroke_current_idling_time += dtime;
        if self_.stroke_total_painting_time == 0.0 {
            if self_.stroke_current_idling_time > 1.0 {
                return 1;
            }
        } else {
            if self_.stroke_total_painting_time + self_.stroke_current_idling_time > (0.9 + 5.0 * pressure) as f64 {
                return 1;
            }
        }
    }
    0
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_from_string(
    self_: *mut MyPaintBrush,
    string: *const u8)
    -> bool
{
    false
}

// temp definition, should be in mypaint-brush-settings
#[link(name = "mypaint")]
extern {
    fn mypaint_brush_setting_info(s: u32) -> *const MyPaintBrushSettingInfo;
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_from_defaults(
    self_: *mut MyPaintBrush)
{
    assert!(!self_.is_null());
    let self_ = &mut *self_;
    for s in 0..MYPAINT_BRUSH_SETTINGS_COUNT as u32 {
        for i in 0..MYPAINT_BRUSH_INPUTS_COUNT as u32 {
            mypaint_brush_set_mapping_n(self_ as *mut _, s, i, 0);
        }

        let def = (*mypaint_brush_setting_info(s)).def;
        mypaint_brush_set_base_value(self_ as *mut _, s, def);
    }

    self_.settings.opaque_multiply.set_n(MYPAINT_BRUSH_INPUT_PRESSURE as usize, 2);
    self_.settings.opaque_multiply.set_point(MYPAINT_BRUSH_INPUT_PRESSURE as usize, 0, (0.0, 0.0));
    self_.settings.opaque_multiply.set_point(MYPAINT_BRUSH_INPUT_PRESSURE as usize, 1, (1.0, 1.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_minimum() {
        unsafe {
            let brush = mypaint_brush_new();
        }
    }
}
