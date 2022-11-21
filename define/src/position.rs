use core::Describe;


#[derive(Debug, Clone, Copy)]
#[derive(Describe)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

