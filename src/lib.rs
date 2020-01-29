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

#[derive(Copy, Clone)]
pub struct AFWindowConfig<'a> {

    pub icon: Option<&'a AFImage<'a>>,
    pub start_size: AFSize2D,
    pub min_size: AFSize2D,
    pub max_size: AFSize2D,
    pub resizeable: bool,
    pub visible: bool,
    pub always_on_top: bool,
    pub maximized: bool,

}
