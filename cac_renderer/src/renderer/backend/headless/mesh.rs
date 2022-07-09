use log::info;

///Headless Mesh implementation
pub struct Mesh {
    id: u32,
}

static mut ID_COUNTER: u32 = 0;

impl Mesh {
    pub(crate) fn new() -> Self {
        let id = unsafe {
            ID_COUNTER += 1;
            ID_COUNTER
        };

        info!("Created Mesh: {id}");
        Self { id }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        info!("Dropping Mesh: {}", self.id);
        unsafe {
            ID_COUNTER -= 1;
        }
    }
}

impl crate::Mesh for Mesh {}
