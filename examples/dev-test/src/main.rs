use std::error::Error;

use katabatic::{core::app::App, winit::WinitPlugin};

fn main() -> Result<(), Box<dyn Error>> {
    App::new().add_plugin(WinitPlugin::new())?.run()?;
    Ok(())
}
