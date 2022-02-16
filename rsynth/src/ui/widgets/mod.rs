use super::Context;

pub mod text;
pub mod button;

pub trait Widget {
    fn render(&mut self, context: &mut Context);
    fn input(&mut self);
}
