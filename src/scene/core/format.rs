#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Format
{
	Undefined,
	RUNorm(u8),
	RSNorm(u8),
	RUScaled(u8),
	RSScaled(u8),
	RUInt(u8),
	RSInt(u8),
	RSFloat(u8),
	RGUNorm(u8,u8),
	RGSNorm(u8,u8),
	RGUScaled(u8,u8),
	RGSScaled(u8,u8),
	RGUInt(u8,u8),
	RGSInt(u8,u8),
	RGSFloat(u8,u8),
	RGBUNorm(u8,u8,u8),
	RGBSNorm(u8,u8,u8),
	RGBUScaled(u8,u8,u8),
	RGBSScaled(u8,u8,u8),
	RGBUInt(u8,u8,u8),
	RGBSInt(u8,u8,u8),
	RGBSRGB(u8,u8,u8),
	RGBSFloat(u8,u8,u8),
	RGBAUNorm(u8,u8,u8,u8),
	RGBASNorm(u8,u8,u8,u8),
	RGBAUScaled(u8,u8,u8,u8),
	RGBASScaled(u8,u8,u8,u8),
	RGBAUInt(u8,u8,u8,u8),
	RGBASInt(u8,u8,u8,u8),
	RGBASRGB(u8,u8,u8,u8),
	RGBASFloat(u8,u8,u8,u8),
	BGRUNorm(u8,u8,u8),
	BGRSNorm(u8,u8,u8),
	BGRUScaled(u8,u8,u8),
	BGRSScaled(u8,u8,u8),
	BGRUInt(u8,u8,u8),
	BGRSInt(u8,u8,u8),
	BGRSRGB(u8,u8,u8),
	BGRSFloat(u8,u8,u8),
	ABGRUNorm(u8,u8,u8,u8),
	ABGRSNorm(u8,u8,u8,u8),
	ABGRUScaled(u8,u8,u8,u8),
	ABGRSScaled(u8,u8,u8,u8),
	ABGRUInt(u8,u8,u8,u8),
	ABGRSInt(u8,u8,u8,u8),
	ABGRSRGB(u8,u8,u8,u8),
	ABGRSFloat(u8,u8,u8,u8),
	SUInt(u8),
	DUNorm(u8, u8),
	DUNormSUInt(u8, u8),
	DSFloat(u8, u8),
	DSFloatSUInt(u8, u8),
	BC1RGBUNormBlock,
	BC1RGBSRGBBlock,
	BC1RGBAUNormBlock,
	BC1RGBASRGBBlock,
	BC2UNormBlock,
	BC2SRGBBlock,
	BC3UNormBlock,
	BC3SRGBBlock,
	BC4UNormBlock,
	BC4SNormBlock,
	BC5UNormBlock,
	BC5SNormBlock,
	BC6HUFloatBlock,
	BC6HSFloatBlock,
	BC7UNormBlock,
	BC7SRGBBlock,
	ETC2R8G8B8UNormBlock,
	ETC2R8G8B8SRGBBlock,
	ETC2R8G8B8A1UNormBlock,
	ETC2R8G8B8A1SRGBBlock,
	ETC2R8G8B8A8UNormBlock,
	ETC2R8G8B8A8SRGBBlock,
	EACR11UNormBlock,
	EACR11SNormBlock,
	EACR11G11UNormBlock,
	EACR11G11SNormBlock,
	ASTCUNormBlock(u8, u8),
	ASTCSRGBBlock(u8, u8),
}

impl Format
{
	pub fn type_size(&self) -> usize
	{
		match self
		{
			Format::Undefined => { 0 },
			Format::RUNorm(size) => { *size as usize },
			Format::RSNorm(size) => { *size as usize },
			Format::RUScaled(size) => { *size as usize },
			Format::RSScaled(size) => { *size as usize },
			Format::RUInt(size) => { *size as usize },
			Format::RSInt(size) => { *size as usize },
			Format::RSFloat(size) => { *size as usize },
			Format::RGUNorm(r,g) => { (r+g) as usize },
			Format::RGSNorm(r,g) => { (r+g) as usize },
			Format::RGUScaled(r,g) => { (r+g) as usize },
			Format::RGSScaled(r,g) => { (r+g) as usize },
			Format::RGUInt(r,g) => { (r+g) as usize },
			Format::RGSInt(r,g) => { (r+g) as usize },
			Format::RGSFloat(r,g) => { (r+g) as usize },
			Format::RGBUNorm(r,g,b) => { (r+g+b) as usize },
			Format::RGBSNorm(r,g,b) => { (r+g+b) as usize },
			Format::RGBUScaled(r,g,b) => { (r+g+b) as usize },
			Format::RGBSScaled(r,g,b) => { (r+g+b) as usize },
			Format::RGBUInt(r,g,b) => { (r+g+b) as usize },
			Format::RGBSInt(r,g,b) => { (r+g+b) as usize },
			Format::RGBSRGB(r,g,b) => { (r+g+b) as usize },
			Format::RGBSFloat(r,g,b) => { (r+g+b) as usize },
			Format::RGBAUNorm(r,g,b,a) => { (r+g+b+a) as usize },
			Format::RGBASNorm(r,g,b,a) => { (r+g+b+a) as usize },
			Format::RGBAUScaled(r,g,b,a) => { (r+g+b+a) as usize },
			Format::RGBASScaled(r,g,b,a) => { (r+g+b+a) as usize },
			Format::RGBAUInt(r,g,b,a) => { (r+g+b+a) as usize },
			Format::RGBASInt(r,g,b,a) => { (r+g+b+a) as usize },
			Format::RGBASRGB(r,g,b,a) => { (r+g+b+a) as usize },
			Format::RGBASFloat(r,g,b,a) => { (r+g+b+a) as usize },
			Format::BGRUNorm(r,g,b) => { (r+g+b) as usize },
			Format::BGRSNorm(r,g,b) => { (r+g+b) as usize },
			Format::BGRUScaled(r,g,b) => { (r+g+b) as usize },
			Format::BGRSScaled(r,g,b) => { (r+g+b) as usize },
			Format::BGRUInt(r,g,b) => { (r+g+b) as usize },
			Format::BGRSInt(r,g,b) => { (r+g+b) as usize },
			Format::BGRSRGB(r,g,b) => { (r+g+b) as usize },
			Format::BGRSFloat(r,g,b) => { (r+g+b) as usize },
			Format::ABGRUNorm(r,g,b,a) => { (r+g+b+a) as usize },
			Format::ABGRSNorm(r,g,b,a) => { (r+g+b+a) as usize },
			Format::ABGRUScaled(r,g,b,a) => { (r+g+b+a) as usize },
			Format::ABGRSScaled(r,g,b,a) => { (r+g+b+a) as usize },
			Format::ABGRUInt(r,g,b,a) => { (r+g+b+a) as usize },
			Format::ABGRSInt(r,g,b,a) => { (r+g+b+a) as usize },
			Format::ABGRSRGB(r,g,b,a) => { (r+g+b+a) as usize },
			Format::ABGRSFloat(r,g,b,a) => { (r+g+b+a) as usize },
			Format::SUInt(s) => { *s as usize },
			Format::DUNorm(d,s) => { (d+s) as usize },
			Format::DUNormSUInt(d,s) => { (d+s) as usize },
			Format::DSFloat(d,s) => { (d+s) as usize },
			Format::DSFloatSUInt(d,s) => { (d+s) as usize },
			_ => { panic!("Invalid enum"); }
		}
	}

	pub fn count(&self) -> u8
	{
		match self
		{
			Format::Undefined => { 0 },
			Format::RUNorm(_) => { 1 },
			Format::RSNorm(_) => { 1 },
			Format::RUScaled(_) => { 1 },
			Format::RSScaled(_) => { 1 },
			Format::RUInt(_) => { 1 },
			Format::RSInt(_) => { 1 },
			Format::RSFloat(_) => { 1 },
			Format::RGUNorm(_,_) => { 2 },
			Format::RGSNorm(_,_) => { 2 },
			Format::RGUScaled(_,_) => { 2 },
			Format::RGSScaled(_,_) => { 2 },
			Format::RGUInt(_,_) => { 2 },
			Format::RGSInt(_,_) => { 2 },
			Format::RGSFloat(_,_) => { 2 },
			Format::RGBUNorm(_,_,_) => { 3 },
			Format::RGBSNorm(_,_,_) => { 3 },
			Format::RGBUScaled(_,_,_) => { 3 },
			Format::RGBSScaled(_,_,_) => { 3 },
			Format::RGBUInt(_,_,_) => { 3 },
			Format::RGBSInt(_,_,_) => { 3 },
			Format::RGBSRGB(_,_,_) => { 3 },
			Format::RGBSFloat(_,_,_) => { 3 },
			Format::RGBAUNorm(_,_,_,_) => { 4 },
			Format::RGBASNorm(_,_,_,_) => { 4 },
			Format::RGBAUScaled(_,_,_,_) => { 4 },
			Format::RGBASScaled(_,_,_,_) => { 4 },
			Format::RGBAUInt(_,_,_,_) => { 4 },
			Format::RGBASInt(_,_,_,_) => { 4 },
			Format::RGBASRGB(_,_,_,_) => { 4 },
			Format::RGBASFloat(_,_,_,_) => { 4 },
			Format::BGRUNorm(_,_,_) => { 3 },
			Format::BGRSNorm(_,_,_) => { 3 },
			Format::BGRUScaled(_,_,_) => { 3 },
			Format::BGRSScaled(_,_,_) => { 3 },
			Format::BGRUInt(_,_,_) => { 3 },
			Format::BGRSInt(_,_,_) => { 3 },
			Format::BGRSRGB(_,_,_) => { 3 },
			Format::BGRSFloat(_,_,_) => { 3 },
			Format::ABGRUNorm(_,_,_,_) => { 4 },
			Format::ABGRSNorm(_,_,_,_) => { 4 },
			Format::ABGRUScaled(_,_,_,_) => { 4 },
			Format::ABGRSScaled(_,_,_,_) => { 4 },
			Format::ABGRUInt(_,_,_,_) => { 4 },
			Format::ABGRSInt(_,_,_,_) => { 4 },
			Format::ABGRSRGB(_,_,_,_) => { 4 },
			Format::ABGRSFloat(_,_,_,_) => { 4 },
			Format::SUInt(_) => { 1 },
			Format::DUNorm(_,_) => { 2 },
			Format::DUNormSUInt(_,_) => { 2 },
			Format::DSFloat(_,_) => { 2 },
			Format::DSFloatSUInt(_,_) => { 2 },
			_ => { panic!("Invalid enum"); }
		}
	}
}