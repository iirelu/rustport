macro_rules! fake_enum {
    (
        pub struct $name:ident {
            $($num:expr => $prop:ident),*,
        }
    ) => {
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $name {
            $(pub $prop: f32),*
        }
        impl $name {
            pub fn int_to_state(&mut self, int: usize) -> &mut f32 {
                match int {
                    $($num => &mut self.$prop),*,
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[repr(C)]
pub enum MyPaintBrushInput {
    MYPAINT_BRUSH_INPUT_PRESSURE = 0,
    MYPAINT_BRUSH_INPUT_SPEED1,
    MYPAINT_BRUSH_INPUT_SPEED2,
    MYPAINT_BRUSH_INPUT_RANDOM,
    MYPAINT_BRUSH_INPUT_STROKE,
    MYPAINT_BRUSH_INPUT_DIRECTION,
    MYPAINT_BRUSH_INPUT_TILT_DECLINATION,
    MYPAINT_BRUSH_INPUT_TILT_ASCENSION,
    MYPAINT_BRUSH_INPUT_CUSTOM,
    // note: this is a trick used by the c code which doesn't work in rust due
    // to the fact that enum varians aren't constant expressions
    // MYPAINT_BRUSH_INPUTS_COUNT
}

#[repr(C)]
pub enum MyPaintBrushSetting {
    MYPAINT_BRUSH_SETTING_OPAQUE = 0,
    MYPAINT_BRUSH_SETTING_OPAQUE_MULTIPLY,
    MYPAINT_BRUSH_SETTING_OPAQUE_LINEARIZE,
    MYPAINT_BRUSH_SETTING_RADIUS_LOGARITHMIC,
    MYPAINT_BRUSH_SETTING_HARDNESS,
    MYPAINT_BRUSH_SETTING_ANTI_ALIASING,
    MYPAINT_BRUSH_SETTING_DABS_PER_BASIC_RADIUS,
    MYPAINT_BRUSH_SETTING_DABS_PER_ACTUAL_RADIUS,
    MYPAINT_BRUSH_SETTING_DABS_PER_SECOND,
    MYPAINT_BRUSH_SETTING_RADIUS_BY_RANDOM,
    MYPAINT_BRUSH_SETTING_SPEED1_SLOWNESS,
    MYPAINT_BRUSH_SETTING_SPEED2_SLOWNESS,
    MYPAINT_BRUSH_SETTING_SPEED1_GAMMA,
    MYPAINT_BRUSH_SETTING_SPEED2_GAMMA,
    MYPAINT_BRUSH_SETTING_OFFSET_BY_RANDOM,
    MYPAINT_BRUSH_SETTING_OFFSET_BY_SPEED,
    MYPAINT_BRUSH_SETTING_OFFSET_BY_SPEED_SLOWNESS,
    MYPAINT_BRUSH_SETTING_SLOW_TRACKING,
    MYPAINT_BRUSH_SETTING_SLOW_TRACKING_PER_DAB,
    MYPAINT_BRUSH_SETTING_TRACKING_NOISE,
    MYPAINT_BRUSH_SETTING_COLOR_H,
    MYPAINT_BRUSH_SETTING_COLOR_S,
    MYPAINT_BRUSH_SETTING_COLOR_V,
    MYPAINT_BRUSH_SETTING_RESTORE_COLOR,
    MYPAINT_BRUSH_SETTING_CHANGE_COLOR_H,
    MYPAINT_BRUSH_SETTING_CHANGE_COLOR_L,
    MYPAINT_BRUSH_SETTING_CHANGE_COLOR_HSL_S,
    MYPAINT_BRUSH_SETTING_CHANGE_COLOR_V,
    MYPAINT_BRUSH_SETTING_CHANGE_COLOR_HSV_S,
    MYPAINT_BRUSH_SETTING_SMUDGE,
    MYPAINT_BRUSH_SETTING_SMUDGE_LENGTH,
    MYPAINT_BRUSH_SETTING_SMUDGE_RADIUS_LOG,
    MYPAINT_BRUSH_SETTING_ERASER,
    MYPAINT_BRUSH_SETTING_STROKE_THRESHOLD,
    MYPAINT_BRUSH_SETTING_STROKE_DURATION_LOGARITHMIC,
    MYPAINT_BRUSH_SETTING_STROKE_HOLDTIME,
    MYPAINT_BRUSH_SETTING_CUSTOM_INPUT,
    MYPAINT_BRUSH_SETTING_CUSTOM_INPUT_SLOWNESS,
    MYPAINT_BRUSH_SETTING_ELLIPTICAL_DAB_RATIO,
    MYPAINT_BRUSH_SETTING_ELLIPTICAL_DAB_ANGLE,
    MYPAINT_BRUSH_SETTING_DIRECTION_FILTER,
    MYPAINT_BRUSH_SETTING_LOCK_ALPHA,
    MYPAINT_BRUSH_SETTING_COLORIZE,
    MYPAINT_BRUSH_SETTING_SNAP_TO_PIXEL,
    MYPAINT_BRUSH_SETTING_PRESSURE_GAIN_LOG,
    // MYPAINT_BRUSH_SETTINGS_COUNT
}

/// direct structification of the previous enum of the same name
fake_enum! {
    pub struct MyPaintBrushState {
        0 => x,
        1 => y,
        2 => pressure,
        3 => partial_dabs,
        4 => actual_radius,
        5 => smudge_ra,
        6 => smudge_ga,
        7 => smudge_ba,
        8 => smudge_a,
        9 => last_getcolor_r,
        10 => last_getcolor_g,
        11 => last_getcolor_b,
        12 => last_getcolor_a,
        13 => last_getcolor_recentness,
        14 => actual_x,
        15 => actual_y,
        16 => norm_dx_slow,
        17 => norm_dy_slow,
        18 => norm_speed1_slow,
        19 => norm_speed2_slow,
        20 => stroke,
        21 => stroke_started,
        22 => custom_input,
        23 => rng_seed,
        24 => actual_elliptical_dab_ratio,
        25 => actual_elliptical_dab_angle,
        26 => direction_dx,
        27 => direction_dy,
        28 => declination,
        29 => ascension,
    }
}
