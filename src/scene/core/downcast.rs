use std::any::Any;

pub trait Downcast: Any
{
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
}