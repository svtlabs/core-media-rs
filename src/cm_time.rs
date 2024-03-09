#[repr(C)]
#[derive(Default)]
pub struct CMTime {
    value: i64,
    timescale: i32,
    flags: u32,
    epoch: i64,
}
