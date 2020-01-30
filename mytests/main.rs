extern crate AlfredGF_rs;

use AlfredGF_rs::{
    util_structs::{
        AFSize2D,
        AFImage,
    },
    config_structs::{
        AFWindowConfig,
        AFContextConfig,
    },
    constructors::{
        AFWindowConstructor,
        AFContextConstructor,
    },
    enums::{
        AFBackendLibrary,
        AFPowerPreference,
    },

    implementation::{
        AFWindow,
        AFContext,
    },
};

pub fn main(){
    let size: AFSize2D = AFSize2D {
        width: 1280, height: 720
    };
    let window_config: AFWindowConfig = AFWindowConfig{
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
    let window: AFWindow = AFWindow::new(&window_config);

    let context_config: AFContextConfig = AFContextConfig {
        vsync: true,
        anisotropic_filtering: false,
        backend_lib: AFBackendLibrary::Metal,
        power_preference: AFPowerPreference::LowPower,
    };
}
