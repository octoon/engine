use rand::prelude::*;


pub trait OsRandNewV4
{
    fn new_v4_osrng() -> uuid::Uuid;
}

impl OsRandNewV4 for uuid::Uuid
{
    fn new_v4_osrng() -> uuid::Uuid
    {
        let mut rng = rand::rngs::OsRng::new().unwrap();
        let mut bytes = [0; 16];

        rng.fill_bytes(&mut bytes);

        uuid::Builder::from_bytes(bytes).build()
    }
}