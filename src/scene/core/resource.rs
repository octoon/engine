pub trait Resource
{
	fn uuid(&self) -> &uuid::Uuid;
}