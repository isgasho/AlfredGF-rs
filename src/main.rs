mod alfredgf;

use wgpu::{
    PowerPreference
};

fn main() {
    let window_config: alfredgf::AFWindowConfig = alfredgf::AFWindowConfig {
        icon: &alfredgf::AFImage {
            data: &[],
            width: 0,
            height: 0,
        },
        min_size: [1280, 720],
        max_size: [1280, 720],
        start_size: [1280, 720],
        title: "Test",
        resizeable: true,
        visible: true,
        always_on_top: false,
        maximized: false,
    };
    let window: &alfredgf::AFWindow = alfredgf::AFWindow::new(&window_config);

    let context_config: alfredgf::AFContextConfig = alfredgf::AFContextConfig {
        anisotropic_filtering: false,
        power_preference: PowerPreference::Default,
    };
    let context: &alfredgf::AFContext = alfredgf::AFContext::new(&window, &context_config);
}
