#[repr(C)]
pub struct MyPaintBrushSettingInfo {
    pub cname: *const u8,
    pub name: *const u8,
    pub constant: bool,
    pub min: f32,
    pub def: f32,
    pub max: f32,
    pub tooltip: *const u8,
}

#[repr(C)]
pub struct MyPaintBrushInputInfo {
    cname: *const u8,
    hard_min: f32,
    soft_min: f32,
    normal: f32,
    soft_max: f32,
    hard_max: f32,
    name: *const u8,
    tooltip: *const u8,
}
