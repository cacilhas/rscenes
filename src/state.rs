#[derive(Debug)]
pub enum State {
    Keep,
    New(Box<dyn crate::scene::Scene>),
    Previous(usize),
}
