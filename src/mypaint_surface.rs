use mypaint_rectangle::MyPaintRectangle;

type MyPaintSurfaceGetColorFunction = unsafe extern fn(
    self_: *mut MyPaintSurface,
    x: f32, y: f32, radius: f32,
    color_r: *mut f32,
    color_g: *mut f32,
    color_b: *mut f32,
    color_a: *mut f32);

type MyPaintSurfaceDrawDabFunction = unsafe extern fn(
    self_: *mut MyPaintSurface,
    x: f32, y: f32, radius: f32,
    color_r: f32, color_g: f32, color_b: f32,
    opaque: f32, hardness: f32,
    alpha_eraser: f32,
    aspect_ratio: f32,
    angle: f32,
    lock_alpha: f32,
    colorize: f32) -> i32;

type MyPaintSurfaceDestroyFunction = unsafe extern fn(
    self_: *mut MyPaintSurface);

type MyPaintSurfaceSavePngFunction = unsafe extern fn(
    self_: *mut MyPaintSurface,
    path: *const u8,
    x: i32, y: i32, width: i32, height: i32);

type MyPaintSurfaceBeginAtomicFunction = unsafe extern fn(
    self_: *mut MyPaintSurface);

type MyPaintSurfaceEndAtomicFunction = unsafe extern fn(
    self_: *mut MyPaintSurface,
    roi: *mut MyPaintRectangle);

#[repr(C)]
pub struct MyPaintSurface {
    pub draw_dab: *mut MyPaintSurfaceDrawDabFunction,
    pub get_color: *mut MyPaintSurfaceGetColorFunction,
    pub begin_atomic: *mut MyPaintSurfaceBeginAtomicFunction,
    pub end_atomic: *mut MyPaintSurfaceEndAtomicFunction,
    pub destroy: *mut MyPaintSurfaceDestroyFunction,
    pub save_png: *mut MyPaintSurfaceSavePngFunction,
    pub ref_count: i32,
}

#[link(name = "mypaint")]
extern {
    pub fn mypaint_surface_get_color(
        self_: *mut MyPaintSurface,
        x: f32, y: f32,
        radius: f32,
        color_r: *mut f32, color_g: *mut f32, color_b: *mut f32, color_a: *mut f32);

    pub fn mypaint_surface_draw_dab(
        self_: *mut MyPaintSurface,
        x: f32, y: f32,
        radius: f32,
        color_r: f32, color_g: f32, color_b: f32,
        opaque: f32, hardness: f32,
        alpha_eraser: f32,
        aspect_ratio: f32,
        angle: f32,
        lock_alpha: f32,
        colorize: f32) -> i32;
}

// note: all this is dead code until i can figure out why it segfaults
// it'll probably only return once everything else is rustic

// #[no_mangle]
// pub unsafe extern fn mypaint_surface_draw_dab(
//     self_: *mut MyPaintSurface,
//     x: f32, y: f32,
//     radius: f32,
//     color_r: f32, color_g: f32, color_b: f32,
//     opaque: f32,
//     hardness: f32,
//     alpha_eraser: f32,
//     aspect_ratio: f32,
//     angle: f32,
//     lock_alpha: f32,
//     colorize: f32)
//     -> i32
// {
//     assert!(!self_.is_null());
//     assert!(!(*self_).draw_dab.is_null());
//     (*(*self_).draw_dab)(self_, x, y, radius, color_r, color_g, color_b,
//         opaque, hardness, alpha_eraser, aspect_ratio,
//         angle, lock_alpha, colorize)
// }
//
// #[no_mangle]
// pub unsafe extern fn mypaint_surface_get_color(
//     self_: *mut MyPaintSurface,
//     x: f32, y: f32,
//     radius: f32,
//     color_r: *mut f32, color_g: *mut f32, color_b: *mut f32, color_a: *mut f32)
// {
//     assert!(!self_.is_null());
//     assert!(!(*self_).get_color.is_null());
//     (*(*self_).get_color)(self_, x, y, radius, color_r, color_g, color_b, color_a)
// }
//
// #[no_mangle]
// pub unsafe extern fn mypaint_surface_init(
//     self_: *mut MyPaintSurface)
// {
//     assert!(!self_.is_null());
//     (*self_).ref_count = 1;
// }
//
// #[no_mangle]
// pub unsafe extern fn mypaint_surface_ref(
//     self_: *mut MyPaintSurface)
// {
//     assert!(!self_.is_null());
//     (*self_).ref_count += 1;
// }
//
// #[no_mangle]
// pub unsafe extern fn mypaint_surface_unref(
//     self_: *mut MyPaintSurface)
// {
//     assert!(!self_.is_null());
//     (*self_).ref_count -= 1;
//     if (*self_).ref_count <= 0 {
//         assert!(!(*self_).destroy.is_null());
//         (*(*self_).destroy)(self_);
//     }
// }
//
// #[no_mangle]
// pub unsafe extern fn mypaint_surface_get_alpha(
//     self_: *mut MyPaintSurface,
//     x: f32, y: f32,
//     radius: f32)
//     -> f32
// {
//     assert!(!self_.is_null());
//     assert!(!(*self_).get_color.is_null());
//     let mut color_r = 0.0f32;
//     let mut color_g = 0.0f32;
//     let mut color_b = 0.0f32;
//     let mut color_a = 0.0f32;
//     (*(*self_).get_color)(self_, x, y, radius,
//         &mut color_r as *mut _,
//         &mut color_g as *mut _,
//         &mut color_b as *mut _,
//         &mut color_a as *mut _);
//     color_a
// }
//
// #[no_mangle]
// pub unsafe extern fn mypaint_surface_save_png(
//     self_: *mut MyPaintSurface,
//     path: *const u8,
//     x: i32, y: i32,
//     width: i32, height: i32)
// {
//     assert!(!self_.is_null());
//     if !(*self_).save_png.is_null() {
//         (*(*self_).save_png)(self_, path, x, y, width, height);
//     }
// }
//
// #[no_mangle]
// pub unsafe extern fn mypaint_surface_begin_atomic(
//     self_: *mut MyPaintSurface)
// {
//     assert!(!self_.is_null());
//     if !(*self_).begin_atomic.is_null() {
//         (*(*self_).begin_atomic)(self_);
//     }
// }
//
// #[no_mangle]
// pub unsafe extern fn mypaint_surface_end_atomic(
//     self_: *mut MyPaintSurface,
//     roi: *mut MyPaintRectangle)
// {
//     assert!(!self_.is_null());
//     if !(*self_).end_atomic.is_null() {
//         (*(*self_).end_atomic)(self_, roi);
//     }
// }
