use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wgpu::{BindGroup, BindGroupLayout, Buffer, RenderPipeline};
pub struct GFX {
    buffers: Vec<Arc<Mutex<Buffer>>>,
    bind_groups: Vec<Rc<BindGroup>>,
    bind_group_layouts: Vec<Rc<BindGroupLayout>>,
    pipelines: Vec<Rc<RenderPipeline>>,
}

pub trait SceneDescribe {
    fn ready_model(&self);
    fn ready_texture(&self);
    fn model_to_buffer(&self);
    fn camera_setting(&self);
    fn update(&self);
}
