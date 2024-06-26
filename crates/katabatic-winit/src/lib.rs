use std::cell::Cell;

use katabatic_core::{app::App, plugin::Plugin, runner::Runner};
use katabatic_scene::node::Node;
use katabatic_util::error::KResult;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::Window,
};

pub struct WinitPlugin {
    event_loop_id: Cell<Option<Node>>,
    window_id: Cell<Option<Node>>,
}

impl Default for WinitPlugin {
    fn default() -> Self {
        Self {
            event_loop_id: Cell::new(None),
            window_id: Cell::new(None),
        }
    }
}

impl WinitPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn event_loop_id(&self) -> Option<Node> {
        self.event_loop_id.get()
    }

    pub fn window_id(&self) -> Option<Node> {
        self.window_id.get()
    }
}

impl Plugin for WinitPlugin {
    fn build(&mut self, app: &mut App) -> KResult<()> {
        let event_loop = EventLoopBuilder::new().build();

        let window = Window::new(&event_loop).expect("WinitPlugin::build(): Error creating window");

        let event_loop_id = app.root_scene().write().create_node_with(event_loop);

        self.event_loop_id.set(Some(event_loop_id));

        let window_id = app.root_scene().write().create_node_with(window);

        self.window_id.set(Some(window_id));

        app.set_runner(WinitRunner);

        Ok(())
    }
}

#[derive(Default)]
pub struct WinitRunner;

impl Runner for WinitRunner {
    fn run(&mut self, app: App) -> KResult<()> {
        let plugin = app
            .get_plugin::<WinitPlugin>()
            .expect("WinitRunner::run(): WinitPlugin not present in App");

        let event_loop_id = plugin
            .event_loop_id()
            .expect("WinitRunner::run(): Winit event loop not initialized");

        let event_loop = app
            .world()
            .write()
            .remove_component::<EventLoop<()>>(event_loop_id.entity)
            .unwrap();

        app.run_init_hooks()?;

        event_loop.run(move |event, _window, control_flow| match event {
            Event::WindowEvent { event, .. } => {
                if event == WindowEvent::CloseRequested {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::DeviceEvent { event: _, .. } => {}
            Event::RedrawRequested(_) => {
                app.run_update_hooks().unwrap();
                app.run_render_hooks().unwrap();
            }
            Event::LoopDestroyed => {
                app.run_cleanup_hooks().unwrap();
            }
            _ => {}
        });
    }
}
