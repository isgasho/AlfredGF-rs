use crate::util_structs::{
    AFImage,
    AFSize2D,
};
use crate::enums::{
    AFBackendLibrary,
};

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
    pub title: &'a str,

}

#[derive(Copy, Clone)]
pub struct AFContextConfig {

    pub vsync: bool,
    pub anisotropic_filtering: bool,
    pub backend_lib: AFBackendLibrary,

}
