use std::borrow::Cow;

use wgpu::{
    util::DeviceExt, BindGroup, Buffer, BufferBinding, BufferBindingType, BufferDescriptor,
    BufferUsages, FrontFace, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
    ShaderModuleDescriptor, VertexState,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{gfx::GFX, window_init::GameSurface};

pub struct Plane {
    pub size: PhysicalSize<i32>,
    pub block_resolution_size: PhysicalSize<i32>,
    pub running: Option<Running>,
}

pub struct Running {
    pub buffer: Buffer,
    pub bind_group: BindGroup,
    pub pipeline: RenderPipeline,
}

impl Plane {
    pub fn new(window: &Window, block_resolution_x: i32, block_resolution_y: i32) -> Self {
        let size = window.inner_size();
        let width = size.width as i32;
        let height = size.height as i32;
        let size = PhysicalSize { width, height };
        let block_resolution_size = PhysicalSize {
            width: block_resolution_x,
            height: block_resolution_y,
        };
        Plane {
            size,
            block_resolution_size,
            running: None,
        }
    }

    pub fn display(&mut self, gs: &GameSurface) {
        println!(
            "init Buffer :: {:?}, {:?}",
            self.size, self.block_resolution_size
        );
        let bg_buffer = gs
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bg buffer"),
                contents: bytemuck::cast_slice(&[BackgroudBuffer {
                    width: self.size.width as u32,
                    height: self.size.height as u32,
                    block_w: self.block_resolution_size.width as u32,
                    block_h: self.block_resolution_size.height as u32,
                }]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        // bind group layout
        let bg_bind_group_layout =
            gs.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("bg bind group layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });
        // buffer bind group
        let bg_bind_group = gs.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bg bind group"),
            layout: &bg_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(BufferBinding {
                    buffer: &bg_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        let code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/asset/plane.wgsl"));
        let shader = gs
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(code)),
            });
        let pipeline = gs
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(
                    &gs.device.create_pipeline_layout(&PipelineLayoutDescriptor {
                        label: Some("plane pipeline layout"),
                        bind_group_layouts: &[&bg_bind_group_layout],
                        ..Default::default()
                    }),
                ),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    front_face: FrontFace::Ccw,
                    polygon_mode: PolygonMode::Fill,
                    ..Default::default()
                },
                vertex: VertexState {
                    module: &shader,
                    entry_point: "display_vs",
                    buffers: &[],
                    compilation_options: PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "display_fs",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: PipelineCompilationOptions::default(),
                }),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });
        self.running = Some(Running {
            buffer: bg_buffer,
            bind_group: bg_bind_group,
            pipeline,
        })
    }
}

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct BackgroudBuffer {
    width: u32,
    height: u32,
    block_w: u32,
    block_h: u32,
}
