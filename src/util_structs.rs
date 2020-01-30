#[derive(Copy, Clone)]
pub struct AFSize2D {
    pub width: u32,
    pub height: u32,
}

#[derive(Copy, Clone)]
pub struct AFImage<'a> {
    pub size: AFSize2D,
    pub data: &'a [u8],
}
