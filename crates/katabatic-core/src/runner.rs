use katabatic_util::error::KResult;

use crate::app::App;

pub trait Runner {
    fn run(&mut self, app: App) -> KResult<()>;
}

#[derive(Default)]
pub struct NoOpRunner;

impl Runner for NoOpRunner {
    fn run(&mut self, _app: App) -> KResult<()> {
        Ok(())
    }
}

#[allow(unused)]
pub trait Hook: 'static {
    fn init(&self, app: &App) -> KResult<()> {
        Ok(())
    }

    fn update(&self, app: &App) -> KResult<()> {
        Ok(())
    }

    fn render(&self, app: &App) -> KResult<()> {
        Ok(())
    }

    fn cleanup(&self, app: &App) -> KResult<()> {
        Ok(())
    }
}
