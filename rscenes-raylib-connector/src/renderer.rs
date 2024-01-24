use crate::rcore::Rcore;

#[derive(Clone, Copy, Debug)]
pub struct Renderer {
    pub rcore: Rcore,
}

impl Renderer {
    pub(crate) fn new() -> Self {
        Self {
            rcore: Rcore::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use super::*;

    #[test]
    fn should_export_rcore() {
        let renderer = Renderer::new();
        assert_eq!(renderer.rcore.type_id(), TypeId::of::<Rcore>())
    }
}
