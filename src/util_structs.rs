#[derive(Copy, Clone)]
pub struct AFSize2D<T> {
    pub width: T,
    pub height: T,
}

#[derive(Copy, Clone)]
pub struct AFImage<'a> {
    pub size: AFSize2D<u32>,
    pub data: &'a [u8],
}
