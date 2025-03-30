use crate::{FrameStats, GpuContext, TextEngine};
use glyphon::{Color, Resolution};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

enum AppState {
    Uninitialized,
    Initialized {
        window: Arc<Window>,
        gpu_context: GpuContext,
        frame_stats: FrameStats,
        text_engine: TextEngine,
    },
}

pub struct App {
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Uninitialized,
        }
    }

    fn initialize(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Ferrite Engine")
                        .with_visible(false)
                        .with_min_inner_size(LogicalSize::new(800, 600)),
                )
                .expect("Failed to create window"),
        );

        let gpu_context = GpuContext::default(window.clone());
        let text_engine = TextEngine::default(&gpu_context, &window);

        self.state = AppState::Initialized {
            window,
            gpu_context,
            frame_stats: FrameStats::new(),
            text_engine,
        };
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if let AppState::Initialized {
            window: _window,
            gpu_context,
            frame_stats,
            text_engine,
        } = &mut self.state
        {
            text_engine.viewport.update(
                &gpu_context.queue,
                Resolution {
                    width: gpu_context.current_size.width,
                    height: gpu_context.current_size.height,
                },
            );

            text_engine.draw_text(
                &gpu_context,
                &*format!("{:.0} fps", frame_stats.fps).to_string(),
                15.0,
                0.0,
                20.0,
                Color::rgb(100, 255, 100),
            );

            let surface_texture = gpu_context.surface.get_current_texture()?;

            let texture_view = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut command_encoder =
                gpu_context
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Pass Encoder"),
                    });

            {
                let mut render_pass =
                    command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &texture_view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        occlusion_query_set: None,
                        timestamp_writes: None,
                    });

                render_pass.set_pipeline(&gpu_context.render_pipeline);
                render_pass.draw(0..3, 0..1);

                text_engine
                    .text_renderer
                    .render(&text_engine.atlas, &text_engine.viewport, &mut render_pass)
                    .unwrap();
            }

            gpu_context
                .queue
                .submit(std::iter::once(command_encoder.finish()));
            surface_texture.present();

            text_engine.atlas.trim();

            frame_stats.update();
        }

        Ok(())
    }

    fn resize_surface(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if let AppState::Initialized { gpu_context, .. } = &mut self.state {
            gpu_context.current_size = new_size;
            gpu_context.config.width = new_size.width;
            gpu_context.config.height = new_size.height;
            gpu_context
                .surface
                .configure(&gpu_context.device, &gpu_context.config);
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let AppState::Uninitialized = self.state {
            self.initialize(event_loop);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(value) => self.resize_surface(value),
            WindowEvent::RedrawRequested => {
                if let AppState::Initialized { .. } = &self.state {
                    let result = self.render();
                    if let Err(err) = result {
                        println!("Failed to render: {:?}", err);
                    }
                }

                if let AppState::Initialized { window, .. } = &mut self.state {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}
