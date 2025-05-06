use crate::vector3;

pub type Color = vector3::Vector3;

impl Color {
    // Translate the [0,1] component values to the byte range [0,255].
    // use 255.999 to approximate: values near to 1
    // map to 255 due to truncation and 1 still map
    // to 255.
    pub fn to_rgb_bytes(self) -> (u8, u8, u8) {
        let r = (self.x().clamp(0.0, 1.0) * 255.999) as u8;
        let g = (self.y().clamp(0.0, 1.0) * 255.999) as u8;
        let b = (self.z().clamp(0.0, 1.0) * 255.999) as u8;
        (r, g, b)
    }

    // graphic trick to linearly scale 0.0 <= a <= 1.0 (i.e., a "lerp"
    // or "linear interpolation" function)
    // blendedValue = (1 − a) * startValue + a * endValue
    // with a in [0.0, 1.0]
    pub fn lerp(start: Color, end: Color, alpha: f64) -> Color {
        (1.0 - alpha) * start + alpha * end
    }
}
