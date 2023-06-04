/// Used to tell the manager what to do next.
#[derive(Debug)]
pub enum State<R> {
    /// Keep tells to keep the current scene running.
    Keep,

    /// New adds a new scene to the stack, switching to it.
    New(Box<dyn crate::scene::Scene<R>>),

    /// Previous unstack `usize` scenes.
    Previous(usize),
}
