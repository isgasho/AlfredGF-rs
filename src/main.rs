mod alfredgf;

use wgpu::{
    PowerPreference,
    BindGroupLayoutBinding,
    Binding,
    BindingResource,
    ShaderStage,
    BindingType,
    Buffer,
    BufferUsage,
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

    // making a buffer
//    let test: Buffer = alfredgf::create_buffer(context, BufferUsage::STORAGE,
//                                               &[
//                                                   0, 1,
//                                                   1, 0,
//                                                   0, 0
//                                               ]);

    // TODO make the buffer creation be on AlfredGF's end
    // bind groups
//    let render_bind_group: alfredgf::AFBindGroup = alfredgf::AFBindGroup::new(
//        context,
//        &[BindGroupLayoutBinding {
//            binding: 0,
//            visibility: ShaderStage::VERTEX,
//            ty: BindingType::StorageBuffer {
//                dynamic: false,
//                readonly: false,
//            },
//        }],
//        &[Binding {
//            binding: 0,
//            resource: BindingResource::Buffer {
//                buffer: &test,
//                range: 0..4,
//            },
//        }],
//    );
}
