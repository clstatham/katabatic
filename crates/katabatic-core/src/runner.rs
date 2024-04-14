use katabatic_util::error::KResult;

use crate::app::App;

pub trait Runner {
    fn run(&mut self, app: &mut App) -> KResult<()>;
}

#[derive(Default)]
pub struct DefaultRunner;

impl Runner for DefaultRunner {
    fn run(&mut self, _app: &mut App) -> KResult<()> {
        Ok(())
    }
}
