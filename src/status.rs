#[derive(Debug)]
pub enum Status {
    Keep,
    New(Box<dyn crate::scene::Scene>),
    Previous(usize),
}
