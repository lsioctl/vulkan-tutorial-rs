use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
   pretty_env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

#[derive(Default)]
struct App {
    window: Option<Window>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Vulkan tutorial with Ash")
                    .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT)),
            )
            .unwrap();

        self.window = Some(window)
    }

    // Called by `EventLoop::run_app` when a new event happens on the window.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => (),
        }
    }
}


