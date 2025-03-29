use std::sync::Arc;
use wgpu::{Device, DeviceDescriptor, Features, Instance, Limits, MemoryHints, MultisampleState, PowerPreference, PresentMode, Queue, RequestAdapterOptions};
use winit::window::Window;
use crate::TextEngine;

pub struct WgpuState {
    pub surface: wgpu::Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub render_pipeline: wgpu::RenderPipeline,
    pub config: wgpu::SurfaceConfiguration,
    pub current_size: winit::dpi::PhysicalSize<u32>,
    pub text_engine: TextEngine,
}

impl WgpuState {
    pub fn default(window: Arc<Window>) -> WgpuState {
        let instance = Instance::default();
        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
            .expect("Failed to retrieve adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                required_features: Features::default(),
                required_limits: Limits::default(),
                label: None,
                memory_hints: MemoryHints::default(),
            },
            None,
        ))
            .expect("Failed to request device");

        let capabilities = surface.get_capabilities(&adapter);

        let surface_format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: capabilities
                .present_modes
                .iter()
                .copied()
                .find(|&mode| mode == PresentMode::Immediate)
                .unwrap_or(PresentMode::Fifo),
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let text_engine = TextEngine::default(&device, &queue, &window);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        window.set_visible(true);

        Self {
            surface,
            device,
            queue,
            render_pipeline,
            config,
            current_size: window.inner_size(),
            text_engine,
        }
    }
}