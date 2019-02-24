use gl;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl f32_f32_f32 {
    pub fn new(x: f32, y: f32, z: f32) -> f32_f32_f32 {
        f32_f32_f32 {
            x, y, z
        }
    }
}

impl From<(f32, f32, f32)> for f32_f32_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}