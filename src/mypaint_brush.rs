// everything is swamped with unused var warnings so this hides it for now
#![allow(unused_variables)]

use mypaint_brush_settings_gen::MyPaintBrushSetting::*;
use mypaint_brush_settings_gen::MyPaintBrushState::*;
use mypaint_brush_settings_gen::MyPaintBrushInput::*;
use mypaint_brush_settings::MyPaintBrushSettingInfo;
use mypaint_mapping::*;
use mypaint_surface::*;
use rng_double::*;
use helpers::*;
use std::ptr;

// note: in the c code these are enum variants from mypaint_brush_settings_gen
const MYPAINT_BRUSH_INPUTS_COUNT: usize = 9;
const MYPAINT_BRUSH_SETTINGS_COUNT: usize = 45;
const MYPAINT_BRUSH_STATES_COUNT: usize = 30;

const ACTUAL_RADIUS_MIN: f32 = 0.2;
const ACTUAL_RADIUS_MAX: f32 = 1000.0;

const TAU: f32 = 6.2831853071;

#[repr(C)]
pub struct MyPaintBrush {
    print_inputs: bool,
    stroke_total_painting_time: f64,
    stroke_current_idling_time: f64,
    states: [f32; MYPAINT_BRUSH_STATES_COUNT],
    rng: *mut RngDouble,
    settings: [*mut MypaintMapping; MYPAINT_BRUSH_SETTINGS_COUNT],
    settings_value: [f32; MYPAINT_BRUSH_SETTINGS_COUNT],

    speed_mapping_gamma: [f32; 2],
    speed_mapping_m: [f32; 2],
    speed_mapping_q: [f32; 2],

    reset_requested: bool,
    refcount: i32,
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_new() -> *mut MyPaintBrush {
    let settings = {
        let mut data = [ptr::null_mut(); MYPAINT_BRUSH_SETTINGS_COUNT];
        for elem in data.iter_mut() {
            *elem = mypaint_mapping_new(MYPAINT_BRUSH_INPUTS_COUNT as i32);
        }
        data
    };
    let brush = Box::into_raw(Box::new(MyPaintBrush {
        print_inputs: false,
        stroke_total_painting_time: 0.0,
        stroke_current_idling_time: 0.0,
        states: [0.0; MYPAINT_BRUSH_STATES_COUNT],
        rng: rng_double_new(1000),
        settings: settings,
        settings_value: [0.0; MYPAINT_BRUSH_SETTINGS_COUNT],
        speed_mapping_gamma: [0.0; 2],
        speed_mapping_m: [0.0; 2],
        speed_mapping_q: [0.0; 2],
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
    for elem in self_.settings.iter() {
        mypaint_mapping_free(*elem);
    }
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
    mypaint_mapping_set_base_value((*self_).settings[id as usize], value);
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
    mypaint_mapping_get_base_value((*self_).settings[id as usize])
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
    mypaint_mapping_set_n((*self_).settings[id as usize], input as i32, n);
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
    mypaint_mapping_get_n((*self_).settings[id as usize], input as i32)
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_is_constant(
    self_: *mut MyPaintBrush,
    id: u32)
    -> bool
{
    assert!(!self_.is_null());
    assert!(id < MYPAINT_BRUSH_SETTINGS_COUNT as u32);
    mypaint_mapping_is_constant((*self_).settings[id as usize])
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_get_inputs_used_n(
    self_: *mut MyPaintBrush,
    id: u32)
    -> i32
{
    assert!(!self_.is_null());
    assert!(id < MYPAINT_BRUSH_SETTINGS_COUNT as u32);
    mypaint_mapping_get_inputs_used_n((*self_).settings[id as usize])
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
    mypaint_mapping_set_point(
        (*self_).settings[id as usize], input as i32, index, x, y);
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
    mypaint_mapping_get_point(
        (*self_).settings[id as usize], input as i32, index, x, y);
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_get_state(
    self_: *mut MyPaintBrush,
    i: u32)
    -> f32
{
    assert!(!self_.is_null());
    assert!(i < MYPAINT_BRUSH_STATES_COUNT as u32);
    (*self_).states[i as usize]
}

#[no_mangle]
pub unsafe extern fn mypaint_brush_set_state(
    self_: *mut MyPaintBrush,
    i: u32,
    value: f32)
{
    assert!(!self_.is_null());
    assert!(i < MYPAINT_BRUSH_STATES_COUNT as u32);
    (*self_).states[i as usize] = value;
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
    assert!(!self_.is_null());
    let self_ = &mut *self_;
    for i in 0..2 {
        let gamma = {
            let index = if i == 0 {
                MYPAINT_BRUSH_SETTING_SPEED1_GAMMA as usize
            } else {
                MYPAINT_BRUSH_SETTING_SPEED2_GAMMA as usize
            };
            mypaint_mapping_get_base_value(self_.settings[index])
        };

        let fix1_x = 45.0;
        let fix1_y = 0.5;
        let fix2_x = 45.0;
        let fix2_dy = 0.015;

        let c1 = (fix1_x+gamma).ln();
        let m = fix2_dy * (fix2_x + gamma);
        let q = fix1_y - m*c1;

        self_.speed_mapping_gamma[i] = gamma;
        self_.speed_mapping_m[i] = m;
        self_.speed_mapping_q[i] = q;
    }
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

    self_.states[MYPAINT_BRUSH_STATE_X as usize] += step_dx;
    self_.states[MYPAINT_BRUSH_STATE_Y as usize] += step_dy;
    self_.states[MYPAINT_BRUSH_STATE_PRESSURE as usize] += step_dpressure;

    self_.states[MYPAINT_BRUSH_STATE_DECLINATION as usize] += step_declination;
    self_.states[MYPAINT_BRUSH_STATE_ASCENSION as usize] += step_ascension;

    let base_radius = mypaint_mapping_get_base_value(
        self_.settings[MYPAINT_BRUSH_SETTING_RADIUS_LOGARITHMIC as usize])
        .exp();

    if self_.states[MYPAINT_BRUSH_STATE_PRESSURE as usize] <= 0.0 {
        self_.states[MYPAINT_BRUSH_STATE_PRESSURE as usize] = 0.0;
    }
    let pressure = self_.states[MYPAINT_BRUSH_STATE_PRESSURE as usize];

    {
        let base_threshold = mypaint_mapping_get_base_value(
            self_.settings[MYPAINT_BRUSH_SETTING_STROKE_THRESHOLD as usize]);

        if self_.states[MYPAINT_BRUSH_STATE_STROKE_STARTED as usize] == 0.0 {
            if pressure > base_threshold + 0.0001 {
                self_.states[MYPAINT_BRUSH_STATE_STROKE_STARTED as usize] = 1.0;
                self_.states[MYPAINT_BRUSH_STATE_STROKE as usize] = 0.0;
            }
        } else {
            if pressure <= base_threshold * 0.9 + 0.0001 {
                self_.states[MYPAINT_BRUSH_STATE_STROKE_STARTED as usize] = 0.0;
            }
        }
    }

    let norm_dx = step_dx / step_dtime / base_radius;
    let norm_dy = step_dy / step_dtime / base_radius;
    let norm_speed = (norm_dx*norm_dx + norm_dy*norm_dy).sqrt();
    let norm_dist = norm_speed * step_dtime;

    let mut inputs = [
        pressure * mypaint_mapping_get_base_value(self_.settings[MYPAINT_BRUSH_SETTING_PRESSURE_GAIN_LOG as usize]).exp(),
        (self_.speed_mapping_gamma[0] + self_.states[MYPAINT_BRUSH_STATE_NORM_SPEED1_SLOW as usize]).ln()
            * self_.speed_mapping_m[0] + self_.speed_mapping_q[0],
        (self_.speed_mapping_gamma[1] + self_.states[MYPAINT_BRUSH_STATE_NORM_SPEED2_SLOW as usize]).ln()
            * self_.speed_mapping_m[1] + self_.speed_mapping_q[1],
        rng_double_next(self_.rng) as f32,
        self_.states[MYPAINT_BRUSH_STATE_STROKE as usize].min(1.0),
        {
            let dx = self_.states[MYPAINT_BRUSH_STATE_DIRECTION_DX as usize];
            let dy = self_.states[MYPAINT_BRUSH_STATE_DIRECTION_DY as usize];
            (dx.atan2(dy) / 6.283185 * 360.0 + 180.0) % 180.0
        },
        self_.states[MYPAINT_BRUSH_STATE_DECLINATION as usize],
        ((self_.states[MYPAINT_BRUSH_STATE_ASCENSION as usize] + 180.0) % 360.0) - 180.0,
        self_.states[MYPAINT_BRUSH_STATE_CUSTOM_INPUT as usize]
    ];

    for i in 0..MYPAINT_BRUSH_SETTINGS_COUNT {
        self_.settings_value[i] = mypaint_mapping_calculate(self_.settings[i], inputs.as_mut_ptr());
    }

    {
        let fac = 1.0 - exp_decay(self_.settings_value[MYPAINT_BRUSH_SETTING_SLOW_TRACKING_PER_DAB as usize], step_ddab);
        self_.states[MYPAINT_BRUSH_STATE_ACTUAL_X as usize] +=
            (self_.states[MYPAINT_BRUSH_STATE_X as usize] - self_.states[MYPAINT_BRUSH_STATE_ACTUAL_X as usize])
            * fac;
        self_.states[MYPAINT_BRUSH_STATE_ACTUAL_Y as usize] +=
            (self_.states[MYPAINT_BRUSH_STATE_Y as usize] - self_.states[MYPAINT_BRUSH_STATE_ACTUAL_Y as usize])
            * fac;
    }

    {
        let fac = 1.0 - exp_decay(self_.settings_value[MYPAINT_BRUSH_SETTING_SPEED1_SLOWNESS as usize], step_dtime);
        self_.states[MYPAINT_BRUSH_STATE_NORM_SPEED1_SLOW as usize] +=
            (norm_speed - self_.states[MYPAINT_BRUSH_STATE_NORM_SPEED1_SLOW as usize]) * fac;
        let fac = 1.0 - exp_decay(self_.settings_value[MYPAINT_BRUSH_SETTING_SPEED2_SLOWNESS as usize], step_dtime);
        self_.states[MYPAINT_BRUSH_STATE_NORM_SPEED2_SLOW as usize] +=
            (norm_speed - self_.states[MYPAINT_BRUSH_STATE_NORM_SPEED2_SLOW as usize]) * fac;
    }

    {
        let mut time_constant = (
            self_.settings_value[MYPAINT_BRUSH_SETTING_OFFSET_BY_SPEED_SLOWNESS as usize]*0.01)
            .exp() - 1.0;
        time_constant = time_constant.max(0.002);
        let fac = 1.0 - exp_decay(time_constant, step_dtime);
        self_.states[MYPAINT_BRUSH_STATE_NORM_DX_SLOW as usize] +=
            (norm_dx - self_.states[MYPAINT_BRUSH_STATE_NORM_DX_SLOW as usize]) * fac;
        self_.states[MYPAINT_BRUSH_STATE_NORM_DY_SLOW as usize] +=
            (norm_dy - self_.states[MYPAINT_BRUSH_STATE_NORM_DY_SLOW as usize]) * fac;
    }

    {
        let mut dx = step_dx / base_radius;
        let mut dy = step_dy / base_radius;
        let step_in_dabtime = (dx*dx + dy*dy).sqrt();
        let fac = 1.0 - exp_decay(
            (self_.settings_value[MYPAINT_BRUSH_SETTING_DIRECTION_FILTER as usize]*0.5).exp() - 1.0,
            step_in_dabtime);

        let dx_old = self_.states[MYPAINT_BRUSH_STATE_DIRECTION_DX as usize];
        let dy_old = self_.states[MYPAINT_BRUSH_STATE_DIRECTION_DY as usize];
        if sq(dx_old-dx) + sq(dy_old-dy) > sq(dx_old+dx) + sq(dy_old+dy) {
            dx = -dx;
            dy = -dy;
        }
        self_.states[MYPAINT_BRUSH_STATE_DIRECTION_DX as usize] +=
            (dx - self_.states[MYPAINT_BRUSH_STATE_DIRECTION_DX as usize]) * fac;
        self_.states[MYPAINT_BRUSH_STATE_DIRECTION_DY as usize] +=
            (dy - self_.states[MYPAINT_BRUSH_STATE_DIRECTION_DY as usize]) * fac;
    }

    {
        let fac = 1.0 - exp_decay(self_.settings_value[MYPAINT_BRUSH_SETTING_CUSTOM_INPUT_SLOWNESS as usize], 0.1);
        self_.states[MYPAINT_BRUSH_STATE_CUSTOM_INPUT as usize] +=
            (self_.settings_value[MYPAINT_BRUSH_SETTING_CUSTOM_INPUT as usize]
             - self_.states[MYPAINT_BRUSH_STATE_CUSTOM_INPUT as usize])
            * fac;
    }

    {
        let frequency = (-self_.settings_value[MYPAINT_BRUSH_SETTING_STROKE_DURATION_LOGARITHMIC as usize]).exp();
        self_.states[MYPAINT_BRUSH_STATE_STROKE as usize] +=
            norm_dist * frequency;
        self_.states[MYPAINT_BRUSH_STATE_STROKE as usize] =
            self_.states[MYPAINT_BRUSH_STATE_STROKE as usize].max(0.0);
        let wrap = 1.0 + self_.settings_value[MYPAINT_BRUSH_SETTING_STROKE_HOLDTIME as usize];

        if self_.states[MYPAINT_BRUSH_STATE_STROKE as usize] > wrap {
            self_.states[MYPAINT_BRUSH_STATE_STROKE as usize] = if wrap > 10.9 {
                1.0
            } else {
                (self_.states[MYPAINT_BRUSH_STATE_STROKE as usize] % wrap).max(0.0)
            }
        }
    }

    let radius_log = self_.settings_value[MYPAINT_BRUSH_SETTING_RADIUS_LOGARITHMIC as usize];
    self_.states[MYPAINT_BRUSH_STATE_ACTUAL_RADIUS as usize] =
        radius_log.exp().min(ACTUAL_RADIUS_MAX).max(ACTUAL_RADIUS_MIN);

    self_.states[MYPAINT_BRUSH_STATE_ACTUAL_ELLIPTICAL_DAB_RATIO as usize] =
        self_.settings_value[MYPAINT_BRUSH_SETTING_ELLIPTICAL_DAB_RATIO as usize];
    self_.states[MYPAINT_BRUSH_STATE_ACTUAL_ELLIPTICAL_DAB_ANGLE as usize] =
        self_.settings_value[MYPAINT_BRUSH_SETTING_ELLIPTICAL_DAB_ANGLE as usize];
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
    self_.settings_value[MYPAINT_BRUSH_SETTING_OPAQUE as usize] =
        self_.settings_value[MYPAINT_BRUSH_SETTING_OPAQUE as usize].max(0.0);

    let mut opaque = self_.settings_value[MYPAINT_BRUSH_SETTING_OPAQUE as usize]
        * self_.settings_value[MYPAINT_BRUSH_SETTING_OPAQUE_MULTIPLY as usize];
    opaque = opaque.min(1.0).max(0.0);

    if self_.settings_value[MYPAINT_BRUSH_SETTING_OPAQUE_LINEARIZE as usize] != 0.0 {
        let mut dabs_per_pixel = (
            mypaint_mapping_get_base_value(self_.settings[MYPAINT_BRUSH_SETTING_DABS_PER_ACTUAL_RADIUS as usize])
            + mypaint_mapping_get_base_value(self_.settings[MYPAINT_BRUSH_SETTING_DABS_PER_BASIC_RADIUS as usize])
        ) * 2.0;

        dabs_per_pixel = dabs_per_pixel.max(1.0);
        dabs_per_pixel = 1.0 + mypaint_mapping_get_base_value(
            self_.settings[MYPAINT_BRUSH_SETTING_OPAQUE_LINEARIZE as usize])
            * (dabs_per_pixel - 1.0);

        let alpha = opaque;
        let beta = 1.0 - alpha;
        let beta_dab = beta.powf(1.0 / dabs_per_pixel);
        let alpha_dab = 1.0 - beta_dab;
        opaque = alpha_dab;
    }
    let mut x = self_.states[MYPAINT_BRUSH_STATE_ACTUAL_X as usize];
    let mut y = self_.states[MYPAINT_BRUSH_STATE_ACTUAL_Y as usize];

    let base_radius = mypaint_mapping_get_base_value(
        self_.settings[MYPAINT_BRUSH_SETTING_RADIUS_LOGARITHMIC as usize]).exp();

    if self_.settings_value[MYPAINT_BRUSH_SETTING_OFFSET_BY_SPEED as usize] != 0.0 {
        let mult = self_.settings_value[MYPAINT_BRUSH_SETTING_OFFSET_BY_SPEED as usize] * 0.1 * base_radius;
        x += self_.states[MYPAINT_BRUSH_STATE_NORM_DX_SLOW as usize] * mult;
        y += self_.states[MYPAINT_BRUSH_STATE_NORM_DY_SLOW as usize] * mult;
    }

    {
        let mut amp = self_.settings_value[MYPAINT_BRUSH_SETTING_OFFSET_BY_RANDOM as usize];
        if amp != 0.0 {
            amp = amp.max(0.0);
            x += rand_gauss(self_.rng) * amp * base_radius;
            y += rand_gauss(self_.rng) * amp * base_radius;
        }
    }

    let mut radius = self_.states[MYPAINT_BRUSH_STATE_ACTUAL_RADIUS as usize];

    if self_.settings_value[MYPAINT_BRUSH_SETTING_RADIUS_BY_RANDOM as usize] != 0.0 {
        let mut radius_log = self_.settings_value[MYPAINT_BRUSH_SETTING_RADIUS_LOGARITHMIC as usize];
        radius_log += rand_gauss(self_.rng)
            * self_.settings_value[MYPAINT_BRUSH_SETTING_RADIUS_BY_RANDOM as usize];

        radius = radius_log.exp().min(ACTUAL_RADIUS_MAX).max(ACTUAL_RADIUS_MIN);

        let alpha_correction = sq(self_.states[MYPAINT_BRUSH_STATE_ACTUAL_RADIUS as usize] / radius);
        if alpha_correction <= 1.0 {
            opaque *= alpha_correction;
        }
    }

    if self_.settings_value[MYPAINT_BRUSH_SETTING_SMUDGE_LENGTH as usize] < 1.0
        && (self_.settings_value[MYPAINT_BRUSH_SETTING_SMUDGE as usize] != 0.0
            || !mypaint_mapping_is_constant(self_.settings[MYPAINT_BRUSH_SETTING_SMUDGE as usize]))
    {
        let mut fac = self_.settings_value[MYPAINT_BRUSH_SETTING_SMUDGE_LENGTH as usize]
            .max(0.01);
        let px = x.round();
        let py = y.round();

        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        let mut a = 0.0;

        self_.states[MYPAINT_BRUSH_STATE_LAST_GETCOLOR_RECENTNESS as usize] *= fac;
        if self_.states[MYPAINT_BRUSH_STATE_LAST_GETCOLOR_RECENTNESS as usize] < 0.5*fac {
            if self_.states[MYPAINT_BRUSH_STATE_LAST_GETCOLOR_RECENTNESS as usize] == 0.0 {
                fac = 0.0;
            }
            self_.states[MYPAINT_BRUSH_STATE_LAST_GETCOLOR_RECENTNESS as usize] = 1.0;

            let mut smudge_radius =
                radius * self_.settings_value[MYPAINT_BRUSH_SETTING_SMUDGE_RADIUS_LOG as usize].exp();
            smudge_radius = smudge_radius.min(1.0).max(0.0);
            mypaint_surface_get_color(surface, px, py, smudge_radius,
                &mut r as *mut _,
                &mut g as *mut _,
                &mut b as *mut _,
                &mut a as *mut _);
            self_.states[MYPAINT_BRUSH_STATE_LAST_GETCOLOR_R as usize] = r;
            self_.states[MYPAINT_BRUSH_STATE_LAST_GETCOLOR_G as usize] = g;
            self_.states[MYPAINT_BRUSH_STATE_LAST_GETCOLOR_B as usize] = b;
            self_.states[MYPAINT_BRUSH_STATE_LAST_GETCOLOR_A as usize] = a;
        } else {
            r = self_.states[MYPAINT_BRUSH_STATE_LAST_GETCOLOR_R as usize];
            g = self_.states[MYPAINT_BRUSH_STATE_LAST_GETCOLOR_G as usize];
            b = self_.states[MYPAINT_BRUSH_STATE_LAST_GETCOLOR_B as usize];
            a = self_.states[MYPAINT_BRUSH_STATE_LAST_GETCOLOR_A as usize];
        }

        self_.states[MYPAINT_BRUSH_STATE_SMUDGE_A as usize] =
            (fac*self_.states[MYPAINT_BRUSH_STATE_SMUDGE_A as usize] + (1.0-fac)*a)
            .min(1.0).max(0.0);
        self_.states[MYPAINT_BRUSH_STATE_SMUDGE_RA as usize] =
            fac*self_.states[MYPAINT_BRUSH_STATE_SMUDGE_RA as usize] + (1.0-fac)*r*a;
        self_.states[MYPAINT_BRUSH_STATE_SMUDGE_GA as usize] =
            fac*self_.states[MYPAINT_BRUSH_STATE_SMUDGE_GA as usize] + (1.0-fac)*g*a;
        self_.states[MYPAINT_BRUSH_STATE_SMUDGE_BA as usize] =
            fac*self_.states[MYPAINT_BRUSH_STATE_SMUDGE_BA as usize] + (1.0-fac)*b*a;
    }

    let mut color_h = mypaint_mapping_get_base_value(
        self_.settings[MYPAINT_BRUSH_SETTING_COLOR_H as usize]);
    let mut color_s = mypaint_mapping_get_base_value(
        self_.settings[MYPAINT_BRUSH_SETTING_COLOR_S as usize]);
    let mut color_v = mypaint_mapping_get_base_value(
        self_.settings[MYPAINT_BRUSH_SETTING_COLOR_V as usize]);
    let mut eraser_target_alpha = 1.0;

    if self_.settings_value[MYPAINT_BRUSH_SETTING_SMUDGE as usize] > 0.0 {
        hsv_to_rgb_float(
            &mut color_h as *mut _,
            &mut color_s as *mut _,
            &mut color_v as *mut _);
        let fac = self_.settings_value[MYPAINT_BRUSH_SETTING_SMUDGE as usize]
            .min(1.0);
        eraser_target_alpha = ((1.0-fac) + fac*self_.states[MYPAINT_BRUSH_STATE_SMUDGE_A as usize])
            .min(1.0).max(0.0);
        if eraser_target_alpha > 0.0 {
            color_h = (fac*self_.states[MYPAINT_BRUSH_STATE_SMUDGE_RA as usize] + (1.0-fac)*color_h) / eraser_target_alpha;
            color_s = (fac*self_.states[MYPAINT_BRUSH_STATE_SMUDGE_GA as usize] + (1.0-fac)*color_s) / eraser_target_alpha;
            color_v = (fac*self_.states[MYPAINT_BRUSH_STATE_SMUDGE_BA as usize] + (1.0-fac)*color_v) / eraser_target_alpha;
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

    eraser_target_alpha *= 1.0 - self_.settings_value[MYPAINT_BRUSH_SETTING_ERASER as usize];

    color_h += self_.settings_value[MYPAINT_BRUSH_SETTING_CHANGE_COLOR_H as usize];
    color_s += self_.settings_value[MYPAINT_BRUSH_SETTING_CHANGE_COLOR_HSV_S as usize];
    color_v += self_.settings_value[MYPAINT_BRUSH_SETTING_CHANGE_COLOR_V as usize];

    if self_.settings_value[MYPAINT_BRUSH_SETTING_CHANGE_COLOR_L as usize] != 0.0
        || self_.settings_value[MYPAINT_BRUSH_SETTING_CHANGE_COLOR_HSL_S as usize] != 0.0
    {
        hsv_to_rgb_float(
            &mut color_h as *mut _,
            &mut color_s as *mut _,
            &mut color_v as *mut _);
        rgb_to_hsl_float(
            &mut color_h as *mut _,
            &mut color_s as *mut _,
            &mut color_v as *mut _);
        color_v += self_.settings_value[MYPAINT_BRUSH_SETTING_CHANGE_COLOR_L as usize];
        color_s += self_.settings_value[MYPAINT_BRUSH_SETTING_CHANGE_COLOR_HSL_S as usize];
        hsl_to_rgb_float(
            &mut color_h as *mut _,
            &mut color_s as *mut _,
            &mut color_v as *mut _);
        rgb_to_hsv_float(
            &mut color_h as *mut _,
            &mut color_s as *mut _,
            &mut color_v as *mut _);
    }

    let mut hardness = self_.settings_value[MYPAINT_BRUSH_SETTING_HARDNESS as usize]
        .min(1.0).max(0.0);

    let current_fadeout_in_pixels = radius * (1.0 - hardness);
    let min_fadeout_in_pixels = self_.settings_value[MYPAINT_BRUSH_SETTING_ANTI_ALIASING as usize];

    if current_fadeout_in_pixels < min_fadeout_in_pixels {
        let current_optical_radius = radius - (1.0 - hardness)*radius/2.0;

        hardness = (current_optical_radius - (min_fadeout_in_pixels/2.0))
            / (current_optical_radius + (min_fadeout_in_pixels/2.0));
        radius = min_fadeout_in_pixels / (1.0 - hardness);
    }

    let snap_to_pixel = self_.settings_value[MYPAINT_BRUSH_SETTING_SNAP_TO_PIXEL as usize];
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
        self_.states[MYPAINT_BRUSH_STATE_ACTUAL_ELLIPTICAL_DAB_RATIO as usize],
        self_.states[MYPAINT_BRUSH_STATE_ACTUAL_ELLIPTICAL_DAB_ANGLE as usize],
        self_.settings_value[MYPAINT_BRUSH_SETTING_LOCK_ALPHA as usize],
        self_.settings_value[MYPAINT_BRUSH_SETTING_COLORIZE as usize]) != 0
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
        let rad = &mut self_.states[MYPAINT_BRUSH_STATE_ACTUAL_RADIUS as usize];
        if *rad == 0.0 {
            *rad = mypaint_mapping_get_base_value(self_.settings[MYPAINT_BRUSH_SETTING_RADIUS_LOGARITHMIC as usize]).exp();
        }
        *rad = rad.min(ACTUAL_RADIUS_MAX).max(ACTUAL_RADIUS_MIN);
    }

    let base_radius = mypaint_mapping_get_base_value(self_.settings[MYPAINT_BRUSH_SETTING_RADIUS_LOGARITHMIC as usize])
        .exp().max(ACTUAL_RADIUS_MIN).min(ACTUAL_RADIUS_MAX);

    let xx = x - self_.states[MYPAINT_BRUSH_STATE_X as usize];
    let yy = y - self_.states[MYPAINT_BRUSH_STATE_Y as usize];

    let dist = {
        let dab_ratio = &mut self_.states[MYPAINT_BRUSH_STATE_ACTUAL_ELLIPTICAL_DAB_RATIO as usize];
        if *dab_ratio > 1.0 {
            let angle_rad = *dab_ratio / 360.0 * TAU;
            let (sn, cs) = angle_rad.sin_cos();
            ((yy*cs - xx*sn) * *dab_ratio).hypot(yy*sn + xx*cs)
        } else {
            xx.hypot(yy)
        }
    };

    let res1 = dist / self_.states[MYPAINT_BRUSH_STATE_ACTUAL_RADIUS as usize] *
        mypaint_mapping_get_base_value(self_.settings[MYPAINT_BRUSH_SETTING_DABS_PER_ACTUAL_RADIUS as usize]);
    let res2 = dist / base_radius *
        mypaint_mapping_get_base_value(self_.settings[MYPAINT_BRUSH_SETTING_DABS_PER_BASIC_RADIUS as usize]);
    let res3 = dt *
        mypaint_mapping_get_base_value(self_.settings[MYPAINT_BRUSH_SETTING_DABS_PER_SECOND as usize]);
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

    if dtime > 0.1 && pressure != 0.0 && self_.states[MYPAINT_BRUSH_STATE_PRESSURE as usize] == 0.0 {
        // workaround for tablets that don't report motion without pressure
        mypaint_brush_stroke_to(self_ as *mut _, surface, x, y, 0.0, 9.0, 0.0, dtime - 0.0001);
        dtime = 0.0001;
    }

    {
        let tracking_noise = mypaint_mapping_get_base_value(self_.settings[MYPAINT_BRUSH_SETTING_TRACKING_NOISE as usize]);
        if tracking_noise != 0.0 {
            let base_radius = mypaint_mapping_get_base_value(self_.settings[MYPAINT_BRUSH_SETTING_RADIUS_LOGARITHMIC as usize])
                .exp();
            x += rand_gauss(self_.rng) * tracking_noise * base_radius;
            y += rand_gauss(self_.rng) * tracking_noise * base_radius;
        }
        let fac = 1.0 - exp_decay(
            mypaint_mapping_get_base_value(self_.settings[MYPAINT_BRUSH_SETTING_SLOW_TRACKING as usize]),
            100.0 * dtime as f32);
        let sx = self_.states[MYPAINT_BRUSH_STATE_X as usize];
        let sy = self_.states[MYPAINT_BRUSH_STATE_Y as usize];
        x = sx + (x - sx)*fac;
        y = sy + (y - sy)*fac;
    }

    let mut dabs_moved = self_.states[MYPAINT_BRUSH_STATE_PARTIAL_DABS as usize];
    let mut dabs_todo = count_dabs_to(self_ as *mut _, x, y, pressure, dtime as f32);

    if dtime > 5.0 || self_.reset_requested {
        self_.reset_requested = false;

        for i in 0..MYPAINT_BRUSH_STATES_COUNT {
            self_.states[i] = 0.0;
        }

        self_.states[MYPAINT_BRUSH_STATE_X as usize] = x;
        self_.states[MYPAINT_BRUSH_STATE_Y as usize] = y;
        self_.states[MYPAINT_BRUSH_STATE_PRESSURE as usize] = pressure;

        self_.states[MYPAINT_BRUSH_STATE_ACTUAL_X as usize] = x;
        self_.states[MYPAINT_BRUSH_STATE_ACTUAL_Y as usize] = y;
        self_.states[MYPAINT_BRUSH_STATE_STROKE as usize] = 1.0;

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
        step_dx = frac * (x - self_.states[MYPAINT_BRUSH_STATE_X as usize]);
        step_dy = frac * (y - self_.states[MYPAINT_BRUSH_STATE_Y as usize]);
        step_dpressure = frac * (pressure - self_.states[MYPAINT_BRUSH_STATE_PRESSURE as usize]);
        step_dtime = frac as f64 * dtime_left;

        step_declination = frac * (tilt_declination - self_.states[MYPAINT_BRUSH_STATE_DECLINATION as usize]);
        step_ascension = frac * smallest_angular_difference(self_.states[MYPAINT_BRUSH_STATE_ASCENSION as usize], tilt_ascension);

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
    step_dx = x - self_.states[MYPAINT_BRUSH_STATE_X as usize];
    step_dy = y - self_.states[MYPAINT_BRUSH_STATE_Y as usize];
    step_dpressure = pressure - self_.states[MYPAINT_BRUSH_STATE_PRESSURE as usize];
    step_declination = tilt_declination - self_.states[MYPAINT_BRUSH_STATE_DECLINATION as usize];
    step_ascension = smallest_angular_difference(self_.states[MYPAINT_BRUSH_STATE_ASCENSION as usize], tilt_ascension);
    step_dtime = dtime_left;

    update_states_and_setting_values(self_ as *mut _,
        step_ddab, step_dx, step_dy, step_dpressure,
        step_declination, step_ascension, step_dtime as f32);

    self_.states[MYPAINT_BRUSH_STATE_PARTIAL_DABS as usize] = dabs_moved + dabs_todo;

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
    for s in 0..MYPAINT_BRUSH_SETTINGS_COUNT as u32 {
        for i in 0..MYPAINT_BRUSH_INPUTS_COUNT as u32 {
            mypaint_brush_set_mapping_n(self_, s, i, 0);
        }

        let def = (*mypaint_brush_setting_info(s)).def;
        mypaint_brush_set_base_value(self_, s, def);
    }

    mypaint_brush_set_mapping_n(self_, MYPAINT_BRUSH_SETTING_OPAQUE_MULTIPLY as u32, MYPAINT_BRUSH_INPUT_PRESSURE as u32, 2);
    mypaint_brush_set_mapping_point(self_, MYPAINT_BRUSH_SETTING_OPAQUE_MULTIPLY as u32, MYPAINT_BRUSH_INPUT_PRESSURE as u32, 0, 0.0, 0.0);
    mypaint_brush_set_mapping_point(self_, MYPAINT_BRUSH_SETTING_OPAQUE_MULTIPLY as u32, MYPAINT_BRUSH_INPUT_PRESSURE as u32, 1, 1.0, 1.0);
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
