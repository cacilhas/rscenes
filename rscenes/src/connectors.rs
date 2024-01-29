use rscenes_raylib_connector::interface::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct PlainConnector;

impl Rcore for PlainConnector {}
impl Rgestures for PlainConnector {}
impl Rcamera for PlainConnector {}
impl Rtextures for PlainConnector {}
impl Rtext for PlainConnector {}
impl Raudio for PlainConnector {}

#[derive(Clone, Copy, Debug, Default)]
pub struct Connector2D;

impl Rcore for Connector2D {}
impl Rgestures for Connector2D {}
impl Rshapes for Connector2D {}
impl Rtextures for Connector2D {}
impl Rtext for Connector2D {}
impl Raudio for Connector2D {}

#[derive(Clone, Copy, Debug, Default)]
pub struct Connector3D;

impl Rcore for Connector3D {}
impl Rgestures for Connector3D {}
impl Rcamera for Connector3D {}
impl Rtextures for Connector3D {}
impl Rmodels for Connector3D {}
impl Raudio for Connector3D {}
