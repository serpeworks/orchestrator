
pub trait System {
    fn observe(&mut self);
    fn affect(&mut self);
}

