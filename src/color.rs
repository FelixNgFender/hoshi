use crate::vec3;

pub type Color = vec3::Vector3;

impl Color {
    pub fn to_rgb_bytes(&self) -> (u8, u8, u8) {
        // Translate the [0,1] component values to the byte range [0,255].
        // use 255.999 to approximate: values near to 1
        // map to 255 due to truncation and 1 still map
        // to 255.
        let r = (self.x().clamp(0.0, 1.0) * 255.999) as u8;
        let g = (self.y().clamp(0.0, 1.0) * 255.999) as u8;
        let b = (self.z().clamp(0.0, 1.0) * 255.999) as u8;
        (r, g, b)
    }
}
