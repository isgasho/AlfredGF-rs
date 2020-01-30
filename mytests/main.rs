extern crate AlfredGF_rs;

use AlfredGF_rs::{
    util_structs::*,
    config_structs::*,
    constructors::*,
    enums::*,
    implementation::*,
};
use AlfredGF_rs::implementation::AFShaderModule;
use AlfredGF_rs::config_structs::AFShaderConfig;

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
        backend_lib: {
            #[cfg(target_os = "macos")]{
                AFBackendLibrary::Metal
            }
            #[cfg(target_os = "ios")]{
                AFBackendLibrary::Metal
            }
            #[cfg(target_os = "android")]{
                AFBackendLibrary::OpenGL
            }
            #[cfg(target_os = "windows")]{
                AFBackendLibrary::Vulkan
            }
            #[cfg(target_os = "linux")]{
                AFBackendLibrary::Vulkan
            }
        },
        power_preference: AFPowerPreference::LowPower,
    };

    let context: AFContext = AFContext::new(window, &context_config);

    let v_s_c: AFShaderConfig = AFShaderConfig {
        stage: AFShaderStage::Vertex,
        bytecode: include_bytes!("shader.vert.spv"),
        entry_point: "main"
    };
    let f_s_c: AFShaderConfig = AFShaderConfig {
        stage: AFShaderStage::Fragment,
        bytecode: include_bytes!("shader.frag.spv"),
        entry_point: "main"
    };
    let vertex_shader: AFShaderModule = AFShaderModule::new(&context, &v_s_c);
    let fragment_shader: AFShaderModule = AFShaderModule::new(&context, &f_s_c);
}
