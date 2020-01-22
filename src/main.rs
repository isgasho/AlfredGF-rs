mod alfredgf;

use wgpu::{
    PowerPreference
};

fn main() {
    // window creation
    let window_config: alfredgf::AFWindowConfig = alfredgf::AFWindowConfig {
        icon: &alfredgf::AFImage { // stub image
            data: &[],
            width: 0,
            height: 0,
        },
        min_size: [1, 1],
        max_size: [1280, 720],
        start_size: [1280, 720],
        title: "Test",
        resizeable: true,
        visible: true,
        always_on_top: false,
        maximized: false,
    };
    let window: &alfredgf::AFWindow = alfredgf::AFWindow::new(&window_config);

    // context
    let context_config: alfredgf::AFContextConfig = alfredgf::AFContextConfig {
        anisotropic_filtering: false,
        power_preference: PowerPreference::Default,
    };
    let context: &alfredgf::AFContext = alfredgf::AFContext::new(&window, &context_config);

    // shaders
    let vertex_data = include_bytes!("shader.vert.spv");
    let fragment_data = include_bytes!("shader.frag.spv");
    let entry_point = "main";
    let v_s: alfredgf::AFShaderModule =
        alfredgf::AFShaderModule::new_with_bytes(context, vertex_data, entry_point);
    let f_s: alfredgf::AFShaderModule =
        alfredgf::AFShaderModule::new_with_bytes(context, fragment_data, entry_point);
}
