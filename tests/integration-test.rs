extern crate AlfredGF_rs;

use AlfredGF_rs::{
    AFSize2D,
    AFImage,
    AFWindowConfig,
};

#[test]
fn test() -> () {
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
    };
}
