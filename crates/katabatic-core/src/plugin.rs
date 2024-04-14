use downcast_rs::{impl_downcast, Downcast};
use katabatic_util::error::KResult;

use crate::app::App;

pub trait Plugin: Downcast {
    fn build(&self, app: &mut App) -> KResult<()>;

    #[allow(unused)]
    fn ready(&self, app: &App) -> bool {
        true
    }

    #[allow(unused)]
    fn finish(&self, app: &mut App) -> KResult<()> {
        Ok(())
    }

    #[allow(unused)]
    fn cleanup(&self, app: &mut App) -> KResult<()> {
        Ok(())
    }
}

impl_downcast!(Plugin);
