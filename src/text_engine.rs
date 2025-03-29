use glyphon::{Attrs, Buffer, Cache, Family, FontSystem, Metrics, Shaping, SwashCache, TextAtlas, TextRenderer, Viewport};
use wgpu::{Device, MultisampleState, Queue, TextureFormat};
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
    pub fn default(device: &Device, queue: &Queue, window: &Window) -> Self {
        let mut font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(&device);
        let viewport = Viewport::new(&device, &cache);
        let mut atlas = TextAtlas::new(&device, &queue, &cache, TextureFormat::Bgra8UnormSrgb);
        let text_renderer =
            TextRenderer::new(&mut atlas, &device, MultisampleState::default(), None);
        let mut text_buffer = Buffer::new(&mut font_system, Metrics::new(30.0, 42.0));

        let physical_width = (window.inner_size().width as f64 * window.scale_factor()) as f32;
        let physical_height = (window.inner_size().height as f64 * window.scale_factor()) as f32;

        text_buffer.set_size(
            &mut font_system,
            Some(physical_width),
            Some(physical_height),
        );
        text_buffer.set_text(&mut font_system, "Hello world!", Attrs::new().family(Family::SansSerif), Shaping::Advanced);
        text_buffer.shape_until_scroll(&mut font_system, false);


        Self {
            font_system,
            swash_cache,
            viewport,
            atlas,
            text_renderer,
            text_buffer,
        }
    }
}