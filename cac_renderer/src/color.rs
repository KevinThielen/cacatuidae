/// Color struct with 8 bits per channel, ideally to save space compared to the 4x bigger [Color32]
/// struct
/// range is 0 - 255
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Color8 {
    /// red component. Range [0 - 255]
    pub r: u8,
    /// gree component. Range [0 - 255]
    pub g: u8,
    /// blue component. Range [0 - 255]
    pub b: u8,
    /// alpha component. Range [0 - 255]
    pub a: u8,
}

impl Default for Color8 {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl Color8 {
    /// New from rgb values, setting alpha to 255
    /// ```
    /// # use cac_renderer::Color8;
    /// let red = Color8::new_rgb(255, 0, 0);
    /// ```
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            ..Default::default()
        }
    }
    /// New with alpha channel
    /// ```
    /// # use cac_renderer::Color8;
    /// let red = Color8::new_rgba(255, 0, 0, 0);
    /// ```
    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Returns components as f32 tuple in the range of [0.0 - 1.0]
    pub fn as_f32(&self) -> (f32, f32, f32, f32) {
        (
            f32::from(self.r) / 255_f32,
            f32::from(self.g) / 255_f32,
            f32::from(self.b) / 255_f32,
            f32::from(self.a) / 255_f32,
        )
    }
}

impl From<(u8, u8, u8, u8)> for Color8 {
    fn from(color: (u8, u8, u8, u8)) -> Self {
        Self::new_rgba(color.0, color.1, color.2, color.3)
    }
}
impl From<(u8, u8, u8)> for Color8 {
    fn from(color: (u8, u8, u8)) -> Self {
        Self::new_rgba(color.0, color.1, color.2, 255)
    }
}
/// Values are clamped into 0-1 range
/// ```
/// # use cac_renderer::Color8;
/// let color: Color8 = (1.0, 0.0, 1.0, 0.0).into();
/// assert_eq!(color.as_f32(), (1.0, 0.0, 1.0, 0.0))
/// ```
impl From<(f32, f32, f32, f32)> for Color8 {
    fn from(color: (f32, f32, f32, f32)) -> Self {
        Self::new_rgba(
            (color.0.clamp(0.0, 1.0) * 255_f32) as u8,
            (color.1.clamp(0.0, 1.0) * 255_f32) as u8,
            (color.2.clamp(0.0, 1.0) * 255_f32) as u8,
            (color.3.clamp(0.0, 1.0) * 255_f32) as u8,
        )
    }
}

/// Color struct with 32 bits per channel
/// Internal representation is a linear color space
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color32 {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Default for Color32 {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

//small helper for conversion.
//TODO: Ideally would be const but powf blocks it.
//Come back if it ever becomes a performance issue.
fn into_linear(value: f32) -> f32 {
    if value < 0.0405 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

fn into_srgb(value: f32) -> f32 {
    if value < 0.0031308 {
        value * 12.92
    } else {
        (1.055 * value.powf(1.0 / 2.4)) - 0.055
    }
}
impl Color32 {
    // Default Colors
    /// The Color Black (0.0, 0.0, 0.0)
    pub const BLACK: Self = Self::from_rgb(0.0, 0.0, 0.0);
    /// The Color Red (1.0, 0.0, 0.0)
    pub const RED: Self = Self::from_rgb(1.0, 0.0, 0.0);
    /// The Color Blue (0.0, 0.0, 1.0)
    pub const BLUE: Self = Self::from_rgb(0.0, 0.0, 1.0);
    /// The Color Green (0.0, 1.0, 0.0)
    pub const GREEN: Self = Self::from_rgb(0.0, 1.0, 0.0);
    /// The Color Yellow (1.0, 1.0, 0.0)
    pub const YELLOW: Self = Self::from_rgb(1.0, 1.0, 0.0);
    /// The Color White (1.0, 1.0, 1.0)
    pub const WHITE: Self = Self::from_rgb(1.0, 1.0, 1.0);

    /// Almost black with a touch of green
    pub const DARK_JUNGLE_GREEN: Self = Self::from_rgb(0.102, 0.141, 0.129);
    /// Grape like purple
    pub const PERSIAN_INDIGO: Self = Self::from_rgb(0.20, 0.0, 0.30);
    /// Dirty White
    pub const GAINSBORO: Self = Self::from_rgb(0.79, 0.92, 0.87);
    /// It's really nice to look at
    pub const UNITY_YELLOW: Self = Self::from_rgb(1.0, 0.92, 0.016);

    /// Constructor using rgba in linear color space
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Constructor using rgb in linear color space
    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    /// Constructor using rgba in sRGB color space
    /// The struct converts and stores them into linear space.
    /// Use [Self::as_srgba()] to get the color in sRGBA space.
    pub fn from_srgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::from_rgba(
            into_linear(r),
            into_linear(g),
            into_linear(b),
            into_linear(a),
        )
    }

    /// Constructor using rgb in sRGB color space
    /// The struct converts and stores them into linear space.
    /// Use [Self::as_srgb()] to get the color in sRGBA space.
    pub fn from_srgb(r: f32, g: f32, b: f32) -> Self {
        Self::from_rgba(
            into_linear(r),
            into_linear(g),
            into_linear(b),
            into_linear(1.0),
        )
    }

    /// returns the Color in sRGB space
    pub fn as_srgb(&self) -> (f32, f32, f32) {
        (into_srgb(self.r), into_srgb(self.g), into_srgb(self.b))
    }

    /// returns the Color in sRGB space
    pub fn as_srgba(&self) -> (f32, f32, f32, f32) {
        (
            into_srgb(self.r),
            into_srgb(self.g),
            into_srgb(self.b),
            into_srgb(self.a),
        )
    }

    /// returns the Color in linear space(as it is stored in the struct)
    pub fn as_rgb(&self) -> (f32, f32, f32) {
        (self.r, self.g, self.b)
    }

    /// returns the Color in linear space(as it is stored in the struct)
    pub fn as_rgba(&self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn f32_into_color() {
        let color_f32 = (1.0, -1.0, 2.0, 0.0);
        let color = Color8::from(color_f32);

        assert_eq!(
            color,
            Color8 {
                r: 255,
                g: 0,
                b: 255,
                a: 0
            }
        );

        assert_eq!(color.as_f32(), (1.0, 0.0, 1.0, 0.0))
    }

    #[test]
    fn same_color() {
        let color_1 = Color8::new_rgb(255, 124, 12);
        let color_2 = Color8::new_rgba(255, 124, 12, 255);

        assert_eq!(color_1, color_2);
    }

    #[test]
    fn color_spaces() {
        let linear_color = Color32::from_rgb(0.45, 0.002, 0.734).as_rgb();

        // linear color conversion into a tuple works
        assert_eq!(linear_color, (0.45, 0.002, 0.734));

        // now create an srgb tuple
        let linear_color =
            Color32::from_rgb(linear_color.0, linear_color.1, linear_color.2).as_srgb();
        let srgb_color = (0.7014107, 0.025840001, 0.87245417);

        // approx difference is fine
        let diff = (linear_color.0 - srgb_color.0).abs()
            + (linear_color.1 - srgb_color.1).abs()
            + (linear_color.2 - srgb_color.2).abs();

        assert!(diff <= 0.00001);
    }
}
