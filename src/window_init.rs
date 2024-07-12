use anyhow::anyhow;
use std::sync::Arc;
use wgpu::{util::RenderEncoder, SurfaceConfiguration};
use winit::{event::WindowEvent, window};

pub use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

use crate::{gfx::GFX, plane::Plane};

enum Start {
    Loading(WindowAttributes),
    Ready(GameLoop),
}

pub struct GameLoop {
    pub window: Arc<Window>,
    pub request_surface: OpenSurface,
    pub gfx: Option<Arc<GFX>>,
}

pub enum OpenSurface {
    NotReady,
    Now(GameSurface),
}
pub struct GameSurface {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
    pub surface_config: Option<wgpu::SurfaceConfiguration>,
}

impl GameLoop {
    pub async fn ready_surface(&mut self) {
        println!("GameLoop ready at window initiating");

        let wgpu_instance = wgpu::Instance::default();
        let surface = wgpu_instance.create_surface(self.window.clone()).unwrap();
        let adapter = wgpu_instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::util::power_preference_from_env()
                    .unwrap_or(wgpu::PowerPreference::HighPerformance),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("No suitable GPU adapters found on the system!");
        let adapter_info = adapter.get_info();
        println!("Using {} ({:?})", adapter_info.name, adapter_info.backend);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|_| anyhow!("Failed to create device"))
            .unwrap();

        self.request_surface = OpenSurface::Now(GameSurface {
            surface,
            device,
            queue,
            adapter,
            surface_config: None,
        });
    }

    fn init_config_surface(&self, window: &Window) -> Option<SurfaceConfiguration> {
        if let OpenSurface::Now(game_surface) = &self.request_surface {
            let device = &game_surface.device;
            let size = window.inner_size();
            let surface_config = game_surface
                .surface
                .get_default_config(&game_surface.adapter, size.width, size.height)
                .unwrap();
            game_surface.surface.configure(device, &surface_config);
            return Some(surface_config);
        } else {
            return None;
        }
    }
}

pub fn enter() {
    let event_loop = EventLoop::new().unwrap();
    let prepare_window_attrs =
        WindowAttributes::default().with_inner_size(PhysicalSize::new(512, 512));
    let mut process = Start::Loading(prepare_window_attrs);
    let _ = event_loop.run_app(&mut process);
}

impl ApplicationHandler for Start {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self {
            Start::Loading(window_attrs) => {
                println!("Loading window");
                let window = event_loop.create_window(window_attrs.clone()).unwrap();
                *self = Start::Ready(GameLoop {
                    request_surface: OpenSurface::NotReady,
                    window: Arc::new(window),
                    gfx: None,
                });

                pollster::block_on(async move {
                    if let Start::Ready(game_loop) = self {
                        println!("Ready to init surface");
                        game_loop.ready_surface().await;
                        let surface_config =
                            game_loop.init_config_surface(&game_loop.window).unwrap();
                        match &mut game_loop.request_surface {
                            OpenSurface::Now(game_surface) => {
                                game_surface.surface_config = Some(surface_config);
                            }
                            _ => {}
                        }
                    } else {
                        println!("Not ready yet! in Loading");
                    }
                });
            }
            _ => {}
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Self::Ready(game_loop) = self {
            match event {
                WindowEvent::Resized(size) => {
                    println!("Resized:: {:?}", size);
                    match &mut game_loop.request_surface {
                        OpenSurface::Now(game_surface) => {
                            let surface_config = game_surface.surface_config.as_mut().unwrap();
                            surface_config.width = size.width;
                            surface_config.height = size.height;
                            game_surface
                                .surface
                                .configure(&game_surface.device, &surface_config);
                        }
                        _ => {}
                    }
                    game_loop.window.request_redraw();
                }
                WindowEvent::RedrawRequested { .. } => {
                    println!("RedrawRequested");
                    let mut plane = Plane::new(&game_loop.window, 50, 50);
                    if let OpenSurface::Now(game_surface) = &game_loop.request_surface {
                        println!("Displaying surface");
                        plane.display(&game_surface);
                        let mut encoder = game_surface.device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor {
                                label: Some("render frame"),
                            },
                        );

                        println!("Creating render pass");
                        let frame = game_surface.surface.get_current_texture().unwrap();
                        let render_target = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        {
                            let mut render_pass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: Some("display pass"),
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &render_target,
                                        resolve_target: None,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    depth_stencil_attachment: None,
                                    ..Default::default()
                                });
                            let running = plane.running.as_ref().unwrap();
                            let bind_group = &running.bind_group;
                            let pipeline = &running.pipeline;

                            render_pass.set_pipeline(&pipeline);
                            render_pass.set_bind_group(0, bind_group, &[]);
                            render_pass.draw(0..6, 0..1);
                            println!("Drawing");
                        };

                        game_surface.queue.submit(Some(encoder.finish()));
                        frame.present();
                    }
                }
                WindowEvent::KeyboardInput {
                    device_id,
                    event,
                    is_synthetic,
                } => {}
                WindowEvent::CloseRequested => {
                    println!("CloseRequested");
                }
                _ => {}
            }
        } else {
            println!("Not ready yet! in Loading");
        }
    }
}
