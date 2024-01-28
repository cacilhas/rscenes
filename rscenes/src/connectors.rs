use rscenes_raylib_connector::interface::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Connector2D {
    pub rcore: Rcore,
    pub rgestures: Rgestures,
    pub rshapes: Rshapes,
    pub rtextures: Rtextures,
    pub rtext: Rtext,
    pub raudio: Raudio,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Connector3D {
    pub rcore: Rcore,
    pub rgestures: Rgestures,
    pub rcamera: Rcamera,
    pub rtextures: Rtextures,
    pub rmodels: Rmodels,
    pub raudio: Raudio,
}
