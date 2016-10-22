macro_rules! fake_enum {
    (
        pub struct $name:ident {
            $($num:expr => $prop:ident),*,
        }
    ) => {
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $name<T> {
            $(pub $prop: T),*
        }
        impl<T> $name<T> {
            pub fn int_to_state(&mut self, int: usize) -> &mut T {
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

fake_enum! {
    pub struct MyPaintBrushSetting {
        0 => opaque,
        1 => opaque_multiply,
        2 => opaque_linearize,
        3 => radius_logarithmic,
        4 => hardness,
        5 => anti_aliasing,
        6 => dabs_per_basic_radius,
        7 => dabs_per_actual_radius,
        8 => dabs_per_second,
        9 => radius_by_random,
        10 => speed1_slowness,
        11 => speed2_slowness,
        12 => speed1_gamma,
        13 => speed2_gamma,
        14 => offset_by_random,
        15 => offset_by_speed,
        16 => offset_by_speed_slowness,
        17 => slow_tracking,
        18 => slow_tracking_per_dab,
        19 => tracking_noise,
        20 => color_h,
        21 => color_s,
        22 => color_v,
        23 => restore_color,
        24 => change_color_h,
        25 => change_color_l,
        26 => change_color_hsl_s,
        27 => change_color_v,
        28 => change_color_hsv_s,
        29 => smudge,
        30 => smudge_length,
        31 => smudge_radius_log,
        32 => eraser,
        33 => stroke_threshold,
        34 => stroke_duration_logarithmic,
        35 => stroke_holdtime,
        36 => custom_input,
        37 => custom_input_slowness,
        38 => elliptical_dab_ratio,
        39 => elliptical_dab_angle,
        40 => direction_filter,
        41 => lock_alpha,
        42 => colorize,
        43 => snap_to_pixel,
        44 => pressure_gain_log,
        // mypaint_brush_settings_count
    }
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
