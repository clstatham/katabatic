use std::error::Error;

use katabatic::{core::app::App, wgpu::WgpuPlugin, winit::WinitPlugin};

fn main() -> Result<(), Box<dyn Error>> {
    App::new()
        .add_plugin(WinitPlugin::new())?
        .add_plugin(WgpuPlugin::new())?
        .run()?;
    Ok(())
}
