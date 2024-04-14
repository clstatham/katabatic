use crate::{data::Data, scene::Scene};

pub enum Node {
    Data(Data),
    Scene(Scene),
}
