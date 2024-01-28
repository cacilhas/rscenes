use crate::scene::Scene;

#[derive(Debug)]
pub enum State {
    Keep,
    Next(Box<dyn Scene>),
    Prev,
}
