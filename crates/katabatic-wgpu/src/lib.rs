use std::sync::Arc;

use katabatic_core::{app::App, plugin::Plugin};
use katabatic_scene::node::Node;
use katabatic_util::error::KResult;
use katabatic_winit::WinitPlugin;
use winit::window::Window;

pub(crate) struct WgpuPluginInner {
    pub(crate) surface: Arc<wgpu::Surface>,
    pub(crate) device: Arc<wgpu::Device>,
    pub(crate) queue: Arc<wgpu::Queue>,
}

#[derive(Default)]
pub struct WgpuPlugin {
    pub(crate) inner: Option<WgpuPluginInner>,
}

impl WgpuPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn surface(&self) -> &Arc<wgpu::Surface> {
        &self.inner.as_ref().unwrap().surface
    }

    pub fn device(&self) -> &Arc<wgpu::Device> {
        &self.inner.as_ref().unwrap().device
    }

    pub fn queue(&self) -> &Arc<wgpu::Queue> {
        &self.inner.as_ref().unwrap().queue
    }
}

impl Plugin for WgpuPlugin {
    fn build(&mut self, app: &mut App) -> KResult<()> {
        let winit_plugin = app
            .get_plugin::<WinitPlugin>()
            .expect("WgpuPlugin::build(): Winit plugin not present");
        let window_id = winit_plugin
            .window_id()
            .expect("WgpuPlugin::build(): Winit plugin not initialized");

        app.world()
            .with_node(window_id.node_id, |window_node| match window_node {
                Node::Data(window_data) => {
                    let window = window_data.downcast_ref::<Window>().unwrap();

                    let instance = wgpu::Instance::default();

                    let surface = unsafe { instance.create_surface(window) }.unwrap();

                    let adapter = pollster::block_on(instance.request_adapter(
                        &wgpu::RequestAdapterOptions {
                            power_preference: wgpu::PowerPreference::HighPerformance,
                            force_fallback_adapter: false,
                            compatible_surface: Some(&surface),
                        },
                    ))
                    .expect("WgpuPlugin::build(): Error requesting adapter");

                    let (device, queue) = pollster::block_on(adapter.request_device(
                        &wgpu::DeviceDescriptor {
                            label: Some("Katabatic Engine Main Device"),
                            features: wgpu::Features::all_webgpu_mask(),
                            limits: wgpu::Limits::downlevel_defaults(),
                        },
                        None,
                    ))
                    .expect("WgpuPlugin::build(): Error requesting device");

                    let surface_caps = surface.get_capabilities(&adapter);

                    let config = wgpu::SurfaceConfiguration {
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                            | wgpu::TextureUsages::COPY_DST,
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        width: window.inner_size().width,
                        height: window.inner_size().height,
                        present_mode: wgpu::PresentMode::AutoNoVsync,
                        alpha_mode: surface_caps.alpha_modes[0],
                        view_formats: vec![],
                    };
                    surface.configure(&device, &config);

                    self.inner = Some(WgpuPluginInner {
                        surface: Arc::new(surface),
                        device: Arc::new(device),
                        queue: Arc::new(queue),
                    });
                }
                Node::Scene(_) => {
                    unreachable!("WgpuPlugin::build(): Window node is not a Data node")
                }
            });

        Ok(())
    }
}
