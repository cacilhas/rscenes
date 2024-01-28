pub use crate::raudio::Raudio;
pub use crate::rcamera::Rcamera;
pub use crate::rcore::Rcore;
pub use crate::rgestures::Rgestures;
pub use crate::rmodels::Rmodels;
pub use crate::rshapes::Rshapes;
pub use crate::rtext::Rtext;
pub use crate::rtextures::Rtextures;

#[derive(Clone, Copy, Debug, Default)]
pub struct RaylibConnector {
    pub rcore: Rcore,
    pub rgestures: Rgestures,
    pub rcamera: Rcamera,
    pub rshapes: Rshapes,
    pub rtextures: Rtextures,
    pub rtext: Rtext,
    pub rmodels: Rmodels,
    pub raudio: Raudio,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::{Any, TypeId};

    #[test]
    fn should_export_rcore() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rcore.type_id(), TypeId::of::<Rcore>())
    }

    #[test]
    fn should_export_rgestures() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rgestures.type_id(), TypeId::of::<Rgestures>())
    }

    #[test]
    fn should_export_rcamera() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rcamera.type_id(), TypeId::of::<Rcamera>())
    }

    #[test]
    fn should_export_rshapes() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rshapes.type_id(), TypeId::of::<Rshapes>())
    }

    #[test]
    fn should_export_rtextures() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rtextures.type_id(), TypeId::of::<Rtextures>())
    }

    #[test]
    fn should_export_rtext() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rtext.type_id(), TypeId::of::<Rtext>())
    }

    #[test]
    fn should_export_rmodels() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.rmodels.type_id(), TypeId::of::<Rmodels>())
    }

    #[test]
    fn should_export_raudio() {
        let renderer = RaylibConnector::default();
        assert_eq!(renderer.raudio.type_id(), TypeId::of::<Raudio>())
    }
}
