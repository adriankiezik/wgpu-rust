// #![windows_subsystem = "windows"]
use winit::event_loop::{ControlFlow, EventLoop};
use ferrite_engine::App;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().expect("Failed to create event loop");

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();

    event_loop
        .run_app(&mut app)
        .expect("Failed to start event loop");
}
