//! `config_structs` is a collection of
//! all structs used for creating a
//! struct from `returned_structs`

use crate::util_structs::{
    AFImage,
    AFSize2D,
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
