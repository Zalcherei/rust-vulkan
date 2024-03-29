use nalgebra_glm as glm;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UniformBufferObject {
    pub view: glm::Mat4,
    pub proj: glm::Mat4,
}
