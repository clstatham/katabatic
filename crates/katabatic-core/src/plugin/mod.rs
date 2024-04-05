use crate::{app::App, error::KResult};

pub trait Plugin {
    fn build(&self, app: &mut App) -> KResult<()>;
}
