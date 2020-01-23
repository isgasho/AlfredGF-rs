mod alfredgf;

use wgpu::{
    BindGroupLayoutBinding, Binding, BindingResource, BindingType, Buffer, BufferUsage,
    PowerPreference, ShaderStage, PrimitiveTopology, BlendDescriptor, IndexFormat,
    VertexBufferDescriptor, InputStepMode, VertexAttributeDescriptor, FrontFace, CullMode,
};
use crate::alfredgf::AFRenderPipelineConfig;

fn main() {
    // window creation
    let window_config: alfredgf::AFWindowConfig = alfredgf::AFWindowConfig {
        icon: &alfredgf::AFImage {
            // stub image
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

    let render_pipeline: alfredgf::AFRenderPipeline = alfredgf::AFRenderPipeline::new
        (context, &AFRenderPipelineConfig{
            uniforms: &[],
            vertex_buffer_slots: &[],
            vertex_shader: &v_s,
            fragment_shader: &f_s,
            index_format: IndexFormat::Uint16,
            primitive_topology: PrimitiveTopology::TriangleList,
            front_face: FrontFace::Cw,
            cull_mode: CullMode::None,
            colour_blend: BlendDescriptor::REPLACE,
            alpha_blend: BlendDescriptor::REPLACE,
        });
}
