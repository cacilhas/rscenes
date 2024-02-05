use crate::scene::Scene;

#[derive(Debug)]
pub enum State {
    /// Keep running the current scene
    Keep,
    /// Add a new scene and run it
    Next(Box<dyn Scene>),
    /// Get back to the previous scene (or quit if there's no one)
    Prev(usize),
    /// Quit
    Quit,
}
