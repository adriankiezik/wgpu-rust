use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};
use crate::WgpuState;

enum AppState {
    Uninitialized,
    Initialized {
        window: Arc<Window>,
        wgpu_state: WgpuState,
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

        let wgpu_state = WgpuState::default(window.clone());

        self.state = AppState::Initialized { window, wgpu_state };
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if let AppState::Initialized {
            window: _window,
            wgpu_state,
        } = &self.state
        {
            let surface_texture = wgpu_state.surface.get_current_texture()?;

            let texture_view = surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let mut command_encoder =
                wgpu_state
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

            {
                let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

                render_pass.set_pipeline(&wgpu_state.render_pipeline);
                render_pass.draw(0..3, 0..1);
            }

            wgpu_state
                .queue
                .submit(std::iter::once(command_encoder.finish()));
            surface_texture.present();
        }

        Ok(())
    }

    fn resize_surface(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if let AppState::Initialized { wgpu_state, .. } = &mut self.state {
            wgpu_state.current_size = new_size;
            wgpu_state.config.width = new_size.width;
            wgpu_state.config.height = new_size.height;
            wgpu_state
                .surface
                .configure(&wgpu_state.device, &wgpu_state.config);
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