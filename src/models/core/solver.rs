#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneLink
{
    pub bone:i16,
    pub rotate_limited:bool,
    pub minimum_radian:(f32,f32,f32),
    pub maximum_radian:(f32,f32,f32),
}

impl BoneLink
{
    pub fn new() -> Self
    {
        Self
        {
            bone:0,
            rotate_limited:false,
            minimum_radian:(0.0,0.0,0.0),
            maximum_radian:(0.0,0.0,0.0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solver
{
    pub bone:i16,
    pub target_bone:i16,
    pub loop_count:u32,
    pub limited_radian:f32,
    pub chain_length:u32,
    pub links:Vec<BoneLink>
}

impl Solver
{
    pub fn new() -> Self
    {
        Self
        {
            bone:0,
            target_bone:0,
            loop_count:0,
            limited_radian:0.0,
            chain_length:0,
            links:Vec::new(),
        }
    }
}