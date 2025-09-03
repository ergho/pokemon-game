/// A generic trait for any type of in-game encounter
pub trait Encounter {
    /// Process a single turn or tick of the encounter
    fn process_turn(&mut self);

    /// Check if the encounter is over
    fn is_over(&self) -> bool;
}
