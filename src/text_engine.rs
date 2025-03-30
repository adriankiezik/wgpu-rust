use crate::GpuContext;
use glyphon::{
    Attrs, Buffer, Cache, Color, Family, FontSystem, Metrics, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer, Viewport,
};
use wgpu::{MultisampleState, TextureFormat};
use winit::window::Window;

pub struct TextEngine {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub viewport: Viewport,
    pub atlas: TextAtlas,
    pub text_renderer: TextRenderer,
    pub text_buffer: Buffer,
}

impl TextEngine {
    pub fn default(gpu_context: &GpuContext, window: &Window) -> Self {
        let mut font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(&gpu_context.device);
        let viewport = Viewport::new(&gpu_context.device, &cache);
        let mut atlas = TextAtlas::new(
            &gpu_context.device,
            &gpu_context.queue,
            &cache,
            TextureFormat::Bgra8UnormSrgb,
        );
        let text_renderer = TextRenderer::new(
            &mut atlas,
            &gpu_context.device,
            MultisampleState::default(),
            None,
        );
        let scale_factor = window.scale_factor() as f32;
        let mut text_buffer = Buffer::new(
            &mut font_system,
            Metrics::new(24.0 * scale_factor, 42.0 * scale_factor),
        );

        Self {
            font_system,
            swash_cache,
            viewport,
            atlas,
            text_renderer,
            text_buffer,
        }
    }

    pub fn draw_text(
        &mut self,
        gpu_context: &GpuContext,
        text: &str,
        x: f32,
        y: f32,
        font_size: f32,
        color: Color,
    ) {
        self.text_buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );
        self.text_buffer
            .shape_until_scroll(&mut self.font_system, false);

        self.text_renderer
            .prepare(
                &gpu_context.device,
                &gpu_context.queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                [TextArea {
                    buffer: &self.text_buffer,
                    left: x,
                    top: y,
                    scale: font_size / 24.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: 600,
                        bottom: 600,
                    },
                    default_color: color,
                    custom_glyphs: &[],
                }],
                &mut self.swash_cache,
            )
            .expect("Text renderer failed to prepare");
    }
}
