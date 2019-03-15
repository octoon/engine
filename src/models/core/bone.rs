#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bone
{
    pub name:String,
    pub parent:Option<u16>,
    pub position:(f32,f32,f32),
    pub rotation:(f32,f32,f32),
}

impl Bone
{
    pub fn new() -> Self
    {
        Self
        {
            name:String::new(),
            parent:None,
            position:(0.0,0.0,0.0),
            rotation:(0.0,0.0,0.0),
        }
    }
}