use eldritch::app::Application;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	Application::new().run()
}
