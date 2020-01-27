mod alfredgf;

use wgpu::{
    BindGroupLayoutBinding, Binding, BindingResource, BindingType, BlendDescriptor, Buffer,
    BufferUsage, CullMode, FrontFace, IndexFormat, InputStepMode, PowerPreference,
    PrimitiveTopology, ShaderStage, VertexAttributeDescriptor, VertexBufferDescriptor,
    VertexFormat,
};

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
    let window: alfredgf::AFWindow = alfredgf::AFWindow::new(&window_config);

    // context
    let context_config: alfredgf::AFContextConfig = alfredgf::AFContextConfig {
        anisotropic_filtering: false,
        power_preference: PowerPreference::Default,
        vsync: true,
        size: [1280, 720],
    };
    let context: &mut alfredgf::AFContext = alfredgf::AFContext::new(&window, &context_config);

    // shaders
    let vertex_data = include_bytes!("shader.vert.spv");
    let fragment_data = include_bytes!("shader.frag.spv");
    let entry_point = "main";
    let v_s: alfredgf::AFShaderModule =
        alfredgf::AFShaderModule::new_with_bytes(context, vertex_data, entry_point);
    let f_s: alfredgf::AFShaderModule =
        alfredgf::AFShaderModule::new_with_bytes(context, fragment_data, entry_point);

    // pipeline
    let render_pipeline: alfredgf::AFRenderPipeline = alfredgf::AFRenderPipeline::new(
        context,
        &alfredgf::AFRenderPipelineConfig {
            uniforms: &[&alfredgf::AFUniform {
                id: 0,
                stage: ShaderStage::VERTEX,
                dynamic: false,
                byte_size: 4, // 4 byte float
            }],
            vertex_buffer_slots: &[&alfredgf::AFVertexBufferSlot {
                stride: 0,
                step_mode: InputStepMode::Vertex,
                attribs: &[&alfredgf::AFVertexAttrib {
                    location: 0,
                    offset: 0,
                    format: VertexFormat::Float4,
                }],
            }],
            vertex_shader: &v_s,
            fragment_shader: &f_s,
            index_format: IndexFormat::Uint16,
            primitive_topology: PrimitiveTopology::TriangleList,
            front_face: FrontFace::Cw,
            cull_mode: CullMode::None,
            colour_blend: BlendDescriptor::REPLACE,
            alpha_blend: BlendDescriptor::REPLACE,
        },
    );

    alfredgf::mainloop(context, window, &[render_pipeline], |state|{
        alfredgf::AFMainloop{
            destroy: state.close_requested,
            update_surface: state.was_resized,
        }
    }, ||{
        //
    });
}
