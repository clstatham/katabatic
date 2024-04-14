use std::cell::Cell;

use katabatic_core::{app::App, plugin::Plugin};
use katabatic_scene::id::Id;
use katabatic_util::error::KResult;
use winit::{event_loop::EventLoopBuilder, window::Window};

pub struct WinitPlugin {
    event_loop_id: Cell<Option<Id>>,
    window_id: Cell<Option<Id>>,
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
}

impl Plugin for WinitPlugin {
    fn build(&self, app: &mut App) -> KResult<()> {
        let event_loop = EventLoopBuilder::new()
            .build()
            .expect("WinitPlugin::build(): Error creating event loop");

        let window = Window::new(&event_loop).expect("WinitPlugin::build(): Error creating window");

        let event_loop_id = app.world().with_root_mut(|root| root.add_data(event_loop));

        self.event_loop_id.set(Some(event_loop_id));

        let window_id = app.world().with_root_mut(|root| root.add_data(window));

        self.window_id.set(Some(window_id));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katabatic_core::app::App;

    #[test]
    fn test_render_plugin() {
        App::new()
            .add_plugin(WinitPlugin::new())
            .unwrap()
            .run()
            .unwrap();
    }
}
