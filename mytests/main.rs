extern crate AlfredGF_rs;

use AlfredGF_rs::{
    util_structs::{
        AFSize2D,
        AFImage,
    },
    config_structs::{
        AFWindowConfig,
    },
    returned_structs::{
        AFWindow,
    },
    enums::{
        //
    },
};

pub fn main(){
    let size: AFSize2D = AFSize2D {
        width: 1280, height: 720
    };
    let config: AFWindowConfig = AFWindowConfig{
        icon: None,
        start_size: size,
        min_size: size,
        max_size: size,
        resizeable: false,
        visible: true,
        always_on_top: false,
        maximized: false,
        title: "Test",
    };
    let window: AFWindow = AFWindow::new(&config);
}
