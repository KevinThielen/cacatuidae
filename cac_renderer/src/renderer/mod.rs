mod backend;

pub use backend::{Backend, Context};

mod mesh;
pub use mesh::{Mesh, Primitive};

mod render_target;
pub use render_target::{ClearFlags, RenderTarget};

mod shader;
pub use shader::{ProgramStorage, Shader, ShaderProgram, Uniform, UniformDescription, UniformKind};

mod buffer;
pub use buffer::{Buffer, BufferAttributes, BufferData, BufferStorage, BufferUsage, CreateBuffer};

mod vertex_layout;
pub use vertex_layout::{
    AttributeSemantic, CreateVertexLayout, VertexAttribute, VertexAttributeKind, VertexLayout,
};

mod material;
pub use material::{Material, MaterialProperty, PropertyId, PropertyValue};

mod draw_list;
pub use draw_list::DrawList;

mod texture;
pub use texture::Texture;

use crate::{generation_vec::GenerationVec, Handle, RendererError};

/// Renderer abstraction
///
/// Provides a high level API for the graphics backends. The backend can be chosen dynamically,
/// depending on the supported features, version and graphics device capabilities. See
/// [StaticRenderer] if you want to use a compile-time dependent one to avoid the boxing.
///
/// [Shader]s, [Mesh]es, [Texture]s can be created on the graphics device, while the renderer returns handles to them.
/// The user is responsible to releasing them with the handles. It is recommended to
/// keep track of the Handles with a human readable constant to associate them with a name or id.
/// The Renderer sorts and batches draw calls.

pub struct Renderer<T: Context> {
    context: T::Context,
    pub buffers: GenerationVec<Buffer, T::Buffer>,
    pub layouts: GenerationVec<VertexLayout, T::VertexLayout>,
    pub shaders: GenerationVec<Shader, T::Shader>,
    pub programs: GenerationVec<ShaderProgram, T::ShaderProgram>,
    materials: GenerationVec<Material, Material>,
}

impl<T: Context> Renderer<T> {
    pub fn create_material(
        &mut self,
        program: Handle<ShaderProgram>,
        properties: &[MaterialProperty],
    ) -> Result<Handle<Material>, RendererError> {
        if let Some(shader_program) = self.programs.get(program) {
            let mut material = Material {
                program,
                data: vec![0; shader_program.data_size() * 4],
            };

            material.update(shader_program.uniforms(), properties);

            Ok(self.materials.push(material))
        } else {
            Err(RendererError::ResourceNotFound {
                resource: format!("Shaderprogram: {program:?}"),
            })
        }
    }

    pub fn use_material(&mut self, handle: Handle<Material>) {
        if let Some(material) = self.materials.get(handle) {
            if let Some(program) = self.programs.get_mut(material.program) {
                program.set_uniform_data(&material.data);
            }
        }
    }

    pub fn update_material(&mut self, handle: Handle<Material>, properties: &[MaterialProperty]) {
        if let Some(material) = self.materials.get_mut(handle) {
            if let Some(shader_program) = self.programs.get(material.program) {
                material.update(shader_program.uniforms(), properties);
            }
        }
    }
}

//pub trait GraphicsStorage {
//type Handle: Copy;
//type Resource;
//fn get(&self, handle: Self::Handle) -> Option<&Self::Resource>;
//fn get_mut(&mut self, handle: Self::Handle) -> Option<&mut Self::Resource>;
//}
