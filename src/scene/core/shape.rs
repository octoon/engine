use std::sync::Arc;
use super::{Object, Geometry, Material};

pub trait Shape : Object
{
    fn geometry(&self) -> Arc<Geometry + 'static>;
    fn material(&self) -> Arc<Material + 'static>;

    fn set_geometry(&mut self, geometry: Arc<Geometry + 'static>);
    fn set_material(&mut self, material: Arc<Material + 'static>);
}