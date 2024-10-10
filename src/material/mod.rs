/// A [Material] can change the state of another [Material] by touching it
/// The state of the [Material] can change with the [Status] received
/// Maybe [Material] should not be an enum but a trait
pub trait Material {
    fn apply_material(&mut self, material: impl Material);
    fn status(&self) -> Status;
}

/// Un élément communique son effet à un matériau
/// Le matériau peut changer d'état avec l'effet reçu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Burning,
    Wet,
    None,
}
