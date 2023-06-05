/// Used to tell the manager what to do next.
#[derive(Debug)]
pub enum State<R> {
    /// Tells to keep the current scene running.
    Keep,

    /// Adds a new scene to the stack, switching to it.
    New(Box<dyn crate::scene::Scene<R>>),

    /// Unstacks `usize` scenes.
    Previous(usize),
}
