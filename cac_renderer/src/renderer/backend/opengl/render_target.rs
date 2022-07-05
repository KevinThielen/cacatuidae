use gl::types::GLbitfield;

use crate::ClearFlags;

#[derive(Debug, Copy, Clone)]
pub struct ScreenTarget {
    clear_flags: GLbitfield,
}

impl Default for ScreenTarget {
    fn default() -> Self {
        Self {
            clear_flags: gl::COLOR_BUFFER_BIT,
        }
    }
}

impl From<ClearFlags> for GLbitfield {
    fn from(flag: ClearFlags) -> Self {
        let mut clear_flags = 0;
        if flag == ClearFlags::COLOR {
            clear_flags |= gl::COLOR_BUFFER_BIT;
        }
        if flag == ClearFlags::DEPTH {
            clear_flags |= gl::DEPTH_BUFFER_BIT;
        }
        if flag == ClearFlags::STENCIL {
            clear_flags |= gl::STENCIL_BUFFER_BIT;
        }

        clear_flags
    }
}

impl crate::RenderTarget for ScreenTarget {
    fn set_clear_color(&mut self, color: crate::Color32) {
        let (r, g, b, a) = color.as_rgba();
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    fn clear(&mut self) {
        unsafe {
            gl::Clear(self.clear_flags);
        }
    }

    fn set_clear_flags(&mut self, flags: ClearFlags) {
        self.clear_flags = flags.into();
    }
}
impl ScreenTarget {}
