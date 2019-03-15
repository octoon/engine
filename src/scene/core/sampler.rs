use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

use super::{Resource};
use super::super::util::uuid::OsRandNewV4;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SamplerAnis
{
	Anis0,
	Anis1,
	Anis2,
	Anis4,
	Anis8,
	Anis16,
	Anis32,
	Anis64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SamplerWrap
{
	None,
	Repeat,
	Mirror,
	ClampToEdge,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SamplerFilter
{
	Nearest,
	Linear,
	NearestMipmapLinear,
	NearestMipmapNearest,
	LinearMipmapNearest,
	LinearMipmapLinear,
}

#[derive(Copy, PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Sampler
{
	pub anis:SamplerAnis,
	pub wrap:SamplerWrap,
	pub filter_min:SamplerFilter,
	pub filter_mag:SamplerFilter,
	uuid:uuid::Uuid
}

impl Sampler
{
	pub fn new(wrap:SamplerWrap,filter_min:SamplerFilter, filter_mag:SamplerFilter) -> Self
	{
		Self
		{
			uuid:uuid::Uuid::new_v4_osrng(),
			anis:SamplerAnis::Anis0,
			wrap:wrap,
			filter_min:filter_min,
			filter_mag:filter_mag,
		}
	}
}

impl Resource for Sampler
{
	#[inline(always)]
	fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}
}

impl From<Sampler> for Rc<RefCell<Sampler>>
{
	#[inline(always)]
	fn from(sampler:Sampler) -> Self
	{
		Rc::new(RefCell::new(sampler))
	}
}

impl From<Sampler> for Arc<RefCell<Sampler>>
{
	#[inline(always)]
	fn from(sampler:Sampler) -> Self
	{
		Arc::new(RefCell::new(sampler))
	}
}