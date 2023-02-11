use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::{Window, WindowBuilder},
};

pub struct Application;

impl Application {
	pub fn new() -> Self {
		Self {}
	}

	pub fn run(self) {
		let (event_loop, window) = create_window();

		event_loop.run(move |event, _, control_flow| {
			*control_flow = ControlFlow::Wait;

			match event {
				Event::WindowEvent {
					event: WindowEvent::CloseRequested,
					window_id,
				} if window_id == window.id() => *control_flow = ControlFlow::Exit,
				_ => (),
			}
		});
	}
}

fn create_window() -> (EventLoop<()>, Window) {
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new().build(&event_loop).unwrap();
	(event_loop, window)
}
