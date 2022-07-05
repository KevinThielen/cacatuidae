pub(super) struct RenderTarget {
    clear_color: crate::Color32,
    clear_flags: crate::ClearFlags,
}

impl Default for RenderTarget {
    fn default() -> Self {
        Self {
            clear_color: crate::Color32::BLACK,
            clear_flags: crate::ClearFlags::NONE,
        }
    }
}

impl crate::RenderTarget for RenderTarget {
    fn set_clear_color(&mut self, color: crate::Color32) {
        self.clear_color = color;
    }

    fn clear(&mut self) {
        log::info!("Cleared {} with {:?}", self.clear_flags, self.clear_color);
    }

    fn set_clear_flags(&mut self, flags: crate::renderer::render_target::ClearFlags) {
        self.clear_flags = flags
    }
}
