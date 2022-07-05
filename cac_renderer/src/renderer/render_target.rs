use std::fmt::Display;

use crate::color::Color32;

/// The buffers of a render target that should be cleared at the start of every frame
/// ```
/// # use cac_renderer::*;
/// # let mut renderer = Renderer::new_headless().unwrap();
///
/// let render_target = renderer.screen_target();
/// render_target.set_clear_flags(ClearFlags::COLOR | ClearFlags::DEPTH);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ClearFlags(u8);

impl ClearFlags {
    ///Clear Nothing
    pub const NONE: Self = Self(0x00);
    ///Clear Color Buffer
    pub const COLOR: Self = Self(0x01);
    ///Clear Depth Buffer
    pub const DEPTH: Self = Self(0x02);
    ///Clear Stencil Buffer
    pub const STENCIL: Self = Self(0x04);
}

impl std::ops::BitAnd for ClearFlags {
    type Output = bool;

    fn bitand(self, rhs: Self) -> Self::Output {
        ClearFlags(self.0 & rhs.0).0 > 0
    }
}
impl std::ops::BitOr for ClearFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ClearFlags(self.0 | rhs.0)
    }
}

impl Display for ClearFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClearFlags {{ ")?;
        if *self & Self::COLOR {
            write!(f, "COLOR |")?;
        }
        if *self & Self::DEPTH {
            write!(f, "DEPTH |")?;
        }
        if *self & Self::STENCIL {
            write!(f, "STENCIL")?;
        }

        writeln!(f, "}} ")
    }
}

/// Target to render to.
/// Can be either a screen target, a texture or a graphics specific render target.
/// Setting clear flags will make the target clear at the start of every [renderer
/// update][Renderer::update].
pub trait RenderTarget {
    /// The color used to clear the target
    fn set_clear_color(&mut self, color: Color32);
    /// actually clear the target.
    /// There is no need to call this manually, because the renderer will automatically clear
    /// render targets if they are used as render targets, at the beginning of every [renderer
    /// update][Renderer::update]
    fn clear(&mut self);

    /// The bits to clear. Multiple flags can be used with a bitwiseor |.
    fn set_clear_flags(&mut self, flags: ClearFlags);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn or() {
        let flags = ClearFlags::COLOR | ClearFlags::DEPTH;

        assert_eq!(flags, ClearFlags(1 | 2));
    }
}
